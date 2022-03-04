use crate::alpha_beta_entry::{AlphaBetaEntry, ScoreType};
use crate::move_selector::MoveSelector;
use crate::search::{Search, SearchCommand, SearchInfo, SearchResult, REPETITIONS_TO_DRAW};
use crate::search_data::SearchData;
use crossbeam_channel::{Receiver, Sender};
use eval::{Score, CHECKMATE_BLACK, CHECKMATE_WHITE, EQUAL_POSITION, NEGATIVE_INF, POSITIVE_INF};
use movegen::move_generator::MoveGenerator;
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use movegen::side::Side;
use movegen::transposition_table::TranspositionTable;
use movegen::zobrist::Zobrist;
use std::time::Instant;

pub type AlphaBetaTable = TranspositionTable<Zobrist, AlphaBetaEntry>;

// Alpha-beta search with fail-hard cutoffs
pub struct AlphaBeta {
    eval_relative: fn(&Position) -> Score,
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

    fn search(
        &mut self,
        pos_history: PositionHistory,
        max_depth: usize,
        command_receiver: &Receiver<SearchCommand>,
        info_sender: &Sender<SearchInfo>,
    ) {
        let start_time = Instant::now();
        let mut search_data = SearchData::new(command_receiver, info_sender, pos_history);

        for d in 1..=max_depth {
            search_data.increase_search_depth();

            if search_data.search_depth() > 1 {
                if let Ok(SearchCommand::Stop) = search_data.try_recv_cmd() {
                    break;
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
    pub fn new(eval_relative: fn(&Position) -> Score, table_idx_bits: usize) -> Self {
        assert!(table_idx_bits > 0);
        Self {
            eval_relative,
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
        }

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
                    Some(node)
                } else {
                    while let Some(m) = MoveSelector::select_next_move(
                        search_data,
                        &mut self.transpos_table,
                        depth,
                        &mut move_list,
                    ) {
                        search_data.increment_nodes(depth);
                        search_data.pos_history_mut().do_move(m);
                        let opt_neg_res =
                            self.search_recursive(search_data, depth - 1, -beta, -alpha);
                        search_data.pos_history_mut().undo_last_move();

                        match opt_neg_res {
                            Some(neg_search_res) => {
                                let search_res = -neg_search_res;
                                let score = search_res.score();
                                if score >= beta {
                                    let node =
                                        AlphaBetaEntry::new(depth, beta, ScoreType::LowerBound, m);
                                    self.update_table(pos_hash, node);
                                    return Some(node);
                                }
                                if score > alpha {
                                    alpha = score;
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
        let pos_hash = search_data.pos_history().current_pos_hash();

        debug_assert!(search_data.pos_history().current_pos_repetitions() < REPETITIONS_TO_DRAW);
        if let Some(entry) = self.lookup_table_entry(pos_hash, depth) {
            if let Some(bounded) = entry.bound_hard(alpha, beta) {
                search_data.increment_cache_hits(depth);
                return bounded;
            }
        }

        search_data.increment_eval_calls();
        let pos = search_data.pos_history().current_pos();
        let mut score = (self.eval_relative)(pos);
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
        while let Some(m) = MoveSelector::select_next_move_quiescence(
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

    fn update_table(&mut self, pos_hash: Zobrist, node: AlphaBetaEntry) {
        self.transpos_table.insert(pos_hash, node);
    }

    fn lookup_table_entry(&self, pos_hash: Zobrist, depth: usize) -> Option<&AlphaBetaEntry> {
        match self.transpos_table.get(&pos_hash) {
            Some(entry) if entry.depth() == depth => Some(entry),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eval::material_mobility::MaterialMobility;
    use eval::Eval;

    const EVAL_RELATIVE: fn(pos: &Position) -> Score = MaterialMobility::eval_relative;

    #[test]
    fn set_hash_size() {
        let mut searcher = AlphaBeta::new(EVAL_RELATIVE, 1);
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
