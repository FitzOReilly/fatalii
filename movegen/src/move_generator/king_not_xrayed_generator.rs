use crate::move_generator::move_generator_template::MoveGeneratorTemplate;

use crate::attacks_to::AttacksTo;
use crate::bitboard::Bitboard;
use crate::square::Square;

// Since the king is not xrayed, legality checks are only done for:
// - King move targets
// - Castles
pub struct KingNotXrayedGenerator;

impl MoveGeneratorTemplate for KingNotXrayedGenerator {
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

    fn is_legal_non_capture(
        _attacks_to_king: &AttacksTo,
        _origin: Square,
        _target: Square,
    ) -> bool {
        true
    }

    fn is_legal_capture(_attacks_to_king: &AttacksTo, _origin: Square, _target: Square) -> bool {
        true
    }

    fn is_legal_en_passant_capture(
        _attacks_to_king: &AttacksTo,
        _origin: Square,
        _target: Square,
    ) -> bool {
        true
    }

    fn is_legal_king_move(_attacks_to_king: &AttacksTo, _origin: Square, _target: Square) -> bool {
        true
    }
}
