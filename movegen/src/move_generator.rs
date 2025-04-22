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
    use std::collections::HashSet;

    use super::*;
    use crate::fen::Fen;
    use crate::position::Position;
    use crate::position_history::PositionHistory;
    use crate::r#move::{Move, MoveType};
    use crate::square::Square;
    use rand::seq::IndexedMutRandom;

    fn verify_generated_moves(pos: &Position, expected_moves: &[&str]) {
        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves(&mut move_list, pos);
        let expected: HashSet<_> = expected_moves.iter().map(|m| m.to_string()).collect();
        let actual: HashSet<_> = move_list.iter().map(|m| m.to_string()).collect();
        assert_eq!(
            expected,
            actual,
            "expected - actual: {:?}, actual - expected: {:?}",
            expected.difference(&actual),
            actual.difference(&expected),
        );
    }

    fn verify_generated_moves_contain(
        pos: &Position,
        expected_contained: &[&str],
        expected_not_contained: &[&str],
    ) {
        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves(&mut move_list, pos);
        let actual: HashSet<_> = move_list.iter().map(|m| m.to_string()).collect();
        for ec in expected_contained {
            assert!(
                actual.contains(*ec),
                "move list does not contain expected move\nmove list: {actual:?}\nmove: {ec}",
            );
        }
        for enc in expected_not_contained {
            assert!(
                !actual.contains(*enc),
                "move list contains unexpected move\nmove list: {actual:?}\nmove: {enc}",
            );
        }
    }

    #[test]
    fn initial_position() {
        let pos = Position::initial();
        let expected_moves = [
            "a2a3", "b2b3", "c2c3", "d2d3", "e2e3", "f2f3", "g2g3", "h2h3", "a2a4", "b2b4", "c2c4",
            "d2d4", "e2e4", "f2f4", "g2g4", "h2h4", "b1a3", "b1c3", "g1f3", "g1h3",
        ];
        verify_generated_moves(&pos, &expected_moves);
    }

    #[test]
    fn position_after_1_e4() {
        let pos = Position::initial();
        let mut pos_hist = PositionHistory::new(pos);
        pos_hist.do_move(Move::new(
            Square::E2,
            Square::E4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));
        let expected_moves = [
            "a7a6", "b7b6", "c7c6", "d7d6", "e7e6", "f7f6", "g7g6", "h7h6", "a7a5", "b7b5", "c7c5",
            "d7d5", "e7e5", "f7f5", "g7g5", "h7h5", "b8a6", "b8c6", "g8f6", "g8h6",
        ];
        verify_generated_moves(pos_hist.current_pos(), &expected_moves);
    }

    #[test]
    fn white_pawn_captures() {
        let fen = "4k3/8/8/8/8/pp4pp/P6P/4K3 w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_moves = ["e1d1", "e1d2", "e1e2", "e1f1", "a2xb3", "h2xg3"];
        verify_generated_moves(&pos, &expected_moves);
    }

    #[test]
    fn black_pawn_captures() {
        let fen = "4k3/p6p/PP4PP/8/8/8/8/4K3 b - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_moves = ["e8d8", "e8d7", "e8e7", "e8f8", "a7xb6", "h7xg6"];
        verify_generated_moves(&pos, &expected_moves);
    }

    #[test]
    fn king_moves() {
        let fen = "4k3/8/8/8/8/8/5p2/4K3 w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_moves = ["e1d1", "e1d2", "e1e2", "e1f1", "e1xf2"];
        verify_generated_moves(&pos, &expected_moves);
    }

    #[test]
    fn knight_moves() {
        let fen = "4k3/8/8/8/8/p1P5/8/1N2K3 w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_moves = [
            "e1d1", "e1d2", "e1e2", "e1f1", "e1f2", "c3c4", "b1d2", "b1xa3",
        ];
        verify_generated_moves(&pos, &expected_moves);
    }

    #[test]
    fn bishop_moves() {
        let fen = "4k3/6p1/8/8/8/2B5/8/4K3 w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_moves = [
            "e1d1", "e1d2", "e1e2", "e1f1", "e1f2", "c3b2", "c3a1", "c3d2", "c3b4", "c3a5", "c3d4",
            "c3e5", "c3f6", "c3xg7",
        ];
        verify_generated_moves(&pos, &expected_moves);
    }

    #[test]
    fn rook_moves() {
        let fen = "4k3/4p3/8/8/8/4R3/8/4K3 w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_moves = [
            "e1d1", "e1d2", "e1e2", "e1f1", "e1f2", "e3e2", "e3d3", "e3c3", "e3b3", "e3a3", "e3f3",
            "e3g3", "e3h3", "e3e4", "e3e5", "e3e6", "e3xe7",
        ];
        verify_generated_moves(&pos, &expected_moves);
    }

    #[test]
    fn queen_moves() {
        let fen = "4k3/2p3p1/8/8/8/2Q1P3/8/4K3 w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_moves = [
            "e1d1", "e1d2", "e1e2", "e1f1", "e1f2", "e3e4", "c3c2", "c3c1", "c3b3", "c3a3", "c3d3",
            "c3c4", "c3c5", "c3c6", "c3xc7", "c3b2", "c3a1", "c3d2", "c3b4", "c3a5", "c3d4",
            "c3e5", "c3f6", "c3xg7",
        ];
        verify_generated_moves(&pos, &expected_moves);
    }

    #[test]
    fn white_promotions() {
        let fen = "r1b1k3/1P6/8/8/8/8/8/4K3 w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_moves = [
            "e1d1", "e1d2", "e1e2", "e1f1", "e1f2", "b7xa8=N", "b7xa8=B", "b7xa8=R", "b7xa8=Q",
            "b7b8=N", "b7b8=B", "b7b8=R", "b7b8=Q", "b7xc8=N", "b7xc8=B", "b7xc8=R", "b7xc8=Q",
        ];
        verify_generated_moves(&pos, &expected_moves);
    }

    #[test]
    fn black_promotions() {
        let fen = "4k3/8/8/8/8/8/1p6/R1B1K3 b - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_moves = [
            "e8d7", "e8d8", "e8e7", "e8f7", "e8f8", "b2xa1=N", "b2xa1=B", "b2xa1=R", "b2xa1=Q",
            "b2b1=N", "b2b1=B", "b2b1=R", "b2b1=Q", "b2xc1=N", "b2xc1=B", "b2xc1=R", "b2xc1=Q",
        ];
        verify_generated_moves(&pos, &expected_moves);
    }

    #[test]
    fn white_en_passant_captures() {
        let fen = "4k3/8/8/2pP4/8/8/8/4K3 w - c6 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_moves = ["e1d1", "e1d2", "e1e2", "e1f2", "e1f1", "d5d6", "d5xc6"];
        verify_generated_moves(&pos, &expected_moves);
    }

    #[test]
    fn black_en_passant_captures() {
        let fen = "4k3/8/8/8/2Pp4/8/8/4K3 b - c3 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_moves = ["e8d8", "e8d7", "e8e7", "e8f7", "e8f8", "d4d3", "d4xc3"];
        verify_generated_moves(&pos, &expected_moves);
    }

    #[test]
    fn white_castles() {
        let fen = "4k3/8/8/8/8/8/8/R3K2R w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0", "0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        let fen = "4k3/8/8/8/8/8/8/R3K2R w K - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = ["0-0"];
        let expected_not_contained = ["0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        let fen = "4k3/8/8/8/8/8/8/R3K2R w Q - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = ["0-0-0"];
        let expected_not_contained = ["0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        let fen = "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = ["0-0", "0-0-0"];
        let expected_not_contained = [];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Square between king and rook blocked
        let fen = "4k3/8/8/8/8/8/8/Rn2K1NR w KQ - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0", "0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // King attacked
        let fen = "4k3/8/8/8/8/8/4r3/R3K2R w KQ - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0", "0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Square traversed by king attacked
        let fen = "4k3/8/8/8/8/8/4b3/R3K2R w KQ - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0", "0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Target square attacked
        let fen = "4k3/8/8/8/8/4b3/8/R3K2R w KQ - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0", "0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Rook attacked (castling is legal)
        let fen = "4k3/8/8/4b3/4b3/8/8/R3K2R w KQ - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = ["0-0", "0-0-0"];
        let expected_not_contained = [];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn black_castles() {
        let fen = "r3k2r/8/8/8/8/8/8/4K3 b - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0", "0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        let fen = "r3k2r/8/8/8/8/8/8/4K3 b k - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = ["0-0"];
        let expected_not_contained = ["0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        let fen = "r3k2r/8/8/8/8/8/8/4K3 b q - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = ["0-0-0"];
        let expected_not_contained = ["0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        let fen = "r3k2r/8/8/8/8/8/8/4K3 b kq - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = ["0-0", "0-0-0"];
        let expected_not_contained = [];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Square between king and rook blocked
        let fen = "rN2k1nr/8/8/8/8/8/8/4K3 b kq - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0", "0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // King attacked
        let fen = "r3k2r/4R3/8/8/8/8/8/4K3 b kq - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0", "0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Square traversed by king attacked
        let fen = "r3k2r/4B3/8/8/8/8/8/4K3 b kq - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0", "0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Target square attacked
        let fen = "r3k2r/8/4B3/8/8/8/8/4K3 b kq - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0", "0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Rook attacked (castling is legal)
        let fen = "r3k2r/8/8/4B3/4B3/8/8/4K3 b kq - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = ["0-0", "0-0-0"];
        let expected_not_contained = [];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn king_not_left_in_check_after_pawn_moves() {
        let fen = "4k3/8/8/8/8/5r2/3KP2r/8 w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["e2e3", "e2e4", "e2xf3"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        let fen = "2r1k3/KP5r/8/8/8/8/8/8 w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["b7b8=Q", "b7xc8=Q"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        let fen = "4k3/8/b7/1pP5/8/8/4K3/8 w - b6 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["c5xb6"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn king_not_left_in_check_after_knight_moves() {
        let fen = "4k3/8/8/8/8/8/8/4K1Nr w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["g1e2", "g1f3", "g1h3"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn king_not_left_in_check_after_bishop_moves() {
        let fen = "4k3/8/8/8/8/8/8/4K1Br w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["g1f2", "g1h2"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn king_not_left_in_check_after_rook_moves() {
        let fen = "4k3/8/8/8/8/8/8/4K1Rr w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = ["g1f1", "g1xh1"];
        let expected_not_contained = ["g1g2"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn king_not_left_in_check_after_queen_moves() {
        let fen = "4k3/8/8/8/8/8/8/4K1Qr w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = ["g1f1", "g1xh1"];
        let expected_not_contained = ["g1f2", "g1g2", "g1h2"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn king_does_not_move_into_check() {
        let fen = "4k3/8/8/8/8/8/7r/4K1R1 w - - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["e1e2"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn capture_pawn_in_check() {
        let fen = "rnbq1k1r/pp1Pb1pp/2p5/8/2B2p2/6K1/PPP1N1PP/RNBQ3R w - - 0 10";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = ["c1xf4", "e2xf4", "g3xf4"];
        let expected_not_contained = [];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn en_passant_capture_illegal() {
        // Pawns pinned by rook
        let fen = "8/8/8/KPp4r/7k/8/8/8 w - c6 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["b5xc6"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Opponent pawn pinned by bishop
        let fen = "5b2/8/8/1Pp5/7k/K7/8/8 w - c6 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["b5xc6"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn en_passant_capture_in_check() {
        let fen = "8/8/8/1pP5/K6k/8/8/8 w - b6 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = ["c5xb6"];
        let expected_not_contained = [];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn en_passant_capture_in_check_illegal_king_attacked_by_bishop() {
        let fen = "3b4/8/8/KPp5/7k/8/8/8 w - c6 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["b5xc6"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn en_passant_capture_in_check_illegal_own_pawn_pinned() {
        let fen = "r7/8/8/Pp6/K6k/8/8/8 w - b6 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["a5xb6"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
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
        let expected_contained = ["0-0-0"];
        let expected_not_contained = ["0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // White kingside
        let fen = "7k/8/8/8/8/8/8/1RK3R1 w G - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = ["0-0"];
        let expected_not_contained = ["0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Black queenside
        let fen = "1rk3r1/8/8/8/8/8/8/7K b b - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = ["0-0-0"];
        let expected_not_contained = ["0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Black kingside
        let fen = "1rk3r1/8/8/8/8/8/8/7K b g - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = ["0-0"];
        let expected_not_contained = ["0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Edge cases for queenside castling:
        // Opponents queen/rook on a1/a8 is blocked by our rook on b1/b8.
        // The king would be attacked after castling queenside, so it's illegal.

        // White
        let fen = "4k3/8/8/8/8/8/8/rRK5 w B - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        let fen = "4k3/8/8/8/8/8/8/qRK5 w B - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Black
        let fen = "Rrk5/8/8/8/8/8/8/4K3 b b - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        let fen = "Qrk5/8/8/8/8/8/8/4K3 b b - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn castles_chess_960_king_moves_away_from_rook() {
        // White
        let fen = "7k/8/8/8/8/8/8/RK6 w A - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = ["0-0-0"];
        let expected_not_contained = [];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Black
        let fen = "rk6/8/8/8/8/8/8/7K b a - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = ["0-0-0"];
        let expected_not_contained = [];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn castles_chess_960_square_attacked() {
        // White kingside
        let fen = "2rk4/8/8/8/8/8/8/RK5R w H - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // White queenside
        let fen = "5rk1/8/8/8/8/8/8/R5KR w A - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Black kingside
        let fen = "rk5r/8/8/8/8/8/8/2RK4 b h - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Black queenside
        let fen = "r5kr/8/8/8/8/8/8/5RK1 b a - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
    }

    #[test]
    fn castles_chess_960_path_blocked() {
        // White kingside
        let fen = "4k3/8/8/8/8/8/8/RKN4R w H - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // White queenside
        let fen = "4k3/8/8/8/8/8/8/RN4KR w A - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Black kingside
        let fen = "rkn4r/8/8/8/8/8/8/4K3 b h - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);

        // Black queenside
        let fen = "rn4kr/8/8/8/8/8/8/4K3 b a - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let expected_contained = [];
        let expected_not_contained = ["0-0-0"];
        verify_generated_moves_contain(&pos, &expected_contained, &expected_not_contained);
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
