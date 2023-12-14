use crate::alpha_beta_entry::{AlphaBetaEntry, ScoreType};
use crate::aspiration_window::AspirationWindow;
use crate::move_selector::MoveSelector;
use crate::search::{
    Search, SearchCommand, SearchInfo, SearchResult, MAX_SEARCH_DEPTH,
    PLIES_WITHOUT_PAWN_MOVE_OR_CAPTURE_TO_DRAW, REPETITIONS_TO_DRAW,
};
use crate::search_data::SearchData;
use crate::time_manager::TimeManager;
use crate::SearchOptions;
use crossbeam_channel::{Receiver, Sender};
use eval::score::is_valid;
use eval::{Eval, Score, BLACK_WIN, EQ_POSITION, NEG_INF, WHITE_WIN};
use movegen::move_generator::MoveGenerator;
use movegen::piece;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use movegen::side::Side;
use movegen::transposition_table::{TranspositionTable, TtEntry};
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

// Prune a move if the static evaluation plus the move's potential improvement
// plus this value is less than alpha.
const DELTA_PRUNING_MARGIN_MOVE: Score = 200;

// Prune all moves if the static evaluation plus this value is less than alpha.
const DELTA_PRUNING_MARGIN_ALL_MOVES: Score = 1800;

// Alpha-beta search with fail-hard cutoffs
pub struct AlphaBeta {
    evaluator: Box<dyn Eval + Send>,
    transpos_table: AlphaBetaTable,
}

impl Search for AlphaBeta {
    fn set_hash_size(&mut self, bytes: usize) {
        debug_assert!(bytes <= u64::MAX as usize);
        // Clear the old table before creating a new one to avoid reserving
        // memory for two potentially large tables
        self.transpos_table = AlphaBetaTable::new(0);
        self.transpos_table = AlphaBetaTable::new(bytes);
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
        let side_to_move = pos_history.current_pos().side_to_move();
        let hard_time_limit = TimeManager::calc_movetime_hard_limit(side_to_move, &search_options);
        let mut soft_time_limit = cmp::min(
            hard_time_limit,
            TimeManager::calc_movetime_soft_limit(side_to_move, &search_options),
        );
        let has_time_limit = soft_time_limit.is_some();
        let mut search_data = SearchData::new(
            command_receiver,
            info_sender,
            pos_history,
            start_time,
            hard_time_limit,
            search_options.nodes,
        );

        let mut root_moves = MoveList::new();
        MoveGenerator::generate_moves(&mut root_moves, search_data.current_pos());
        search_data.set_root_moves(&root_moves);
        let mut aw = AspirationWindow::infinite();

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

            let mut stop_search = false;
            loop {
                match self.search_recursive(&mut search_data, d, aw.alpha(), aw.beta()) {
                    Some(rel_alpha_beta_res) => {
                        if rel_alpha_beta_res.score() <= aw.alpha() {
                            // Fail low
                            search_data.reset_current_search_depth();
                            aw.widen_down();
                            continue;
                        }
                        if rel_alpha_beta_res.score() >= aw.beta() {
                            // Fail high
                            search_data.reset_current_search_depth();
                            aw.widen_up();
                            continue;
                        }
                        aw = AspirationWindow::new(rel_alpha_beta_res.score());
                        let abs_alpha_beta_res = match search_data.current_pos().side_to_move() {
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
                        if has_time_limit && search_data.root_moves().move_list.len() == 1 {
                            // Move is forced, stop search after depth 1
                            stop_search = true;
                            break;
                        }
                        let score = abs_alpha_beta_res.score();
                        if eval::score::is_mating(score)
                            && eval::score::mate_dist(score).unsigned_abs() as usize <= d
                        {
                            if let Some(ref mut limit) = soft_time_limit {
                                // A mate has been found. Don't abort the search immediately, because we
                                // might have pruned away a shorter mate. Instead lower the search time.
                                // This also makes sure that we continue searching if there is no time
                                // limit given.
                                *limit = *limit * 3 / 4;
                            }
                        }
                        break;
                    }
                    None => {
                        stop_search = true;
                        break;
                    }
                }
            }
            if stop_search {
                break;
            }
        }
        info_sender
            .send(SearchInfo::Stopped)
            .expect("Error sending SearchInfo");
    }
}

