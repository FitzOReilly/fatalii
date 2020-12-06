use crate::bishop::Bishop;
use crate::bitboard::Bitboard;
use crate::king::King;
use crate::knight::Knight;
use crate::pawn::Pawn;
use crate::piece;
use crate::position::{CastlingRights, Position};
use crate::queen::Queen;
use crate::rook::Rook;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum MoveType {
    Quiet = 0,
    DoublePawnPush = 1,
    KingsideCastle = 2,
    QueensideCastle = 3,
    Capture = 4,
    EnPassantCapture = 5,
    KnightPromo = 8,
    BishopPromo = 9,
    RookPromo = 10,
    QueenPromo = 11,
    KnightPromoCapture = 12,
    BishopPromoCapture = 13,
    RookPromoCapture = 14,
    QueenPromoCapture = 15,
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    from: u8,
    to: u8,
    move_type: MoveType,
}

impl Move {
    fn new(from: usize, to: usize, move_type: MoveType) -> Move {
        debug_assert!(from < Bitboard::NUM_SQUARES);
        debug_assert!(to < Bitboard::NUM_SQUARES);
        Move {
            from: from as u8,
            to: to as u8,
            move_type,
        }
    }
}

struct MoveGenerator {
    pos: Position,
    move_list: Vec<Move>,
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

    fn new(pos: Position) -> MoveGenerator {
        MoveGenerator {
            pos,
            move_list: Vec::new(),
        }
    }

    fn add_move_if_legal(&mut self, m: Move) {
        let mut pos = self.pos.clone();
        let from_idx = m.from as usize;
        let to_idx = m.to as usize;
        // Promotion piece type is ignored here because it doesn't change the opposing side's
        // attacks
        pos.set_piece_at(to_idx, pos.piece_at(from_idx));
        pos.set_piece_at(from_idx, None);
        if m.move_type == MoveType::EnPassantCapture {
            let side_idx = pos.side_to_move() as usize;
            let captured_idx = (to_idx as i8 - Self::PAWN_PUSH_IDX_SHIFT[side_idx]) as usize;
            pos.set_piece_at(captured_idx, None);
        }

        if !pos.is_in_check(pos.side_to_move()) {
            self.move_list.push(m);
        }
    }

    fn generate_moves(&mut self) {
        self.move_list.clear();
        self.generate_pawn_moves();
        self.generate_knight_moves();
        self.generate_sliding_piece_moves(piece::Type::Bishop, Bishop::targets);
        self.generate_sliding_piece_moves(piece::Type::Rook, Rook::targets);
        self.generate_sliding_piece_moves(piece::Type::Queen, Queen::targets);
        self.generate_king_moves();
        self.generate_castles();
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
            for move_type in [
                MoveType::QueenPromo,
                MoveType::RookPromo,
                MoveType::BishopPromo,
                MoveType::KnightPromo,
            ]
            .iter()
            {
                self.add_move_if_legal(Move::new(origin, target, *move_type));
            }
        }
        while non_promo_targets != Bitboard::EMPTY {
            let target = non_promo_targets.bit_scan_forward_reset();
            let origin = (target as i8 - Self::PAWN_PUSH_IDX_SHIFT[side_idx]) as usize;
            self.add_move_if_legal(Move::new(origin, target, MoveType::Quiet));
        }
        while double_push_targets != Bitboard::EMPTY {
            let target = double_push_targets.bit_scan_forward_reset();
            let origin = (target as i8 - 2 * Self::PAWN_PUSH_IDX_SHIFT[side_idx]) as usize;
            self.add_move_if_legal(Move::new(origin, target, MoveType::DoublePawnPush));
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
            for move_type in [
                MoveType::QueenPromoCapture,
                MoveType::RookPromoCapture,
                MoveType::BishopPromoCapture,
                MoveType::KnightPromoCapture,
            ]
            .iter()
            {
                self.add_move_if_legal(Move::new(origin, target, *move_type));
            }
        }

