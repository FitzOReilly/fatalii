use crate::alpha_beta_entry::{AlphaBetaEntry, ScoreType};
use crate::move_selector::MoveSelector;
use crate::search::{
    Search, SearchCommand, SearchInfo, SearchResult, MAX_SEARCH_DEPTH,
    PLIES_WITHOUT_PAWN_MOVE_OR_CAPTURE_TO_DRAW, REPETITIONS_TO_DRAW,
};
use crate::search_data::SearchData;
use crate::time_manager::TimeManager;
use crate::SearchOptions;
use crossbeam_channel::{Receiver, Sender};
use eval::{
    Eval, Score, CHECKMATE_BLACK, CHECKMATE_WHITE, EQUAL_POSITION, NEGATIVE_INF, POSITIVE_INF,
};
use movegen::move_generator::MoveGenerator;
use movegen::piece;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use movegen::side::Side;
use movegen::transposition_table::TranspositionTable;
use movegen::zobrist::Zobrist;
use std::cmp;
use std::time::Instant;

pub type AlphaBetaTable = TranspositionTable<Zobrist, AlphaBetaEntry>;

// Minimum depth for principal variation search. Disable null-window searches below this depth.
const MIN_PVS_DEPTH: usize = 3;

// Minimum depth for null move pruning.
const MIN_NULL_MOVE_PRUNE_DEPTH: usize = 3;

// Enable futility pruning if the evaluation plus this value is less than alpha.
const FUTILITY_MARGIN: Score = 120;

// Enable reverse futility pruning if the evaluation plus this value is greater than or equal to beta.
const REVERSE_FUTILITY_MARGIN: Score = 120;

// Alpha-beta search with fail-hard cutoffs
pub struct AlphaBeta {
    evaluator: Box<dyn Eval + Send>,
    transpos_table: AlphaBetaTable,
}

impl Search for AlphaBeta {
    fn set_hash_size(&mut self, bytes: usize) {
        debug_assert!(bytes <= u64::MAX as usize);
        debug_assert_ne!(0, AlphaBetaEntry::ENTRY_SIZE);
        let max_num_entries = (bytes / AlphaBetaEntry::ENTRY_SIZE) as u64;
        // The actual number of entries must be a power of 2.
        let index_bits = 64 - max_num_entries.leading_zeros() - 1;
        debug_assert!(index_bits > 0);
        self.transpos_table = AlphaBetaTable::new(index_bits as usize);
    }

    fn clear_hash_table(&mut self) {
        self.transpos_table.clear();
    }

    fn search(
        &mut self,
        pos_history: PositionHistory,
        search_options: SearchOptions,
        command_receiver: &Receiver<SearchCommand>,
        info_sender: &Sender<SearchInfo>,
    ) {
        let start_time = Instant::now();
        let hard_time_limit = TimeManager::calc_movetime_hard_limit(
            pos_history.current_pos().side_to_move(),
            &search_options,
        );
        let soft_time_limit = cmp::min(
            hard_time_limit,
            TimeManager::calc_movetime_soft_limit(
                pos_history.current_pos().side_to_move(),
                &search_options,
            ),
        );
        let mut search_data = SearchData::new(
            command_receiver,
            info_sender,
            pos_history,
            start_time,
            hard_time_limit,
            search_options.nodes,
        );

        for d in 1..=search_options.depth.unwrap_or(MAX_SEARCH_DEPTH) {
            search_data.increase_search_depth();

            if search_data.search_depth() > 1 {
                if let Ok(SearchCommand::Stop) = search_data.try_recv_cmd() {
                    break;
                }
                if let Some(limit) = soft_time_limit {
                    if search_data.start_time().elapsed() > limit {
                        break;
                    }
                }
            }

            match self.search_recursive(&mut search_data, d, NEGATIVE_INF, POSITIVE_INF) {
                Some(rel_alpha_beta_res) => {
                    let abs_alpha_beta_res =
                        match search_data.pos_history().current_pos().side_to_move() {
                            Side::White => rel_alpha_beta_res,
                            Side::Black => -rel_alpha_beta_res,
                        };
                    debug_assert_eq!(ScoreType::Exact, abs_alpha_beta_res.score_type());
                    let search_res = SearchResult::new(
                        d,
                        abs_alpha_beta_res.score(),
                        search_data.node_counter().sum_nodes(),
                        start_time.elapsed().as_micros() as u64,
                        self.transpos_table.load_factor_permille(),
                        abs_alpha_beta_res.best_move(),
                        search_data.pv_owned(d),
                    );
                    search_data.send_info(SearchInfo::DepthFinished(search_res));
                    if let CHECKMATE_WHITE | CHECKMATE_BLACK = abs_alpha_beta_res.score() {
                        break;
                    }
                }
                None => break,
            }
        }
        info_sender
            .send(SearchInfo::Stopped)
            .expect("Error sending SearchInfo");
    }
}

