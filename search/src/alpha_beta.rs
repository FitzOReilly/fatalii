use crate::node_counter::NodeCounter;
use crate::pv_table::PvTable;
use crate::search::{Search, SearchCommand, SearchInfo, SearchResult, MAX_SEARCH_DEPTH};
use crossbeam_channel::{Receiver, Sender};
use eval::eval::{
    Eval, Score, CHECKMATE_BLACK, CHECKMATE_WHITE, EQUAL_POSITION, NEGATIVE_INF, POSITIVE_INF,
};
use movegen::move_generator::MoveGenerator;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use movegen::side::Side;
use movegen::transposition_table::TranspositionTable;
use movegen::zobrist::Zobrist;
use std::mem;
use std::ops::Neg;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum ScoreType {
    Exact = 0,
    LowerBound = 1,
    UpperBound = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AlphaBetaTableEntry {
    depth: u8,
    score: Score,
    score_type: ScoreType,
    best_move: Move,
}

impl AlphaBetaTableEntry {
    fn new(depth: usize, score: Score, score_type: ScoreType, best_move: Move) -> Self {
        debug_assert!(depth <= MAX_SEARCH_DEPTH);
        Self {
            depth: depth as u8,
            score,
            score_type,
            best_move,
        }
    }

    fn depth(&self) -> usize {
        self.depth as usize
    }

    fn score(&self) -> Score {
        self.score
    }

    fn score_type(&self) -> ScoreType {
        self.score_type
    }

    fn best_move(&self) -> Move {
        self.best_move
    }
}

impl Neg for AlphaBetaTableEntry {
    type Output = Self;

    // Changes the sign of the score and leaves the rest unchanged
    fn neg(self) -> Self::Output {
        Self::new(
            self.depth(),
            -self.score(),
            self.score_type(),
            self.best_move(),
        )
    }
}

// Alpha-beta search with fail-hard cutoffs
pub struct AlphaBeta {
    transpos_table: TranspositionTable<Zobrist, AlphaBetaTableEntry>,
    pv_table: PvTable,
    node_counter: NodeCounter,
    search_depth: usize,
    pv_depth: usize,
}

const HASH_ENTRY_SIZE: usize = mem::size_of::<Option<(Zobrist, AlphaBetaTableEntry)>>();

impl Search for AlphaBeta {
    fn set_hash_size(&mut self, bytes: usize) {
        debug_assert!(bytes <= u64::MAX as usize);
        debug_assert_ne!(0, HASH_ENTRY_SIZE);
        let max_num_entries = (bytes / HASH_ENTRY_SIZE) as u64;
        // The actual number of entries must be a power of 2.
        let index_bits = 64 - max_num_entries.leading_zeros() - 1;
        debug_assert!(index_bits > 0);
        self.transpos_table = TranspositionTable::new(index_bits as usize);
    }

    fn search(
        &mut self,
        pos_history: &mut PositionHistory,
        depth: usize,
        command_receiver: &mut Receiver<SearchCommand>,
        info_sender: &mut Sender<SearchInfo>,
    ) {
        self.node_counter.clear();
        for d in 1..=depth {
            self.search_depth = d;
            self.pv_depth = d - 1;

            if self.search_depth > 1 {
                if let Ok(SearchCommand::Stop) = command_receiver.try_recv() {
                    break;
                }
            }

            match self.search_recursive(
                pos_history,
                d,
                NEGATIVE_INF,
                POSITIVE_INF,
                command_receiver,
                info_sender,
            ) {
                Some(rel_alpha_beta_res) => {
                    let abs_alpha_beta_res = match pos_history.current_pos().side_to_move() {
                        Side::White => rel_alpha_beta_res,
                        Side::Black => -rel_alpha_beta_res,
                    };
                    debug_assert_eq!(ScoreType::Exact, abs_alpha_beta_res.score_type());
                    let search_res = SearchResult::new(
                        d,
                        abs_alpha_beta_res.score(),
                        self.node_counter.sum_nodes(),
                        abs_alpha_beta_res.best_move(),
                        self.principal_variation(d),
                    );
                    info_sender
                        .send(SearchInfo::DepthFinished(search_res))
                        .expect("Error sending SearchInfo");
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
    pub fn new(table_idx_bits: usize) -> Self {
        assert!(table_idx_bits > 0);
        Self {
            transpos_table: TranspositionTable::new(table_idx_bits),
            pv_table: PvTable::new(),
            node_counter: NodeCounter::new(),
            search_depth: 0,
            pv_depth: 0,
        }
    }

    fn principal_variation(&self, depth: usize) -> MoveList {
        self.pv_table.pv_into_movelist(depth)
    }

    fn search_recursive(
        &mut self,
        pos_history: &mut PositionHistory,
        depth: usize,
        mut alpha: Score,
        beta: Score,
        command_receiver: &mut Receiver<SearchCommand>,
        info_sender: &mut Sender<SearchInfo>,
    ) -> Option<AlphaBetaTableEntry> {
        if self.search_depth > 1 {
            if let Ok(SearchCommand::Stop) = command_receiver.try_recv() {
                return None;
            }
        }

        let pos = pos_history.current_pos();
        let pos_hash = pos_history.current_pos_hash();

        if let Some(entry) = self.lookup_table_entry(pos_hash, depth) {
            if let Some(bounded) = self.bound_hard(entry, alpha, beta) {
                self.node_counter
                    .increment_cache_hits(self.search_depth, depth);
                match (bounded.score_type(), depth) {
                    (ScoreType::Exact, 0) => return Some(bounded),
                    (ScoreType::Exact, 1) => {
                        self.pv_table
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

        let mut score_type = ScoreType::UpperBound;
        let mut best_move = Move::NULL;

        match depth {
            0 => Some(self.search_quiescence(pos_history, alpha, beta)),
            _ => {
                let mut move_list = MoveList::new();
                MoveGenerator::generate_moves(&mut move_list, pos);
                if move_list.is_empty() {
                    let score = if pos.is_in_check(pos.side_to_move()) {
                        CHECKMATE_WHITE
                    } else {
                        EQUAL_POSITION
                    };
                    let node = AlphaBetaTableEntry::new(depth, score, ScoreType::Exact, Move::NULL);
                    self.update_table(pos_hash, node);
                    self.pv_table.update_move_and_truncate(depth, Move::NULL);
                    Some(node)
                } else {
                    while let Some(m) = self.select_next_move(depth, &mut move_list) {
                        self.node_counter.increment_nodes(self.search_depth, depth);
                        pos_history.do_move(m);
                        let opt_neg_res = self.search_recursive(
                            pos_history,
                            depth - 1,
                            -beta,
                            -alpha,
                            command_receiver,
                            info_sender,
                        );
                        pos_history.undo_last_move();

                        match opt_neg_res {
                            Some(neg_search_res) => {
                                let search_res = -neg_search_res;
                                let score = search_res.score();
                                if score >= beta {
                                    let node = AlphaBetaTableEntry::new(
                                        depth,
                                        beta,
                                        ScoreType::LowerBound,
                                        m,
                                    );
                                    self.update_table(pos_hash, node);
                                    return Some(node);
                                }
                                if score > alpha {
                                    alpha = score;
                                    score_type = ScoreType::Exact;
                                    best_move = m;
                                    self.pv_table.update_move_and_copy(depth, best_move);
                                }
                            }
                            None => return None,
                        }
                    }
                    debug_assert!(
                        score_type == ScoreType::Exact || score_type == ScoreType::UpperBound
                    );
                    let node = AlphaBetaTableEntry::new(depth, alpha, score_type, best_move);
                    self.update_table(pos_hash, node);
                    Some(node)
                }
            }
        }
    }

    fn search_quiescence(
        &mut self,
        pos_history: &mut PositionHistory,
        mut alpha: Score,
        beta: Score,
    ) -> AlphaBetaTableEntry {
        let depth = 0;
        let pos = pos_history.current_pos();
        let pos_hash = pos_history.current_pos_hash();

        if let Some(entry) = self.lookup_table_entry(pos_hash, depth) {
            if let Some(bounded) = self.bound_hard(entry, alpha, beta) {
                self.node_counter
                    .increment_cache_hits(self.search_depth, depth);
                return bounded;
            }
        }

        self.node_counter.increment_eval_calls(self.search_depth);
        let mut score = Eval::eval_relative(pos);
        let mut score_type = ScoreType::UpperBound;
        let mut best_move = Move::NULL;

        if score >= beta {
            let node = AlphaBetaTableEntry::new(depth, beta, ScoreType::LowerBound, Move::NULL);
            self.update_table(pos_hash, node);
            return node;
        }
        if score > alpha {
            alpha = score;
            score_type = ScoreType::Exact;
        }

        let mut move_list = MoveList::new();
        MoveGenerator::generate_captures(&mut move_list, pos);
        for m in move_list.iter() {
            self.node_counter.increment_nodes(self.search_depth, depth);
            pos_history.do_move(*m);
            let search_result = -self.search_quiescence(pos_history, -beta, -alpha);
            score = search_result.score();
            pos_history.undo_last_move();

            if score >= beta {
                let node = AlphaBetaTableEntry::new(depth, beta, ScoreType::LowerBound, *m);
                self.update_table(pos_hash, node);
                return node;
            }
            if score > alpha {
                alpha = score;
                score_type = ScoreType::Exact;
                best_move = *m;
            }
        }
        debug_assert!(score_type == ScoreType::Exact || score_type == ScoreType::UpperBound);
        let node = AlphaBetaTableEntry::new(depth, alpha, score_type, best_move);
        self.update_table(pos_hash, node);
        node
    }

    fn update_table(&mut self, pos_hash: Zobrist, node: AlphaBetaTableEntry) {
        self.transpos_table.insert(pos_hash, node);
    }

    fn lookup_table_entry(&self, pos_hash: Zobrist, depth: usize) -> Option<&AlphaBetaTableEntry> {
        match self.transpos_table.get(&pos_hash) {
            Some(entry) if entry.depth() == depth => Some(entry),
            _ => None,
        }
    }

    fn bound_hard(
        &self,
        entry: &AlphaBetaTableEntry,
        alpha: Score,
        beta: Score,
    ) -> Option<AlphaBetaTableEntry> {
        match entry.score_type() {
            ScoreType::Exact => {
                if entry.score() >= beta {
                    Some(AlphaBetaTableEntry::new(
                        entry.depth(),
                        beta,
                        ScoreType::LowerBound,
                        Move::NULL,
                    ))
                } else if entry.score() < alpha {
                    Some(AlphaBetaTableEntry::new(
                        entry.depth(),
                        alpha,
                        ScoreType::UpperBound,
                        Move::NULL,
                    ))
                } else {
                    Some(*entry)
                }
            }
            ScoreType::LowerBound if entry.score() >= beta => Some(AlphaBetaTableEntry::new(
                entry.depth(),
                beta,
                ScoreType::LowerBound,
                Move::NULL,
            )),
            ScoreType::UpperBound if entry.score() < alpha => Some(AlphaBetaTableEntry::new(
                entry.depth(),
                alpha,
                ScoreType::UpperBound,
                Move::NULL,
            )),
            _ => None,
        }
    }

    fn select_next_move(&mut self, depth: usize, move_list: &mut MoveList) -> Option<Move> {
        if self.pv_depth > 0 {
            debug_assert_eq!(self.pv_depth, depth - 1);
            self.pv_depth -= 1;
            // Select the PV move from the previous iteration
            let prev_pv = self.pv_table.pv(self.search_depth - 1);
            let pv_move = prev_pv[self.search_depth - depth];
            let idx = move_list
                .iter()
                .position(|&x| x == pv_move)
                .unwrap_or_else(|| {
                    panic!(
                        "\nPV move not found in move list\n\
                        Search depth: {}\nDepth: {}\nMove list: {}\nPV move: {}\nPV table:\n{}",
                        self.search_depth, depth, move_list, pv_move, self.pv_table
                    )
                });
            Some(move_list.swap_remove(idx))
        } else {
            move_list.pop()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_hash_size() {
        let mut searcher = AlphaBeta::new(1);

        searcher.set_hash_size(2 * HASH_ENTRY_SIZE);
        assert_eq!(
            2 * HASH_ENTRY_SIZE,
            searcher.transpos_table.reserved_memory()
        );
        searcher.set_hash_size(4 * HASH_ENTRY_SIZE - 1);
        assert_eq!(
            2 * HASH_ENTRY_SIZE,
            searcher.transpos_table.reserved_memory()
        );
        searcher.set_hash_size(4 * HASH_ENTRY_SIZE);
        assert_eq!(
            4 * HASH_ENTRY_SIZE,
            searcher.transpos_table.reserved_memory()
        );
        searcher.set_hash_size(4 * HASH_ENTRY_SIZE + 1);
        assert_eq!(
            4 * HASH_ENTRY_SIZE,
            searcher.transpos_table.reserved_memory()
        );
    }
}
