mod king_in_check_generator;
mod king_not_xrayed_generator;
mod king_xrayed_generator;
mod move_generator_template;

use crate::move_generator::king_in_check_generator::KingInCheckGenerator;
use crate::move_generator::king_not_xrayed_generator::KingNotXrayedGenerator;
use crate::move_generator::king_xrayed_generator::KingXrayedGenerator;
use crate::move_generator::move_generator_template::MoveGeneratorTemplate;

use crate::attacks_to::AttacksTo;
use crate::bitboard::Bitboard;
use crate::piece;
use crate::position::Position;
use crate::r#move::MoveList;

pub struct MoveGenerator;

impl MoveGenerator {
    pub fn generate_moves(move_list: &mut MoveList, pos: &Position) {
        move_list.clear();

        let own_king_bb = pos.piece_occupancy(pos.side_to_move(), piece::Type::King);
        let own_king = own_king_bb.to_square();

        let attacks_to_king = AttacksTo::new(pos, own_king, !pos.side_to_move());

        let king_in_check = own_king_bb & attacks_to_king.all_attack_targets != Bitboard::EMPTY;
        let king_xrayed = attacks_to_king.xrays_to_target != Bitboard::EMPTY;

        if king_in_check {
            debug_assert!(attacks_to_king.attack_origins.pop_count() >= 1);
            debug_assert!(attacks_to_king.attack_origins.pop_count() <= 2);
            debug_assert!(attacks_to_king.each_slider_attack.len() <= 2);
            if attacks_to_king.attack_origins.pop_count() == 2 {
                // Only king moves are legal in double check
                KingInCheckGenerator::generate_king_moves(move_list, &attacks_to_king);
            } else {
                KingInCheckGenerator::generate_moves(move_list, &attacks_to_king);
            }
        } else if king_xrayed {
            KingXrayedGenerator::generate_moves(move_list, &attacks_to_king);
        } else {
            KingNotXrayedGenerator::generate_moves(move_list, &attacks_to_king);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fen::Fen;
    use crate::position::{CastlingRights, Position};
    use crate::r#move::{Move, MoveType};
    use crate::side::Side;
    use crate::square::Square;

    #[test]
    fn initial_position() {
        let mut move_list = MoveList::new();

        let pos = Position::initial();
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // Pawn
            Move::new(Square::A2, Square::A3, MoveType::QUIET),
            Move::new(Square::B2, Square::B3, MoveType::QUIET),
            Move::new(Square::C2, Square::C3, MoveType::QUIET),
            Move::new(Square::D2, Square::D3, MoveType::QUIET),
            Move::new(Square::E2, Square::E3, MoveType::QUIET),
            Move::new(Square::F2, Square::F3, MoveType::QUIET),
            Move::new(Square::G2, Square::G3, MoveType::QUIET),
            Move::new(Square::H2, Square::H3, MoveType::QUIET),
            Move::new(Square::A2, Square::A4, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::B2, Square::B4, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::C2, Square::C4, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::D2, Square::D4, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::E2, Square::E4, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::F2, Square::F4, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::G2, Square::G4, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::H2, Square::H4, MoveType::DOUBLE_PAWN_PUSH),
            // Knight
            Move::new(Square::B1, Square::A3, MoveType::QUIET),
            Move::new(Square::B1, Square::C3, MoveType::QUIET),
            Move::new(Square::G1, Square::F3, MoveType::QUIET),
            Move::new(Square::G1, Square::H3, MoveType::QUIET),
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
        pos.set_piece_at(Square::E2, None);
        pos.set_piece_at(Square::E4, Some(piece::Piece::WHITE_PAWN));
        pos.set_en_passant_square(Bitboard::E3);
        pos.set_side_to_move(Side::Black);
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // Pawn
            Move::new(Square::A7, Square::A6, MoveType::QUIET),
            Move::new(Square::B7, Square::B6, MoveType::QUIET),
            Move::new(Square::C7, Square::C6, MoveType::QUIET),
            Move::new(Square::D7, Square::D6, MoveType::QUIET),
            Move::new(Square::E7, Square::E6, MoveType::QUIET),
            Move::new(Square::F7, Square::F6, MoveType::QUIET),
            Move::new(Square::G7, Square::G6, MoveType::QUIET),
            Move::new(Square::H7, Square::H6, MoveType::QUIET),
            Move::new(Square::A7, Square::A5, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::B7, Square::B5, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::C7, Square::C5, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::D7, Square::D5, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::E7, Square::E5, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::F7, Square::F5, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::G7, Square::G5, MoveType::DOUBLE_PAWN_PUSH),
            Move::new(Square::H7, Square::H5, MoveType::DOUBLE_PAWN_PUSH),
            // Knight
            Move::new(Square::B8, Square::A6, MoveType::QUIET),
            Move::new(Square::B8, Square::C6, MoveType::QUIET),
            Move::new(Square::G8, Square::F6, MoveType::QUIET),
            Move::new(Square::G8, Square::H6, MoveType::QUIET),
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
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::A2, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::H2, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::A3, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::B3, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::G3, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::H3, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Square::E1, Square::D1, MoveType::QUIET),
            Move::new(Square::E1, Square::D2, MoveType::QUIET),
            Move::new(Square::E1, Square::E2, MoveType::QUIET),
            Move::new(Square::E1, Square::F1, MoveType::QUIET),
            // Pawn
            Move::new(Square::A2, Square::B3, MoveType::CAPTURE),
            Move::new(Square::H2, Square::G3, MoveType::CAPTURE),
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
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::A7, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::H7, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::A6, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::B6, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::G6, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::H6, Some(piece::Piece::WHITE_PAWN));
        pos.set_side_to_move(Side::Black);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Square::E8, Square::D8, MoveType::QUIET),
            Move::new(Square::E8, Square::D7, MoveType::QUIET),
            Move::new(Square::E8, Square::E7, MoveType::QUIET),
            Move::new(Square::E8, Square::F8, MoveType::QUIET),
            // Pawn
            Move::new(Square::A7, Square::B6, MoveType::CAPTURE),
            Move::new(Square::H7, Square::G6, MoveType::CAPTURE),
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
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::F2, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            Move::new(Square::E1, Square::D1, MoveType::QUIET),
            Move::new(Square::E1, Square::D2, MoveType::QUIET),
            Move::new(Square::E1, Square::E2, MoveType::QUIET),
            Move::new(Square::E1, Square::F1, MoveType::QUIET),
            Move::new(Square::E1, Square::F2, MoveType::CAPTURE),
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
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::B1, Some(piece::Piece::WHITE_KNIGHT));
        pos.set_piece_at(Square::C3, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::A3, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Square::E1, Square::D1, MoveType::QUIET),
            Move::new(Square::E1, Square::D2, MoveType::QUIET),
            Move::new(Square::E1, Square::E2, MoveType::QUIET),
            Move::new(Square::E1, Square::F1, MoveType::QUIET),
            Move::new(Square::E1, Square::F2, MoveType::QUIET),
            // Pawn
            Move::new(Square::C3, Square::C4, MoveType::QUIET),
            // Knight
            Move::new(Square::B1, Square::D2, MoveType::QUIET),
            Move::new(Square::B1, Square::A3, MoveType::CAPTURE),
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
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::C3, Some(piece::Piece::WHITE_BISHOP));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::G7, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Square::E1, Square::D1, MoveType::QUIET),
            Move::new(Square::E1, Square::D2, MoveType::QUIET),
            Move::new(Square::E1, Square::E2, MoveType::QUIET),
            Move::new(Square::E1, Square::F1, MoveType::QUIET),
            Move::new(Square::E1, Square::F2, MoveType::QUIET),
            // Bishop
            Move::new(Square::C3, Square::B2, MoveType::QUIET),
            Move::new(Square::C3, Square::A1, MoveType::QUIET),
            Move::new(Square::C3, Square::D2, MoveType::QUIET),
            Move::new(Square::C3, Square::B4, MoveType::QUIET),
            Move::new(Square::C3, Square::A5, MoveType::QUIET),
            Move::new(Square::C3, Square::D4, MoveType::QUIET),
            Move::new(Square::C3, Square::E5, MoveType::QUIET),
            Move::new(Square::C3, Square::F6, MoveType::QUIET),
            Move::new(Square::C3, Square::G7, MoveType::CAPTURE),
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
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::E3, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::E7, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Square::E1, Square::D1, MoveType::QUIET),
            Move::new(Square::E1, Square::D2, MoveType::QUIET),
            Move::new(Square::E1, Square::E2, MoveType::QUIET),
            Move::new(Square::E1, Square::F1, MoveType::QUIET),
            Move::new(Square::E1, Square::F2, MoveType::QUIET),
            // Rook
            Move::new(Square::E3, Square::E2, MoveType::QUIET),
            Move::new(Square::E3, Square::D3, MoveType::QUIET),
            Move::new(Square::E3, Square::C3, MoveType::QUIET),
            Move::new(Square::E3, Square::B3, MoveType::QUIET),
            Move::new(Square::E3, Square::A3, MoveType::QUIET),
            Move::new(Square::E3, Square::F3, MoveType::QUIET),
            Move::new(Square::E3, Square::G3, MoveType::QUIET),
            Move::new(Square::E3, Square::H3, MoveType::QUIET),
            Move::new(Square::E3, Square::E4, MoveType::QUIET),
            Move::new(Square::E3, Square::E5, MoveType::QUIET),
            Move::new(Square::E3, Square::E6, MoveType::QUIET),
            Move::new(Square::E3, Square::E7, MoveType::CAPTURE),
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
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::E3, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::C3, Some(piece::Piece::WHITE_QUEEN));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::C7, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::G7, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Square::E1, Square::D1, MoveType::QUIET),
            Move::new(Square::E1, Square::D2, MoveType::QUIET),
            Move::new(Square::E1, Square::E2, MoveType::QUIET),
            Move::new(Square::E1, Square::F1, MoveType::QUIET),
            Move::new(Square::E1, Square::F2, MoveType::QUIET),
            // Pawn
            Move::new(Square::E3, Square::E4, MoveType::QUIET),
            // Queen ranks and files
            Move::new(Square::C3, Square::C2, MoveType::QUIET),
            Move::new(Square::C3, Square::C1, MoveType::QUIET),
            Move::new(Square::C3, Square::B3, MoveType::QUIET),
            Move::new(Square::C3, Square::A3, MoveType::QUIET),
            Move::new(Square::C3, Square::D3, MoveType::QUIET),
            Move::new(Square::C3, Square::C4, MoveType::QUIET),
            Move::new(Square::C3, Square::C5, MoveType::QUIET),
            Move::new(Square::C3, Square::C6, MoveType::QUIET),
            Move::new(Square::C3, Square::C7, MoveType::CAPTURE),
            // Queen diagonals
            Move::new(Square::C3, Square::B2, MoveType::QUIET),
            Move::new(Square::C3, Square::A1, MoveType::QUIET),
            Move::new(Square::C3, Square::D2, MoveType::QUIET),
            Move::new(Square::C3, Square::B4, MoveType::QUIET),
            Move::new(Square::C3, Square::A5, MoveType::QUIET),
            Move::new(Square::C3, Square::D4, MoveType::QUIET),
            Move::new(Square::C3, Square::E5, MoveType::QUIET),
            Move::new(Square::C3, Square::F6, MoveType::QUIET),
            Move::new(Square::C3, Square::G7, MoveType::CAPTURE),
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
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::B7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::A8, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Square::C8, Some(piece::Piece::BLACK_BISHOP));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Square::E1, Square::D1, MoveType::QUIET),
            Move::new(Square::E1, Square::D2, MoveType::QUIET),
            Move::new(Square::E1, Square::E2, MoveType::QUIET),
            Move::new(Square::E1, Square::F1, MoveType::QUIET),
            Move::new(Square::E1, Square::F2, MoveType::QUIET),
            // Pawns
            Move::new(
                Square::B7,
                Square::A8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Knight),
            ),
            Move::new(
                Square::B7,
                Square::A8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
            ),
            Move::new(
                Square::B7,
                Square::A8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Rook),
            ),
            Move::new(
                Square::B7,
                Square::A8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Queen),
            ),
            Move::new(
                Square::B7,
                Square::B8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Knight),
            ),
            Move::new(
                Square::B7,
                Square::B8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Bishop),
            ),
            Move::new(
                Square::B7,
                Square::B8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Rook),
            ),
            Move::new(
                Square::B7,
                Square::B8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Queen),
            ),
            Move::new(
                Square::B7,
                Square::C8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Knight),
            ),
            Move::new(
                Square::B7,
                Square::C8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
            ),
            Move::new(
                Square::B7,
                Square::C8,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Rook),
            ),
            Move::new(
                Square::B7,
                Square::C8,
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
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::B2, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::A1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Square::C1, Some(piece::Piece::WHITE_BISHOP));
        pos.set_side_to_move(Side::Black);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Square::E8, Square::D7, MoveType::QUIET),
            Move::new(Square::E8, Square::D8, MoveType::QUIET),
            Move::new(Square::E8, Square::E7, MoveType::QUIET),
            Move::new(Square::E8, Square::F7, MoveType::QUIET),
            Move::new(Square::E8, Square::F8, MoveType::QUIET),
            // Pawns
            Move::new(
                Square::B2,
                Square::A1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Knight),
            ),
            Move::new(
                Square::B2,
                Square::A1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
            ),
            Move::new(
                Square::B2,
                Square::A1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Rook),
            ),
            Move::new(
                Square::B2,
                Square::A1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Queen),
            ),
            Move::new(
                Square::B2,
                Square::B1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Knight),
            ),
            Move::new(
                Square::B2,
                Square::B1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Bishop),
            ),
            Move::new(
                Square::B2,
                Square::B1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Rook),
            ),
            Move::new(
                Square::B2,
                Square::B1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Queen),
            ),
            Move::new(
                Square::B2,
                Square::C1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Knight),
            ),
            Move::new(
                Square::B2,
                Square::C1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
            ),
            Move::new(
                Square::B2,
                Square::C1,
                MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Rook),
            ),
            Move::new(
                Square::B2,
                Square::C1,
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
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::D5, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::C5, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        pos.set_en_passant_square(Bitboard::C6);
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Square::E1, Square::D1, MoveType::QUIET),
            Move::new(Square::E1, Square::D2, MoveType::QUIET),
            Move::new(Square::E1, Square::E2, MoveType::QUIET),
            Move::new(Square::E1, Square::F2, MoveType::QUIET),
            Move::new(Square::E1, Square::F1, MoveType::QUIET),
            // Pawn
            Move::new(Square::D5, Square::D6, MoveType::QUIET),
            Move::new(Square::D5, Square::C6, MoveType::EN_PASSANT_CAPTURE),
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
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::D4, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::C4, Some(piece::Piece::WHITE_PAWN));
        pos.set_side_to_move(Side::Black);
        pos.set_castling_rights(CastlingRights::empty());
        pos.set_en_passant_square(Bitboard::C3);
        MoveGenerator::generate_moves(&mut move_list, &pos);

        let expected_moves = [
            // King
            Move::new(Square::E8, Square::D8, MoveType::QUIET),
            Move::new(Square::E8, Square::D7, MoveType::QUIET),
            Move::new(Square::E8, Square::E7, MoveType::QUIET),
            Move::new(Square::E8, Square::F7, MoveType::QUIET),
            Move::new(Square::E8, Square::F8, MoveType::QUIET),
            // Pawn
            Move::new(Square::D4, Square::D3, MoveType::QUIET),
            Move::new(Square::D4, Square::C3, MoveType::EN_PASSANT_CAPTURE),
        ];

        assert_eq!(expected_moves.len(), move_list.len());
        for exp_move in &expected_moves {
            assert!(move_list.contains(exp_move));
        }
    }

    #[test]
    fn white_castles() {
        let kingside_castle = Move::new(Square::E1, Square::G1, MoveType::CASTLE_KINGSIDE);
        let queenside_castle = Move::new(Square::E1, Square::C1, MoveType::CASTLE_QUEENSIDE);

        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::A1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Square::H1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
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
        pos_blocked.set_piece_at(Square::G1, Some(piece::Piece::WHITE_KNIGHT));
        pos_blocked.set_piece_at(Square::B1, Some(piece::Piece::BLACK_KNIGHT));
        MoveGenerator::generate_moves(&mut move_list, &pos_blocked);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // King attacked
        let mut pos_in_check = pos.clone();
        pos_in_check.set_piece_at(Square::E2, Some(piece::Piece::BLACK_ROOK));
        MoveGenerator::generate_moves(&mut move_list, &pos_in_check);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // Square traversed by king attacked
        let mut pos_traverse_attacked = pos.clone();
        pos_traverse_attacked.set_piece_at(Square::E2, Some(piece::Piece::BLACK_BISHOP));
        MoveGenerator::generate_moves(&mut move_list, &pos_traverse_attacked);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // Target square attacked
        let mut pos_target_attacked = pos.clone();
        pos_target_attacked.set_piece_at(Square::E3, Some(piece::Piece::BLACK_BISHOP));
        MoveGenerator::generate_moves(&mut move_list, &pos_target_attacked);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // Rook attacked (castling is legal)
        let mut pos_rook_attacked = pos;
        pos_rook_attacked.set_piece_at(Square::E4, Some(piece::Piece::BLACK_BISHOP));
        pos_rook_attacked.set_piece_at(Square::E5, Some(piece::Piece::BLACK_BISHOP));
        MoveGenerator::generate_moves(&mut move_list, &pos_rook_attacked);
        assert!(move_list.contains(&kingside_castle));
        assert!(move_list.contains(&queenside_castle));
    }

    #[test]
    fn black_castles() {
        let kingside_castle = Move::new(Square::E8, Square::G8, MoveType::CASTLE_KINGSIDE);
        let queenside_castle = Move::new(Square::E8, Square::C8, MoveType::CASTLE_QUEENSIDE);

        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::A8, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Square::H8, Some(piece::Piece::BLACK_ROOK));
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
        pos_blocked.set_piece_at(Square::G8, Some(piece::Piece::BLACK_KNIGHT));
        pos_blocked.set_piece_at(Square::B8, Some(piece::Piece::WHITE_KNIGHT));
        MoveGenerator::generate_moves(&mut move_list, &pos_blocked);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // King attacked
        let mut pos_in_check = pos.clone();
        pos_in_check.set_piece_at(Square::E7, Some(piece::Piece::WHITE_ROOK));
        MoveGenerator::generate_moves(&mut move_list, &pos_in_check);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // Square traversed by king attacked
        let mut pos_traverse_attacked = pos.clone();
        pos_traverse_attacked.set_piece_at(Square::E7, Some(piece::Piece::WHITE_BISHOP));
        MoveGenerator::generate_moves(&mut move_list, &pos_traverse_attacked);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // Target square attacked
        let mut pos_target_attacked = pos.clone();
        pos_target_attacked.set_piece_at(Square::E6, Some(piece::Piece::WHITE_BISHOP));
        MoveGenerator::generate_moves(&mut move_list, &pos_target_attacked);
        assert!(!move_list.contains(&kingside_castle));
        assert!(!move_list.contains(&queenside_castle));

        // Rook attacked (castling is legal)
        let mut pos_rook_attacked = pos;
        pos_rook_attacked.set_piece_at(Square::E4, Some(piece::Piece::WHITE_BISHOP));
        pos_rook_attacked.set_piece_at(Square::E5, Some(piece::Piece::WHITE_BISHOP));
        MoveGenerator::generate_moves(&mut move_list, &pos_rook_attacked);
        assert!(move_list.contains(&kingside_castle));
        assert!(move_list.contains(&queenside_castle));
    }

    #[test]
    fn king_not_left_in_check_after_pawn_moves() {
        let mut move_list = MoveList::new();

        let mut pos_pawn = Position::empty();
        pos_pawn.set_piece_at(Square::D2, Some(piece::Piece::WHITE_KING));
        pos_pawn.set_piece_at(Square::E2, Some(piece::Piece::WHITE_PAWN));
        pos_pawn.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos_pawn.set_piece_at(Square::H2, Some(piece::Piece::BLACK_ROOK));
        pos_pawn.set_piece_at(Square::F3, Some(piece::Piece::BLACK_ROOK));
        pos_pawn.set_side_to_move(Side::White);
        pos_pawn.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos_pawn);
        assert!(!move_list.contains(&Move::new(Square::E2, Square::E3, MoveType::QUIET)));
        assert!(!move_list.contains(&Move::new(
            Square::E2,
            Square::E4,
            MoveType::DOUBLE_PAWN_PUSH
        )));
        assert!(!move_list.contains(&Move::new(Square::E2, Square::F3, MoveType::CAPTURE)));

        let mut pos_pawn_promo = Position::empty();
        pos_pawn_promo.set_piece_at(Square::A7, Some(piece::Piece::WHITE_KING));
        pos_pawn_promo.set_piece_at(Square::B7, Some(piece::Piece::WHITE_PAWN));
        pos_pawn_promo.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos_pawn_promo.set_piece_at(Square::H7, Some(piece::Piece::BLACK_ROOK));
        pos_pawn_promo.set_piece_at(Square::C8, Some(piece::Piece::BLACK_ROOK));
        pos_pawn_promo.set_side_to_move(Side::White);
        pos_pawn_promo.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos_pawn_promo);
        assert!(!move_list.contains(&Move::new(
            Square::B7,
            Square::B8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Queen)
        )));
        assert!(!move_list.contains(&Move::new(
            Square::B7,
            Square::C8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Queen)
        )));

        let mut pos_pawn_en_passant = Position::empty();
        pos_pawn_en_passant.set_piece_at(Square::E2, Some(piece::Piece::WHITE_KING));
        pos_pawn_en_passant.set_piece_at(Square::C5, Some(piece::Piece::WHITE_PAWN));
        pos_pawn_en_passant.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos_pawn_en_passant.set_piece_at(Square::A6, Some(piece::Piece::BLACK_BISHOP));
        pos_pawn_en_passant.set_piece_at(Square::B5, Some(piece::Piece::BLACK_PAWN));
        pos_pawn_en_passant.set_side_to_move(Side::White);
        pos_pawn_en_passant.set_castling_rights(CastlingRights::empty());
        pos_pawn_en_passant.set_en_passant_square(Bitboard::B6);
        MoveGenerator::generate_moves(&mut move_list, &pos_pawn_en_passant);
        assert!(!move_list.contains(&Move::new(
            Square::C5,
            Square::B6,
            MoveType::EN_PASSANT_CAPTURE
        )));
    }

    #[test]
    fn king_not_left_in_check_after_knight_moves() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::G1, Some(piece::Piece::WHITE_KNIGHT));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::H1, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&Move::new(Square::G1, Square::E2, MoveType::QUIET)));
        assert!(!move_list.contains(&Move::new(Square::G1, Square::F3, MoveType::QUIET)));
        assert!(!move_list.contains(&Move::new(Square::G1, Square::H3, MoveType::QUIET)));
    }

    #[test]
    fn king_not_left_in_check_after_bishop_moves() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::G1, Some(piece::Piece::WHITE_BISHOP));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::H1, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&Move::new(Square::G1, Square::F2, MoveType::QUIET)));
        assert!(!move_list.contains(&Move::new(Square::G1, Square::H2, MoveType::QUIET)));
    }

    #[test]
    fn king_not_left_in_check_after_rook_moves() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::G1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::H1, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&Move::new(Square::G1, Square::G2, MoveType::QUIET)));
        assert!(move_list.contains(&Move::new(Square::G1, Square::F1, MoveType::QUIET)));
        assert!(move_list.contains(&Move::new(Square::G1, Square::H1, MoveType::CAPTURE)));
    }

    #[test]
    fn king_not_left_in_check_after_queen_moves() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::G1, Some(piece::Piece::WHITE_QUEEN));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::H1, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&Move::new(Square::G1, Square::F2, MoveType::QUIET)));
        assert!(!move_list.contains(&Move::new(Square::G1, Square::G2, MoveType::QUIET)));
        assert!(move_list.contains(&Move::new(Square::G1, Square::F1, MoveType::QUIET)));
        assert!(move_list.contains(&Move::new(Square::G1, Square::H1, MoveType::CAPTURE)));
    }

    #[test]
    fn king_does_not_move_into_check() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::G1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::H2, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&Move::new(Square::E1, Square::E2, MoveType::QUIET)));
    }

    #[test]
    fn capture_pawn_in_check() {
        let mut move_list = MoveList::new();
        let pos = Fen::str_to_pos("rnbq1k1r/pp1Pb1pp/2p5/8/2B2p2/6K1/PPP1N1PP/RNBQ3R w - - 0 10")
            .unwrap();
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(
            move_list.contains(&Move::new(Square::C1, Square::F4, MoveType::CAPTURE)),
            "Position\n{}\nMovelist: {}",
            pos,
            move_list
        );
    }

    #[test]
    fn en_passant_capture_illegal() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Square::A5, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::B5, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::H4, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::H5, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Square::C5, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        pos.set_en_passant_square(Bitboard::C6);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&Move::new(
            Square::B5,
            Square::C6,
            MoveType::EN_PASSANT_CAPTURE
        )));
    }

    #[test]
    fn en_passant_capture_in_check() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Square::A4, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::C5, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::H4, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::B5, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        pos.set_en_passant_square(Bitboard::B6);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(move_list.contains(&Move::new(
            Square::C5,
            Square::B6,
            MoveType::EN_PASSANT_CAPTURE
        )));
    }

    #[test]
    fn en_passant_capture_in_check_illegal_king_attacked_by_bishop() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Square::A5, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::B5, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::H4, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::D8, Some(piece::Piece::BLACK_BISHOP));
        pos.set_piece_at(Square::C5, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        pos.set_en_passant_square(Bitboard::C6);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&Move::new(
            Square::B5,
            Square::C6,
            MoveType::EN_PASSANT_CAPTURE
        )));
    }

    #[test]
    fn en_passant_capture_in_check_illegal_own_pawn_pinned() {
        let mut move_list = MoveList::new();

        let mut pos = Position::empty();
        pos.set_piece_at(Square::A4, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::A5, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::H4, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::A8, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Square::B5, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        pos.set_en_passant_square(Bitboard::B6);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&Move::new(
            Square::A5,
            Square::B6,
            MoveType::EN_PASSANT_CAPTURE
        )));
    }
}