impl AlphaBeta {
    pub fn new(evaluator: Box<dyn Eval + Send>, table_idx_bits: usize) -> Self {
        assert!(table_idx_bits > 0);
        Self {
            evaluator,
            transpos_table: AlphaBetaTable::new(table_idx_bits),
        }
    }

    fn search_recursive(
        &mut self,
        search_data: &mut SearchData,
        depth: usize,
        mut alpha: Score,
        beta: Score,
    ) -> Option<AlphaBetaEntry> {
        if search_data.search_depth() > 1 {
            if let Ok(SearchCommand::Stop) = search_data.try_recv_cmd() {
                return None;
            }
            if let Some(limit) = search_data.hard_time_limit() {
                if search_data.start_time().elapsed() > limit {
                    return None;
                }
            }
            if let Some(max_nodes) = search_data.max_nodes() {
                if search_data.searched_nodes() >= max_nodes {
                    return None;
                }
            }
        }

        if let Some(entry) = Self::check_draw_by_rep(search_data, depth) {
            return Some(entry);
        }
        if let Some(entry) = Self::check_draw_by_moves(search_data, depth) {
            return Some(entry);
        }

        let pos_hash = search_data.pos_history().current_pos_hash();
        if let Some(entry) = self.lookup_table_entry(pos_hash, depth) {
            if let Some(bounded) = entry.bound_hard(alpha, beta) {
                search_data.increment_cache_hits(depth);
                match (bounded.score_type(), depth) {
                    (ScoreType::Exact, 0) => return Some(bounded),
                    (ScoreType::Exact, 1) => {
                        search_data
                            .pv_table_mut()
                            .update_move_and_truncate(depth, bounded.best_move());
                        return Some(bounded);
                    }
                    (ScoreType::Exact, _) => {
                        // For greater depths, we need to keep searching in order to obtain the PV
                    }
                    _ => return Some(bounded),
                }
            }
        }

        let pos = search_data.pos_history().current_pos();
        let mut score_type = ScoreType::UpperBound;
        let mut best_move = Move::NULL;

        match depth {
            0 => Some(self.search_quiescence(search_data, alpha, beta)),
            _ => {
                let mut move_list = MoveList::new();
                MoveGenerator::generate_moves(&mut move_list, pos);
                if move_list.is_empty() {
                    let score = if pos.is_in_check(pos.side_to_move()) {
                        CHECKMATE_WHITE
                    } else {
                        EQUAL_POSITION
                    };
                    let node = AlphaBetaEntry::new(depth, score, ScoreType::Exact, Move::NULL);
                    self.update_table(pos_hash, node);
                    search_data
                        .pv_table_mut()
                        .update_move_and_truncate(depth, Move::NULL);
                    search_data.end_pv();
                    Some(node)
                } else {
                    let pos = search_data.pos_history().current_pos();
                    if depth >= MIN_NULL_MOVE_PRUNE_DEPTH
                        && search_data.pv_depth() == 0
                        && !pos.is_in_check(pos.side_to_move())
                        && pos.has_minor_or_major_piece(pos.side_to_move())
                    {
                        search_data.increment_nodes(depth);
                        search_data.pos_history_mut().do_move(Move::NULL);
                        let reduced_depth = depth - Self::null_move_depth_reduction(depth) - 1;
                        let opt_neg_res =
                            self.search_recursive(search_data, reduced_depth, -beta, -alpha);
                        match opt_neg_res {
                            Some(neg_search_res) => {
                                let search_res = -neg_search_res;
                                let score = search_res.score();
                                if score >= beta {
                                    let node = AlphaBetaEntry::new(
                                        depth,
                                        beta,
                                        ScoreType::LowerBound,
                                        Move::NULL,
                                    );
                                    search_data.pos_history_mut().undo_last_move();
                                    return Some(node);
                                }
                            }
                            None => return None,
                        }
                        search_data.pos_history_mut().undo_last_move();
                    }

                    let mut futility_pruning = false;
                    // Futility pruning
                    let pos = search_data.pos_history().current_pos();
                    if depth == 1 && !pos.is_in_check(pos.side_to_move()) {
                        search_data.increment_eval_calls();
                        let pos = search_data.pos_history().current_pos();
                        let score = self.evaluator.eval_relative(pos);
                        if score - REVERSE_FUTILITY_MARGIN >= beta {
                            let node =
                                AlphaBetaEntry::new(depth, beta, ScoreType::LowerBound, Move::NULL);
                            return Some(node);
                        }
                        if score + FUTILITY_MARGIN < alpha {
                            futility_pruning = true;
                        }
                    }

                    let mut pvs_full_window = true;
                    let mut move_selector = MoveSelector::new();
                    while let Some(m) = move_selector.select_next_move(
                        search_data,
                        &mut self.transpos_table,
                        depth,
                        &mut move_list,
                    ) {
                        if futility_pruning && !m.is_capture() && !m.is_promotion() {
                            continue;
                        }

                        search_data.increment_nodes(depth);
                        search_data.pos_history_mut().do_move(m);

                        // Principal variation search
                        let opt_neg_res = if pvs_full_window || depth < MIN_PVS_DEPTH {
                            self.search_recursive(search_data, depth - 1, -beta, -alpha)
                        } else {
                            // Null window search
                            let onr =
                                self.search_recursive(search_data, depth - 1, -alpha - 1, -alpha);
                            match onr {
                                Some(nr) => {
                                    let search_res = -nr;
                                    let score = search_res.score();
                                    if score > alpha {
                                        // Re-search with full window
                                        self.search_recursive(search_data, depth - 1, -beta, -alpha)
                                    } else {
                                        onr
                                    }
                                }
                                None => return None,
                            }
                        };
                        search_data.pos_history_mut().undo_last_move();

                        match opt_neg_res {
                            Some(neg_search_res) => {
                                let search_res = -neg_search_res;
                                let score = search_res.score();
                                if score >= beta {
                                    let node =
                                        AlphaBetaEntry::new(depth, beta, ScoreType::LowerBound, m);
                                    self.update_table(pos_hash, node);
                                    if !m.is_capture() && !search_data.contains_killer(depth, m) {
                                        search_data.insert_killer(depth, m);
                                        let p = search_data
                                            .pos_history()
                                            .current_pos()
                                            .piece_at(m.origin())
                                            .expect("Expected a piece at move origin");
                                        search_data.prioritize_history(p, m.target(), depth);
                                    }
                                    return Some(node);
                                }
                                if score > alpha {
                                    alpha = score;
                                    pvs_full_window = false;
                                    score_type = ScoreType::Exact;
                                    best_move = m;
                                    search_data
                                        .pv_table_mut()
                                        .update_move_and_copy(depth, best_move);
                                }
                            }
                            None => return None,
                        }
                    }
                    debug_assert!(
                        score_type == ScoreType::Exact || score_type == ScoreType::UpperBound
                    );
                    let node = AlphaBetaEntry::new(depth, alpha, score_type, best_move);
                    self.update_table(pos_hash, node);
                    Some(node)
                }
            }
        }
    }

