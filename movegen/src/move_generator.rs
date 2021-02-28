use crate::bishop::Bishop;
use crate::bitboard::Bitboard;
use crate::king::King;
use crate::knight::Knight;
use crate::pawn::Pawn;
use crate::piece;
use crate::position::{CastlingRights, Position};
use crate::queen::Queen;
use crate::r#move::{Move, MoveList, MoveType};
use crate::rook::Rook;
use crate::side::Side;

pub struct MoveGenerator;

impl MoveGenerator {
    fn add_move_if_legal(move_list: &mut MoveList, pos: &Position, m: Move) {
        if Self::is_legal_move(pos, m) {
            move_list.push(m);
        }
    }

    fn is_legal_move(pos: &Position, m: Move) -> bool {
        let mut pos = pos.clone();
        let origin = m.origin();
        let target = m.target();
        // Promotion piece type is ignored here because it doesn't change the opposing side's
        // attacks
        pos.set_piece_at(target, pos.piece_at(origin));
        pos.set_piece_at(origin, None);
        if m.is_en_passant() {
            let captured_idx = Pawn::idx_push_origin(target, pos.side_to_move());
            pos.set_piece_at(captured_idx, None);
        }

        !pos.is_in_check(pos.side_to_move())
    }

    pub fn generate_moves(move_list: &mut MoveList, pos: &Position) {
        move_list.clear();
        Self::generate_pawn_moves(move_list, pos);
        Self::generate_knight_moves(move_list, pos);
        Self::generate_sliding_piece_moves(move_list, pos, piece::Type::Bishop, Bishop::targets);
        Self::generate_sliding_piece_moves(move_list, pos, piece::Type::Rook, Rook::targets);
        Self::generate_sliding_piece_moves(move_list, pos, piece::Type::Queen, Queen::targets);
        Self::generate_king_moves(move_list, pos);
        Self::generate_castles(move_list, pos);
    }

    fn generate_pawn_moves(move_list: &mut MoveList, pos: &Position) {
        let pawns = pos.piece_occupancy(pos.side_to_move(), piece::Type::Pawn);

        Self::generate_pawn_pushes(move_list, pos, pawns);
        Self::generate_pawn_captures(move_list, pos, pawns);
    }

    fn generate_pawn_pushes(move_list: &mut MoveList, pos: &Position, pawns: Bitboard) {
        let side_to_move = pos.side_to_move();

        let (single_push_targets, mut double_push_targets) =
            Pawn::push_targets(pawns, pos.occupancy(), side_to_move);

        let mut promo_targets = single_push_targets & Pawn::promotion_rank(side_to_move);
        let mut non_promo_targets = single_push_targets & !promo_targets;

        while promo_targets != Bitboard::EMPTY {
            let target = promo_targets.bit_scan_forward_reset();
            let origin = Pawn::idx_push_origin(target, side_to_move);
            for promo_piece in &[
                piece::Type::Queen,
                piece::Type::Rook,
                piece::Type::Bishop,
                piece::Type::Knight,
            ] {
                let m = Move::new(
                    origin,
                    target,
                    MoveType::new_with_promo_piece(MoveType::PROMOTION, *promo_piece),
                );
                Self::add_move_if_legal(move_list, pos, m);
            }
        }
        while non_promo_targets != Bitboard::EMPTY {
            let target = non_promo_targets.bit_scan_forward_reset();
            let origin = Pawn::idx_push_origin(target, side_to_move);
            Self::add_move_if_legal(move_list, pos, Move::new(origin, target, MoveType::QUIET));
        }
        while double_push_targets != Bitboard::EMPTY {
            let target = double_push_targets.bit_scan_forward_reset();
            let origin = Pawn::idx_double_push_origin(target, side_to_move);
            Self::add_move_if_legal(
                move_list,
                pos,
                Move::new(origin, target, MoveType::DOUBLE_PAWN_PUSH),
            );
        }
    }

