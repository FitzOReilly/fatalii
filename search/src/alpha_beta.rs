use crate::search::Search;
use eval::eval::{Eval, Score, CHECKMATE_BLACK, CHECKMATE_WHITE, EQUAL_POSITION};
use movegen::move_generator::MoveGenerator;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use movegen::side::Side;

pub struct AlphaBeta;

impl Search for AlphaBeta {
    fn search(pos_history: &mut PositionHistory, depth: usize) -> (Score, MoveList) {
        let mut move_list_stack = vec![MoveList::new(); depth];

        let pv_size = Self::sum_numbers(depth);
        let mut principal_variation = MoveList::with_capacity(pv_size);
        principal_variation.resize(pv_size, Move::NULL);
        let alpha = CHECKMATE_WHITE;
        let beta = CHECKMATE_BLACK;
        let eval = match pos_history.current_pos().side_to_move() {
            Side::White => Self::search_recursive(
                &mut move_list_stack,
                &mut principal_variation,
                pos_history,
                depth,
                alpha,
                beta,
            ),
            Side::Black => -Self::search_recursive(
                &mut move_list_stack,
                &mut principal_variation,
                pos_history,
                depth,
                -beta,
                -alpha,
            ),
        };

        principal_variation.truncate(depth);
        principal_variation.truncate_at_null_move();
        (eval, principal_variation)
    }
}

impl AlphaBeta {
    fn search_recursive(
        move_list_stack: &mut Vec<MoveList>,
        principal_variation: &mut MoveList,
        pos_history: &mut PositionHistory,
        depth: usize,
        mut alpha: Score,
        beta: Score,
    ) -> Score {
        let mut score = CHECKMATE_WHITE;

        let pos = pos_history.current_pos();
        match depth {
            0 => score = Self::search_quiescence(pos_history, alpha, beta),
            _ => {
                debug_assert!(!move_list_stack.is_empty());
                let mut move_list = move_list_stack.pop().unwrap();
                MoveGenerator::generate_moves(&mut move_list, pos);
                if move_list.is_empty() {
                    score = if pos.is_in_check(pos.side_to_move()) {
                        CHECKMATE_WHITE
                    } else {
                        EQUAL_POSITION
                    };

                    Self::update_pv(principal_variation, depth, Move::NULL);
                } else {
                    for m in move_list.iter() {
                        pos_history.do_move(*m);
                        let new_score = -Self::search_recursive(
                            move_list_stack,
                            principal_variation,
                            pos_history,
                            depth - 1,
                            -beta,
                            -alpha,
                        );
                        pos_history.undo_last_move();

                        if new_score >= beta {
                            if new_score == beta {
                                let best_move = *m;
                                Self::update_pv(principal_variation, depth, best_move);
                            }

                            score = beta;
                            break;
                        }
                        if new_score > alpha {
                            alpha = new_score;
                            score = new_score;
                            let best_move = *m;
                            Self::update_pv(principal_variation, depth, best_move);
                        }
                    }
                }
                move_list_stack.push(move_list);
            }
        }
        score
    }

    fn search_quiescence(
        pos_history: &mut PositionHistory,
        mut alpha: Score,
        beta: Score,
    ) -> Score {
        let pos = pos_history.current_pos();
        let mut score = Eval::eval_relative(pos);

        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }

        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves(&mut move_list, pos);
        if move_list.is_empty() {
            score = if pos.is_in_check(pos.side_to_move()) {
                CHECKMATE_WHITE
            } else {
                EQUAL_POSITION
            };
        } else {
            for m in move_list.iter().filter(|m| m.is_capture()) {
                pos_history.do_move(*m);
                score = -Self::search_quiescence(pos_history, -beta, -alpha);
                pos_history.undo_last_move();

                if score >= beta {
                    score = beta;
                    break;
                }
                if score > alpha {
                    alpha = score;
                }
            }
        }