    fn search_quiescence(
        &mut self,
        search_data: &mut SearchData,
        mut alpha: Score,
        beta: Score,
    ) -> AlphaBetaEntry {
        let depth = 0;

        if let Some(entry) = Self::check_draw_by_rep(search_data, depth) {
            return entry;
        }
        if let Some(entry) = Self::check_draw_by_moves(search_data, depth) {
            return entry;
        }

        let pos_hash = search_data.pos_history().current_pos_hash();
        if let Some(entry) = self.lookup_table_entry(pos_hash, depth) {
            if let Some(bounded) = entry.bound_hard(alpha, beta) {
                search_data.increment_cache_hits(depth);
                return bounded;
            }
        }

        let pos = search_data.pos_history().current_pos();
        let is_in_check = pos.is_in_check(pos.side_to_move());
        if is_in_check {
            return self.search_quiescence_check(search_data, alpha, beta);
        }

        // We might be evaluating a stalemate here. This is ok for now because checking for legal
        // moves is expensive here.
        search_data.increment_eval_calls();
        let pos = search_data.pos_history().current_pos();
        let mut score = self.evaluator.eval_relative(pos);
        let mut score_type = ScoreType::UpperBound;
        let mut best_move = Move::NULL;

        if score >= beta {
            let node = AlphaBetaEntry::new(depth, beta, ScoreType::LowerBound, Move::NULL);
            self.update_table(pos_hash, node);
            return node;
        }
        if score > alpha {
            alpha = score;
            score_type = ScoreType::Exact;
        }

        let mut move_list = MoveList::new();
        MoveGenerator::generate_captures(&mut move_list, pos);
        // Ignore underpromotions
        move_list.retain(|m| !m.is_promotion() || m.promotion_piece() == Some(piece::Type::Queen));
        let mut move_selector = MoveSelector::new();
        while let Some(m) = move_selector.select_next_move_quiescence_capture(
            search_data,
            &mut self.transpos_table,
            &mut move_list,
        ) {
            search_data.increment_nodes(depth);
            search_data.pos_history_mut().do_move(m);
            let search_result = -self.search_quiescence(search_data, -beta, -alpha);
            score = search_result.score();
            search_data.pos_history_mut().undo_last_move();

            if score >= beta {
                let node = AlphaBetaEntry::new(depth, beta, ScoreType::LowerBound, m);
                self.update_table(pos_hash, node);
                return node;
            }
            if score > alpha {
                alpha = score;
                score_type = ScoreType::Exact;
                best_move = m;
            }
        }
        debug_assert!(score_type == ScoreType::Exact || score_type == ScoreType::UpperBound);
        let node = AlphaBetaEntry::new(depth, alpha, score_type, best_move);
        self.update_table(pos_hash, node);
        node
    }

