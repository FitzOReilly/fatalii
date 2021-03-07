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

pub struct Eval;

impl Eval {
    const QUEEN_WEIGHT: i16 = 900;
    const ROOK_WEIGHT: i16 = 500;
    const BISHOP_WEIGHT: i16 = 300;
    const KNIGHT_WEIGHT: i16 = 300;
    const PAWN_WEIGHT: i16 = 100;
    const MOBILITY_WEIGHT: i16 = 10;

    pub fn eval(pos: &Position) -> i16 {
        Self::material_score(pos) + Self::mobility_score(pos)
    }

    fn material_score(pos: &Position) -> i16 {
        Self::material_score_one_side(pos, Side::White)
            - Self::material_score_one_side(pos, Side::Black)
    }

    fn material_score_one_side(pos: &Position, side: Side) -> i16 {
        Self::PAWN_WEIGHT * pos.piece_occupancy(side, piece::Type::Pawn).pop_count() as i16
            + Self::KNIGHT_WEIGHT
                * pos.piece_occupancy(side, piece::Type::Knight).pop_count() as i16
            + Self::BISHOP_WEIGHT
                * pos.piece_occupancy(side, piece::Type::Bishop).pop_count() as i16
            + Self::ROOK_WEIGHT * pos.piece_occupancy(side, piece::Type::Rook).pop_count() as i16
            + Self::QUEEN_WEIGHT * pos.piece_occupancy(side, piece::Type::Queen).pop_count() as i16
    }

    // The mobility calculated here is different from the number of legal moves so that it can be
    // computed faster.
    fn mobility_score(pos: &Position) -> i16 {
        Self::mobility_score_one_side(pos, Side::White)
            - Self::mobility_score_one_side(pos, Side::Black)
    }

    fn mobility_score_one_side(pos: &Position, side: Side) -> i16 {
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
            + west_attack_targets.pop_count()) as i16;

        let mut own_knights = pos.piece_occupancy(side, piece::Type::Knight);
        let mut own_knight_mob = 0i16;
        while own_knights != Bitboard::EMPTY {
            let origin = own_knights.square_scan_forward_reset();
            let targets = Knight::targets(origin) & !own_occupancy;
            own_knight_mob += targets.pop_count() as i16;
        }

        let mut own_bishops = pos.piece_occupancy(side, piece::Type::Bishop);
        let mut own_bishop_mob = 0i16;
        while own_bishops != Bitboard::EMPTY {
            let origin = own_bishops.square_scan_forward_reset();
            let targets = Bishop::targets(origin, occupancy) & !own_occupancy;
            own_bishop_mob += targets.pop_count() as i16;
        }

        let mut own_rooks = pos.piece_occupancy(side, piece::Type::Rook);
        let mut own_rook_mob = 0i16;
        while own_rooks != Bitboard::EMPTY {
            let origin = own_rooks.square_scan_forward_reset();
            let targets = Rook::targets(origin, occupancy) & !own_occupancy;
            own_rook_mob += targets.pop_count() as i16;
        }

        let mut own_queens = pos.piece_occupancy(side, piece::Type::Queen);
        let mut own_queen_mob = 0i16;
        while own_queens != Bitboard::EMPTY {
            let origin = own_queens.square_scan_forward_reset();
            let targets = Queen::targets(origin, occupancy) & !own_occupancy;
            own_queen_mob += targets.pop_count() as i16;
        }

        let own_king = pos.piece_occupancy(side, piece::Type::King);
        let origin = own_king.to_square();
        let targets = King::targets(origin) & !own_occupancy;
        let own_king_mob = targets.pop_count() as i16;

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
    use movegen::square::Square;

    #[test]
    fn material_score() {
        let mut pos = Position::initial();
        assert_eq!(0, Eval::material_score(&pos));

        pos.set_piece_at(Square::D4, Some(piece::Piece::WHITE_PAWN));
        assert_eq!(Eval::PAWN_WEIGHT, Eval::material_score(&pos));
        pos.set_piece_at(Square::D4, Some(piece::Piece::BLACK_PAWN));
        assert_eq!(-Eval::PAWN_WEIGHT, Eval::material_score(&pos));

        pos.set_piece_at(Square::D4, Some(piece::Piece::WHITE_KNIGHT));
        assert_eq!(Eval::KNIGHT_WEIGHT, Eval::material_score(&pos));
        pos.set_piece_at(Square::D4, Some(piece::Piece::BLACK_KNIGHT));
        assert_eq!(-Eval::KNIGHT_WEIGHT, Eval::material_score(&pos));

        pos.set_piece_at(Square::D4, Some(piece::Piece::WHITE_BISHOP));
        assert_eq!(Eval::BISHOP_WEIGHT, Eval::material_score(&pos));
        pos.set_piece_at(Square::D4, Some(piece::Piece::BLACK_BISHOP));
        assert_eq!(-Eval::BISHOP_WEIGHT, Eval::material_score(&pos));

        pos.set_piece_at(Square::D4, Some(piece::Piece::WHITE_ROOK));
        assert_eq!(Eval::ROOK_WEIGHT, Eval::material_score(&pos));
        pos.set_piece_at(Square::D4, Some(piece::Piece::BLACK_ROOK));
        assert_eq!(-Eval::ROOK_WEIGHT, Eval::material_score(&pos));

        pos.set_piece_at(Square::D4, Some(piece::Piece::WHITE_QUEEN));
        assert_eq!(Eval::QUEEN_WEIGHT, Eval::material_score(&pos));
        pos.set_piece_at(Square::D4, Some(piece::Piece::BLACK_QUEEN));
        assert_eq!(-Eval::QUEEN_WEIGHT, Eval::material_score(&pos));
    }
}
