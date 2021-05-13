use crate::bishop::Bishop;
use crate::king::King;
use crate::knight::Knight;
use crate::pawn::Pawn;
use crate::queen::Queen;
use crate::rook::Rook;

use crate::bitboard::Bitboard;
use crate::piece;
use crate::piece_targets::PieceTargets;
use crate::position::Position;
use crate::side::Side;
use crate::square::Square;

pub struct AttacksTo<'a> {
    pub pos: &'a Position,
    pub target: Square,
    pub all_attack_targets: Bitboard,
    pub attack_origins: Bitboard,
    pub each_slider_attack: Vec<PieceTargets>,
    pub xrays_to_target: Bitboard,
    pub each_xray: Vec<PieceTargets>,
}

impl AttacksTo<'_> {
    pub fn new(pos: &Position, target: Square, attacking_side: Side) -> AttacksTo {
        let (all_pawn_targets, pawn_origins) =
            Self::pawn_attacks_towards_target(pos, target, attacking_side);
        let (all_knight_targets, knight_origins) = Self::attacks_towards_target(
            pos,
            piece::Type::Knight,
            &Knight::targets,
            target,
            attacking_side,
        );
        let (
            all_bishop_targets,
            bishop_origins,
            mut each_bishop_attack,
            bishop_xrays,
            mut each_bishop_xray,
        ) = Self::slider_attacks_towards_target(
            pos,
            piece::Type::Bishop,
            &Bishop::targets,
            target,
            attacking_side,
        );
        let (all_rook_targets, rook_origins, mut each_rook_attack, rook_xrays, mut each_rook_xray) =
            Self::slider_attacks_towards_target(
                pos,
                piece::Type::Rook,
                &Rook::targets,
                target,
                attacking_side,
            );
        let (
            all_queen_targets,
            queen_origins,
            mut each_queen_attack,
            queen_xrays,
            mut each_queen_xray,
        ) = Self::slider_attacks_towards_target(
            pos,
            piece::Type::Queen,
            &Queen::targets,
            target,
            attacking_side,
        );
        let (all_king_targets, king_origins) = Self::attacks_towards_target(
            pos,
            piece::Type::King,
            &King::targets,
            target,
            attacking_side,
        );

        let mut each_slider_attack = Vec::new();
        each_slider_attack.append(&mut each_bishop_attack);
        each_slider_attack.append(&mut each_rook_attack);
        each_slider_attack.append(&mut each_queen_attack);

        let all_attack_targets = all_pawn_targets
            | all_knight_targets
            | all_bishop_targets
            | all_rook_targets
            | all_queen_targets
            | all_king_targets;
        let attack_origins = pawn_origins
            | knight_origins
            | bishop_origins
            | rook_origins
            | queen_origins
            | king_origins;

        let xrays_to_target = bishop_xrays | rook_xrays | queen_xrays;
        let mut each_xray = Vec::new();
        each_xray.append(&mut each_bishop_xray);
        each_xray.append(&mut each_rook_xray);
        each_xray.append(&mut each_queen_xray);

        AttacksTo {
            pos,
            target,
            all_attack_targets,
            attack_origins,
            each_slider_attack,
            xrays_to_target,
            each_xray,
        }
    }

    fn pawn_attacks_towards_target(
        pos: &Position,
        target: Square,
        attacking_side: Side,
    ) -> (Bitboard, Bitboard) {
        let target_bb = Bitboard::from_square(target);
        let pawns = pos.piece_occupancy(attacking_side, piece::Type::Pawn);

        let east_targets = Pawn::east_attack_targets(pawns, attacking_side);
        let east_origins = Pawn::east_attack_origins(east_targets & target_bb, attacking_side);
        let west_targets = Pawn::west_attack_targets(pawns, attacking_side);
        let west_origins = Pawn::west_attack_origins(west_targets & target_bb, attacking_side);

        let all_attack_targets = east_targets | west_targets;
        let attack_origins = east_origins | west_origins;

        (all_attack_targets, attack_origins)
    }

    fn attacks_towards_target(
        pos: &Position,
        piece_type: piece::Type,
        piece_targets: &impl Fn(Square) -> Bitboard,
        target: Square,
        attacking_side: Side,
    ) -> (Bitboard, Bitboard) {
        let target_bb = Bitboard::from_square(target);
        let mut pieces = pos.piece_occupancy(attacking_side, piece_type);
        let mut all_attack_targets = Bitboard::EMPTY;
        let mut attack_origins = Bitboard::EMPTY;
        while pieces != Bitboard::EMPTY {
            let attack_origin = pieces.square_scan_forward_reset();
            let attack_targets = piece_targets(attack_origin);
            all_attack_targets |= attack_targets;
            if attack_targets & target_bb != Bitboard::EMPTY {
                attack_origins |= Bitboard::from_square(attack_origin);
            }
        }

        (all_attack_targets, attack_origins)
    }

    fn slider_attacks_towards_target(
        pos: &Position,
        piece_type: piece::Type,
        piece_targets: &impl Fn(Square, Bitboard) -> Bitboard,
        target: Square,
        attacking_side: Side,
    ) -> (
        Bitboard,
        Bitboard,
        Vec<PieceTargets>,
        Bitboard,
        Vec<PieceTargets>,
    ) {
        let target_bb = Bitboard::from_square(target);
        let mut pieces = pos.piece_occupancy(attacking_side, piece_type);
        let mut all_attack_targets = Bitboard::EMPTY;
        let mut attack_origins = Bitboard::EMPTY;
        let mut each_slider_attack = Vec::new();
        let mut xrays_to_target = Bitboard::EMPTY;
        let mut each_xray = Vec::new();
        while pieces != Bitboard::EMPTY {
            let attack_origin = pieces.square_scan_forward_reset();
            let attack_targets = piece_targets(attack_origin, pos.occupancy());
            all_attack_targets |= attack_targets;
            if attack_targets & target_bb != Bitboard::EMPTY {
                attack_origins |= Bitboard::from_square(attack_origin);
                each_slider_attack.push(PieceTargets::new(
                    piece::Piece::new(attacking_side, piece_type),
                    attack_origin,
                    attack_targets,
                ));
            }
            let xray_targets = piece_targets(attack_origin, Bitboard::EMPTY);
            if xray_targets & target_bb != Bitboard::EMPTY {
                xrays_to_target |= xray_targets;
                each_xray.push(PieceTargets::new(
                    piece::Piece::new(attacking_side, piece_type),
                    attack_origin,
                    xray_targets,
                ));
            }
        }

        (
            all_attack_targets,
            attack_origins,
            each_slider_attack,
            xrays_to_target,
            each_xray,
        )
    }
}
