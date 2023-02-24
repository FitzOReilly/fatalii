use crate::negamax_entry::NegamaxEntry;
use crate::search::{
    Search, SearchCommand, SearchInfo, SearchResult, MAX_SEARCH_DEPTH,
    PLIES_WITHOUT_PAWN_MOVE_OR_CAPTURE_TO_DRAW, REPETITIONS_TO_DRAW,
};
use crate::search_data::SearchData;
use crate::time_manager::TimeManager;
use crate::SearchOptions;
use crossbeam_channel::{Receiver, Sender};
use eval::{Eval, BLACK_WIN, EQ_POSITION, NEG_INF, WHITE_WIN};
use movegen::move_generator::MoveGenerator;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use movegen::side::Side;
use movegen::transposition_table::{TranspositionTable, TtEntry};
use movegen::zobrist::Zobrist;
use std::cmp;
use std::time::Instant;

type NegamaxTable = TranspositionTable<Zobrist, NegamaxEntry>;

pub struct Negamax {
    evaluator: Box<dyn Eval + Send>,
    transpos_table: NegamaxTable,
}

impl Search for Negamax {
    fn set_hash_size(&mut self, bytes: usize) {
        debug_assert!(bytes <= u64::MAX as usize);
        // Clear the old table before creating a new one to avoid reserving
        // memory for two potentially large tables
        self.transpos_table = NegamaxTable::new(0);
        self.transpos_table = NegamaxTable::new(bytes);
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
                if let Ok(SearchCommand::Stop) = command_receiver.try_recv() {
                    break;
                }
                if let Some(limit) = soft_time_limit {
                    if search_data.start_time().elapsed() > limit {
                        break;
                    }
                }
            }