    fn search_quiescence_check(
        &mut self,
        search_data: &mut SearchData,
        mut alpha: Score,
        beta: Score,
    ) -> AlphaBetaEntry {
        debug_assert!({
            let pos = search_data.pos_history().current_pos();
            pos.is_in_check(pos.side_to_move())
        });

        let depth = 0;
        let pos_hash = search_data.pos_history().current_pos_hash();

        let mut score;
        let mut score_type = ScoreType::UpperBound;
        let mut best_move = Move::NULL;

        let mut move_list = MoveList::new();
        let mut move_selector = MoveSelector::new();

        MoveGenerator::generate_moves(&mut move_list, search_data.pos_history().current_pos());
        if move_list.is_empty() {
            let node = AlphaBetaEntry::new(depth, CHECKMATE_WHITE, ScoreType::Exact, Move::NULL);
            self.update_table(pos_hash, node);
            return node;
        }
        while let Some(m) = move_selector.select_next_move(
            search_data,
            &mut self.transpos_table,
            depth,
            &mut move_list,
        ) {
            search_data.increment_nodes(depth);
            search_data.pos_history_mut().do_move(m);
            let search_result = -self.search_quiescence(search_data, -beta, -alpha);
            score = search_result.score();
            search_data.pos_history_mut().undo_last_move();

            if score >= beta {
                let node = AlphaBetaEntry::new(depth, beta, ScoreType::LowerBound, m);
                self.update_table(pos_hash, node);
                return node;
            }
            if score > alpha {
                alpha = score;
                score_type = ScoreType::Exact;
                best_move = m;
            }
        }

        debug_assert!(score_type == ScoreType::Exact || score_type == ScoreType::UpperBound);
        let node = AlphaBetaEntry::new(depth, alpha, score_type, best_move);
        self.update_table(pos_hash, node);
        node
    }

