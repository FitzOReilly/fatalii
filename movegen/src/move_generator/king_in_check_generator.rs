use crate::move_generator::move_generator_template::MoveGeneratorTemplate;

use crate::attacks_to::AttacksTo;
use crate::bitboard::Bitboard;
use crate::pawn::Pawn;
use crate::r#move::MoveList;
use crate::square::Square;

// In check, the only legal moves are:
// - Move the king to safety
// - Capture the attacker
// - Block the attack (only if the attacker is a sliding piece)
// In double check, only king moves are legal
pub struct KingInCheckGenerator;

impl MoveGeneratorTemplate for KingInCheckGenerator {
    fn non_capture_target_filter(attacks_to_king: &AttacksTo, targets: Bitboard) -> Bitboard {
        debug_assert_eq!(1, attacks_to_king.attack_origins.pop_count());
        // Blocking is only possible, if the king is attacked by a sliding piece.
        // Otherwise, the king must move or the attacker must be captured.
        match attacks_to_king.each_slider_attack.first() {
            Some(slider) => targets & slider.targets() & !attacks_to_king.pos.occupancy(),
            None => Bitboard::EMPTY,
        }
    }

    fn capture_target_filter(attacks_to_king: &AttacksTo, targets: Bitboard) -> Bitboard {
        debug_assert_eq!(1, attacks_to_king.attack_origins.pop_count());
        targets & attacks_to_king.attack_origins
    }

    fn pawn_capture_target_filter(attacks_to_king: &AttacksTo, targets: Bitboard) -> Bitboard {
        targets & (attacks_to_king.attack_origins | attacks_to_king.pos.en_passant_square())
    }

    fn is_legal_non_capture(attacks_to_king: &AttacksTo, origin: Square, target: Square) -> bool {
        let pos = attacks_to_king.pos;
        let origin_bb = Bitboard::from_square(origin);
        let target_bb = Bitboard::from_square(target);
        let occupancy_after_move = pos.occupancy() & !origin_bb | target_bb;
        let own_king = Bitboard::from_square(attacks_to_king.target);
        let king_in_check_after_move = attacks_to_king
            .each_xray
            .iter()
            .map(|x| Self::sliding_piece_targets(x, occupancy_after_move))
            .any(|x| x & own_king != Bitboard::EMPTY);
        !king_in_check_after_move
    }

    fn is_legal_capture(attacks_to_king: &AttacksTo, origin: Square, target: Square) -> bool {
        let pos = attacks_to_king.pos;
        let origin_bb = Bitboard::from_square(origin);
        let occupancy_after_move = pos.occupancy() & !origin_bb;
        let own_king = Bitboard::from_square(attacks_to_king.target);
        let king_in_check_after_move = attacks_to_king
            .each_xray
            .iter()
            .filter(|x| (x.origin() != target) && (x.targets() & origin_bb != Bitboard::EMPTY))
            .map(|x| Self::sliding_piece_targets(x, occupancy_after_move))
            .any(|x| x & own_king != Bitboard::EMPTY);
        !king_in_check_after_move
    }

    fn is_legal_en_passant_capture(
        attacks_to_king: &AttacksTo,
        origin: Square,
        target: Square,
    ) -> bool {
        let pos = attacks_to_king.pos;
        let origin_bb = Bitboard::from_square(origin);
        let target_bb = Bitboard::from_square(target);
        let captured_square = Pawn::push_origin(target, pos.side_to_move());
        let captured_bb = Bitboard::from_square(captured_square);
        if captured_bb == attacks_to_king.attack_origins {
            // Our king is attacked by the pawn that just moved. In this case we must check if our
            // own pawn is pinned to the king. The opponent's pawn is not blocking a sliding attack
            // (otherwise our king would already have been attacked before the opponent's move
            // which would be an illegal position).
            let own_king = Bitboard::from_square(attacks_to_king.target);
            let occupancy_after_move = pos.occupancy() & !origin_bb & !captured_bb | target_bb;
            let king_in_check_after_move = attacks_to_king
                .each_xray
                .iter()
                .filter(|x| x.targets() & origin_bb != Bitboard::EMPTY)
                .map(|x| Self::sliding_piece_targets(x, occupancy_after_move))
                .any(|x| x & own_king != Bitboard::EMPTY);
            !king_in_check_after_move
        } else {
            // Our king is attacked by a sliding piece (after a discovered attack). Capturing en
            // passant is not possible because the attacker will not be blocked.
            false
        }
    }

    fn is_legal_king_move(attacks_to_king: &AttacksTo, origin: Square, target: Square) -> bool {
        debug_assert_ne!(Bitboard::EMPTY, attacks_to_king.attack_origins);
        let pos = attacks_to_king.pos;
        let target_bb = Bitboard::from_square(target);
        match target_bb & attacks_to_king.xrays_to_target {
            Bitboard::EMPTY => true,
            _ => {
                let origin_bb = Bitboard::from_square(origin);
                let occupancy_after_move = pos.occupancy() & !origin_bb | target_bb;
                let king_in_check_after_move = attacks_to_king.each_slider_attack.iter().any(|x| {
                    (x.targets() & origin_bb != Bitboard::EMPTY)
                        && (Self::sliding_piece_targets(x, occupancy_after_move) & target_bb)
                            != Bitboard::EMPTY
                });
                !king_in_check_after_move
            }
        }
    }

    // Castling is illegal while in check
    fn generate_castles(_move_list: &mut MoveList, _attacks_to_king: &AttacksTo) {}
    fn generate_white_castles(_move_list: &mut MoveList, _attacks_to_king: &AttacksTo) {}
    fn generate_black_castles(_move_list: &mut MoveList, _attacks_to_king: &AttacksTo) {}
}
