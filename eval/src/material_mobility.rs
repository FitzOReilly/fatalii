use crate::eval::{Eval, Score};

use movegen::bishop::Bishop;
use movegen::bitboard::Bitboard;
use movegen::king::King;
use movegen::knight::Knight;
use movegen::pawn::Pawn;
use movegen::piece;
use movegen::position::Position;
use movegen::queen::Queen;
use movegen::rook::Rook;
use movegen::side::Side;

#[derive(Debug, Clone)]
pub struct MaterialMobility;

impl MaterialMobility {
    pub const fn new() -> Self {
        MaterialMobility
    }
}

impl Eval for MaterialMobility {
    fn eval(&mut self, pos: &Position) -> Score {
        Self::material_score(pos) + Self::mobility_score(pos)
    }
}

impl MaterialMobility {
    const QUEEN_WEIGHT: Score = 900;
    const ROOK_WEIGHT: Score = 500;
    const BISHOP_WEIGHT: Score = 300;
    const KNIGHT_WEIGHT: Score = 300;
    const PAWN_WEIGHT: Score = 100;
    const MOBILITY_WEIGHT: Score = 10;

    fn material_score(pos: &Position) -> Score {
        Self::material_score_one_side(pos, Side::White)
            - Self::material_score_one_side(pos, Side::Black)
    }

    fn material_score_one_side(pos: &Position, side: Side) -> Score {
        Self::PAWN_WEIGHT * pos.piece_occupancy(side, piece::Type::Pawn).pop_count() as Score
            + Self::KNIGHT_WEIGHT
                * pos.piece_occupancy(side, piece::Type::Knight).pop_count() as Score
            + Self::BISHOP_WEIGHT
                * pos.piece_occupancy(side, piece::Type::Bishop).pop_count() as Score
            + Self::ROOK_WEIGHT * pos.piece_occupancy(side, piece::Type::Rook).pop_count() as Score
            + Self::QUEEN_WEIGHT
                * pos.piece_occupancy(side, piece::Type::Queen).pop_count() as Score
    }

    // The mobility calculated here is different from the number of legal moves so that it can be
    // computed faster.
    fn mobility_score(pos: &Position) -> Score {
        Self::mobility_score_one_side(pos, Side::White)
            - Self::mobility_score_one_side(pos, Side::Black)
    }