        while non_promo_captures != Bitboard::EMPTY {
            let target = non_promo_captures.bit_scan_forward_reset();
            let origin = (target as i8 - idx_shift) as usize;
            let move_type = if Bitboard(0x1 << target) == en_passant_square {
                MoveType::EnPassantCapture
            } else {
                MoveType::Capture
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
            self.add_move_if_legal(Move::new(origin, target, MoveType::Capture));
        }
        while quiets != Bitboard::EMPTY {
            let target = quiets.bit_scan_forward_reset();
            self.add_move_if_legal(Move::new(origin, target, MoveType::Quiet));
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
                Some(piece::Piece::WhiteKing),
                self.pos.piece_at(Bitboard::IDX_E1)
            );
            debug_assert_eq!(
                Some(piece::Piece::WhiteRook),
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
                    MoveType::KingsideCastle,
                ));
            }
        }
        if castling_rights.contains(CastlingRights::WHITE_QUEENSIDE) {
            debug_assert_eq!(
                Some(piece::Piece::WhiteKing),
                self.pos.piece_at(Bitboard::IDX_E1)
            );
            debug_assert_eq!(
                Some(piece::Piece::WhiteRook),
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
                    MoveType::QueensideCastle,
                ));
            }
        }
    }

    fn generate_black_castles(&mut self) {
        let castling_rights = self.pos.castling_rights();
        let attacked_by_opponent = self.pos.attacked_squares(!self.pos.side_to_move());

        if castling_rights.contains(CastlingRights::BLACK_KINGSIDE) {
            debug_assert_eq!(
                Some(piece::Piece::BlackKing),
                self.pos.piece_at(Bitboard::IDX_E8)
            );
            debug_assert_eq!(
                Some(piece::Piece::BlackRook),
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
                    MoveType::KingsideCastle,
                ));
            }
        }
        if castling_rights.contains(CastlingRights::BLACK_QUEENSIDE) {
            debug_assert_eq!(
                Some(piece::Piece::BlackKing),
                self.pos.piece_at(Bitboard::IDX_E8)
            );
            debug_assert_eq!(
                Some(piece::Piece::BlackRook),
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
                    MoveType::QueensideCastle,
                ));
            }
        }
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
            Move::new(Bitboard::IDX_A2, Bitboard::IDX_A3, MoveType::Quiet),
            Move::new(Bitboard::IDX_B2, Bitboard::IDX_B3, MoveType::Quiet),
            Move::new(Bitboard::IDX_C2, Bitboard::IDX_C3, MoveType::Quiet),
            Move::new(Bitboard::IDX_D2, Bitboard::IDX_D3, MoveType::Quiet),
            Move::new(Bitboard::IDX_E2, Bitboard::IDX_E3, MoveType::Quiet),
            Move::new(Bitboard::IDX_F2, Bitboard::IDX_F3, MoveType::Quiet),
            Move::new(Bitboard::IDX_G2, Bitboard::IDX_G3, MoveType::Quiet),
            Move::new(Bitboard::IDX_H2, Bitboard::IDX_H3, MoveType::Quiet),
            Move::new(Bitboard::IDX_A2, Bitboard::IDX_A4, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_B2, Bitboard::IDX_B4, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_C2, Bitboard::IDX_C4, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_D2, Bitboard::IDX_D4, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_E2, Bitboard::IDX_E4, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_F2, Bitboard::IDX_F4, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_G2, Bitboard::IDX_G4, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_H2, Bitboard::IDX_H4, MoveType::DoublePawnPush),
            // Knight
            Move::new(Bitboard::IDX_B1, Bitboard::IDX_A3, MoveType::Quiet),
            Move::new(Bitboard::IDX_B1, Bitboard::IDX_C3, MoveType::Quiet),
            Move::new(Bitboard::IDX_G1, Bitboard::IDX_F3, MoveType::Quiet),
            Move::new(Bitboard::IDX_G1, Bitboard::IDX_H3, MoveType::Quiet),
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
        pos.set_piece_at(Bitboard::IDX_E4, Some(piece::Piece::WhitePawn));
        pos.set_en_passant_square(Bitboard::E3);
        pos.set_side_to_move(Side::Black);
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

        let expected_moves = [
            // Pawn
            Move::new(Bitboard::IDX_A7, Bitboard::IDX_A6, MoveType::Quiet),
            Move::new(Bitboard::IDX_B7, Bitboard::IDX_B6, MoveType::Quiet),
            Move::new(Bitboard::IDX_C7, Bitboard::IDX_C6, MoveType::Quiet),
            Move::new(Bitboard::IDX_D7, Bitboard::IDX_D6, MoveType::Quiet),
            Move::new(Bitboard::IDX_E7, Bitboard::IDX_E6, MoveType::Quiet),
            Move::new(Bitboard::IDX_F7, Bitboard::IDX_F6, MoveType::Quiet),
            Move::new(Bitboard::IDX_G7, Bitboard::IDX_G6, MoveType::Quiet),
            Move::new(Bitboard::IDX_H7, Bitboard::IDX_H6, MoveType::Quiet),
            Move::new(Bitboard::IDX_A7, Bitboard::IDX_A5, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_B7, Bitboard::IDX_B5, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_C7, Bitboard::IDX_C5, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_D7, Bitboard::IDX_D5, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_E7, Bitboard::IDX_E5, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_F7, Bitboard::IDX_F5, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_G7, Bitboard::IDX_G5, MoveType::DoublePawnPush),
            Move::new(Bitboard::IDX_H7, Bitboard::IDX_H5, MoveType::DoublePawnPush),
            // Knight
            Move::new(Bitboard::IDX_B8, Bitboard::IDX_A6, MoveType::Quiet),
            Move::new(Bitboard::IDX_B8, Bitboard::IDX_C6, MoveType::Quiet),
            Move::new(Bitboard::IDX_G8, Bitboard::IDX_F6, MoveType::Quiet),
            Move::new(Bitboard::IDX_G8, Bitboard::IDX_H6, MoveType::Quiet),
        ];

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn white_pawn_captures() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_A2, Some(piece::Piece::WhitePawn));
        pos.set_piece_at(Bitboard::IDX_H2, Some(piece::Piece::WhitePawn));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_A3, Some(piece::Piece::BlackPawn));
        pos.set_piece_at(Bitboard::IDX_B3, Some(piece::Piece::BlackPawn));
        pos.set_piece_at(Bitboard::IDX_G3, Some(piece::Piece::BlackPawn));
        pos.set_piece_at(Bitboard::IDX_H3, Some(piece::Piece::BlackPawn));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::Quiet),
            // Pawn
            Move::new(Bitboard::IDX_A2, Bitboard::IDX_B3, MoveType::Capture),
            Move::new(Bitboard::IDX_H2, Bitboard::IDX_G3, MoveType::Capture),
        ];

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn black_pawn_captures() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_A7, Some(piece::Piece::BlackPawn));
        pos.set_piece_at(Bitboard::IDX_H7, Some(piece::Piece::BlackPawn));
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_A6, Some(piece::Piece::WhitePawn));
        pos.set_piece_at(Bitboard::IDX_B6, Some(piece::Piece::WhitePawn));
        pos.set_piece_at(Bitboard::IDX_G6, Some(piece::Piece::WhitePawn));
        pos.set_piece_at(Bitboard::IDX_H6, Some(piece::Piece::WhitePawn));
        pos.set_side_to_move(Side::Black);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_D8, MoveType::Quiet),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_D7, MoveType::Quiet),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_E7, MoveType::Quiet),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_F8, MoveType::Quiet),
            // Pawn
            Move::new(Bitboard::IDX_A7, Bitboard::IDX_B6, MoveType::Capture),
            Move::new(Bitboard::IDX_H7, Bitboard::IDX_G6, MoveType::Capture),
        ];

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn king_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_F2, Some(piece::Piece::BlackPawn));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

        let expected_moves = [
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::Capture),
        ];

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn knight_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_B1, Some(piece::Piece::WhiteKnight));
        pos.set_piece_at(Bitboard::IDX_C3, Some(piece::Piece::WhitePawn));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_A3, Some(piece::Piece::BlackPawn));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::Quiet),
            // Pawn
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C4, MoveType::Quiet),
            // Knight
            Move::new(Bitboard::IDX_B1, Bitboard::IDX_D2, MoveType::Quiet),
            Move::new(Bitboard::IDX_B1, Bitboard::IDX_A3, MoveType::Capture),
        ];

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn bishop_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_C3, Some(piece::Piece::WhiteBishop));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_G7, Some(piece::Piece::BlackPawn));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::Quiet),
            // Bishop
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_B2, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_A1, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_D2, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_B4, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_A5, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_D4, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_E5, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_F6, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_G7, MoveType::Capture),
        ];

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn rook_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_E3, Some(piece::Piece::WhiteRook));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_E7, Some(piece::Piece::BlackPawn));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::Quiet),
            // Rook
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_E2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_D3, MoveType::Quiet),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_C3, MoveType::Quiet),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_B3, MoveType::Quiet),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_A3, MoveType::Quiet),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_F3, MoveType::Quiet),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_G3, MoveType::Quiet),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_H3, MoveType::Quiet),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_E4, MoveType::Quiet),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_E5, MoveType::Quiet),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_E6, MoveType::Quiet),
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_E7, MoveType::Capture),
        ];

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn queen_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_E3, Some(piece::Piece::WhitePawn));
        pos.set_piece_at(Bitboard::IDX_C3, Some(piece::Piece::WhiteQueen));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_C7, Some(piece::Piece::BlackPawn));
        pos.set_piece_at(Bitboard::IDX_G7, Some(piece::Piece::BlackPawn));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::Quiet),
            // Pawn
            Move::new(Bitboard::IDX_E3, Bitboard::IDX_E4, MoveType::Quiet),
            // Queen ranks and files
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C2, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C1, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_B3, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_A3, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_D3, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C4, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C5, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C6, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_C7, MoveType::Capture),
            // Queen diagonals
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_B2, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_A1, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_D2, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_B4, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_A5, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_D4, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_E5, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_F6, MoveType::Quiet),
            Move::new(Bitboard::IDX_C3, Bitboard::IDX_G7, MoveType::Capture),
        ];

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn white_promotions() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_B7, Some(piece::Piece::WhitePawn));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_A8, Some(piece::Piece::BlackRook));
        pos.set_piece_at(Bitboard::IDX_C8, Some(piece::Piece::BlackBishop));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::Quiet),
            // Pawns
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_A8,
                MoveType::KnightPromoCapture,
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_A8,
                MoveType::BishopPromoCapture,
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_A8,
                MoveType::RookPromoCapture,
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_A8,
                MoveType::QueenPromoCapture,
            ),
            Move::new(Bitboard::IDX_B7, Bitboard::IDX_B8, MoveType::KnightPromo),
            Move::new(Bitboard::IDX_B7, Bitboard::IDX_B8, MoveType::BishopPromo),
            Move::new(Bitboard::IDX_B7, Bitboard::IDX_B8, MoveType::RookPromo),
            Move::new(Bitboard::IDX_B7, Bitboard::IDX_B8, MoveType::QueenPromo),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_C8,
                MoveType::KnightPromoCapture,
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_C8,
                MoveType::BishopPromoCapture,
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_C8,
                MoveType::RookPromoCapture,
            ),
            Move::new(
                Bitboard::IDX_B7,
                Bitboard::IDX_C8,
                MoveType::QueenPromoCapture,
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
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_B2, Some(piece::Piece::BlackPawn));
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_A1, Some(piece::Piece::WhiteRook));
        pos.set_piece_at(Bitboard::IDX_C1, Some(piece::Piece::WhiteBishop));
        pos.set_side_to_move(Side::Black);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_D7, MoveType::Quiet),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_D8, MoveType::Quiet),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_E7, MoveType::Quiet),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_F7, MoveType::Quiet),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_F8, MoveType::Quiet),
            // Pawns
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_A1,
                MoveType::KnightPromoCapture,
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_A1,
                MoveType::BishopPromoCapture,
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_A1,
                MoveType::RookPromoCapture,
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_A1,
                MoveType::QueenPromoCapture,
            ),
            Move::new(Bitboard::IDX_B2, Bitboard::IDX_B1, MoveType::KnightPromo),
            Move::new(Bitboard::IDX_B2, Bitboard::IDX_B1, MoveType::BishopPromo),
            Move::new(Bitboard::IDX_B2, Bitboard::IDX_B1, MoveType::RookPromo),
            Move::new(Bitboard::IDX_B2, Bitboard::IDX_B1, MoveType::QueenPromo),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_C1,
                MoveType::KnightPromoCapture,
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_C1,
                MoveType::BishopPromoCapture,
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_C1,
                MoveType::RookPromoCapture,
            ),
            Move::new(
                Bitboard::IDX_B2,
                Bitboard::IDX_C1,
                MoveType::QueenPromoCapture,
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
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_D5, Some(piece::Piece::WhitePawn));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_C5, Some(piece::Piece::BlackPawn));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        pos.set_en_passant_square(Bitboard::C6);
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D1, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_D2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_E2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F2, MoveType::Quiet),
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_F1, MoveType::Quiet),
            // Pawn
            Move::new(Bitboard::IDX_D5, Bitboard::IDX_D6, MoveType::Quiet),
            Move::new(
                Bitboard::IDX_D5,
                Bitboard::IDX_C6,
                MoveType::EnPassantCapture,
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
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_D4, Some(piece::Piece::BlackPawn));
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_C4, Some(piece::Piece::WhitePawn));
        pos.set_side_to_move(Side::Black);
        pos.set_castling_rights(CastlingRights::empty());
        pos.set_en_passant_square(Bitboard::C3);
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();

        let expected_moves = [
            // King
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_D8, MoveType::Quiet),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_D7, MoveType::Quiet),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_E7, MoveType::Quiet),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_F7, MoveType::Quiet),
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_F8, MoveType::Quiet),
            // Pawn
            Move::new(Bitboard::IDX_D4, Bitboard::IDX_D3, MoveType::Quiet),
            Move::new(
                Bitboard::IDX_D4,
                Bitboard::IDX_C3,
                MoveType::EnPassantCapture,
            ),
        ];

        assert_eq!(expected_moves.len(), movegen.move_list.len());
        for exp_move in expected_moves.iter() {
            assert!(movegen.move_list.contains(exp_move));
        }
    }

    #[test]
    fn white_castles() {
        let kingside_castle =
            Move::new(Bitboard::IDX_E1, Bitboard::IDX_G1, MoveType::KingsideCastle);
        let queenside_castle = Move::new(
            Bitboard::IDX_E1,
            Bitboard::IDX_C1,
            MoveType::QueensideCastle,
        );

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_A1, Some(piece::Piece::WhiteRook));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::WhiteRook));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
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
        pos_blocked.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WhiteKnight));
        pos_blocked.set_piece_at(Bitboard::IDX_B1, Some(piece::Piece::BlackKnight));
        let mut movegen = MoveGenerator::new(pos_blocked);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // King attacked
        let mut pos_in_check = pos.clone();
        pos_in_check.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::BlackRook));
        let mut movegen = MoveGenerator::new(pos_in_check);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // Square traversed by king attacked
        let mut pos_traverse_attacked = pos.clone();
        pos_traverse_attacked.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::BlackBishop));
        let mut movegen = MoveGenerator::new(pos_traverse_attacked);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // Target square attacked
        let mut pos_target_attacked = pos.clone();
        pos_target_attacked.set_piece_at(Bitboard::IDX_E3, Some(piece::Piece::BlackBishop));
        let mut movegen = MoveGenerator::new(pos_target_attacked);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // Rook attacked (castling is legal)
        let mut pos_target_attacked = pos.clone();
        pos_target_attacked.set_piece_at(Bitboard::IDX_E4, Some(piece::Piece::BlackBishop));
        pos_target_attacked.set_piece_at(Bitboard::IDX_E5, Some(piece::Piece::BlackBishop));
        let mut movegen = MoveGenerator::new(pos_target_attacked);
        movegen.generate_moves();
        assert!(movegen.move_list.contains(&kingside_castle));
        assert!(movegen.move_list.contains(&queenside_castle));
    }

    #[test]
    fn black_castles() {
        let kingside_castle =
            Move::new(Bitboard::IDX_E8, Bitboard::IDX_G8, MoveType::KingsideCastle);
        let queenside_castle = Move::new(
            Bitboard::IDX_E8,
            Bitboard::IDX_C8,
            MoveType::QueensideCastle,
        );

        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_A8, Some(piece::Piece::BlackRook));
        pos.set_piece_at(Bitboard::IDX_H8, Some(piece::Piece::BlackRook));
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
        pos_blocked.set_piece_at(Bitboard::IDX_G8, Some(piece::Piece::BlackKnight));
        pos_blocked.set_piece_at(Bitboard::IDX_B8, Some(piece::Piece::WhiteKnight));
        let mut movegen = MoveGenerator::new(pos_blocked);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // King attacked
        let mut pos_in_check = pos.clone();
        pos_in_check.set_piece_at(Bitboard::IDX_E7, Some(piece::Piece::WhiteRook));
        let mut movegen = MoveGenerator::new(pos_in_check);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // Square traversed by king attacked
        let mut pos_traverse_attacked = pos.clone();
        pos_traverse_attacked.set_piece_at(Bitboard::IDX_E7, Some(piece::Piece::WhiteBishop));
        let mut movegen = MoveGenerator::new(pos_traverse_attacked);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // Target square attacked
        let mut pos_target_attacked = pos.clone();
        pos_target_attacked.set_piece_at(Bitboard::IDX_E6, Some(piece::Piece::WhiteBishop));
        let mut movegen = MoveGenerator::new(pos_target_attacked);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&kingside_castle));
        assert!(!movegen.move_list.contains(&queenside_castle));

        // Rook attacked (castling is legal)
        let mut pos_target_attacked = pos.clone();
        pos_target_attacked.set_piece_at(Bitboard::IDX_E4, Some(piece::Piece::WhiteBishop));
        pos_target_attacked.set_piece_at(Bitboard::IDX_E5, Some(piece::Piece::WhiteBishop));
        let mut movegen = MoveGenerator::new(pos_target_attacked);
        movegen.generate_moves();
        assert!(movegen.move_list.contains(&kingside_castle));
        assert!(movegen.move_list.contains(&queenside_castle));
    }

    #[test]
    fn king_not_left_in_check_after_pawn_moves() {
        let mut pos_pawn = Position::empty();
        pos_pawn.set_piece_at(Bitboard::IDX_D2, Some(piece::Piece::WhiteKing));
        pos_pawn.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::WhitePawn));
        pos_pawn.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos_pawn.set_piece_at(Bitboard::IDX_H2, Some(piece::Piece::BlackRook));
        pos_pawn.set_piece_at(Bitboard::IDX_F3, Some(piece::Piece::BlackRook));
        pos_pawn.set_side_to_move(Side::White);
        pos_pawn.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos_pawn.clone());
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_E2,
            Bitboard::IDX_E3,
            MoveType::Quiet
        )));
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_E2,
            Bitboard::IDX_E4,
            MoveType::DoublePawnPush
        )));
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_E2,
            Bitboard::IDX_F3,
            MoveType::Capture
        )));

        let mut pos_pawn_promo = Position::empty();
        pos_pawn_promo.set_piece_at(Bitboard::IDX_A7, Some(piece::Piece::WhiteKing));
        pos_pawn_promo.set_piece_at(Bitboard::IDX_B7, Some(piece::Piece::WhitePawn));
        pos_pawn_promo.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos_pawn_promo.set_piece_at(Bitboard::IDX_H7, Some(piece::Piece::BlackRook));
        pos_pawn_promo.set_piece_at(Bitboard::IDX_C8, Some(piece::Piece::BlackRook));
        pos_pawn_promo.set_side_to_move(Side::White);
        pos_pawn_promo.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos_pawn_promo);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_B7,
            Bitboard::IDX_B8,
            MoveType::QueenPromo
        )));
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_B7,
            Bitboard::IDX_C8,
            MoveType::QueenPromoCapture
        )));

        let mut pos_pawn_en_passant = Position::empty();
        pos_pawn_en_passant.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::WhiteKing));
        pos_pawn_en_passant.set_piece_at(Bitboard::IDX_C5, Some(piece::Piece::WhitePawn));
        pos_pawn_en_passant.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos_pawn_en_passant.set_piece_at(Bitboard::IDX_A6, Some(piece::Piece::BlackBishop));
        pos_pawn_en_passant.set_piece_at(Bitboard::IDX_B5, Some(piece::Piece::BlackPawn));
        pos_pawn_en_passant.set_side_to_move(Side::White);
        pos_pawn_en_passant.set_castling_rights(CastlingRights::empty());
        pos_pawn_en_passant.set_en_passant_square(Bitboard::B6);
        let mut movegen = MoveGenerator::new(pos_pawn_en_passant);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_C5,
            Bitboard::IDX_B6,
            MoveType::EnPassantCapture
        )));
    }

    #[test]
    fn king_not_left_in_check_after_knight_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WhiteKnight));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::BlackRook));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_E2,
            MoveType::Quiet
        )));
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F3,
            MoveType::Quiet
        )));
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_H3,
            MoveType::Quiet
        )));
    }

    #[test]
    fn king_not_left_in_check_after_bishop_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WhiteBishop));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::BlackRook));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F2,
            MoveType::Quiet
        )));
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_H2,
            MoveType::Quiet
        )));
    }

    #[test]
    fn king_not_left_in_check_after_rook_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WhiteRook));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::BlackRook));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_G2,
            MoveType::Quiet
        )));
        assert!(movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F1,
            MoveType::Quiet
        )));
        assert!(movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_H1,
            MoveType::Capture
        )));
    }

    #[test]
    fn king_not_left_in_check_after_queen_moves() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WhiteQueen));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::BlackRook));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F2,
            MoveType::Quiet
        )));
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_G2,
            MoveType::Quiet
        )));
        assert!(movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_F1,
            MoveType::Quiet
        )));
        assert!(movegen.move_list.contains(&Move::new(
            Bitboard::IDX_G1,
            Bitboard::IDX_H1,
            MoveType::Capture
        )));
    }

    #[test]
    fn king_does_not_move_into_check() {
        let mut pos = Position::empty();
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_G1, Some(piece::Piece::WhiteRook));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_H2, Some(piece::Piece::BlackRook));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        let mut movegen = MoveGenerator::new(pos);
        movegen.generate_moves();
        assert!(!movegen.move_list.contains(&Move::new(
            Bitboard::IDX_E1,
            Bitboard::IDX_E2,
            MoveType::Quiet
        )));
    }
}