    fn generate_pawn_captures(move_list: &mut MoveList, pos: &Position, pawns: Bitboard) {
        Self::generate_pawn_captures_one_side(
            move_list,
            pos,
            pawns,
            Pawn::east_attack_targets,
            Pawn::idx_east_attack_origin,
        );

        Self::generate_pawn_captures_one_side(
            move_list,
            pos,
            pawns,
            Pawn::west_attack_targets,
            Pawn::idx_west_attack_origin,
        );
    }

    fn generate_pawn_captures_one_side(
        move_list: &mut MoveList,
        pos: &Position,
        pawns: Bitboard,
        attacks: fn(Bitboard, Side) -> Bitboard,
        idx_attack_origin: fn(usize, Side) -> usize,
    ) {
        let en_passant_square = pos.en_passant_square();
        let side_to_move = pos.side_to_move();
        let promo_rank = Pawn::promotion_rank(side_to_move);
        let targets = attacks(pawns, side_to_move);
        let opponents = pos.side_occupancy(!pos.side_to_move());
        let captures = targets & (opponents | en_passant_square);
        let mut promo_captures = captures & promo_rank;
        let mut non_promo_captures = captures & !promo_captures;

        while promo_captures != Bitboard::EMPTY {
            let target = promo_captures.bit_scan_forward_reset();
            let origin = idx_attack_origin(target, side_to_move);
            for promo_piece in &[
                piece::Type::Queen,
                piece::Type::Rook,
                piece::Type::Bishop,
                piece::Type::Knight,
            ] {
                let m = Move::new(
                    origin,
                    target,
                    MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, *promo_piece),
                );
                Self::add_move_if_legal(move_list, pos, m);
            }
        }

