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
use crate::r#move::{Move, MoveList};
use crate::square::Square;

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

    pub fn generate_moves_quiescence(move_list: &mut MoveList, pos: &Position) {
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
                KingInCheckGenerator::generate_king_captures(move_list, &attacks_to_king);
            } else {
                KingInCheckGenerator::generate_moves_quiescence(move_list, &attacks_to_king);
            }
        } else if king_xrayed {
            KingXrayedGenerator::generate_moves_quiescence(move_list, &attacks_to_king);
        } else {
            KingNotXrayedGenerator::generate_moves_quiescence(move_list, &attacks_to_king);
        }
    }

    pub fn least_valuable_attacker(pos: &Position, target: Square) -> Option<Move> {
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
                KingInCheckGenerator::king_attacker(&attacks_to_king, target)
            } else {
                KingInCheckGenerator::least_valuable_attacker(&attacks_to_king, target)
            }
        } else if king_xrayed {
            KingXrayedGenerator::least_valuable_attacker(&attacks_to_king, target)
        } else {
            KingNotXrayedGenerator::least_valuable_attacker(&attacks_to_king, target)
        }
    }

    pub fn has_en_passant_capture(pos: &Position) -> bool {
        let own_king_bb = pos.piece_occupancy(pos.side_to_move(), piece::Type::King);
        let own_king = own_king_bb.to_square();

        let attacks_to_king = AttacksTo::new(pos, own_king, !pos.side_to_move());

        let king_in_check = own_king_bb & attacks_to_king.all_attack_targets != Bitboard::EMPTY;
        let king_xrayed = attacks_to_king.xrays_to_target != Bitboard::EMPTY;

        if king_in_check {
            debug_assert!(attacks_to_king.attack_origins.pop_count() >= 1);
            debug_assert!(attacks_to_king.attack_origins.pop_count() <= 2);
            debug_assert!(attacks_to_king.each_slider_attack.len() <= 2);
            KingInCheckGenerator::has_en_passant_capture(&attacks_to_king)
        } else if king_xrayed {
            KingXrayedGenerator::has_en_passant_capture(&attacks_to_king)
        } else {
            KingNotXrayedGenerator::has_en_passant_capture(&attacks_to_king)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fen::Fen;
    use crate::position::{CastlingRights, Position};
    use crate::position_history::PositionHistory;
    use crate::r#move::{Move, MoveType};
    use crate::side::Side;
    use crate::square::Square;
    use rand::seq::IndexedMutRandom;

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

        assert_eq!(
            expected_moves.len(),
            move_list.len(),
            "\nExpected moves: {}\nActual moves: {}",
            expected_moves
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            move_list
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        );
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

    #[test]
    fn random_moves_for_quiescence_and_is_in_check_and_gives_check() {
        let pos = Position::initial();
        let mut pos_hist = PositionHistory::new(pos.clone());
        let gen_then_filter = |move_list: &mut MoveList, pos: &Position| {
            MoveGenerator::generate_moves(move_list, pos);
            move_list.retain(|x| x.is_capture() || x.promotion_piece() == Some(piece::Type::Queen));
        };

        // Use simple and slow closures to verify the more complex and faster
        // Position::is_in_check and Position::gives_check
        let verify_is_in_check = |pos: &Position| {
            let simple_is_in_check = |pos: &Position| -> bool {
                pos.piece_occupancy(pos.side_to_move(), piece::Type::King)
                    & pos.attacked_squares(!pos.side_to_move())
                    != Bitboard::EMPTY
            };
            assert_eq!(simple_is_in_check(pos), pos.is_in_check(pos.side_to_move()));
        };
        let verify_gives_check = |pos_hist: &mut PositionHistory, move_list: &MoveList| {
            let simple_gives_check = |pos_hist: &mut PositionHistory, m: Move| -> bool {
                pos_hist.do_move(m);
                let pos = pos_hist.current_pos();
                let res = pos.is_in_check(pos.side_to_move());
                pos_hist.undo_last_move();
                res
            };
            for m in move_list.iter() {
                assert_eq!(
                    simple_gives_check(pos_hist, *m),
                    pos_hist.current_pos().gives_check(*m),
                );
            }
        };

        let game_count = 100;
        let max_moves = 100;

        let mut move_list = MoveList::new();
        let mut move_list_fn = MoveList::new();
        let mut move_list_closure = MoveList::new();

        for _ in 0..game_count {
            for _ in 0..max_moves {
                let side_to_move = pos_hist.current_pos().side_to_move();
                if !pos_hist.current_pos().is_in_check(side_to_move) {
                    MoveGenerator::generate_moves_quiescence(
                        &mut move_list_fn,
                        pos_hist.current_pos(),
                    );
                    gen_then_filter(&mut move_list_closure, pos_hist.current_pos());

                    assert_eq!(move_list_fn.len(), move_list_closure.len());
                    for m in move_list_fn.iter() {
                        assert!(move_list_closure.contains(m));
                    }
                }

                verify_is_in_check(pos_hist.current_pos());
                MoveGenerator::generate_moves(&mut move_list, pos_hist.current_pos());
                verify_gives_check(&mut pos_hist, &move_list);
                match move_list.choose_mut(&mut rand::rng()) {
                    Some(m) => pos_hist.do_move(*m),
                    None => pos_hist = PositionHistory::new(pos.clone()),
                }
            }
        }
    }

    #[test]
    fn castles_chess_960_castling_rights() {
        // White queenside
        let fen = "7k/8/8/8/8/8/8/1RK3R1 w B - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        let white_kingside_castle = Move::new(Square::C1, Square::G1, MoveType::CASTLE_KINGSIDE);
        let white_queenside_castle = Move::new(Square::C1, Square::C1, MoveType::CASTLE_QUEENSIDE);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&white_kingside_castle));
        assert!(move_list.contains(&white_queenside_castle));

        // White kingside
        let fen = "7k/8/8/8/8/8/8/1RK3R1 w G - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(move_list.contains(&white_kingside_castle));
        assert!(!move_list.contains(&white_queenside_castle));

        // Black queenside
        let fen = "1rk3r1/8/8/8/8/8/8/7K b b - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        let black_kingside_castle = Move::new(Square::C8, Square::G8, MoveType::CASTLE_KINGSIDE);
        let black_queenside_castle = Move::new(Square::C8, Square::C8, MoveType::CASTLE_QUEENSIDE);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&black_kingside_castle));
        assert!(move_list.contains(&black_queenside_castle));

        // Black kingside
        let fen = "1rk3r1/8/8/8/8/8/8/7K b g - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(move_list.contains(&black_kingside_castle));
        assert!(!move_list.contains(&black_queenside_castle));

        // Edge case for queenside castling:
        // Opponents queen/rook on a1/a8 is blocked by our rook on b1/b8.
        // The king would be attacked after castling queenside, so it's illegal.
        // White
        let white_queenside_castle = Move::new(Square::C1, Square::C1, MoveType::CASTLE_QUEENSIDE);

        let fen = "4k3/8/8/8/8/8/8/rRK5 w B - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&white_queenside_castle));

        let fen = "4k3/8/8/8/8/8/8/qRK5 w B - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&white_queenside_castle));

        // Black
        let black_queenside_castle = Move::new(Square::C8, Square::C8, MoveType::CASTLE_QUEENSIDE);

        let fen = "Rrk5/8/8/8/8/8/8/4K3 b b - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&black_queenside_castle));

        let fen = "Qrk5/8/8/8/8/8/8/4K3 b b - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&black_queenside_castle));
    }

    #[test]
    fn castles_chess_960_king_moves_away_from_rook() {
        // White
        let fen = "7k/8/8/8/8/8/8/RK6 w A - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        let white_queenside_castle = Move::new(Square::B1, Square::C1, MoveType::CASTLE_QUEENSIDE);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(move_list.contains(&white_queenside_castle));

        // Black
        let fen = "rk6/8/8/8/8/8/8/7K b a - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        let black_queenside_castle = Move::new(Square::B8, Square::C8, MoveType::CASTLE_QUEENSIDE);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(move_list.contains(&black_queenside_castle));
    }

    #[test]
    fn castles_chess_960_square_attacked() {
        // White kingside
        let fen = "2rk4/8/8/8/8/8/8/RK5R w H - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        let white_kingside_castle = Move::new(Square::B1, Square::G1, MoveType::CASTLE_KINGSIDE);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&white_kingside_castle));

        // White queenside
        let fen = "5rk1/8/8/8/8/8/8/R5KR w A - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        let white_queenside_castle = Move::new(Square::G1, Square::C1, MoveType::CASTLE_QUEENSIDE);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&white_queenside_castle));

        // Black kingside
        let fen = "rk5r/8/8/8/8/8/8/2RK4 b h - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        let black_kingside_castle = Move::new(Square::B8, Square::G8, MoveType::CASTLE_KINGSIDE);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&black_kingside_castle));

        // Black queenside
        let fen = "r5kr/8/8/8/8/8/8/5RK1 b a - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        let black_queenside_castle = Move::new(Square::G8, Square::C8, MoveType::CASTLE_QUEENSIDE);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&black_queenside_castle));
    }

    #[test]
    fn castles_chess_960_path_blocked() {
        // White kingside
        let fen = "4k3/8/8/8/8/8/8/RKN4R w H - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        let white_kingside_castle = Move::new(Square::B1, Square::G1, MoveType::CASTLE_KINGSIDE);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&white_kingside_castle));

        // White queenside
        let fen = "4k3/8/8/8/8/8/8/RN4KR w A - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        let white_queenside_castle = Move::new(Square::G1, Square::C1, MoveType::CASTLE_QUEENSIDE);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&white_queenside_castle));

        // Black kingside
        let fen = "rkn4r/8/8/8/8/8/8/4K3 b h - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        let black_kingside_castle = Move::new(Square::B8, Square::G8, MoveType::CASTLE_KINGSIDE);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&black_kingside_castle));

        // Black queenside
        let fen = "rn4kr/8/8/8/8/8/8/4K3 b a - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut move_list = MoveList::new();

        let black_queenside_castle = Move::new(Square::G8, Square::C8, MoveType::CASTLE_QUEENSIDE);

        MoveGenerator::generate_moves(&mut move_list, &pos);
        assert!(!move_list.contains(&black_queenside_castle));
    }

    #[test]
    fn has_en_passant_capture() {
        let pos = Position::initial();
        let mut pos_hist = PositionHistory::new(pos);

        let d2d4 = Move::new(Square::D2, Square::D4, MoveType::DOUBLE_PAWN_PUSH);
        let c7c5 = Move::new(Square::C7, Square::C5, MoveType::DOUBLE_PAWN_PUSH);
        let d4d5 = Move::new(Square::D4, Square::D5, MoveType::QUIET);
        let e7e5 = Move::new(Square::E7, Square::E5, MoveType::DOUBLE_PAWN_PUSH);

        pos_hist.do_move(d2d4);
        assert!(!MoveGenerator::has_en_passant_capture(
            pos_hist.current_pos()
        ));
        pos_hist.do_move(c7c5);
        assert!(!MoveGenerator::has_en_passant_capture(
            pos_hist.current_pos()
        ));
        pos_hist.do_move(d4d5);
        assert!(!MoveGenerator::has_en_passant_capture(
            pos_hist.current_pos()
        ));
        pos_hist.do_move(e7e5);
        assert!(MoveGenerator::has_en_passant_capture(
            pos_hist.current_pos()
        ));
    }
}
