use crate::bishop::Bishop;
use crate::bitboard::Bitboard;
use crate::king::King;
use crate::knight::Knight;
use crate::pawn::Pawn;
use crate::piece;
use crate::position::{CastlingRights, Position};
use crate::queen::Queen;
use crate::r#move::{Move, MoveType};
use crate::rook::Rook;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct IrreversibleProperties {
    en_passant_square: Bitboard,
    castling_rights: CastlingRights,
    plies_since_pawn_move_or_capture: usize,
    captured_piece: Option<piece::Piece>,
}

impl IrreversibleProperties {
    fn new(
        en_passant_square: Bitboard,
        castling_rights: CastlingRights,
        plies_since_pawn_move_or_capture: usize,
        captured_piece: Option<piece::Piece>,
    ) -> IrreversibleProperties {
        IrreversibleProperties {
            en_passant_square,
            castling_rights,
            plies_since_pawn_move_or_capture,
            captured_piece,
        }
    }
}

pub struct MoveGenerator {
    pos: Position,
    move_list: Vec<Move>,
    irreversible_properties: Vec<IrreversibleProperties>,
}

impl MoveGenerator {
    const PAWN_PUSHES: [fn(Bitboard, Bitboard) -> (Bitboard, Bitboard); 2] =
        [Pawn::white_push_targets, Pawn::black_push_targets];
    const PAWN_PUSH_IDX_SHIFT: [i8; 2] = [Bitboard::IDX_SHIFT_NORTH, Bitboard::IDX_SHIFT_SOUTH];
    const PAWN_PROMO_RANK: [Bitboard; 2] = [
        Bitboard::WHITE_PROMOTION_RANK,
        Bitboard::BLACK_PROMOTION_RANK,
    ];
    const PAWN_EAST_ATTACKS: [fn(Bitboard) -> Bitboard; 2] = [
        Pawn::white_east_attack_targets,
        Pawn::black_east_attack_targets,
    ];
    const PAWN_EAST_ATTACK_IDX_SHIFT: [i8; 2] = [
        Bitboard::IDX_SHIFT_NORTH_EAST,
        Bitboard::IDX_SHIFT_SOUTH_EAST,
    ];
    const PAWN_WEST_ATTACKS: [fn(Bitboard) -> Bitboard; 2] = [
        Pawn::white_west_attack_targets,
        Pawn::black_west_attack_targets,
    ];
    const PAWN_WEST_ATTACK_IDX_SHIFT: [i8; 2] = [
        Bitboard::IDX_SHIFT_NORTH_WEST,
        Bitboard::IDX_SHIFT_SOUTH_WEST,
    ];

    pub fn new(pos: Position) -> MoveGenerator {
        MoveGenerator {
            pos,
            move_list: Vec::new(),
            irreversible_properties: Vec::new(),
        }
    }

    pub fn move_list(&self) -> Vec<Move> {
        self.move_list.clone()
    }

    fn add_move_if_legal(&mut self, m: Move) {
        let mut pos = self.pos.clone();
        let origin = m.origin();
        let target = m.target();
        // Promotion piece type is ignored here because it doesn't change the opposing side's
        // attacks
        pos.set_piece_at(target, pos.piece_at(origin));
        pos.set_piece_at(origin, None);
        if m.is_en_passant() {
            let side_idx = pos.side_to_move() as usize;
            let captured_idx = (target as i8 - Self::PAWN_PUSH_IDX_SHIFT[side_idx]) as usize;
            pos.set_piece_at(captured_idx, None);
        }

        if !pos.is_in_check(pos.side_to_move()) {
            self.move_list.push(m);
        }
    }

    pub fn generate_moves(&mut self) {
        self.move_list.clear();
        self.generate_pawn_moves();
        self.generate_knight_moves();
        self.generate_sliding_piece_moves(piece::Type::Bishop, Bishop::targets);
        self.generate_sliding_piece_moves(piece::Type::Rook, Rook::targets);
        self.generate_sliding_piece_moves(piece::Type::Queen, Queen::targets);
        self.generate_king_moves();
        self.generate_castles();
    }

