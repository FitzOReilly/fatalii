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
        SearchResult::new(node.score(), node.best_move())
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

#[cfg(test)]
mod tests {
    use super::*;
    use eval::eval::CHECKMATE_BLACK;
    use movegen::piece;
    use movegen::position::Position;
    use movegen::r#move::MoveType;
    use movegen::square::Square;

    const TABLE_IDX_BITS: usize = 16;

    #[test]
    #[ignore]
    fn search_result_independent_of_transposition_table_size() {
        // Expected: The search result should be the same for different table sizes. The
        // transposition table should only improve the performance of the search, but not the
        // evaluation or the best move.

        let max_table_idx_bits = TABLE_IDX_BITS;
        let min_depth = 1;
        let max_depth = 2;

        let mut negamax = Negamax::new(1);
        let mut pos_history = PositionHistory::new(Position::initial());
        let mut exp_search_result = Vec::new();
        for depth in min_depth..=max_depth {
            exp_search_result.push(negamax.search(&mut pos_history, depth));
        }

        for table_idx_bits in 2..=max_table_idx_bits {
            let mut negamax = Negamax::new(table_idx_bits);
            let mut pos_history = PositionHistory::new(Position::initial());

            for depth in min_depth..=max_depth {
                let act_search_result = negamax.search(&mut pos_history, depth);
                assert_eq!(exp_search_result[depth - 1], act_search_result);
            }
        }
    }

    #[test]
    fn checkmate_white() {
        let mut negamax = Negamax::new(TABLE_IDX_BITS);

        let mut pos_history = PositionHistory::new(Position::initial());
        pos_history.do_move(Move::new(Square::F2, Square::F3, MoveType::QUIET));
        pos_history.do_move(Move::new(Square::E7, Square::E6, MoveType::QUIET));
        pos_history.do_move(Move::new(
            Square::G2,
            Square::G4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));

        let depth = 2;
        let search_result = negamax.search(&mut pos_history, depth);
        assert_eq!(
            SearchResult::new(
                CHECKMATE_WHITE,
                Move::new(Square::D8, Square::H4, MoveType::QUIET)
            ),
            search_result
        );
    }

    #[test]
    fn checkmate_black() {
        let mut negamax = Negamax::new(TABLE_IDX_BITS);
        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::A1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Square::G7, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::H7, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::H8, Some(piece::Piece::BLACK_KING));
        pos.set_side_to_move(Side::White);
        let mut pos_history = PositionHistory::new(pos);

        let depth = 2;
        let search_result = negamax.search(&mut pos_history, depth);
        assert_eq!(
            SearchResult::new(
                CHECKMATE_BLACK,
                Move::new(Square::A1, Square::A8, MoveType::QUIET)
            ),
            search_result
        );
    }

    #[test]
    fn stalemate() {
        let mut negamax = Negamax::new(TABLE_IDX_BITS);
        let mut pos = Position::empty();
        pos.set_piece_at(Square::E6, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::E7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_side_to_move(Side::Black);
        let mut pos_history = PositionHistory::new(pos);

        let depth = 1;
        let search_result = negamax.search(&mut pos_history, depth);
        assert_eq!(SearchResult::new(EQUAL_POSITION, Move::NULL), search_result);
    }

    #[test]
    fn search_quiescence() {
        // Expected: If there are no captures, the quiescence search equals the static evaluation
        let mut negamax = Negamax::new(TABLE_IDX_BITS);
        let mut pos_history = PositionHistory::new(Position::initial());

        pos_history.do_move(Move::new(
            Square::E2,
            Square::E4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));
        assert_eq!(
            Eval::eval(pos_history.current_pos()),
            -negamax.search_quiescence(&mut pos_history).score()
        );

        pos_history.do_move(Move::new(
            Square::C7,
            Square::C5,
            MoveType::DOUBLE_PAWN_PUSH,
        ));
        assert_eq!(
            Eval::eval(pos_history.current_pos()),
            negamax.search_quiescence(&mut pos_history).score()
        );

        pos_history.do_move(Move::new(Square::G1, Square::F3, MoveType::QUIET));
        assert_eq!(
            Eval::eval(pos_history.current_pos()),
            -negamax.search_quiescence(&mut pos_history).score()
        );

        pos_history.do_move(Move::new(Square::D7, Square::D6, MoveType::QUIET));
        assert_eq!(
            Eval::eval(pos_history.current_pos()),
            negamax.search_quiescence(&mut pos_history).score()
        );

        pos_history.do_move(Move::new(
            Square::D2,
            Square::D4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));

        pos_history.do_move(Move::new(Square::C5, Square::D4, MoveType::CAPTURE));
        let score_current = Eval::eval(pos_history.current_pos());

        // Position after 1. e4 c5 2. Nf3 d6 3. d4 cxd4.
        // The only possible captures are 4. Nxd4 and 4. Qxd4.
        // Expected: The quiescence search equals the maximum of the static evaluation of the
        // current position and all the captures
        pos_history.do_move(Move::new(Square::F3, Square::D4, MoveType::CAPTURE));
        let score_nxd4 = Eval::eval(pos_history.current_pos());
        pos_history.undo_last_move();
        pos_history.do_move(Move::new(Square::D1, Square::D4, MoveType::CAPTURE));
        let score_qxd4 = Eval::eval(pos_history.current_pos());
        pos_history.undo_last_move();
        let score = *[score_current, score_nxd4, score_qxd4]
            .iter()
            .max()
            .unwrap();
        assert_eq!(score, negamax.search_quiescence(&mut pos_history).score());

        pos_history.do_move(Move::new(Square::F3, Square::D4, MoveType::CAPTURE));
        assert_eq!(
            Eval::eval(pos_history.current_pos()),
            -negamax.search_quiescence(&mut pos_history).score()
        );
    }
}