impl AlphaBeta {
    pub fn new(evaluator: Box<dyn Eval + Send>, table_size: usize) -> Self {
        Self {
            evaluator,
            transpos_table: AlphaBetaTable::new(table_size),
        }
    }

    fn search_recursive(
        &mut self,
        search_data: &mut SearchData,
        depth: usize,
        alpha: Score,
        beta: Score,
    ) -> Option<AlphaBetaEntry> {
        if search_data.should_stop_search_immediately() {
            return None;
        }

        if let Some(entry) = Self::check_draw_by_rep(search_data, depth) {
            return Some(entry);
        }
        if let Some(entry) = Self::check_draw_by_moves(search_data, depth) {
            return Some(entry);
        }

        if let Some(entry) = self.usable_table_entry(search_data, depth, alpha, beta) {
            return Some(entry);
        }

        match depth {
            0 => Some(self.search_quiescence(search_data, alpha, beta)),
            _ => {
                let mut move_list = MoveList::new();
                MoveGenerator::generate_moves(&mut move_list, search_data.current_pos());
                let node = if move_list.is_empty() {
                    self.checkmate_or_stalemate(search_data, depth, alpha)
                } else {
                    match self.search_recursive_next_ply(search_data, depth, alpha, beta, move_list)
                    {
                        Some(n) => n,
                        None => return None,
                    }
                };
                // If the score is not valid, we need to widen the aspiration window.
                if is_valid(node.score()) {
                    self.update_table(search_data.current_pos_hash(), node);
                }
                Some(node)
            }
        }
    }

    fn search_recursive_next_ply(
        &mut self,
        search_data: &mut SearchData,
        depth: usize,
        mut alpha: Score,
        beta: Score,
        move_list: MoveList,
    ) -> Option<AlphaBetaEntry> {
        if let Some(opt_node) = self.prune_null_move(search_data, depth, alpha, beta) {
            return opt_node;
        }

        let mut futility_pruning = false;
        if let Some(node) =
            self.prune_futility(search_data, depth, alpha, beta, &mut futility_pruning)
        {
            return Some(node);
        }

        let mut score_type = ScoreType::UpperBound;
        let mut best_score = NEG_INF;
        let mut best_move = Move::NULL;

        let mut pvs_full_window = true;
        let mut move_selector = MoveSelector::new(move_list);
        let mut prev_node_count = search_data.node_counter().sum_nodes();
        search_data.reset_killers_next_ply();
        while let Some(m) =
            move_selector.select_next_move(search_data, &mut self.transpos_table, depth)
        {
            debug_assert!(if depth == search_data.search_depth() {
                prev_node_count == search_data.node_counter().sum_nodes()
            } else {
                true
            });

            if futility_pruning
                && !m.is_capture()
                && !m.is_promotion()
                && !search_data.pos_history_mut().gives_check(m)
            {
                debug_assert_ne!(depth, search_data.search_depth());
                continue;
            }

            search_data.do_move(m);
            let search_res = match self.principal_variation_search(
                search_data,
                depth,
                alpha,
                beta,
                pvs_full_window,
            ) {
                Some(node) => -node,
                None => return None,
            };
            search_data.undo_last_move();
            let score = eval::score::inc_mate_dist(search_res.score());

            if score >= beta {
                let node =
                    AlphaBetaEntry::new(depth, score, ScoreType::LowerBound, m, search_data.age());
                if !m.is_capture() {
                    search_data.insert_killer(m);
                    let last_move = search_data.pos_history().last_move().copied();
                    let last_moved_piece = search_data.pos_history().last_moved_piece();
                    if let (Some(lmp), Some(lm)) = (last_moved_piece, last_move) {
                        search_data.update_counter(lmp, lm.target(), m);
                    }
                    let piece_to_move = search_data
                        .current_pos()
                        .piece_at(m.origin())
                        .expect("Expected a piece at move origin");
                    search_data.prioritize_history(piece_to_move, m.target(), depth);
                }

                return Some(node);
            }
            if score > best_score {
                best_score = score;
                best_move = m;
                if score > alpha {
                    alpha = score;

                    pvs_full_window = false;
                    score_type = ScoreType::Exact;
                    if score == WHITE_WIN - 1 {
                        search_data
                            .pv_table_mut()
                            .update_move_and_truncate(depth, best_move);
                    } else {
                        search_data
                            .pv_table_mut()
                            .update_move_and_copy(depth, best_move);
                    }
                    // Root move ordering: move the new best move to the front
                    if depth == search_data.search_depth() {
                        search_data.move_to_front(best_move);
                    }
                }
            }
            if depth == search_data.search_depth() {
                let node_count = search_data.node_counter().sum_nodes();
                search_data.set_subtree_size(m, node_count - prev_node_count);
                prev_node_count = node_count;
            }
        }
        let node = AlphaBetaEntry::new(depth, best_score, score_type, best_move, search_data.age());
        debug_assert!(
            node.score_type() == ScoreType::Exact || node.score_type() == ScoreType::UpperBound
        );
        Some(node)
    }