    pub fn do_move(&mut self, m: Move) {
        let origin = m.origin();
        let target = m.target();
        let moving_piece = self.pos.piece_at(origin).unwrap();
        let side_to_move = self.pos.side_to_move();
        let side_idx = side_to_move as usize;

        let capture_square = if m.is_en_passant() {
            (target as i8 - Self::PAWN_PUSH_IDX_SHIFT[side_idx]) as usize
        } else {
            target
        };
        let captured_piece = self.pos.piece_at(capture_square);

        self.irreversible_properties
            .push(IrreversibleProperties::new(
                self.pos.en_passant_square(),
                self.pos.castling_rights(),
                self.pos.plies_since_pawn_move_or_capture(),
                captured_piece,
            ));

        let target_piece = if m.is_promotion() {
            piece::Piece::new(side_to_move, m.move_type().promo_piece_unchecked())
        } else {
            moving_piece
        };

        self.pos.set_piece_at(target, Some(target_piece));
        self.pos.set_piece_at(origin, None);

        let en_passant_square = match m.move_type() {
            MoveType::DOUBLE_PAWN_PUSH => {
                let en_passant_idx = (origin as i8 + Self::PAWN_PUSH_IDX_SHIFT[side_idx]) as usize;
                Bitboard(0x1 << en_passant_idx)
            }
            _ => Bitboard::EMPTY,
        };
        self.pos.set_en_passant_square(en_passant_square);

        if m.is_capture() {
            if m.is_en_passant() {
                let captured_idx = (target as i8 - Self::PAWN_PUSH_IDX_SHIFT[side_idx]) as usize;
                self.pos.set_piece_at(captured_idx, None);
            }
            self.remove_castling_rights(target);
        }

        if m.is_capture() || moving_piece.piece_type() == piece::Type::Pawn {
            self.pos.set_plies_since_pawn_move_or_capture(0);
        } else {
            self.pos.set_plies_since_pawn_move_or_capture(
                self.pos.plies_since_pawn_move_or_capture() + 1,
            );
        }

        self.remove_castling_rights(origin);

        let move_count = self.pos.move_count();
        self.pos.set_move_count(move_count + side_idx);
        self.pos.set_side_to_move(!side_to_move);
    }

    pub fn undo_move(&mut self, m: Move) {
        let origin = m.origin();
        let target = m.target();
        let moving_piece = self.pos.piece_at(target).unwrap();

        let origin_piece = if m.is_promotion() {
            piece::Piece::new(moving_piece.piece_side(), piece::Type::Pawn)
        } else {
            moving_piece
        };

        self.pos.set_piece_at(origin, Some(origin_piece));
        self.pos.set_piece_at(target, None);

        self.pos.set_side_to_move(!self.pos.side_to_move());
        self.pos
            .set_move_count(self.pos.move_count() - self.pos.side_to_move() as usize);

        debug_assert!(!self.irreversible_properties.is_empty());
        let irr = self.irreversible_properties.pop().unwrap();
        self.pos.set_en_passant_square(irr.en_passant_square);
        self.pos.set_castling_rights(irr.castling_rights);
        self.pos
            .set_plies_since_pawn_move_or_capture(irr.plies_since_pawn_move_or_capture);

        if m.is_capture() {
            let capture_square = if m.is_en_passant() {
                let side_idx = self.pos.side_to_move() as usize;
                (target as i8 - Self::PAWN_PUSH_IDX_SHIFT[side_idx]) as usize
            } else {
                target
            };
            self.pos.set_piece_at(capture_square, irr.captured_piece);
        }
    }

    fn generate_pawn_moves(&mut self) {
        let pawns = self
            .pos
            .piece_occupancy(self.pos.side_to_move(), piece::Type::Pawn);
        let side_idx = self.pos.side_to_move() as usize;

        self.generate_pawn_pushes(pawns, side_idx);
        self.generate_pawn_captures(pawns, side_idx);
    }

    fn generate_pawn_pushes(&mut self, pawns: Bitboard, side_idx: usize) {
        let (single_push_targets, mut double_push_targets) =
            Self::PAWN_PUSHES[side_idx](pawns, self.pos.occupancy());

        let mut promo_targets = single_push_targets & Self::PAWN_PROMO_RANK[side_idx];
        let mut non_promo_targets = single_push_targets & !promo_targets;

        while promo_targets != Bitboard::EMPTY {
            let target = promo_targets.bit_scan_forward_reset();
            let origin = (target as i8 - Self::PAWN_PUSH_IDX_SHIFT[side_idx]) as usize;
            for promo_piece in [
                piece::Type::Queen,
                piece::Type::Rook,
                piece::Type::Bishop,
                piece::Type::Knight,
            ]
            .iter()
            {
                self.add_move_if_legal(Move::new(
                    origin,
                    target,
                    MoveType::new_with_promo_piece(MoveType::PROMOTION, *promo_piece),
                ));
            }
        }
        while non_promo_targets != Bitboard::EMPTY {
            let target = non_promo_targets.bit_scan_forward_reset();
            let origin = (target as i8 - Self::PAWN_PUSH_IDX_SHIFT[side_idx]) as usize;
            self.add_move_if_legal(Move::new(origin, target, MoveType::QUIET));
        }
        while double_push_targets != Bitboard::EMPTY {
            let target = double_push_targets.bit_scan_forward_reset();
            let origin = (target as i8 - 2 * Self::PAWN_PUSH_IDX_SHIFT[side_idx]) as usize;
            self.add_move_if_legal(Move::new(origin, target, MoveType::DOUBLE_PAWN_PUSH));
        }
    }