    fn mobility_score_one_side(pos: &Position, side: Side) -> Score {
        let occupancy = pos.occupancy();
        let own_occupancy = pos.side_occupancy(side);
        let opponent_occupancy = pos.side_occupancy(!side);
        let en_passant_square = if pos.side_to_move() == side {
            pos.en_passant_square()
        } else {
            Bitboard::EMPTY
        };

        let own_pawns = pos.piece_occupancy(side, piece::Type::Pawn);
        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(own_pawns, occupancy, side);
        let east_attack_targets =
            Pawn::east_attack_targets(own_pawns, side) & (opponent_occupancy | en_passant_square);
        let west_attack_targets =
            Pawn::west_attack_targets(own_pawns, side) & (opponent_occupancy | en_passant_square);
        let own_pawn_mob = (single_push_targets.pop_count()
            + double_push_targets.pop_count()
            + east_attack_targets.pop_count()
            + west_attack_targets.pop_count()) as Score;

        let mut own_knights = pos.piece_occupancy(side, piece::Type::Knight);
        let mut own_knight_mob = 0i16;
        while own_knights != Bitboard::EMPTY {
            let origin = own_knights.square_scan_forward_reset();
            let targets = Knight::targets(origin) & !own_occupancy;
            own_knight_mob += targets.pop_count() as Score;
        }

        let mut own_bishops = pos.piece_occupancy(side, piece::Type::Bishop);
        let mut own_bishop_mob = 0i16;
        while own_bishops != Bitboard::EMPTY {
            let origin = own_bishops.square_scan_forward_reset();
            let targets = Bishop::targets(origin, occupancy) & !own_occupancy;
            own_bishop_mob += targets.pop_count() as Score;
        }

        let mut own_rooks = pos.piece_occupancy(side, piece::Type::Rook);
        let mut own_rook_mob = 0i16;
        while own_rooks != Bitboard::EMPTY {
            let origin = own_rooks.square_scan_forward_reset();
            let targets = Rook::targets(origin, occupancy) & !own_occupancy;
            own_rook_mob += targets.pop_count() as Score;
        }

        let mut own_queens = pos.piece_occupancy(side, piece::Type::Queen);
        let mut own_queen_mob = 0i16;
        while own_queens != Bitboard::EMPTY {
            let origin = own_queens.square_scan_forward_reset();
            let targets = Queen::targets(origin, occupancy) & !own_occupancy;
            own_queen_mob += targets.pop_count() as Score;
        }

        let own_king = pos.piece_occupancy(side, piece::Type::King);
        let origin = own_king.to_square();
        let targets = King::targets(origin) & !own_occupancy;
        let own_king_mob = targets.pop_count() as Score;

        Self::MOBILITY_WEIGHT
            * (own_pawn_mob
                + own_knight_mob
                + own_bishop_mob
                + own_rook_mob
                + own_queen_mob
                + own_king_mob)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use movegen::position_history::PositionHistory;
    use movegen::r#move::{Move, MoveType};
    use movegen::square::Square;

    #[test]
    fn material_score() {
        let mut pos = Position::initial();
        assert_eq!(0, MaterialMobility::material_score(&pos));

        pos.set_piece_at(Square::D4, Some(piece::Piece::WHITE_PAWN));
        assert_eq!(
            MaterialMobility::PAWN_WEIGHT,
            MaterialMobility::material_score(&pos)
        );
        pos.set_piece_at(Square::D4, Some(piece::Piece::BLACK_PAWN));
        assert_eq!(
            -MaterialMobility::PAWN_WEIGHT,
            MaterialMobility::material_score(&pos)
        );

        pos.set_piece_at(Square::D4, Some(piece::Piece::WHITE_KNIGHT));
        assert_eq!(
            MaterialMobility::KNIGHT_WEIGHT,
            MaterialMobility::material_score(&pos)
        );
        pos.set_piece_at(Square::D4, Some(piece::Piece::BLACK_KNIGHT));
        assert_eq!(
            -MaterialMobility::KNIGHT_WEIGHT,
            MaterialMobility::material_score(&pos)
        );

        pos.set_piece_at(Square::D4, Some(piece::Piece::WHITE_BISHOP));
        assert_eq!(
            MaterialMobility::BISHOP_WEIGHT,
            MaterialMobility::material_score(&pos)
        );
        pos.set_piece_at(Square::D4, Some(piece::Piece::BLACK_BISHOP));
        assert_eq!(
            -MaterialMobility::BISHOP_WEIGHT,
            MaterialMobility::material_score(&pos)
        );

        pos.set_piece_at(Square::D4, Some(piece::Piece::WHITE_ROOK));
        assert_eq!(
            MaterialMobility::ROOK_WEIGHT,
            MaterialMobility::material_score(&pos)
        );
        pos.set_piece_at(Square::D4, Some(piece::Piece::BLACK_ROOK));
        assert_eq!(
            -MaterialMobility::ROOK_WEIGHT,
            MaterialMobility::material_score(&pos)
        );

        pos.set_piece_at(Square::D4, Some(piece::Piece::WHITE_QUEEN));
        assert_eq!(
            MaterialMobility::QUEEN_WEIGHT,
            MaterialMobility::material_score(&pos)
        );
        pos.set_piece_at(Square::D4, Some(piece::Piece::BLACK_QUEEN));
        assert_eq!(
            -MaterialMobility::QUEEN_WEIGHT,
            MaterialMobility::material_score(&pos)
        );
    }

    #[test]
    fn eval_relative() {
        let mut pos_history = PositionHistory::new(Position::initial());
        let mut evaluator = MaterialMobility::new();
        assert_eq!(
            evaluator.eval(pos_history.current_pos()),
            evaluator.eval_relative(pos_history.current_pos())
        );

        pos_history.do_move(Move::new(
            Square::D2,
            Square::D4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));
        assert_eq!(
            evaluator.eval(pos_history.current_pos()),
            -evaluator.eval_relative(pos_history.current_pos())
        );

        pos_history.do_move(Move::new(
            Square::D7,
            Square::D5,
            MoveType::DOUBLE_PAWN_PUSH,
        ));
        assert_eq!(
            evaluator.eval(pos_history.current_pos()),
            evaluator.eval_relative(pos_history.current_pos())
        );

        pos_history.do_move(Move::new(
            Square::C2,
            Square::C4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));
        assert_eq!(
            evaluator.eval(pos_history.current_pos()),
            -evaluator.eval_relative(pos_history.current_pos())
        );
    }
}