    fn principal_variation_search(
        &mut self,
        search_data: &mut SearchData<'_>,
        depth: usize,
        alpha: i16,
        beta: i16,
        pvs_full_window: bool,
    ) -> Option<AlphaBetaEntry> {
        if pvs_full_window || depth < MIN_PVS_DEPTH {
            self.search_recursive(
                search_data,
                depth - 1,
                eval::score::dec_mate_dist(-beta),
                eval::score::dec_mate_dist(-alpha),
            )
        } else {
            // Null window search
            let mate_dist_alpha = eval::score::dec_mate_dist(alpha);
            let onr = self.search_recursive(
                search_data,
                depth - 1,
                -mate_dist_alpha - 1,
                -mate_dist_alpha,
            );
            match onr {
                Some(nr) => {
                    let search_res = -nr;
                    let score = eval::score::inc_mate_dist(search_res.score());
                    if score > alpha && score < beta {
                        // Re-search with full window
                        self.search_recursive(
                            search_data,
                            depth - 1,
                            eval::score::dec_mate_dist(-beta),
                            eval::score::dec_mate_dist(-alpha),
                        )
                    } else {
                        onr
                    }
                }
                None => None,
            }
        }
    }

    // Meaning of the outer and inner options:
    // - outer option: if None is returned, the condition for null move pruning
    //   is not fulfilled (score < beta)
    // - inner option: if Some(None) is returned, the search has been stopped
    //   (time limit, node limit, stop command)
    fn prune_null_move(
        &mut self,
        search_data: &mut SearchData<'_>,
        depth: usize,
        alpha: i16,
        beta: i16,
    ) -> Option<Option<AlphaBetaEntry>> {
        if depth >= MIN_NULL_MOVE_PRUNE_DEPTH
            && search_data.pv_depth() == 0
            && !search_data.is_in_check(search_data.current_pos().side_to_move())
            && search_data
                .current_pos()
                .has_minor_or_major_piece(search_data.current_pos().side_to_move())
        {
            search_data.do_move(Move::NULL);
            let reduced_depth = depth - Self::null_move_depth_reduction(depth) - 1;
            let opt_neg_res = self.search_recursive(
                search_data,
                reduced_depth,
                eval::score::dec_mate_dist(-beta),
                eval::score::dec_mate_dist(-alpha),
            );
            match opt_neg_res {
                Some(neg_search_res) => {
                    let search_res = -neg_search_res;
                    let score = eval::score::inc_mate_dist(search_res.score());
                    if score >= beta {
                        let node = AlphaBetaEntry::new(
                            depth,
                            score,
                            ScoreType::LowerBound,
                            Move::NULL,
                            search_data.age(),
                        );
                        search_data.undo_last_move();
                        return Some(Some(node));
                    }
                }
                None => return Some(None),
            }
            search_data.undo_last_move();
        }
        None
    }