        score
    }

    fn sum_numbers(n: usize) -> usize {
        n * (n + 1) / 2
    }

    fn update_pv(pv: &mut MoveList, depth: usize, m: Move) {
        let dist_from_end = Self::sum_numbers(depth);
        let idx = pv.len() - dist_from_end;
        pv[idx] = m;
        for i in 1..depth {
            pv[idx + i] = pv[idx + i + depth - 1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use movegen::piece;
    use movegen::position::Position;
    use movegen::r#move::MoveType;
    use movegen::square::Square;

    #[test]
    fn search() {
        let mut pos_history = PositionHistory::new(Position::initial());

        for depth in 0..=3 {
            let (score, pv) = AlphaBeta::search(&mut pos_history, depth);

            for m in pv.iter() {
                pos_history.do_move(*m);
            }
            match pos_history.current_pos().side_to_move() {
                Side::White => assert_eq!(
                    AlphaBeta::search_quiescence(
                        &mut pos_history,
                        CHECKMATE_WHITE,
                        CHECKMATE_BLACK
                    ),
                    score
                ),
                Side::Black => assert_eq!(
                    -AlphaBeta::search_quiescence(
                        &mut pos_history,
                        -CHECKMATE_BLACK,
                        -CHECKMATE_WHITE
                    ),
                    score
                ),
            }
            for _ in 0..depth {
                pos_history.undo_last_move();
            }
        }
    }

    #[test]
    fn checkmate_white() {
        let mut pos_history = PositionHistory::new(Position::initial());
        pos_history.do_move(Move::new(Square::F2, Square::F3, MoveType::QUIET));
        pos_history.do_move(Move::new(Square::E7, Square::E6, MoveType::QUIET));
        pos_history.do_move(Move::new(
            Square::G2,
            Square::G4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));

        let depth = 2;
        let (score, pv) = AlphaBeta::search(&mut pos_history, depth);
        assert_eq!(CHECKMATE_WHITE, score);
        assert_eq!(1, pv.len());
        assert_eq!(Move::new(Square::D8, Square::H4, MoveType::QUIET), pv[0]);
    }

    #[test]
    fn checkmate_black() {
        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::A1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Square::G7, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::H7, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::H8, Some(piece::Piece::BLACK_KING));
        pos.set_side_to_move(Side::White);
        let mut pos_history = PositionHistory::new(pos);

        let depth = 2;
        let (score, pv) = AlphaBeta::search(&mut pos_history, depth);
        assert_eq!(CHECKMATE_BLACK, score);
        assert_eq!(1, pv.len());
        assert_eq!(Move::new(Square::A1, Square::A8, MoveType::QUIET), pv[0]);
    }

    #[test]
    fn stalemate() {
        let mut pos = Position::empty();
        pos.set_piece_at(Square::E6, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::E7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_side_to_move(Side::Black);
        let mut pos_history = PositionHistory::new(pos);

        let depth = 1;
        let (score, pv) = AlphaBeta::search(&mut pos_history, depth);
        assert_eq!(EQUAL_POSITION, score);
        assert_eq!(0, pv.len());
    }

    #[test]
    fn search_quiescence() {
        let mut pos_history = PositionHistory::new(Position::initial());

        // Expected: If there are no captures, the quiescence search equals the static evaluation
        pos_history.do_move(Move::new(
            Square::E2,
            Square::E4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));
        assert_eq!(
            Eval::eval(pos_history.current_pos()),
            -AlphaBeta::search_quiescence(&mut pos_history, -CHECKMATE_BLACK, -CHECKMATE_WHITE)
        );

        pos_history.do_move(Move::new(
            Square::C7,
            Square::C5,
            MoveType::DOUBLE_PAWN_PUSH,
        ));
        assert_eq!(
            Eval::eval(pos_history.current_pos()),
            AlphaBeta::search_quiescence(&mut pos_history, CHECKMATE_WHITE, CHECKMATE_BLACK)
        );

        pos_history.do_move(Move::new(Square::G1, Square::F3, MoveType::QUIET));
        assert_eq!(
            Eval::eval(pos_history.current_pos()),
            -AlphaBeta::search_quiescence(&mut pos_history, -CHECKMATE_BLACK, -CHECKMATE_WHITE)
        );

        pos_history.do_move(Move::new(Square::D7, Square::D6, MoveType::QUIET));
        assert_eq!(
            Eval::eval(pos_history.current_pos()),
            AlphaBeta::search_quiescence(&mut pos_history, CHECKMATE_WHITE, CHECKMATE_BLACK)
        );

        pos_history.do_move(Move::new(
            Square::D2,
            Square::D4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));

        // Expected: The quiescence search equals the maximum of the static evaluation of the
        // current position and all the captures
        pos_history.do_move(Move::new(Square::C5, Square::D4, MoveType::CAPTURE));
        let score_current = Eval::eval(pos_history.current_pos());
        pos_history.do_move(Move::new(Square::F3, Square::D4, MoveType::CAPTURE));
        let score_nxd4 = Eval::eval(pos_history.current_pos());
        pos_history.undo_last_move();
        pos_history.do_move(Move::new(Square::D1, Square::D4, MoveType::CAPTURE));
        let score_qxd4 = Eval::eval(pos_history.current_pos());
        pos_history.undo_last_move();
        let score = [score_current, score_nxd4, score_qxd4]
            .iter()
            .max()
            .unwrap()
            .clone();
        assert_eq!(
            score,
            AlphaBeta::search_quiescence(&mut pos_history, CHECKMATE_WHITE, CHECKMATE_BLACK)
        );

        pos_history.do_move(Move::new(Square::F3, Square::D4, MoveType::CAPTURE));
        assert_eq!(
            Eval::eval(pos_history.current_pos()),
            -AlphaBeta::search_quiescence(&mut pos_history, -CHECKMATE_BLACK, -CHECKMATE_WHITE)
        );
    }
}
