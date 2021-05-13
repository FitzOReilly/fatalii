use crate::move_generator::move_generator_template::MoveGeneratorTemplate;

use crate::attacks_to::AttacksTo;
use crate::bitboard::Bitboard;
use crate::pawn::Pawn;
use crate::square::Square;

// Legality checks are done for
// - King move targets
// - Castles
// - Other pieces:
//   Only if the piece is attacked and xrayed (potentially pinned to the king)
// - En passant:
//   Only if own or opponent pawn is attacked and xrayed (potentially pinned to the king)
pub struct KingXrayedGenerator;

impl MoveGeneratorTemplate for KingXrayedGenerator {
    fn non_capture_target_filter(attacks_to_king: &AttacksTo, targets: Bitboard) -> Bitboard {
        targets & !attacks_to_king.pos.occupancy()
    }

    fn capture_target_filter(attacks_to_king: &AttacksTo, targets: Bitboard) -> Bitboard {
        targets
            & attacks_to_king
                .pos
                .side_occupancy(!attacks_to_king.pos.side_to_move())
    }

    fn pawn_capture_target_filter(attacks_to_king: &AttacksTo, targets: Bitboard) -> Bitboard {
        targets
            & (attacks_to_king
                .pos
                .side_occupancy(!attacks_to_king.pos.side_to_move())
                | attacks_to_king.pos.en_passant_square())
    }

    fn is_legal_non_capture(attacks_to_king: &AttacksTo, origin: Square, target: Square) -> bool {
        debug_assert!(!attacks_to_king.each_xray.is_empty());
        let pos = attacks_to_king.pos;
        let origin_bb = Bitboard::from_square(origin);
        match origin_bb & attacks_to_king.all_attack_targets & attacks_to_king.xrays_to_target {
            Bitboard::EMPTY => true,
            _ => {
                let target_bb = Bitboard::from_square(target);
                let own_king = Bitboard::from_square(attacks_to_king.target);
                let occupancy_after_move = pos.occupancy() & !origin_bb | target_bb;
                let king_in_check_after_move = attacks_to_king
                    .each_xray
                    .iter()
                    .filter(|x| x.targets() & origin_bb != Bitboard::EMPTY)
                    .map(|x| Self::sliding_piece_targets(x, occupancy_after_move))
                    .any(|x| x & own_king != Bitboard::EMPTY);
                !king_in_check_after_move
            }
        }
    }

    fn is_legal_capture(attacks_to_king: &AttacksTo, origin: Square, target: Square) -> bool {
        debug_assert!(!attacks_to_king.each_xray.is_empty());
        let pos = attacks_to_king.pos;
        let origin_bb = Bitboard::from_square(origin);
        match origin_bb & attacks_to_king.all_attack_targets & attacks_to_king.xrays_to_target {
            Bitboard::EMPTY => true,
            _ => {
                let own_king = Bitboard::from_square(attacks_to_king.target);
                let occupancy_after_move = pos.occupancy() & !origin_bb;
                let king_in_check_after_move = attacks_to_king
                    .each_xray
                    .iter()
                    .filter(|x| {
                        (x.origin() != target) && (x.targets() & origin_bb != Bitboard::EMPTY)
                    })
                    .map(|x| Self::sliding_piece_targets(x, occupancy_after_move))
                    .any(|x| x & own_king != Bitboard::EMPTY);
                !king_in_check_after_move
            }
        }
    }

    fn is_legal_en_passant_capture(
        attacks_to_king: &AttacksTo,
        origin: Square,
        target: Square,
    ) -> bool {
        debug_assert!(!attacks_to_king.each_xray.is_empty());
        let pos = attacks_to_king.pos;
        let origin_bb = Bitboard::from_square(origin);
        let target_bb = Bitboard::from_square(target);
        let captured_square = Pawn::push_origin(target, pos.side_to_move());
        let captured_bb = Bitboard::from_square(captured_square);
        match (origin_bb | captured_bb)
            & attacks_to_king.all_attack_targets
            & attacks_to_king.xrays_to_target
        {
            Bitboard::EMPTY => true,
            _ => {
                let own_king = Bitboard::from_square(attacks_to_king.target);
                let occupancy_after_move = pos.occupancy() & !origin_bb & !captured_bb | target_bb;
                let king_in_check_after_move = attacks_to_king
                    .each_xray
                    .iter()
                    .filter(|x| x.targets() & (origin_bb | captured_bb) != Bitboard::EMPTY)
                    .map(|x| Self::sliding_piece_targets(x, occupancy_after_move))
                    .any(|x| x & own_king != Bitboard::EMPTY);
                !king_in_check_after_move
            }
        }
    }

    fn is_legal_king_move(_attacks_to_king: &AttacksTo, _origin: Square, _target: Square) -> bool {
        debug_assert!(_attacks_to_king.each_slider_attack.is_empty());
        true
    }
}