    fn prune_futility(
        &mut self,
        search_data: &mut SearchData<'_>,
        depth: usize,
        alpha: i16,
        beta: i16,
        futility_pruning: &mut bool,
    ) -> Option<AlphaBetaEntry> {
        if depth == 1 && !search_data.is_in_check(search_data.current_pos().side_to_move()) {
            let score = search_data.eval_relative(&mut self.evaluator);
            if score - REVERSE_FUTILITY_MARGIN >= beta {
                let node = AlphaBetaEntry::new(
                    depth,
                    score,
                    ScoreType::LowerBound,
                    Move::NULL,
                    search_data.age(),
                );
                return Some(node);
            }
            if score + FUTILITY_MARGIN < alpha {
                *futility_pruning = true;
            }
        }
        None
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

        let pos_hash = search_data.current_pos_hash();
        if let Some(entry) = self.lookup_table_entry(pos_hash, depth) {
            if let Some(bounded) = entry.bound_soft(alpha, beta) {
                search_data.increment_cache_hits();
                return bounded;
            }
        }

        let is_in_check = search_data.is_in_check(search_data.current_pos().side_to_move());
        if is_in_check {
            return self.search_quiescence_check(search_data, alpha, beta);
        }

        // We might be evaluating a stalemate here. This is ok for now because checking for legal
        // moves is expensive here.
        let stand_pat = search_data.eval_relative(&mut self.evaluator);
        let mut score = stand_pat;
        let mut score_type = ScoreType::UpperBound;
        let mut best_score = stand_pat;
        let mut best_move = Move::NULL;

        if score >= beta {
            let node = AlphaBetaEntry::new(
                depth,
                score,
                ScoreType::LowerBound,
                Move::NULL,
                search_data.age(),
            );
            self.update_table(pos_hash, node);
            return node;
        }
        if score > alpha {
            alpha = score;
            score_type = ScoreType::Exact;
        }
        if stand_pat + DELTA_PRUNING_MARGIN_ALL_MOVES < alpha {
            debug_assert!(score_type == ScoreType::UpperBound);
            let node = AlphaBetaEntry::new(depth, alpha, score_type, best_move, search_data.age());
            self.update_table(pos_hash, node);
            return node;
        }

        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves_quiescence(&mut move_list, search_data.current_pos());
        let mut move_selector = MoveSelector::new(move_list);
        while let Some(m) =
            move_selector.select_next_move_quiescence_capture(search_data, &mut self.transpos_table)
        {
            let mut potential_improvement = if m.is_capture() {
                if m.is_en_passant() {
                    100
                } else {
                    match search_data
                        .current_pos()
                        .piece_at(m.target())
                        .expect("No piece on target square")
                        .piece_type()
                    {
                        piece::Type::Pawn => 100,
                        piece::Type::Knight => 300,
                        piece::Type::Bishop => 300,
                        piece::Type::Rook => 500,
                        piece::Type::Queen => 900,
                        piece::Type::King => panic!("Cannot capture king"),
                    }
                }
            } else {
                0
            };
            if m.is_promotion() {
                potential_improvement += 800;
            }
            if stand_pat + potential_improvement + DELTA_PRUNING_MARGIN_MOVE < alpha {
                continue;
            }

            search_data.do_move(m);
            let search_result = -self.search_quiescence(
                search_data,
                eval::score::dec_mate_dist(-beta),
                eval::score::dec_mate_dist(-alpha),
            );
            score = eval::score::inc_mate_dist(search_result.score());
            search_data.undo_last_move();

            if score >= beta {
                let node =
                    AlphaBetaEntry::new(depth, score, ScoreType::LowerBound, m, search_data.age());
                self.update_table(pos_hash, node);
                return node;
            }
            if score > best_score {
                best_score = score;
                best_move = m;
                if score > alpha {
                    alpha = score;
                    score_type = ScoreType::Exact;
                }
            }
        }
        debug_assert!(score_type == ScoreType::Exact || score_type == ScoreType::UpperBound);
        let node = AlphaBetaEntry::new(depth, best_score, score_type, best_move, search_data.age());
        self.update_table(pos_hash, node);
        node
    }

    fn search_quiescence_check(
        &mut self,
        search_data: &mut SearchData,
        mut alpha: Score,
        beta: Score,
    ) -> AlphaBetaEntry {
        debug_assert!(search_data.is_in_check(search_data.current_pos().side_to_move()));

        let depth = 0;
        let pos_hash = search_data.current_pos_hash();

        let mut score;
        let mut score_type = ScoreType::UpperBound;
        let mut best_score = NEG_INF;
        let mut best_move = Move::NULL;

        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves(&mut move_list, search_data.current_pos());

        if move_list.is_empty() {
            score = BLACK_WIN;
            if score > best_score {
                best_score = score;
                best_move = Move::NULL;
                if score > alpha {
                    score_type = ScoreType::Exact;
                }
            }
        } else {
            let mut move_selector = MoveSelector::new(move_list);
            while let Some(m) =
                move_selector.select_next_move(search_data, &mut self.transpos_table, depth)
            {
                search_data.do_move(m);
                let search_result = -self.search_quiescence(
                    search_data,
                    eval::score::dec_mate_dist(-beta),
                    eval::score::dec_mate_dist(-alpha),
                );
                score = eval::score::inc_mate_dist(search_result.score());
                search_data.undo_last_move();

                if score >= beta {
                    let node = AlphaBetaEntry::new(
                        depth,
                        score,
                        ScoreType::LowerBound,
                        m,
                        search_data.age(),
                    );
                    self.update_table(pos_hash, node);
                    return node;
                }
                if score > best_score {
                    best_score = score;
                    best_move = m;
                    if score > alpha {
                        alpha = score;
                        score_type = ScoreType::Exact;
                    }
                }
            }
        }

        debug_assert!(score_type == ScoreType::Exact || score_type == ScoreType::UpperBound);
        let node = AlphaBetaEntry::new(depth, best_score, score_type, best_move, search_data.age());
        self.update_table(pos_hash, node);
        node
    }