        while non_promo_captures != Bitboard::EMPTY {
            let target = non_promo_captures.bit_scan_forward_reset();
            let origin = idx_attack_origin(target, side_to_move);
            let move_type = if Bitboard(0x1 << target) == en_passant_square {
                MoveType::EN_PASSANT_CAPTURE
            } else {
                MoveType::CAPTURE
            };
            Self::add_move_if_legal(move_list, pos, Move::new(origin, target, move_type));
        }
    }

    fn generate_knight_moves(move_list: &mut MoveList, pos: &Position) {
        let mut knights = pos.piece_occupancy(pos.side_to_move(), piece::Type::Knight);
        let own_occupancy = pos.side_occupancy(pos.side_to_move());
        while knights != Bitboard::EMPTY {
            let origin = knights.bit_scan_forward_reset();
            let targets = Knight::targets(origin) & !own_occupancy;
            Self::generate_piece_moves(move_list, pos, origin, &targets);
        }
    }

    fn generate_king_moves(move_list: &mut MoveList, pos: &Position) {
        let king = pos.piece_occupancy(pos.side_to_move(), piece::Type::King);
        let own_occupancy = pos.side_occupancy(pos.side_to_move());
        let origin = king.bit_idx();
        let targets = King::targets(origin) & !own_occupancy;
        Self::generate_piece_moves(move_list, pos, origin, &targets);
    }

    fn generate_sliding_piece_moves(
        move_list: &mut MoveList,
        pos: &Position,
        piece_type: piece::Type,
        piece_targets: fn(usize, Bitboard) -> Bitboard,
    ) {
        let mut piece_occupancy = pos.piece_occupancy(pos.side_to_move(), piece_type);
        let own_occupancy = pos.side_occupancy(pos.side_to_move());
        while piece_occupancy != Bitboard::EMPTY {
            let origin = piece_occupancy.bit_scan_forward_reset();
            let targets = piece_targets(origin, pos.occupancy()) & !own_occupancy;
            Self::generate_piece_moves(move_list, pos, origin, &targets);
        }
    }

    fn generate_piece_moves(
        move_list: &mut MoveList,
        pos: &Position,
        origin: usize,
        targets: &Bitboard,
    ) {
        let opponents = pos.side_occupancy(!pos.side_to_move());
        let mut captures = targets & opponents;
        let mut quiets = targets & !captures;

        while captures != Bitboard::EMPTY {
            let target = captures.bit_scan_forward_reset();
            Self::add_move_if_legal(move_list, pos, Move::new(origin, target, MoveType::CAPTURE));
        }
        while quiets != Bitboard::EMPTY {
            let target = quiets.bit_scan_forward_reset();
            Self::add_move_if_legal(move_list, pos, Move::new(origin, target, MoveType::QUIET));
        }
    }

    fn generate_castles(move_list: &mut MoveList, pos: &Position) {
        const CASTLES: [fn(&mut MoveList, &Position); 2] = [
            MoveGenerator::generate_white_castles,
            MoveGenerator::generate_black_castles,
        ];
        let side_idx = pos.side_to_move() as usize;
        CASTLES[side_idx](move_list, pos);
    }

    fn generate_white_castles(move_list: &mut MoveList, pos: &Position) {
        let castling_rights = pos.castling_rights();
        let attacked_by_opponent = pos.attacked_squares(!pos.side_to_move());

        if castling_rights.contains(CastlingRights::WHITE_KINGSIDE) {
            debug_assert_eq!(
                Some(piece::Piece::WHITE_KING),
                pos.piece_at(Bitboard::IDX_E1)
            );
            debug_assert_eq!(
                Some(piece::Piece::WHITE_ROOK),
                pos.piece_at(Bitboard::IDX_H1)
            );
            let squares_passable =
                pos.occupancy() & (Bitboard::F1 | Bitboard::G1) == Bitboard::EMPTY;
            let squares_attacked = attacked_by_opponent
                & (Bitboard::E1 | Bitboard::F1 | Bitboard::G1)
                != Bitboard::EMPTY;
            if squares_passable && !squares_attacked {
                move_list.push(Move::new(
                    Bitboard::IDX_E1,
                    Bitboard::IDX_G1,
                    MoveType::CASTLE_KINGSIDE,
                ));
            }
        }
        if castling_rights.contains(CastlingRights::WHITE_QUEENSIDE) {
            debug_assert_eq!(
                Some(piece::Piece::WHITE_KING),
                pos.piece_at(Bitboard::IDX_E1)
            );
            debug_assert_eq!(
                Some(piece::Piece::WHITE_ROOK),
                pos.piece_at(Bitboard::IDX_A1)
            );
            let squares_passable =
                pos.occupancy() & (Bitboard::B1 | Bitboard::C1 | Bitboard::D1) == Bitboard::EMPTY;
            let squares_attacked = attacked_by_opponent
                & (Bitboard::C1 | Bitboard::D1 | Bitboard::E1)
                != Bitboard::EMPTY;
            if squares_passable && !squares_attacked {
                move_list.push(Move::new(
                    Bitboard::IDX_E1,
                    Bitboard::IDX_C1,
                    MoveType::CASTLE_QUEENSIDE,
                ));
            }
        }
    }

    fn generate_black_castles(move_list: &mut MoveList, pos: &Position) {
        let castling_rights = pos.castling_rights();
        let attacked_by_opponent = pos.attacked_squares(!pos.side_to_move());

        if castling_rights.contains(CastlingRights::BLACK_KINGSIDE) {
            debug_assert_eq!(
                Some(piece::Piece::BLACK_KING),
                pos.piece_at(Bitboard::IDX_E8)
            );
            debug_assert_eq!(
                Some(piece::Piece::BLACK_ROOK),
                pos.piece_at(Bitboard::IDX_H8)
            );
            let squares_passable =
                pos.occupancy() & (Bitboard::F8 | Bitboard::G8) == Bitboard::EMPTY;
            let squares_attacked = attacked_by_opponent
                & (Bitboard::E8 | Bitboard::F8 | Bitboard::G8)
                != Bitboard::EMPTY;
            if squares_passable && !squares_attacked {
                move_list.push(Move::new(
                    Bitboard::IDX_E8,
                    Bitboard::IDX_G8,
                    MoveType::CASTLE_KINGSIDE,
                ));
            }
        }
        if castling_rights.contains(CastlingRights::BLACK_QUEENSIDE) {
            debug_assert_eq!(
                Some(piece::Piece::BLACK_KING),
                pos.piece_at(Bitboard::IDX_E8)
            );
            debug_assert_eq!(
                Some(piece::Piece::BLACK_ROOK),
                pos.piece_at(Bitboard::IDX_A8)
            );
            let squares_passable =
                pos.occupancy() & (Bitboard::B8 | Bitboard::C8 | Bitboard::D8) == Bitboard::EMPTY;
            let squares_attacked = attacked_by_opponent
                & (Bitboard::C8 | Bitboard::D8 | Bitboard::E8)
                != Bitboard::EMPTY;
            if squares_passable && !squares_attacked {
                move_list.push(Move::new(
                    Bitboard::IDX_E8,
                    Bitboard::IDX_C8,
                    MoveType::CASTLE_QUEENSIDE,
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_position() {
        let mut move_list = MoveList::new();

        let pos = Position::initial();
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // Pawn
            Move::new(Bitboard::IDX_A2, Bitboard::IDX_A3, MoveType::QUIET),
            Move::new(Bitboard::IDX_B2, Bitboard::IDX_B3, MoveType::QUIET),
            Move::new(Bitboard::IDX_C2, Bitboard::IDX_C3, MoveType::QUIET),
            Move::new(Bitboard::IDX_D2, Bitboard::IDX_D3, MoveType::QUIET),
            Move::new(Bitboard::IDX_E2, Bitboard::IDX_E3, MoveType::QUIET),
            Move::new(Bitboard::IDX_F2, Bitboard::IDX_F3, MoveType::QUIET),
            Move::new(Bitboard::IDX_G2, Bitboard::IDX_G3, MoveType::QUIET),
            Move::new(Bitboard::IDX_H2, Bitboard::IDX_H3, MoveType::QUIET),
            Move::new(
                Bitboard::IDX_A2,
                Bitboard::IDX_A4,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_B4,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_C2,
                Bitboard::IDX_C4,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_D2,
                Bitboard::IDX_D4,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_E2,
                Bitboard::IDX_E4,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_F2,
                Bitboard::IDX_F4,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_G2,
                Bitboard::IDX_G4,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_H2,
                Bitboard::IDX_H4,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            // Knight
            Move::new(Bitboard::IDX_B1, Bitboard::IDX_A3, MoveType::QUIET),
            Move::new(Bitboard::IDX_B1, Bitboard::IDX_C3, MoveType::QUIET),
            Move::new(Bitboard::IDX_G1, Bitboard::IDX_F3, MoveType::QUIET),
            Move::new(Bitboard::IDX_G1, Bitboard::IDX_H3, MoveType::QUIET),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn position_after_1_e4() {
        let mut move_list = MoveList::new();

        let mut pos = Position::initial();
        pos.set_piece_at(Bitboard::IDX_E2, None);
        pos.set_piece_at(Bitboard::IDX_E4, Some(piece::Piece::WHITE_PAWN));
        pos.set_en_passant_square(Bitboard::E3);
        pos.set_side_to_move(Side::Black);
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // Pawn
            Move::new(Bitboard::IDX_A7, Bitboard::IDX_A6, MoveType::QUIET),
            Move::new(Bitboard::IDX_B7, Bitboard::IDX_B6, MoveType::QUIET),
            Move::new(Bitboard::IDX_C7, Bitboard::IDX_C6, MoveType::QUIET),
            Move::new(Bitboard::IDX_D7, Bitboard::IDX_D6, MoveType::QUIET),
            Move::new(Bitboard::IDX_E7, Bitboard::IDX_E6, MoveType::QUIET),
            Move::new(Bitboard::IDX_F7, Bitboard::IDX_F6, MoveType::QUIET),
            Move::new(Bitboard::IDX_G7, Bitboard::IDX_G6, MoveType::QUIET),
            Move::new(Bitboard::IDX_H7, Bitboard::IDX_H6, MoveType::QUIET),
            Move::new(
                Bitboard::IDX_A7,
                Bitboard::IDX_A5,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_B5,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_C7,
                Bitboard::IDX_C5,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_D7,
                Bitboard::IDX_D5,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_E7,
                Bitboard::IDX_E5,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_F7,
                Bitboard::IDX_F5,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_G7,
                Bitboard::IDX_G5,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            Move::new(
                Bitboard::IDX_H7,
                Bitboard::IDX_H5,
                MoveType::DOUBLE_PAWN_PUSH,
            ),
            // Knight
            Move::new(Bitboard::IDX_B8, Bitboard::IDX_A6, MoveType::QUIET),
            Move::new(Bitboard::IDX_B8, Bitboard::IDX_C6, MoveType::QUIET),
            Move::new(Bitboard::IDX_G8, Bitboard::IDX_F6, MoveType::QUIET),
            Move::new(Bitboard::IDX_G8, Bitboard::IDX_H6, MoveType::QUIET),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn white_pawn_captures() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_A2, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_H2, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_A3, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_B3, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_G3, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_H3, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::QUIET),
            // Pawn
            Move::new(Bitboard::IDX_A2, Bitboard::IDX_B3, MoveType::CAPTURE),
            Move::new(Bitboard::IDX_H2, Bitboard::IDX_G3, MoveType::CAPTURE),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn black_pawn_captures() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_A7, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_H7, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_A6, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_B6, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_G6, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_H6, Some(piece::Piece::WHITE_PAWN));
        pos.set_side_to_move(Side::Black);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_D8, MoveType::QUIET),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_D7, MoveType::QUIET),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_E7, MoveType::QUIET),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_F8, MoveType::QUIET),
            // Pawn
            Move::new(Bitboard::IDX_A7, Bitboard::IDX_B6, MoveType::CAPTURE),
            Move::new(Bitboard::IDX_H7, Bitboard::IDX_G6, MoveType::CAPTURE),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn king_moves() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_F2, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::CAPTURE),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn knight_moves() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_B1, Some(piece::Piece::WHITE_KNIGHT));
        pos.set_piece_at(Bitboard::IDX_C3, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_A3, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::QUIET),
            // Pawn
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C4, MoveType::QUIET),
            // Knight
            Move::new(Bitboard::IDX_B1, Bitboard::IDX_D2, MoveType::QUIET),
            Move::new(Bitboard::IDX_B1, Bitboard::IDX_A3, MoveType::CAPTURE),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn bishop_moves() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_C3, Some(piece::Piece::WHITE_BISHOP));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_G7, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::QUIET),
            // Bishop
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_B2, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_A1, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_D2, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_B4, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_A5, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_D4, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_E5, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_F6, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_G7, MoveType::CAPTURE),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn rook_moves() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_E3, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_E7, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::QUIET),
            // Rook
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_E2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_D3, MoveType::QUIET),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_C3, MoveType::QUIET),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_B3, MoveType::QUIET),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_A3, MoveType::QUIET),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_F3, MoveType::QUIET),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_G3, MoveType::QUIET),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_H3, MoveType::QUIET),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_E4, MoveType::QUIET),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_E5, MoveType::QUIET),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_E6, MoveType::QUIET),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_E7, MoveType::CAPTURE),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn queen_moves() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_E3, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_C3, Some(piece::Piece::WHITE_QUEEN));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_C7, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_G7, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::QUIET),
            // Pawn
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_E4, MoveType::QUIET),
            // Queen ranks and files
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C2, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C1, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_B3, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_A3, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_D3, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C4, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C5, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C6, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C7, MoveType::CAPTURE),
            // Queen diagonals
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_B2, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_A1, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_D2, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_B4, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_A5, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_D4, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_E5, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_F6, MoveType::QUIET),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_G7, MoveType::CAPTURE),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn white_promotions() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_B7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_A8, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Bitboard::IDX_C8, Some(piece::Piece::BLACK_BISHOP));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::QUIET),
            // Pawns
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_A8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Knight),
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_A8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_A8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Rook),
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_A8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Queen),
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_B8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Knight),
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_B8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Bishop),
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_B8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Rook),
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_B8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Queen),
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_C8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Knight),
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_C8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_C8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Rook),
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_C8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Queen),
            ),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn black_promotions() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_B2, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_A1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_C1, Some(piece::Piece::WHITE_BISHOP));
        pos.set_side_to_move(Side::Black);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_D7, MoveType::QUIET),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_D8, MoveType::QUIET),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_E7, MoveType::QUIET),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_F7, MoveType::QUIET),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_F8, MoveType::QUIET),
            // Pawns
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_A1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Knight),
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_A1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_A1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Rook),
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_A1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Queen),
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_B1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Knight),
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_B1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Bishop),
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_B1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Rook),
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_B1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Queen),
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_C1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Knight),
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_C1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_C1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Rook),
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_C1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Queen),
            ),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn white_en_passant_captures() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_D5, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_C5, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        pos.set_en_passant_square(Bitboard::C6);
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::QUIET),
            // Pawn
            Move::new(Bitboard::IDX_D5, Bitboard::IDX_D6, MoveType::QUIET),
            Move::new(
                Bitboard::IDX_D5,
                Bitboard::IDX_C6,
                MoveType::EN_PASSANT_CAPTURE,
            ),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn black_en_passant_captures() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_D4, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_C4, Some(piece::Piece::WHITE_PAWN));
        pos.set_side_to_move(Side::Black);
        pos.set_castling_rights(CastlingRights::empty());
        pos.set_en_passant_square(Bitboard::C3);
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_D8, MoveType::QUIET),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_D7, MoveType::QUIET),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_E7, MoveType::QUIET),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_F7, MoveType::QUIET),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_F8, MoveType::QUIET),
            // Pawn
            Move::new(Bitboard::IDX_D4, Bitboard::IDX_D3, MoveType::QUIET),
            Move::new(
                Bitboard::IDX_D4,
                Bitboard::IDX_C3,
                MoveType::EN_PASSANT_CAPTURE,
            ),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn white_castles() {
        let kingside_castle = Move::new(
            Bitboard::IDX_E1,
            Bitboard::IDX_G1,
            MoveType::CASTLE_KINGSIDE,
        );
        let queenside_castle = Move::new(
            Bitboard::IDX_E1,
            Bitboard::IDX_C1,
            MoveType::CASTLE_QUEENSIDE,
        );

        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_A1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        pos.set_castling_rights(CastlingRights::WHITE_KINGSIDE);
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        pos.set_castling_rights(CastlingRights::WHITE_QUEENSIDE);
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&kingside_castle));
        assert!(move_list.contains(&queenside_castle));

        pos.set_castling_rights(CastlingRights::WHITE_BOTH);
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(move_list.contains(&kingside_castle));
        assert!(move_list.contains(&queenside_castle));

        // Square between king and rook blocked
        let mut pos_blocked = pos.clone();
        pos_blocked.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WHITE_KNIGHT));
        pos_blocked.set_piece_at(Bitboard::IDX_B1, Some(piece::Piece::BLACK_KNIGHT));
        MoveGenerator::generate_moves(&mut move_list, &pos_blocked);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // King attacked
        let mut pos_in_check = pos.clone();
        pos_in_check.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::BLACK_ROOK));
        MoveGenerator::generate_moves(&mut move_list, &pos_in_check);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // Square traversed by king attacked
        let mut pos_traverse_attacked = pos.clone();
        pos_traverse_attacked.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::BLACK_BISHOP));
        MoveGenerator::generate_moves(&mut move_list, &pos_traverse_attacked);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // Target square attacked
        let mut pos_target_attacked = pos.clone();
        pos_target_attacked.set_piece_at(Bitboard::IDX_E3, Some(piece::Piece::BLACK_BISHOP));
        MoveGenerator::generate_moves(&mut move_list, &pos_target_attacked);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // Rook attacked (castling is legal)
        let mut pos_rook_attacked = pos.clone();
        pos_rook_attacked.set_piece_at(Bitboard::IDX_E4, Some(piece::Piece::BLACK_BISHOP));
        pos_rook_attacked.set_piece_at(Bitboard::IDX_E5, Some(piece::Piece::BLACK_BISHOP));
        MoveGenerator::generate_moves(&mut move_list, &pos_rook_attacked);
        assert!(move_list.contains(&kingside_castle));
        assert!(move_list.contains(&queenside_castle));
    }

    #[test]
    fn black_castles() {
        let kingside_castle = Move::new(
            Bitboard::IDX_E8,
            Bitboard::IDX_G8,
            MoveType::CASTLE_KINGSIDE,
        );
        let queenside_castle = Move::new(
            Bitboard::IDX_E8,
            Bitboard::IDX_C8,
            MoveType::CASTLE_QUEENSIDE,
        );

        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_A8, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Bitboard::IDX_H8, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::Black);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        pos.set_castling_rights(CastlingRights::BLACK_KINGSIDE);
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        pos.set_castling_rights(CastlingRights::BLACK_QUEENSIDE);
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&kingside_castle));
        assert!(move_list.contains(&queenside_castle));

        pos.set_castling_rights(CastlingRights::BLACK_BOTH);
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(move_list.contains(&kingside_castle));
        assert!(move_list.contains(&queenside_castle));

        // Square between king and rook blocked
        let mut pos_blocked = pos.clone();
        pos_blocked.set_piece_at(Bitboard::IDX_G8, Some(piece::Piece::BLACK_KNIGHT));
        pos_blocked.set_piece_at(Bitboard::IDX_B8, Some(piece::Piece::WHITE_KNIGHT));
        MoveGenerator::generate_moves(&mut move_list, &pos_blocked);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // King attacked
        let mut pos_in_check = pos.clone();
        pos_in_check.set_piece_at(Bitboard::IDX_E7, Some(piece::Piece::WHITE_ROOK));
        MoveGenerator::generate_moves(&mut move_list, &pos_in_check);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // Square traversed by king attacked
        let mut pos_traverse_attacked = pos.clone();
        pos_traverse_attacked.set_piece_at(Bitboard::IDX_E7, Some(piece::Piece::WHITE_BISHOP));
        MoveGenerator::generate_moves(&mut move_list, &pos_traverse_attacked);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // Target square attacked
        let mut pos_target_attacked = pos.clone();
        pos_target_attacked.set_piece_at(Bitboard::IDX_E6, Some(piece::Piece::WHITE_BISHOP));
        MoveGenerator::generate_moves(&mut move_list, &pos_target_attacked);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // Rook attacked (castling is legal)
        let mut pos_rook_attacked = pos.clone();
        pos_rook_attacked.set_piece_at(Bitboard::IDX_E4, Some(piece::Piece::WHITE_BISHOP));
        pos_rook_attacked.set_piece_at(Bitboard::IDX_E5, Some(piece::Piece::WHITE_BISHOP));
        MoveGenerator::generate_moves(&mut move_list, &pos_rook_attacked);
        assert!(move_list.contains(&kingside_castle));
        assert!(move_list.contains(&queenside_castle));
    }

    #[test]
    fn king_not_left_in_check_after_pawn_moves() {
        let mut move_list = MoveList::new();

        let mut pos_pawn = Position::empty();
        pos_pawn.set_piece_at(Bitboard::IDX_D2, Some(piece::Piece::WHITE_KING));
        pos_pawn.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::WHITE_PAWN));
        pos_pawn.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos_pawn.set_piece_at(Bitboard::IDX_H2, Some(piece::Piece::BLACK_ROOK));
        pos_pawn.set_piece_at(Bitboard::IDX_F3, Some(piece::Piece::BLACK_ROOK));
        pos_pawn.set_side_to_move(Side::White);
        pos_pawn.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos_pawn);
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_E2,
            Bitboard::IDX_E3,
            MoveType::QUIET
        )));
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_E2,
            Bitboard::IDX_E4,
            MoveType::DOUBLE_PAWN_PUSH
        )));
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_E2,
            Bitboard::IDX_F3,
            MoveType::CAPTURE
        )));

        let mut pos_pawn_promo = Position::empty();
        pos_pawn_promo.set_piece_at(Bitboard::IDX_A7, Some(piece::Piece::WHITE_KING));
        pos_pawn_promo.set_piece_at(Bitboard::IDX_B7, Some(piece::Piece::WHITE_PAWN));
        pos_pawn_promo.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos_pawn_promo.set_piece_at(Bitboard::IDX_H7, Some(piece::Piece::BLACK_ROOK));
        pos_pawn_promo.set_piece_at(Bitboard::IDX_C8, Some(piece::Piece::BLACK_ROOK));
        pos_pawn_promo.set_side_to_move(Side::White);
        pos_pawn_promo.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos_pawn_promo);
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_B7,
            Bitboard::IDX_B8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Queen)
        )));
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_B7,
            Bitboard::IDX_C8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Queen)
        )));

        let mut pos_pawn_en_passant = Position::empty();
        pos_pawn_en_passant.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::WHITE_KING));
        pos_pawn_en_passant.set_piece_at(Bitboard::IDX_C5, Some(piece::Piece::WHITE_PAWN));
        pos_pawn_en_passant.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos_pawn_en_passant.set_piece_at(Bitboard::IDX_A6, Some(piece::Piece::BLACK_BISHOP));
        pos_pawn_en_passant.set_piece_at(Bitboard::IDX_B5, Some(piece::Piece::BLACK_PAWN));
        pos_pawn_en_passant.set_side_to_move(Side::White);
        pos_pawn_en_passant.set_castling_rights(CastlingRights::empty());
        pos_pawn_en_passant.set_en_passant_square(Bitboard::B6);
        MoveGenerator::generate_moves(&mut move_list, &pos_pawn_en_passant);
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_C5,
            Bitboard::IDX_B6,
            MoveType::EN_PASSANT_CAPTURE
        )));
    }

    #[test]
    fn king_not_left_in_check_after_knight_moves() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WHITE_KNIGHT));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_E2,
            MoveType::QUIET
        )));
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F3,
            MoveType::QUIET
        )));
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_H3,
            MoveType::QUIET
        )));
    }

    #[test]
    fn king_not_left_in_check_after_bishop_moves() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WHITE_BISHOP));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F2,
            MoveType::QUIET
        )));
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_H2,
            MoveType::QUIET
        )));
    }

    #[test]
    fn king_not_left_in_check_after_rook_moves() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_G2,
            MoveType::QUIET
        )));
        assert!(move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F1,
            MoveType::QUIET
        )));
        assert!(move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_H1,
            MoveType::CAPTURE
        )));
    }

    #[test]
    fn king_not_left_in_check_after_queen_moves() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WHITE_QUEEN));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F2,
            MoveType::QUIET
        )));
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_G2,
            MoveType::QUIET
        )));
        assert!(move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F1,
            MoveType::QUIET
        )));
        assert!(move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_H1,
            MoveType::CAPTURE
        )));
    }

    #[test]
    fn king_does_not_move_into_check() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_H2, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&Move::new(
            Bitboard::IDX_E1,
            Bitboard::IDX_E2,
            MoveType::QUIET
        )));
    }
}