    fn update_table(&mut self, pos_hash: Zobrist, node: AlphaBetaEntry) {
        self.transpos_table.insert(pos_hash, node);
    }

    fn lookup_table_entry(&self, pos_hash: Zobrist, depth: usize) -> Option<&AlphaBetaEntry> {
        match self.transpos_table.get(&pos_hash) {
            Some(entry) if entry.depth() == depth => Some(entry),
            _ => None,
        }
    }

    fn check_draw_by_rep(search_data: &mut SearchData, depth: usize) -> Option<AlphaBetaEntry> {
        if search_data.pos_history().current_pos_repetitions() >= REPETITIONS_TO_DRAW {
            let entry = AlphaBetaEntry::new(depth, EQUAL_POSITION, ScoreType::Exact, Move::NULL);
            if depth > 0 {
                search_data
                    .pv_table_mut()
                    .update_move_and_truncate(depth, entry.best_move());
                search_data.end_pv();
            }
            return Some(entry);
        }
        None
    }

    fn check_draw_by_moves(search_data: &mut SearchData, depth: usize) -> Option<AlphaBetaEntry> {
        let pos = search_data.pos_history().current_pos();
        if pos.plies_since_pawn_move_or_capture() >= PLIES_WITHOUT_PAWN_MOVE_OR_CAPTURE_TO_DRAW {
            let mut score = EQUAL_POSITION;
            if pos.is_in_check(pos.side_to_move()) {
                let mut move_list = MoveList::new();
                MoveGenerator::generate_moves(&mut move_list, pos);
                if move_list.is_empty() {
                    score = CHECKMATE_WHITE;
                }
            }
            let entry = AlphaBetaEntry::new(depth, score, ScoreType::Exact, Move::NULL);
            if depth > 0 {
                search_data
                    .pv_table_mut()
                    .update_move_and_truncate(depth, entry.best_move());
                search_data.end_pv();
            }
            return Some(entry);
        }
        None
    }

    fn null_move_depth_reduction(depth: usize) -> usize {
        debug_assert!(depth >= MIN_NULL_MOVE_PRUNE_DEPTH);
        match depth {
            3 => 1,
            4 | 5 => 2,
            6 | 7 => 3,
            _ => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eval::material_mobility::MaterialMobility;

    #[test]
    fn set_hash_size() {
        let evaluator = Box::new(MaterialMobility::new());
        let mut searcher = AlphaBeta::new(evaluator, 1);
        let entry_size = AlphaBetaEntry::ENTRY_SIZE;

        searcher.set_hash_size(2 * entry_size);
        assert_eq!(2 * entry_size, searcher.transpos_table.reserved_memory());
        searcher.set_hash_size(4 * entry_size - 1);
        assert_eq!(2 * entry_size, searcher.transpos_table.reserved_memory());
        searcher.set_hash_size(4 * entry_size);
        assert_eq!(4 * entry_size, searcher.transpos_table.reserved_memory());
        searcher.set_hash_size(4 * entry_size + 1);
        assert_eq!(4 * entry_size, searcher.transpos_table.reserved_memory());
    }
}