    fn generate_pawn_captures(&mut self, pawns: Bitboard, side_idx: usize) {
        let opponents = self.pos.side_occupancy(!self.pos.side_to_move());
        let en_passant_square = self.pos.en_passant_square();

        self.generate_pawn_captures_one_side(
            pawns,
            opponents,
            en_passant_square,
            Self::PAWN_EAST_ATTACKS[side_idx],
            Self::PAWN_PROMO_RANK[side_idx],
            Self::PAWN_EAST_ATTACK_IDX_SHIFT[side_idx],
        );

        self.generate_pawn_captures_one_side(
            pawns,
            opponents,
            en_passant_square,
            Self::PAWN_WEST_ATTACKS[side_idx],
            Self::PAWN_PROMO_RANK[side_idx],
            Self::PAWN_WEST_ATTACK_IDX_SHIFT[side_idx],
        );
    }

    fn generate_pawn_captures_one_side(
        &mut self,
        pawns: Bitboard,
        opponents: Bitboard,
        en_passant_square: Bitboard,
        attacks: fn(Bitboard) -> Bitboard,
        promo_rank: Bitboard,
        idx_shift: i8,
    ) {
        let targets = attacks(pawns);
        let captures = targets & (opponents | en_passant_square);
        let mut promo_captures = captures & promo_rank;
        let mut non_promo_captures = captures & !promo_captures;

        while promo_captures != Bitboard::EMPTY {
            let target = promo_captures.bit_scan_forward_reset();
            let origin = (target as i8 - idx_shift) as usize;
            for promo_piece in [
                piece::Type::Queen,
                piece::Type::Rook,
                piece::Type::Bishop,
                piece::Type::Knight,
            ]
            .iter()
            {
                self.add_move_if_legal(Move::new(
                    origin,
                    target,
                    MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, *promo_piece),
                ));
            }
        }

