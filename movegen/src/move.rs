use crate::bitboard::Bitboard;
use crate::piece;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MoveType(u8);

impl MoveType {
    pub const QUIET: Self = MoveType(0b0000);
    pub const DOUBLE_PAWN_PUSH: Self = MoveType(0b0001);
    pub const CASTLE_KINGSIDE: Self = MoveType(0b0010);
    pub const CASTLE_QUEENSIDE: Self = MoveType(0b0011);
    pub const CAPTURE: Self = MoveType(0b0100);
    pub const EN_PASSANT_CAPTURE: Self = MoveType(0b0101);
    pub const PROMOTION: Self = MoveType(0b1000);
    pub const PROMOTION_CAPTURE: Self = MoveType(Self::CAPTURE.0 | Self::PROMOTION.0);
    pub const PROMOTION_KNIGHT: Self = MoveType(Self::PROMOTION.0 | piece::Type::Knight as u8);
    pub const PROMOTION_BISHOP: Self = MoveType(Self::PROMOTION.0 | piece::Type::Bishop as u8);
    pub const PROMOTION_ROOK: Self = MoveType(Self::PROMOTION.0 | piece::Type::Rook as u8);
    pub const PROMOTION_QUEEN: Self = MoveType(Self::PROMOTION.0 | piece::Type::Queen as u8);
    pub const PROMOTION_CAPTURE_KNIGHT: Self =
        MoveType(Self::PROMOTION_CAPTURE.0 | piece::Type::Knight as u8);
    pub const PROMOTION_CAPTURE_BISHOP: Self =
        MoveType(Self::PROMOTION_CAPTURE.0 | piece::Type::Bishop as u8);
    pub const PROMOTION_CAPTURE_ROOK: Self =
        MoveType(Self::PROMOTION_CAPTURE.0 | piece::Type::Rook as u8);
    pub const PROMOTION_CAPTURE_QUEEN: Self =
        MoveType(Self::PROMOTION_CAPTURE.0 | piece::Type::Queen as u8);

    pub fn new_with_promo_piece(move_type: MoveType, promo_piece: piece::Type) -> Self {
        // Bits 0-1: promotion piece
        // Bit 2: capture flag
        // Bit 3: promotion flag
        debug_assert!(move_type.is_promotion());
        debug_assert!(promo_piece as u8 <= 0b11);
        MoveType(move_type.0 | promo_piece as u8)
    }

    fn is_capture(&self) -> bool {
        self.0 & Self::CAPTURE.0 != 0
    }

    fn is_en_passant(&self) -> bool {
        self.0 == Self::EN_PASSANT_CAPTURE.0
    }

    fn is_promotion(&self) -> bool {
        self.0 & Self::PROMOTION.0 != 0
    }

    // This method must not be called if the move type is not a promotion.
    pub fn promo_piece_unchecked(&self) -> piece::Type {
        debug_assert!(self.is_promotion());
        unsafe { std::mem::transmute::<u8, piece::Type>(self.0 & 0b11) }
    }
}

// The moves are encoded to fit into a u16
// Bits 0-5: origin square
// Bits 6-11: target square
// Bits 12-15: move type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move(u16);

impl Move {
    pub fn new(origin: usize, target: usize, move_type: MoveType) -> Move {
        debug_assert!(origin < Bitboard::NUM_SQUARES);
        debug_assert!(target < Bitboard::NUM_SQUARES);
        debug_assert!(move_type.0 < 0b1_0000);
        Move(origin as u16 | (target as u16) << 6 | (move_type.0 as u16) << 12)
    }

    pub fn origin(&self) -> usize {
        (self.0 & 0b11_1111) as usize
    }

    pub fn target(&self) -> usize {
        (self.0 >> 6 & 0b11_1111) as usize
    }

    pub fn move_type(&self) -> MoveType {
        unsafe { std::mem::transmute::<u8, MoveType>((self.0 >> 12) as u8) }
    }

    pub fn is_capture(&self) -> bool {
        self.move_type().is_capture()
    }

    pub fn is_en_passant(&self) -> bool {
        self.move_type().is_en_passant()
    }

