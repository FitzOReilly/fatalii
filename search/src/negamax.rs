use crate::search::{Search, SearchResult};
use eval::eval::{Eval, Score, CHECKMATE_WHITE, EQUAL_POSITION, NEGATIVE_INF};
use movegen::move_generator::MoveGenerator;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use movegen::side::Side;
use movegen::transposition_table::TranspositionTable;
use movegen::zobrist::Zobrist;
use std::cmp;
use std::ops::Neg;

#[derive(Clone, Copy, Debug)]
struct NegamaxTableEntry {
    depth: u8,
    score: Score,
    best_move: Move,
}

impl NegamaxTableEntry {
    fn new(depth: usize, score: Score, best_move: Move) -> Self {
        debug_assert!(depth < 256);
        Self {
            depth: depth as u8,
            score,
            best_move,
        }
    }

    fn depth(&self) -> usize {
        self.depth as usize
    }

    fn score(&self) -> Score {
        self.score
    }

    fn best_move(&self) -> Move {
        self.best_move
    }
}

impl Neg for NegamaxTableEntry {
    type Output = Self;

    // Changes the sign of the score and leaves the rest unchanged
    fn neg(self) -> Self::Output {
        Self::new(self.depth(), -self.score(), self.best_move())
    }
}

pub struct Negamax {
    transpos_table: TranspositionTable<Zobrist, NegamaxTableEntry>,
}

impl Search for Negamax {
    fn search(&mut self, pos_history: &mut PositionHistory, depth: usize) -> SearchResult {
        let node = match pos_history.current_pos().side_to_move() {
            Side::White => self.search_recursive(pos_history, depth),
            Side::Black => -self.search_recursive(pos_history, depth),
        };
        SearchResult::new(depth, node.score(), node.best_move())
    }
}

impl Negamax {
    pub fn new(table_idx_bits: usize) -> Self {
        assert!(table_idx_bits > 0);
        Self {
            transpos_table: TranspositionTable::new(table_idx_bits),
        }
    }

    fn search_recursive(
        &mut self,
        pos_history: &mut PositionHistory,
        depth: usize,
    ) -> NegamaxTableEntry {
        let pos = pos_history.current_pos();
        let pos_hash = pos_history.current_pos_hash();

        if let Some(entry) = self.lookup_table_entry(pos_hash, depth) {
            return *entry;
        }

        let mut best_score = NEGATIVE_INF;
        let mut best_move = Move::NULL;

        match depth {
            0 => self.search_quiescence(pos_history),
            _ => {
                let mut move_list = MoveList::new();
                MoveGenerator::generate_moves(&mut move_list, pos);
                if move_list.is_empty() {
                    let score = if pos.is_in_check(pos.side_to_move()) {
                        CHECKMATE_WHITE
                    } else {
                        EQUAL_POSITION
                    };
                    let node = NegamaxTableEntry::new(depth, score, Move::NULL);
                    self.update_table(pos_hash, node);
                    best_score = cmp::max(best_score, score);
                } else {
                    for m in move_list.iter() {
                        pos_history.do_move(*m);
                        let search_result = -self.search_recursive(pos_history, depth - 1);
                        let score = search_result.score();
                        pos_history.undo_last_move();

                        if score > best_score {
                            best_score = score;
                            best_move = *m;
                        }
                    }
                    let node = NegamaxTableEntry::new(depth, best_score, best_move);
                    self.update_table(pos_hash, node);
                }
                debug_assert!(self.transpos_table.get(&pos_hash).is_some());
                let node = NegamaxTableEntry::new(depth, best_score, best_move);
                self.update_table(pos_hash, node);
                node
            }
        }
    }

    fn search_quiescence(&mut self, pos_history: &mut PositionHistory) -> NegamaxTableEntry {
        let depth = 0;
        let pos = pos_history.current_pos();
        let pos_hash = pos_history.current_pos_hash();

        if let Some(entry) = self.lookup_table_entry(pos_hash, depth) {
            return *entry;
        }

        let mut score = Eval::eval_relative(pos);
        let mut best_score = score;
        let mut best_move = Move::NULL;

        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves(&mut move_list, pos);
        if move_list.is_empty() {
            score = if pos.is_in_check(pos.side_to_move()) {
                CHECKMATE_WHITE
            } else {
                EQUAL_POSITION
            };
            let node = NegamaxTableEntry::new(depth, score, Move::NULL);
            self.update_table(pos_hash, node);
            node
        } else {
            for m in move_list.iter().filter(|m| m.is_capture()) {
                pos_history.do_move(*m);
                let search_result = -self.search_quiescence(pos_history);
                score = search_result.score();
                pos_history.undo_last_move();

                if score > best_score {
                    best_score = score;
                    best_move = *m;
                }
            }
            let node = NegamaxTableEntry::new(depth, best_score, best_move);
            self.update_table(pos_hash, node);
            node
        }
    }

    fn update_table(&mut self, pos_hash: Zobrist, node: NegamaxTableEntry) {
        self.transpos_table.insert(pos_hash, node);
    }

    fn lookup_table_entry(&self, pos_hash: Zobrist, depth: usize) -> Option<&NegamaxTableEntry> {
        match self.transpos_table.get(&pos_hash) {
            Some(entry) if entry.depth() == depth => Some(entry),
            _ => None,
        }
    }
}