        while non_promo_captures != Bitboard::EMPTY {
            let target = non_promo_captures.bit_scan_forward_reset();
            let origin = (target as i8 - idx_shift) as usize;
            let move_type = if Bitboard(0x1 << target) == en_passant_square {
                MoveType::EN_PASSANT_CAPTURE
            } else {
                MoveType::CAPTURE
            };
            self.add_move_if_legal(Move::new(origin, target, move_type));
        }
    }

    fn generate_knight_moves(&mut self) {
        let mut knights = self
            .pos
            .piece_occupancy(self.pos.side_to_move(), piece::Type::Knight);
        let own_occupancy = self.pos.side_occupancy(self.pos.side_to_move());
        while knights != Bitboard::EMPTY {
            let origin = knights.bit_scan_forward_reset();
            let targets = Knight::targets(origin) & !own_occupancy;
            self.generate_piece_moves(origin, &targets);
        }
    }

    fn generate_king_moves(&mut self) {
        let mut kings = self
            .pos
            .piece_occupancy(self.pos.side_to_move(), piece::Type::King);
        let own_occupancy = self.pos.side_occupancy(self.pos.side_to_move());
        while kings != Bitboard::EMPTY {
            let origin = kings.bit_scan_forward_reset();
            let targets = King::targets(origin) & !own_occupancy;
            self.generate_piece_moves(origin, &targets);
        }
    }

    fn generate_sliding_piece_moves(
        &mut self,
        piece_type: piece::Type,
        piece_targets: fn(usize, Bitboard) -> Bitboard,
    ) {
        let mut piece_occupancy = self
            .pos
            .piece_occupancy(self.pos.side_to_move(), piece_type);
        let own_occupancy = self.pos.side_occupancy(self.pos.side_to_move());
        while piece_occupancy != Bitboard::EMPTY {
            let origin = piece_occupancy.bit_scan_forward_reset();
            let targets = piece_targets(origin, self.pos.occupancy()) & !own_occupancy;
            self.generate_piece_moves(origin, &targets);
        }
    }

    fn generate_piece_moves(&mut self, origin: usize, targets: &Bitboard) {
        let opponents = self.pos.side_occupancy(!self.pos.side_to_move());
        let mut captures = targets & opponents;
        let mut quiets = targets & !captures;
        while captures != Bitboard::EMPTY {
            let target = captures.bit_scan_forward_reset();
            self.add_move_if_legal(Move::new(origin, target, MoveType::CAPTURE));
        }
        while quiets != Bitboard::EMPTY {
            let target = quiets.bit_scan_forward_reset();
            self.add_move_if_legal(Move::new(origin, target, MoveType::QUIET));
        }
    }

    fn generate_castles(&mut self) {
        const CASTLES: [fn(&mut MoveGenerator); 2] = [
            MoveGenerator::generate_white_castles,
            MoveGenerator::generate_black_castles,
        ];
        let side_idx = self.pos.side_to_move() as usize;
        CASTLES[side_idx](self);
    }

    fn generate_white_castles(&mut self) {
        let castling_rights = self.pos.castling_rights();
        let attacked_by_opponent = self.pos.attacked_squares(!self.pos.side_to_move());

        if castling_rights.contains(CastlingRights::WHITE_KINGSIDE) {
            debug_assert_eq!(
                Some(piece::Piece::WHITE_KING),
                self.pos.piece_at(Bitboard::IDX_E1)
            );
            debug_assert_eq!(
                Some(piece::Piece::WHITE_ROOK),
                self.pos.piece_at(Bitboard::IDX_H1)
            );
            let squares_passable =
                self.pos.occupancy() & (Bitboard::F1 | Bitboard::G1) == Bitboard::EMPTY;
            let squares_attacked = attacked_by_opponent
                & (Bitboard::E1 | Bitboard::F1 | Bitboard::G1)
                != Bitboard::EMPTY;
            if squares_passable && !squares_attacked {
                self.move_list.push(Move::new(
                    Bitboard::IDX_E1,
                    Bitboard::IDX_G1,
                    MoveType::CASTLE_KINGSIDE,
                ));
            }
        }
        if castling_rights.contains(CastlingRights::WHITE_QUEENSIDE) {
            debug_assert_eq!(
                Some(piece::Piece::WHITE_KING),
                self.pos.piece_at(Bitboard::IDX_E1)
            );
            debug_assert_eq!(
                Some(piece::Piece::WHITE_ROOK),
                self.pos.piece_at(Bitboard::IDX_A1)
            );
            let squares_passable = self.pos.occupancy()
                & (Bitboard::B1 | Bitboard::C1 | Bitboard::D1)
                == Bitboard::EMPTY;
            let squares_attacked = attacked_by_opponent
                & (Bitboard::C1 | Bitboard::D1 | Bitboard::E1)
                != Bitboard::EMPTY;
            if squares_passable && !squares_attacked {
                self.move_list.push(Move::new(
                    Bitboard::IDX_E1,
                    Bitboard::IDX_C1,
                    MoveType::CASTLE_QUEENSIDE,
                ));
            }
        }
    }

    fn generate_black_castles(&mut self) {
        let castling_rights = self.pos.castling_rights();
        let attacked_by_opponent = self.pos.attacked_squares(!self.pos.side_to_move());

        if castling_rights.contains(CastlingRights::BLACK_KINGSIDE) {
            debug_assert_eq!(
                Some(piece::Piece::BLACK_KING),
                self.pos.piece_at(Bitboard::IDX_E8)
            );
            debug_assert_eq!(
                Some(piece::Piece::BLACK_ROOK),
                self.pos.piece_at(Bitboard::IDX_H8)
            );
            let squares_passable =
                self.pos.occupancy() & (Bitboard::F8 | Bitboard::G8) == Bitboard::EMPTY;
            let squares_attacked = attacked_by_opponent
                & (Bitboard::E8 | Bitboard::F8 | Bitboard::G8)
                != Bitboard::EMPTY;
            if squares_passable && !squares_attacked {
                self.move_list.push(Move::new(
                    Bitboard::IDX_E8,
                    Bitboard::IDX_G8,
                    MoveType::CASTLE_KINGSIDE,
                ));
            }
        }
        if castling_rights.contains(CastlingRights::BLACK_QUEENSIDE) {
            debug_assert_eq!(
                Some(piece::Piece::BLACK_KING),
                self.pos.piece_at(Bitboard::IDX_E8)
            );
            debug_assert_eq!(
                Some(piece::Piece::BLACK_ROOK),
                self.pos.piece_at(Bitboard::IDX_A8)
            );
            let squares_passable = self.pos.occupancy()
                & (Bitboard::B8 | Bitboard::C8 | Bitboard::D8)
                == Bitboard::EMPTY;
            let squares_attacked = attacked_by_opponent
                & (Bitboard::C8 | Bitboard::D8 | Bitboard::E8)
                != Bitboard::EMPTY;
            if squares_passable && !squares_attacked {
                self.move_list.push(Move::new(
                    Bitboard::IDX_E8,
                    Bitboard::IDX_C8,
                    MoveType::CASTLE_QUEENSIDE,
                ));
            }
        }
    }

    fn remove_castling_rights(&mut self, square: usize) {
        let removed_castling_rights = match square {
            Bitboard::IDX_A1 => CastlingRights::WHITE_QUEENSIDE,
            Bitboard::IDX_H1 => CastlingRights::WHITE_KINGSIDE,
            Bitboard::IDX_E1 => CastlingRights::WHITE_BOTH,
            Bitboard::IDX_A8 => CastlingRights::BLACK_QUEENSIDE,
            Bitboard::IDX_H8 => CastlingRights::BLACK_KINGSIDE,
            Bitboard::IDX_E8 => CastlingRights::BLACK_BOTH,
            _ => CastlingRights::empty(),
        };
        self.pos
            .set_castling_rights(self.pos.castling_rights() & !removed_castling_rights);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::side::Side;

    #[test]
    fn initial_position() {
        let mut movegen = MoveGenerator::new(Position::initial());
        movegen.generate_moves();

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

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn position_after_1_e4() {
        let mut pos = Position::initial();
        pos.set_piece_at(Bitboard::IDX_E2, None);
        pos.set_piece_at(Bitboard::IDX_E4, Some(piece::Piece::WHITE_PAWN));
        pos.set_en_passant_square(Bitboard::E3);
        pos.set_side_to_move(Side::Black);
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

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

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn white_pawn_captures() {
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
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

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

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn black_pawn_captures() {
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
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

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

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn king_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_F2, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

        let expected_moves = [
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::QUIET),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::CAPTURE),
        ];

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn knight_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_B1, Some(piece::Piece::WHITE_KNIGHT));
        pos.set_piece_at(Bitboard::IDX_C3, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_A3, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

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

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn bishop_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_C3, Some(piece::Piece::WHITE_BISHOP));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_G7, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

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

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn rook_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_E3, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_E7, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

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

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn queen_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_E3, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_C3, Some(piece::Piece::WHITE_QUEEN));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_C7, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_G7, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

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

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn white_promotions() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_B7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_A8, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Bitboard::IDX_C8, Some(piece::Piece::BLACK_BISHOP));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

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

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn black_promotions() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_B2, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_A1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_C1, Some(piece::Piece::WHITE_BISHOP));
        pos.set_side_to_move(Side::Black);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

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

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn white_en_passant_captures() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_D5, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_C5, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        pos.set_en_passant_square(Bitboard::C6);
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

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

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn black_en_passant_captures() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_D4, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_C4, Some(piece::Piece::WHITE_PAWN));
        pos.set_side_to_move(Side::Black);
        pos.set_castling_rights(CastlingRights::empty());
        pos.set_en_passant_square(Bitboard::C3);
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

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

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
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

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_A1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos.clone());
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        pos.set_castling_rights(CastlingRights::WHITE_KINGSIDE);
        let mut movegen = MoveGenerator::new(pos.clone());
        movegen.generate_moves();
        assert!(movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        pos.set_castling_rights(CastlingRights::WHITE_QUEENSIDE);
        let mut movegen = MoveGenerator::new(pos.clone());
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(movegen.move_list.contains(&queenside_castle));

        pos.set_castling_rights(CastlingRights::WHITE_BOTH);
        let mut movegen = MoveGenerator::new(pos.clone());
        movegen.generate_moves();
        assert!(movegen.move_list.contains(&kingside_castle));
        assert!(movegen.move_list.contains(&queenside_castle));

        // Square between king and rook blocked
        let mut pos_blocked = pos.clone();
        pos_blocked.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WHITE_KNIGHT));
        pos_blocked.set_piece_at(Bitboard::IDX_B1, Some(piece::Piece::BLACK_KNIGHT));
        let mut movegen = MoveGenerator::new(pos_blocked);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // King attacked
        let mut pos_in_check = pos.clone();
        pos_in_check.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::BLACK_ROOK));
        let mut movegen = MoveGenerator::new(pos_in_check);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // Square traversed by king attacked
        let mut pos_traverse_attacked = pos.clone();
        pos_traverse_attacked.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::BLACK_BISHOP));
        let mut movegen = MoveGenerator::new(pos_traverse_attacked);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // Target square attacked
        let mut pos_target_attacked = pos.clone();
        pos_target_attacked.set_piece_at(Bitboard::IDX_E3, Some(piece::Piece::BLACK_BISHOP));
        let mut movegen = MoveGenerator::new(pos_target_attacked);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // Rook attacked (castling is legal)
        let mut pos_target_attacked = pos.clone();
        pos_target_attacked.set_piece_at(Bitboard::IDX_E4, Some(piece::Piece::BLACK_BISHOP));
        pos_target_attacked.set_piece_at(Bitboard::IDX_E5, Some(piece::Piece::BLACK_BISHOP));
        let mut movegen = MoveGenerator::new(pos_target_attacked);
        movegen.generate_moves();
        assert!(movegen.move_list.contains(&kingside_castle));
        assert!(movegen.move_list.contains(&queenside_castle));
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

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_A8, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Bitboard::IDX_H8, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::Black);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos.clone());
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        pos.set_castling_rights(CastlingRights::BLACK_KINGSIDE);
        let mut movegen = MoveGenerator::new(pos.clone());
        movegen.generate_moves();
        assert!(movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        pos.set_castling_rights(CastlingRights::BLACK_QUEENSIDE);
        let mut movegen = MoveGenerator::new(pos.clone());
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(movegen.move_list.contains(&queenside_castle));

        pos.set_castling_rights(CastlingRights::BLACK_BOTH);
        let mut movegen = MoveGenerator::new(pos.clone());
        movegen.generate_moves();
        assert!(movegen.move_list.contains(&kingside_castle));
        assert!(movegen.move_list.contains(&queenside_castle));

        // Square between king and rook blocked
        let mut pos_blocked = pos.clone();
        pos_blocked.set_piece_at(Bitboard::IDX_G8, Some(piece::Piece::BLACK_KNIGHT));
        pos_blocked.set_piece_at(Bitboard::IDX_B8, Some(piece::Piece::WHITE_KNIGHT));
        let mut movegen = MoveGenerator::new(pos_blocked);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // King attacked
        let mut pos_in_check = pos.clone();
        pos_in_check.set_piece_at(Bitboard::IDX_E7, Some(piece::Piece::WHITE_ROOK));
        let mut movegen = MoveGenerator::new(pos_in_check);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // Square traversed by king attacked
        let mut pos_traverse_attacked = pos.clone();
        pos_traverse_attacked.set_piece_at(Bitboard::IDX_E7, Some(piece::Piece::WHITE_BISHOP));
        let mut movegen = MoveGenerator::new(pos_traverse_attacked);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // Target square attacked
        let mut pos_target_attacked = pos.clone();
        pos_target_attacked.set_piece_at(Bitboard::IDX_E6, Some(piece::Piece::WHITE_BISHOP));
        let mut movegen = MoveGenerator::new(pos_target_attacked);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // Rook attacked (castling is legal)
        let mut pos_target_attacked = pos.clone();
        pos_target_attacked.set_piece_at(Bitboard::IDX_E4, Some(piece::Piece::WHITE_BISHOP));
        pos_target_attacked.set_piece_at(Bitboard::IDX_E5, Some(piece::Piece::WHITE_BISHOP));
        let mut movegen = MoveGenerator::new(pos_target_attacked);
        movegen.generate_moves();
        assert!(movegen.move_list.contains(&kingside_castle));
        assert!(movegen.move_list.contains(&queenside_castle));
    }

    #[test]
    fn king_not_left_in_check_after_pawn_moves() {
        let mut pos_pawn = Position::empty();
        pos_pawn.set_piece_at(Bitboard::IDX_D2, Some(piece::Piece::WHITE_KING));
        pos_pawn.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::WHITE_PAWN));
        pos_pawn.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos_pawn.set_piece_at(Bitboard::IDX_H2, Some(piece::Piece::BLACK_ROOK));
        pos_pawn.set_piece_at(Bitboard::IDX_F3, Some(piece::Piece::BLACK_ROOK));
        pos_pawn.set_side_to_move(Side::White);
        pos_pawn.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos_pawn.clone());
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_E2,
            Bitboard::IDX_E3,
            MoveType::QUIET
        )));
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_E2,
            Bitboard::IDX_E4,
            MoveType::DOUBLE_PAWN_PUSH
        )));
        assert!(!movegen.move_list.contains(&Move::new(
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
        let mut movegen = MoveGenerator::new(pos_pawn_promo);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_B7,
            Bitboard::IDX_B8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Queen)
        )));
        assert!(!movegen.move_list.contains(&Move::new(
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
        let mut movegen = MoveGenerator::new(pos_pawn_en_passant);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_C5,
            Bitboard::IDX_B6,
            MoveType::EN_PASSANT_CAPTURE
        )));
    }

    #[test]
    fn king_not_left_in_check_after_knight_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WHITE_KNIGHT));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_E2,
            MoveType::QUIET
        )));
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F3,
            MoveType::QUIET
        )));
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_H3,
            MoveType::QUIET
        )));
    }

    #[test]
    fn king_not_left_in_check_after_bishop_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WHITE_BISHOP));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F2,
            MoveType::QUIET
        )));
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_H2,
            MoveType::QUIET
        )));
    }

    #[test]
    fn king_not_left_in_check_after_rook_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_G2,
            MoveType::QUIET
        )));
        assert!(movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F1,
            MoveType::QUIET
        )));
        assert!(movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_H1,
            MoveType::CAPTURE
        )));
    }

    #[test]
    fn king_not_left_in_check_after_queen_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WHITE_QUEEN));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F2,
            MoveType::QUIET
        )));
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_G2,
            MoveType::QUIET
        )));
        assert!(movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F1,
            MoveType::QUIET
        )));
        assert!(movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_H1,
            MoveType::CAPTURE
        )));
    }

    #[test]
    fn king_does_not_move_into_check() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_H2, Some(piece::Piece::BLACK_ROOK));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_E1,
            Bitboard::IDX_E2,
            MoveType::QUIET
        )));
    }

    #[test]
    fn do_and_undo_move_initial_position() {
        let pos = Position::initial();
        let mut movegen = MoveGenerator::new(pos);
        let mut pos_history = Vec::new();
        let mut move_history = Vec::new();

        // Position after 1. e4
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_E2,
            Bitboard::IDX_E4,
            MoveType::DOUBLE_PAWN_PUSH,
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Bitboard::E4,
            movegen.pos.side_occupancy(Side::White) & (Bitboard::E2 | Bitboard::E4)
        );
        assert_eq!(Bitboard::E3, movegen.pos.en_passant_square());
        assert_eq!(Side::Black, movegen.pos.side_to_move());
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            movegen.pos.castling_rights()
        );
        assert_eq!(0, movegen.pos.plies_since_pawn_move_or_capture());
        assert_eq!(1, movegen.pos.move_count());

        // Position after 1. e4 c5
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_C7,
            Bitboard::IDX_C5,
            MoveType::DOUBLE_PAWN_PUSH,
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Bitboard::C5,
            movegen.pos.side_occupancy(Side::Black) & (Bitboard::C7 | Bitboard::C5)
        );
        assert_eq!(Bitboard::C6, movegen.pos.en_passant_square());
        assert_eq!(Side::White, movegen.pos.side_to_move());
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            movegen.pos.castling_rights()
        );
        assert_eq!(0, movegen.pos.plies_since_pawn_move_or_capture());
        assert_eq!(2, movegen.pos.move_count());

        // Position after 1. e4 c5 2. Nf3
        pos_history.push(movegen.pos.clone());
        let m = Move::new(Bitboard::IDX_G1, Bitboard::IDX_F3, MoveType::QUIET);
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Bitboard::F3,
            movegen.pos.side_occupancy(Side::White) & (Bitboard::G1 | Bitboard::F3)
        );
        assert_eq!(Bitboard::EMPTY, movegen.pos.en_passant_square());
        assert_eq!(Side::Black, movegen.pos.side_to_move());
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            movegen.pos.castling_rights()
        );
        assert_eq!(1, movegen.pos.plies_since_pawn_move_or_capture());
        assert_eq!(2, movegen.pos.move_count());

        // Position after 1. e4 c5 2. Nf3 d6
        pos_history.push(movegen.pos.clone());
        let m = Move::new(Bitboard::IDX_D7, Bitboard::IDX_D6, MoveType::QUIET);
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Bitboard::D6,
            movegen.pos.side_occupancy(Side::Black) & (Bitboard::D7 | Bitboard::D6)
        );
        assert_eq!(Bitboard::EMPTY, movegen.pos.en_passant_square());
        assert_eq!(Side::White, movegen.pos.side_to_move());
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            movegen.pos.castling_rights()
        );
        assert_eq!(0, movegen.pos.plies_since_pawn_move_or_capture());
        assert_eq!(3, movegen.pos.move_count());

        // Position after 1. e4 c5 2. Nf3
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Position after 1. e4 c5
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Position after 1. e4
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Initial position
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);
    }

    #[test]
    fn do_and_undo_move_castling_rights() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_A1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Bitboard::IDX_B7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_G7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_A8, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Bitboard::IDX_H8, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Bitboard::IDX_B2, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_G2, Some(piece::Piece::BLACK_PAWN));
        pos.set_castling_rights(CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH);
        let mut movegen = MoveGenerator::new(pos);
        let mut pos_history = Vec::new();
        let mut move_history = Vec::new();

        // Position after 1. 0-0
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_E1,
            Bitboard::IDX_G1,
            MoveType::CASTLE_KINGSIDE,
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Bitboard::G1,
            movegen.pos.side_occupancy(Side::White) & (Bitboard::E1 | Bitboard::G1)
        );
        assert_eq!(CastlingRights::BLACK_BOTH, movegen.pos.castling_rights());

        // Position after 1. 0-0 Ke7
        pos_history.push(movegen.pos.clone());
        let m = Move::new(Bitboard::IDX_E8, Bitboard::IDX_E7, MoveType::QUIET);
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Bitboard::E7,
            movegen.pos.side_occupancy(Side::Black) & (Bitboard::E8 | Bitboard::E7)
        );
        assert_eq!(CastlingRights::empty(), movegen.pos.castling_rights());

        // Position after 1. 0-0
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Starting position
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Position after 1. Ra2
        pos_history.push(movegen.pos.clone());
        let m = Move::new(Bitboard::IDX_A1, Bitboard::IDX_A2, MoveType::QUIET);
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            CastlingRights::WHITE_KINGSIDE | CastlingRights::BLACK_BOTH,
            movegen.pos.castling_rights()
        );

        // Position after 1. Ra2 Rxh1
        // White loses kingside castling rights after the rook on h1 gets captured
        pos_history.push(movegen.pos.clone());
        let m = Move::new(Bitboard::IDX_H8, Bitboard::IDX_H1, MoveType::CAPTURE);
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            CastlingRights::BLACK_QUEENSIDE,
            movegen.pos.castling_rights()
        );

        // Position after 1. Ra2
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(
            &prev_pos, &movegen.pos,
            "\nExpected Position:\n{}\nActual Position:\n{}\n",
            prev_pos, movegen.pos
        );

        // Starting position
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Position after 1. bxa8=N
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_B7,
            Bitboard::IDX_A8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Knight),
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_KINGSIDE,
            movegen.pos.castling_rights()
        );

        // Position after 1. bxa8=N bxa1=B
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_B2,
            Bitboard::IDX_A1,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            CastlingRights::WHITE_KINGSIDE | CastlingRights::BLACK_KINGSIDE,
            movegen.pos.castling_rights()
        );

        // Position after 1. bxa8=N bxa1=B 2. gxh8=R+
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_G7,
            Bitboard::IDX_H8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Rook),
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            CastlingRights::WHITE_KINGSIDE,
            movegen.pos.castling_rights()
        );

        // Position after 1. bxa8=N bxa1=B
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Position after 1. bxa8=N bxa1=B 2. gxh8=B
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_G7,
            Bitboard::IDX_H8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            CastlingRights::WHITE_KINGSIDE,
            movegen.pos.castling_rights()
        );

        // Position after 1. bxa8=N bxa1=B 2. gxh8=B+ gxh1=Q+
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_G2,
            Bitboard::IDX_H1,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Queen),
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(CastlingRights::empty(), movegen.pos.castling_rights());

        // Position after 1. bxa8=N bxa1=B 2. gxh8=B
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Position after 1. bxa8=N bxa1=B
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Position after 1. bxa8=N
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Starting position
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);
    }

    #[test]
    fn do_and_undo_move_en_passant() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_D5, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_C5, Some(piece::Piece::BLACK_PAWN));
        pos.set_en_passant_square(Bitboard::C6);
        let mut movegen = MoveGenerator::new(pos);
        let mut pos_history = Vec::new();
        let mut move_history = Vec::new();

        // Position after 1. dxc6
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_D5,
            Bitboard::IDX_C6,
            MoveType::EN_PASSANT_CAPTURE,
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Bitboard::C6,
            movegen.pos.side_occupancy(Side::White) & (Bitboard::D5 | Bitboard::C6)
        );
        assert_eq!(
            Bitboard::EMPTY,
            movegen.pos.side_occupancy(Side::Black) & Bitboard::C5
        );

        // Starting position
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);
    }

    #[test]
    fn do_and_undo_move_promotions() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_A7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_H7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_E7, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_A2, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_H2, Some(piece::Piece::BLACK_PAWN));
        let mut movegen = MoveGenerator::new(pos);
        let mut pos_history = Vec::new();
        let mut move_history = Vec::new();

        // Position after 1. a8=Q
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_A7,
            Bitboard::IDX_A8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Queen),
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Some(piece::Piece::WHITE_QUEEN),
            movegen.pos.piece_at(Bitboard::IDX_A8)
        );

        // Position after 1. a8=Q a1=R
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_A2,
            Bitboard::IDX_A1,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Rook),
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Some(piece::Piece::BLACK_ROOK),
            movegen.pos.piece_at(Bitboard::IDX_A1)
        );

        // Position after 1. a8=Q a1=R 2. h8=B
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_H7,
            Bitboard::IDX_H8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Bishop),
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Some(piece::Piece::WHITE_BISHOP),
            movegen.pos.piece_at(Bitboard::IDX_H8)
        );

        // Position after 1. a8=Q a1=R 2. h8=B h1=N
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_H2,
            Bitboard::IDX_H1,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Knight),
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Some(piece::Piece::BLACK_KNIGHT),
            movegen.pos.piece_at(Bitboard::IDX_H1)
        );

        // Position after 1. a8=Q a1=R 2. h8=B
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Position after 1. a8=Q a1=R
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Position after 1. a8=Q
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Starting position
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);
    }

    #[test]
    fn do_and_undo_move_promotion_captures() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Bitboard::IDX_A7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_H7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Bitboard::IDX_B1, Some(piece::Piece::WHITE_KNIGHT));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WHITE_KNIGHT));
        pos.set_piece_at(Bitboard::IDX_E7, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Bitboard::IDX_A2, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_H2, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Bitboard::IDX_B8, Some(piece::Piece::BLACK_KNIGHT));
        pos.set_piece_at(Bitboard::IDX_G8, Some(piece::Piece::BLACK_KNIGHT));
        let mut movegen = MoveGenerator::new(pos);
        let mut pos_history = Vec::new();
        let mut move_history = Vec::new();

        // Position after 1. axb8=Q
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_A7,
            Bitboard::IDX_B8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Queen),
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Some(piece::Piece::WHITE_QUEEN),
            movegen.pos.piece_at(Bitboard::IDX_B8)
        );

        // Position after 1. axb8=Q axb1=R
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_A2,
            Bitboard::IDX_B1,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Rook),
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Some(piece::Piece::BLACK_ROOK),
            movegen.pos.piece_at(Bitboard::IDX_B1)
        );

        // Position after 1. axb8=Q axb1=R 2. hxg8=B
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_H7,
            Bitboard::IDX_G8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Some(piece::Piece::WHITE_BISHOP),
            movegen.pos.piece_at(Bitboard::IDX_G8)
        );

        // Position after 1. axb8=Q axb1=R 2. hxg8=B hxg1=N+
        pos_history.push(movegen.pos.clone());
        let m = Move::new(
            Bitboard::IDX_H2,
            Bitboard::IDX_G1,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Knight),
        );
        move_history.push(m);
        movegen.do_move(m);
        assert_eq!(
            Some(piece::Piece::BLACK_KNIGHT),
            movegen.pos.piece_at(Bitboard::IDX_G1)
        );

        // Position after 1. axb8=Q axb1=R 2. hxg8=B
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Position after 1. axb8=Q axb1=R
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Position after 1. axb8=Q
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);

        // Starting position
        movegen.undo_move(move_history.pop().expect("Expected Some(Move), got None"));
        let prev_pos = pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, &movegen.pos);
    }
}