    pub fn is_promotion(&self) -> bool {
        self.move_type().is_promotion()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_type_properties() {
        assert!(!MoveType::QUIET.is_capture());
        assert!(!MoveType::QUIET.is_en_passant());
        assert!(!MoveType::QUIET.is_promotion());

        assert!(!MoveType::DOUBLE_PAWN_PUSH.is_capture());
        assert!(!MoveType::DOUBLE_PAWN_PUSH.is_en_passant());
        assert!(!MoveType::DOUBLE_PAWN_PUSH.is_promotion());

        assert!(!MoveType::CASTLE_KINGSIDE.is_capture());
        assert!(!MoveType::CASTLE_KINGSIDE.is_en_passant());
        assert!(!MoveType::CASTLE_KINGSIDE.is_promotion());

        assert!(!MoveType::CASTLE_QUEENSIDE.is_capture());
        assert!(!MoveType::CASTLE_QUEENSIDE.is_en_passant());
        assert!(!MoveType::CASTLE_QUEENSIDE.is_promotion());

        assert!(MoveType::CAPTURE.is_capture());
        assert!(!MoveType::CAPTURE.is_en_passant());
        assert!(!MoveType::CAPTURE.is_promotion());

        assert!(MoveType::EN_PASSANT_CAPTURE.is_capture());
        assert!(MoveType::EN_PASSANT_CAPTURE.is_en_passant());
        assert!(!MoveType::EN_PASSANT_CAPTURE.is_promotion());

        assert!(!MoveType::PROMOTION_KNIGHT.is_capture());
        assert!(!MoveType::PROMOTION_KNIGHT.is_en_passant());
        assert!(MoveType::PROMOTION_KNIGHT.is_promotion());
        assert_eq!(
            piece::Type::Knight,
            MoveType::PROMOTION_KNIGHT.promo_piece_unchecked()
        );

        assert!(!MoveType::PROMOTION_BISHOP.is_capture());
        assert!(!MoveType::PROMOTION_BISHOP.is_en_passant());
        assert!(MoveType::PROMOTION_BISHOP.is_promotion());
        assert_eq!(
            piece::Type::Bishop,
            MoveType::PROMOTION_BISHOP.promo_piece_unchecked()
        );

        assert!(!MoveType::PROMOTION_ROOK.is_capture());
        assert!(!MoveType::PROMOTION_ROOK.is_en_passant());
        assert!(MoveType::PROMOTION_ROOK.is_promotion());
        assert_eq!(
            piece::Type::Rook,
            MoveType::PROMOTION_ROOK.promo_piece_unchecked()
        );

        assert!(!MoveType::PROMOTION_QUEEN.is_capture());
        assert!(!MoveType::PROMOTION_QUEEN.is_en_passant());
        assert!(MoveType::PROMOTION_QUEEN.is_promotion());
        assert_eq!(
            piece::Type::Queen,
            MoveType::PROMOTION_QUEEN.promo_piece_unchecked()
        );

        assert!(MoveType::PROMOTION_CAPTURE_KNIGHT.is_capture());
        assert!(!MoveType::PROMOTION_CAPTURE_KNIGHT.is_en_passant());
        assert!(MoveType::PROMOTION_CAPTURE_KNIGHT.is_promotion());
        assert_eq!(
            piece::Type::Knight,
            MoveType::PROMOTION_CAPTURE_KNIGHT.promo_piece_unchecked()
        );

        assert!(MoveType::PROMOTION_CAPTURE_BISHOP.is_capture());
        assert!(!MoveType::PROMOTION_CAPTURE_BISHOP.is_en_passant());
        assert!(MoveType::PROMOTION_CAPTURE_BISHOP.is_promotion());
        assert_eq!(
            piece::Type::Bishop,
            MoveType::PROMOTION_CAPTURE_BISHOP.promo_piece_unchecked()
        );

        assert!(MoveType::PROMOTION_CAPTURE_ROOK.is_capture());
        assert!(!MoveType::PROMOTION_CAPTURE_ROOK.is_en_passant());
        assert!(MoveType::PROMOTION_CAPTURE_ROOK.is_promotion());
        assert_eq!(
            piece::Type::Rook,
            MoveType::PROMOTION_CAPTURE_ROOK.promo_piece_unchecked()
        );

        assert!(MoveType::PROMOTION_CAPTURE_QUEEN.is_capture());
        assert!(!MoveType::PROMOTION_CAPTURE_QUEEN.is_en_passant());
        assert!(MoveType::PROMOTION_CAPTURE_QUEEN.is_promotion());
        assert_eq!(
            piece::Type::Queen,
            MoveType::PROMOTION_CAPTURE_QUEEN.promo_piece_unchecked()
        );
    }

    #[test]
    fn new_with_promo_piece() {
        assert_eq!(
            MoveType::PROMOTION_KNIGHT,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Knight)
        );
        assert_eq!(
            MoveType::PROMOTION_BISHOP,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Bishop)
        );
        assert_eq!(
            MoveType::PROMOTION_ROOK,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Rook)
        );
        assert_eq!(
            MoveType::PROMOTION_QUEEN,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Queen)
        );
        assert_eq!(
            MoveType::PROMOTION_CAPTURE_KNIGHT,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Knight)
        );
        assert_eq!(
            MoveType::PROMOTION_CAPTURE_BISHOP,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop)
        );
        assert_eq!(
            MoveType::PROMOTION_CAPTURE_ROOK,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Rook)
        );
        assert_eq!(
            MoveType::PROMOTION_CAPTURE_QUEEN,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Queen)
        );
    }

    #[test]
    fn move_properties() {
        let m = Move::new(Bitboard::IDX_E2, Bitboard::IDX_E3, MoveType::QUIET);
        assert_eq!(Bitboard::IDX_E2, m.origin());
        assert_eq!(Bitboard::IDX_E3, m.target());
        assert_eq!(MoveType::QUIET, m.move_type());

        let m = Move::new(
            Bitboard::IDX_E2,
            Bitboard::IDX_E4,
            MoveType::DOUBLE_PAWN_PUSH,
        );
        assert_eq!(Bitboard::IDX_E2, m.origin());
        assert_eq!(Bitboard::IDX_E4, m.target());
        assert_eq!(MoveType::DOUBLE_PAWN_PUSH, m.move_type());

        let m = Move::new(
            Bitboard::IDX_E1,
            Bitboard::IDX_G1,
            MoveType::CASTLE_KINGSIDE,
        );
        assert_eq!(Bitboard::IDX_E1, m.origin());
        assert_eq!(Bitboard::IDX_G1, m.target());
        assert_eq!(MoveType::CASTLE_KINGSIDE, m.move_type());

        let m = Move::new(
            Bitboard::IDX_E8,
            Bitboard::IDX_C8,
            MoveType::CASTLE_QUEENSIDE,
        );
        assert_eq!(Bitboard::IDX_E8, m.origin());
        assert_eq!(Bitboard::IDX_C8, m.target());
        assert_eq!(MoveType::CASTLE_QUEENSIDE, m.move_type());

        let m = Move::new(Bitboard::IDX_C4, Bitboard::IDX_D5, MoveType::CAPTURE);
        assert_eq!(Bitboard::IDX_C4, m.origin());
        assert_eq!(Bitboard::IDX_D5, m.target());
        assert_eq!(MoveType::CAPTURE, m.move_type());

        let m = Move::new(
            Bitboard::IDX_D6,
            Bitboard::IDX_E5,
            MoveType::EN_PASSANT_CAPTURE,
        );
        assert_eq!(Bitboard::IDX_D6, m.origin());
        assert_eq!(Bitboard::IDX_E5, m.target());
        assert_eq!(MoveType::EN_PASSANT_CAPTURE, m.move_type());

        let m = Move::new(
            Bitboard::IDX_A7,
            Bitboard::IDX_A8,
            MoveType::PROMOTION_KNIGHT,
        );
        assert_eq!(Bitboard::IDX_A7, m.origin());
        assert_eq!(Bitboard::IDX_A8, m.target());
        assert_eq!(MoveType::PROMOTION_KNIGHT, m.move_type());

        let m = Move::new(
            Bitboard::IDX_A7,
            Bitboard::IDX_A8,
            MoveType::PROMOTION_BISHOP,
        );
        assert_eq!(Bitboard::IDX_A7, m.origin());
        assert_eq!(Bitboard::IDX_A8, m.target());
        assert_eq!(MoveType::PROMOTION_BISHOP, m.move_type());

        let m = Move::new(Bitboard::IDX_A7, Bitboard::IDX_A8, MoveType::PROMOTION_ROOK);
        assert_eq!(Bitboard::IDX_A7, m.origin());
        assert_eq!(Bitboard::IDX_A8, m.target());
        assert_eq!(MoveType::PROMOTION_ROOK, m.move_type());

        let m = Move::new(
            Bitboard::IDX_A7,
            Bitboard::IDX_A8,
            MoveType::PROMOTION_QUEEN,
        );
        assert_eq!(Bitboard::IDX_A7, m.origin());
        assert_eq!(Bitboard::IDX_A8, m.target());
        assert_eq!(MoveType::PROMOTION_QUEEN, m.move_type());

        let m = Move::new(
            Bitboard::IDX_G2,
            Bitboard::IDX_H1,
            MoveType::PROMOTION_CAPTURE_KNIGHT,
        );
        assert_eq!(Bitboard::IDX_G2, m.origin());
        assert_eq!(Bitboard::IDX_H1, m.target());
        assert_eq!(MoveType::PROMOTION_CAPTURE_KNIGHT, m.move_type());

        let m = Move::new(
            Bitboard::IDX_G2,
            Bitboard::IDX_H1,
            MoveType::PROMOTION_CAPTURE_BISHOP,
        );
        assert_eq!(Bitboard::IDX_G2, m.origin());
        assert_eq!(Bitboard::IDX_H1, m.target());
        assert_eq!(MoveType::PROMOTION_CAPTURE_BISHOP, m.move_type());

        let m = Move::new(
            Bitboard::IDX_G2,
            Bitboard::IDX_H1,
            MoveType::PROMOTION_CAPTURE_ROOK,
        );
        assert_eq!(Bitboard::IDX_G2, m.origin());
        assert_eq!(Bitboard::IDX_H1, m.target());
        assert_eq!(MoveType::PROMOTION_CAPTURE_ROOK, m.move_type());

        let m = Move::new(
            Bitboard::IDX_G2,
            Bitboard::IDX_H1,
            MoveType::PROMOTION_CAPTURE_QUEEN,
        );
        assert_eq!(Bitboard::IDX_G2, m.origin());
        assert_eq!(Bitboard::IDX_H1, m.target());
        assert_eq!(MoveType::PROMOTION_CAPTURE_QUEEN, m.move_type());
    }
}