            match self.search_recursive(&mut search_data, d) {
                Some(rel_negamax_res) => {
                    let abs_negamax_res =
                        match search_data.pos_history().current_pos().side_to_move() {
                            Side::White => rel_negamax_res,
                            Side::Black => -rel_negamax_res,
                        };
                    let search_res = SearchResult::new(
                        d,
                        abs_negamax_res.score(),
                        search_data.node_counter().sum_nodes(),
                        start_time.elapsed().as_micros() as u64,
                        self.transpos_table.load_factor_permille(),
                        abs_negamax_res.best_move(),
                        search_data.pv_owned(d),
                    );
                    info_sender
                        .send(SearchInfo::DepthFinished(search_res))
                        .expect("Error sending SearchInfo");
                    if let BLACK_WIN | WHITE_WIN = abs_negamax_res.score() {
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

impl Negamax {
    pub fn new(evaluator: Box<dyn Eval + Send>, table_size: usize) -> Self {
        Self {
            evaluator,
            transpos_table: NegamaxTable::new(table_size),
        }
    }

    fn search_recursive(
        &mut self,
        search_data: &mut SearchData,
        depth: usize,
    ) -> Option<NegamaxEntry> {
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

        if search_data.pos_history().current_pos_repetitions() >= REPETITIONS_TO_DRAW {
            let entry = NegamaxEntry::new(depth, EQ_POSITION, Move::NULL, search_data.age());
            if depth > 0 {
                search_data
                    .pv_table_mut()
                    .update_move_and_truncate(depth, entry.best_move());
            }
            return Some(entry);
        }

        let pos = search_data.pos_history().current_pos();
        if pos.plies_since_pawn_move_or_capture() >= PLIES_WITHOUT_PAWN_MOVE_OR_CAPTURE_TO_DRAW {
            let mut score = EQ_POSITION;
            if pos.is_in_check(pos.side_to_move()) {
                let mut move_list = MoveList::new();
                MoveGenerator::generate_moves(&mut move_list, pos);
                if move_list.is_empty() {
                    score = BLACK_WIN;
                }
            }
            let entry = NegamaxEntry::new(depth, score, Move::NULL, search_data.age());
            if depth > 0 {
                search_data
                    .pv_table_mut()
                    .update_move_and_truncate(depth, entry.best_move());
            }
            return Some(entry);
        }

        let pos_hash = search_data.pos_history().current_pos_hash();
        if let Some(entry) = self.lookup_table_entry(pos_hash, depth) {
            let e = *entry;
            search_data.increment_cache_hits(depth);
            match depth {
                0 => return Some(e),
                1 => {
                    search_data
                        .pv_table_mut()
                        .update_move_and_truncate(depth, e.best_move());
                    return Some(e);
                }
                _ => {
                    // For greater depths, we need to keep searching in order to obtain the PV
                }
            }
        }

        let mut best_score = NEG_INF;
        let mut best_move = Move::NULL;

        match depth {
            0 => Some(self.search_quiescence(search_data)),
            _ => {
                let pos = search_data.pos_history().current_pos();
                let mut move_list = MoveList::new();
                MoveGenerator::generate_moves(&mut move_list, pos);
                if move_list.is_empty() {
                    let score = if pos.is_in_check(pos.side_to_move()) {
                        BLACK_WIN
                    } else {
                        EQ_POSITION
                    };
                    let node = NegamaxEntry::new(depth, score, Move::NULL, search_data.age());
                    self.update_table(pos_hash, node);
                    search_data
                        .pv_table_mut()
                        .update_move_and_truncate(depth, Move::NULL);
                    Some(node)
                } else {
                    for m in move_list.iter() {
                        search_data.increment_nodes(depth);
                        search_data.pos_history_mut().do_move(*m);
                        let opt_neg_res = self.search_recursive(search_data, depth - 1);
                        search_data.pos_history_mut().undo_last_move();

                        match opt_neg_res {
                            Some(neg_search_res) => {
                                let search_result = -neg_search_res;
                                let score = eval::score::inc_mate_dist(search_result.score());
                                if score > best_score {
                                    best_score = score;
                                    best_move = *m;
                                    search_data
                                        .pv_table_mut()
                                        .update_move_and_copy(depth, best_move);
                                }
                            }
                            None => return None,
                        }
                    }
                    let node = NegamaxEntry::new(depth, best_score, best_move, search_data.age());
                    self.update_table(pos_hash, node);
                    Some(node)
                }
            }
        }
    }

    fn search_quiescence(&mut self, search_data: &mut SearchData) -> NegamaxEntry {
        let depth = 0;
        let pos_hash = search_data.pos_history().current_pos_hash();

        debug_assert!(search_data.pos_history().current_pos_repetitions() < REPETITIONS_TO_DRAW);
        debug_assert!(
            search_data
                .pos_history()
                .current_pos()
                .plies_since_pawn_move_or_capture()
                < PLIES_WITHOUT_PAWN_MOVE_OR_CAPTURE_TO_DRAW
        );

        if let Some(entry) = self.lookup_table_entry(pos_hash, depth) {
            let e = *entry;
            search_data.increment_cache_hits(depth);
            return e;
        }

        search_data.increment_eval_calls();
        let pos = search_data.pos_history().current_pos();
        let mut score = self.evaluator.eval_relative(pos);
        let mut best_score = score;
        let mut best_move = Move::NULL;

        let mut move_list = MoveList::new();
        MoveGenerator::generate_captures(&mut move_list, pos);
        for m in move_list.iter() {
            search_data.increment_nodes(depth);
            search_data.pos_history_mut().do_move(*m);
            let search_result = -self.search_quiescence(search_data);
            score = eval::score::inc_mate_dist(search_result.score());
            search_data.pos_history_mut().undo_last_move();

            if score > best_score {
                best_score = score;
                best_move = *m;
            }
        }
        let node = NegamaxEntry::new(depth, best_score, best_move, search_data.age());
        self.update_table(pos_hash, node);
        node
    }

    fn update_table(&mut self, pos_hash: Zobrist, node: NegamaxEntry) {
        self.transpos_table.insert(pos_hash, node);
    }

    fn lookup_table_entry(&self, pos_hash: Zobrist, depth: usize) -> Option<&NegamaxEntry> {
        match self.transpos_table.get(&pos_hash) {
            Some(entry) if entry.depth() == depth => Some(entry),
            _ => None,
        }
    }
}
