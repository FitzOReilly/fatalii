use crate::negamax_entry::NegamaxEntry;
use crate::search::{
    Search, SearchCommand, SearchInfo, SearchResult, MAX_SEARCH_DEPTH,
    PLIES_WITHOUT_PAWN_MOVE_OR_CAPTURE_TO_DRAW, REPETITIONS_TO_DRAW,
};
use crate::search_data::SearchData;
use crate::search_params::SearchParamsEachAlgo;
use crate::time_manager::TimeManager;
use crate::SearchOptions;
use crossbeam_channel::{Receiver, Sender};
use eval::{Eval, Score, BLACK_WIN, EQ_POSITION, NEG_INF, WHITE_WIN};
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

    fn set_params(&mut self, _params: SearchParamsEachAlgo) {}

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
        let mut best_move = Move::NULL;

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
                    let abs_negamax_res = match search_data.current_pos().side_to_move() {
                        Side::White => rel_negamax_res,
                        Side::Black => -rel_negamax_res,
                    };
                    let search_res = SearchResult::new(
                        d,
                        search_data.selective_depth(),
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
                    best_move = abs_negamax_res.best_move();
                    if let BLACK_WIN | WHITE_WIN = abs_negamax_res.score() {
                        break;
                    }
                }
                None => break,
            }
        }
        info_sender
            .send(SearchInfo::Stopped(best_move))
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
        if search_data.should_stop_search_immediately() {
            return None;
        }

        if search_data.pos_history().current_pos_repetitions() >= REPETITIONS_TO_DRAW {
            let entry = NegamaxEntry::new(depth, EQ_POSITION, Move::NULL, search_data.age());
            if depth > 0 {
                search_data.update_pv_move_and_truncate(entry.best_move());
            }
            return Some(entry);
        }

        let pos = search_data.current_pos();
        if pos.plies_since_pawn_move_or_capture() >= PLIES_WITHOUT_PAWN_MOVE_OR_CAPTURE_TO_DRAW {
            let mut score = EQ_POSITION;
            if search_data.is_in_check(pos.side_to_move()) {
                let mut move_list = MoveList::new();
                MoveGenerator::generate_moves(&mut move_list, search_data.current_pos());
                if move_list.is_empty() {
                    score = BLACK_WIN + search_data.ply() as Score;
                }
            }
            let entry = NegamaxEntry::new(depth, score, Move::NULL, search_data.age());
            if depth > 0 {
                search_data.update_pv_move_and_truncate(entry.best_move());
            }
            return Some(entry);
        }

        if let Some(entry) = self.lookup_table_entry(search_data, depth) {
            search_data.increment_cache_hits();
            match depth {
                0 => return Some(entry),
                1 => {
                    search_data.update_pv_move_and_truncate(entry.best_move());
                    return Some(entry);
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
                let pos = search_data.current_pos();
                let mut move_list = MoveList::new();
                MoveGenerator::generate_moves(&mut move_list, pos);
                if move_list.is_empty() {
                    let score = if search_data.is_in_check(pos.side_to_move()) {
                        BLACK_WIN + search_data.ply() as Score
                    } else {
                        EQ_POSITION
                    };
                    let node = NegamaxEntry::new(depth, score, Move::NULL, search_data.age());
                    self.update_table(search_data, node);
                    search_data.update_pv_move_and_truncate(Move::NULL);
                    Some(node)
                } else {
                    for m in move_list.iter() {
                        search_data.do_move(*m);
                        let opt_neg_res = self.search_recursive(search_data, depth - 1);
                        search_data.undo_last_move();

                        match opt_neg_res {
                            Some(neg_search_res) => {
                                let search_result = -neg_search_res;
                                let score = search_result.score();
                                if score > best_score {
                                    best_score = score;
                                    best_move = *m;
                                    search_data.update_pv_move_and_copy(best_move);
                                }
                            }
                            None => return None,
                        }
                    }
                    let node = NegamaxEntry::new(depth, best_score, best_move, search_data.age());
                    self.update_table(search_data, node);
                    Some(node)
                }
            }
        }
    }

    fn search_quiescence(&mut self, search_data: &mut SearchData) -> NegamaxEntry {
        let depth = 0;

        debug_assert!(search_data.pos_history().current_pos_repetitions() < REPETITIONS_TO_DRAW);
        debug_assert!(
            search_data.current_pos().plies_since_pawn_move_or_capture()
                < PLIES_WITHOUT_PAWN_MOVE_OR_CAPTURE_TO_DRAW
        );

        if let Some(entry) = self.lookup_table_entry(search_data, depth) {
            search_data.increment_cache_hits();
            return entry;
        }

        search_data.increment_eval_calls();
        let pos = search_data.current_pos();
        let mut score = self.evaluator.eval_relative(pos);
        let mut best_score = score;
        let mut best_move = Move::NULL;

        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves_quiescence(&mut move_list, pos);
        for m in move_list.iter() {
            search_data.do_move(*m);
            let search_result = -self.search_quiescence(search_data);
            score = search_result.score();
            search_data.undo_last_move();

            if score > best_score {
                best_score = score;
                best_move = *m;
            }
        }
        let node = NegamaxEntry::new(depth, best_score, best_move, search_data.age());
        self.update_table(search_data, node);
        node
    }

    fn update_table(&mut self, search_data: &SearchData<'_>, node: NegamaxEntry) {
        self.transpos_table.insert(
            search_data.current_pos_hash(),
            // Convert mate distance from the search root to the current position
            node.with_decreased_mate_distance(search_data.ply()),
        );
    }

    fn lookup_table_entry(
        &self,
        search_data: &SearchData<'_>,
        depth: usize,
    ) -> Option<NegamaxEntry> {
        match self
            .transpos_table
            .get_depth(&search_data.current_pos_hash(), depth)
        {
            Some(entry) if entry.depth() == depth => {
                // Convert mate distance from the current position to the search root
                Some(entry.with_increased_mate_distance(search_data.ply()))
            }
            _ => None,
        }
    }
}
