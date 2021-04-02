use crate::search::Search;
use eval::eval::{Eval, Score, CHECKMATE_WHITE, EQUAL_POSITION};
use movegen::move_generator::MoveGenerator;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use movegen::side::Side;

pub struct Negamax;

impl Search for Negamax {
    fn search(pos_history: &mut PositionHistory, depth: usize) -> (Score, MoveList) {
        let mut move_list_stack = vec![MoveList::new(); depth];

        let pv_size = Self::sum_numbers(depth);
        let mut principal_variation = MoveList::with_capacity(pv_size);
        principal_variation.resize(pv_size, Move::NULL);
        let eval = match pos_history.current_pos().side_to_move() {
            Side::White => Self::search_recursive(
                &mut move_list_stack,
                &mut principal_variation,
                pos_history,
                depth,
            ),
            Side::Black => -Self::search_recursive(
                &mut move_list_stack,
                &mut principal_variation,
                pos_history,
                depth,
            ),
        };

        principal_variation.truncate(depth);
        principal_variation.truncate_at_null_move();
        (eval, principal_variation)
    }
}

impl Negamax {
    fn search_recursive(
        move_list_stack: &mut Vec<MoveList>,
        principal_variation: &mut MoveList,
        pos_history: &mut PositionHistory,
        depth: usize,
    ) -> Score {
        let mut max = CHECKMATE_WHITE;

        let pos = pos_history.current_pos();
        match depth {
            0 => max = Eval::eval_relative(pos),
            _ => {
                debug_assert!(!move_list_stack.is_empty());
                let mut move_list = move_list_stack.pop().unwrap();
                MoveGenerator::generate_moves(&mut move_list, pos);
                if move_list.is_empty() {
                    max = if pos.is_in_check(pos.side_to_move()) {
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
                        );
                        pos_history.undo_last_move();

                        if new_score > max {
                            max = new_score;
                            let best_move = *m;
                            Self::update_pv(principal_variation, depth, best_move);
                        }
                    }
                }
                move_list_stack.push(move_list);
            }
        }
        max
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
    use eval::eval::CHECKMATE_BLACK;
    use movegen::piece;
    use movegen::position::Position;
    use movegen::r#move::MoveType;
    use movegen::square::Square;

    #[test]
    fn search() {
        let mut pos_history = PositionHistory::new(Position::initial());

        for depth in 0..=3 {
            let (score, pv) = Negamax::search(&mut pos_history, depth);

            for m in pv.iter() {
                pos_history.do_move(*m);
            }
            assert_eq!(Eval::eval(pos_history.current_pos()), score);
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
        let (score, pv) = Negamax::search(&mut pos_history, depth);
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
        let (score, pv) = Negamax::search(&mut pos_history, depth);
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
        let (score, pv) = Negamax::search(&mut pos_history, depth);
        assert_eq!(EQUAL_POSITION, score);
        assert_eq!(0, pv.len());
    }
}