    fn update_table(&mut self, pos_hash: Zobrist, node: AlphaBetaEntry) {
        self.transpos_table.insert(pos_hash, node);
    }

    fn lookup_table_entry(&self, pos_hash: Zobrist, depth: usize) -> Option<&AlphaBetaEntry> {
        match self.transpos_table.get_depth(&pos_hash, depth) {
            Some(entry) if entry.depth() == depth => Some(entry),
            _ => None,
        }
    }

    fn usable_table_entry(
        &self,
        search_data: &mut SearchData<'_>,
        depth: usize,
        alpha: i16,
        beta: i16,
    ) -> Option<AlphaBetaEntry> {
        let pos_hash = search_data.current_pos_hash();
        if let Some(entry) = self.lookup_table_entry(pos_hash, depth) {
            if let Some(bounded) = entry.bound_soft(alpha, beta) {
                search_data.increment_cache_hits();
                match (bounded.score_type(), depth) {
                    (ScoreType::Exact, 0) => return Some(bounded),
                    (ScoreType::Exact, 1) => {
                        search_data
                            .pv_table_mut()
                            .update_move_and_truncate(depth, bounded.best_move());
                        // Root move ordering: move the new best move to the front
                        if depth == search_data.search_depth() {
                            search_data.move_to_front(bounded.best_move());
                        }
                        return Some(bounded);
                    }
                    (ScoreType::Exact, _) => {
                        // For greater depths, we need to keep searching in order to obtain the PV
                    }
                    _ => {
                        // We're not in a PV node, but it might be in the previous search depth's PV.
                        // So we make sure to remove it.
                        search_data.end_pv();
                        return Some(bounded);
                    }
                }
            }
        }
        None
    }

    fn check_draw_by_rep(search_data: &mut SearchData, depth: usize) -> Option<AlphaBetaEntry> {
        if search_data.pos_history().current_pos_repetitions() >= REPETITIONS_TO_DRAW {
            let entry = AlphaBetaEntry::new(
                depth,
                EQ_POSITION,
                ScoreType::Exact,
                Move::NULL,
                search_data.age(),
            );
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
        if search_data.current_pos().plies_since_pawn_move_or_capture()
            >= PLIES_WITHOUT_PAWN_MOVE_OR_CAPTURE_TO_DRAW
        {
            let mut score = EQ_POSITION;
            if search_data.is_in_check(search_data.current_pos().side_to_move()) {
                let mut move_list = MoveList::new();
                MoveGenerator::generate_moves(&mut move_list, search_data.current_pos());
                if move_list.is_empty() {
                    score = BLACK_WIN;
                }
            }
            let entry = AlphaBetaEntry::new(
                depth,
                score,
                ScoreType::Exact,
                Move::NULL,
                search_data.age(),
            );
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
        (depth / 2).min(4)
    }

    fn checkmate_or_stalemate(
        &self,
        search_data: &mut SearchData<'_>,
        depth: usize,
        alpha: i16,
    ) -> AlphaBetaEntry {
        let pos = search_data.current_pos();
        let mut score_type = ScoreType::UpperBound;
        let best_move = Move::NULL;
        let score = if search_data.is_in_check(pos.side_to_move()) {
            BLACK_WIN
        } else {
            EQ_POSITION
        };
        if score > alpha {
            score_type = ScoreType::Exact;
            search_data
                .pv_table_mut()
                .update_move_and_truncate(depth, best_move);
            search_data.end_pv();
        }
        AlphaBetaEntry::new(depth, score, score_type, best_move, search_data.age())
    }
}
