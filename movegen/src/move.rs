use crate::piece;
use crate::square::Square;
use std::fmt;
use std::ops::{Deref, DerefMut};

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

    fn is_castle(&self) -> bool {
        self.0 == Self::CASTLE_KINGSIDE.0 || self.0 == Self::CASTLE_QUEENSIDE.0
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
    pub const NULL: Self = Self(0x0);

    pub fn new(origin: Square, target: Square, move_type: MoveType) -> Move {
        debug_assert!(origin.idx() < Square::NUM_SQUARES);
        debug_assert!(target.idx() < Square::NUM_SQUARES);
        debug_assert!(move_type.0 < 0b1_0000);
        Move(origin.idx() as u16 | (target.idx() as u16) << 6 | (move_type.0 as u16) << 12)
    }

    pub fn origin(&self) -> Square {
        Square::from_idx((self.0 & 0b11_1111) as usize)
    }

    pub fn target(&self) -> Square {
        Square::from_idx((self.0 >> 6 & 0b11_1111) as usize)
    }

    pub fn move_type(&self) -> MoveType {
        unsafe { std::mem::transmute::<u8, MoveType>((self.0 >> 12) as u8) }
    }

    pub fn is_castle(&self) -> bool {
        self.move_type().is_castle()
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

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let move_str = match self {
            &Move::NULL => String::from("null"),
            _ => match self.move_type() {
                MoveType::CASTLE_KINGSIDE => String::from("0-0"),
                MoveType::CASTLE_QUEENSIDE => String::from("0-0-0"),
                _ => {
                    let capture_str = if self.is_capture() { "x" } else { "" };
                    let promo_str = if self.is_promotion() {
                        match self.move_type().promo_piece_unchecked() {
                            piece::Type::Knight => "=N",
                            piece::Type::Bishop => "=B",
                            piece::Type::Rook => "=R",
                            piece::Type::Queen => "=Q",
                            _ => panic!(
                                "Invalid promotion piece `{:?}`",
                                self.move_type().promo_piece_unchecked()
                            ),
                        }
                    } else {
                        ""
                    };
                    format!(
                        "{}{}{}{}",
                        self.origin(),
                        capture_str,
                        self.target(),
                        promo_str
                    )
                }
            },
        };
        write!(f, "{}", move_str).unwrap();
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct MoveList(Vec<Move>);

impl MoveList {
    pub fn new() -> MoveList {
        MoveList(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> MoveList {
        MoveList(Vec::with_capacity(capacity))
    }

    pub fn truncate_at_null_move(&mut self) {
        if let Some(idx) = self.iter().position(|&m| m == Move::NULL) {
            self.truncate(idx);
        };
    }
}

impl Deref for MoveList {
    type Target = Vec<Move>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MoveList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for MoveList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let move_list_str = self
            .iter()
            .map(|&m| format!("{}", m))
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}", move_list_str).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_type_properties() {
        assert!(!MoveType::QUIET.is_castle());
        assert!(!MoveType::QUIET.is_capture());
        assert!(!MoveType::QUIET.is_en_passant());
        assert!(!MoveType::QUIET.is_promotion());

        assert!(!MoveType::DOUBLE_PAWN_PUSH.is_castle());
        assert!(!MoveType::DOUBLE_PAWN_PUSH.is_capture());
        assert!(!MoveType::DOUBLE_PAWN_PUSH.is_en_passant());
        assert!(!MoveType::DOUBLE_PAWN_PUSH.is_promotion());

        assert!(MoveType::CASTLE_KINGSIDE.is_castle());
        assert!(!MoveType::CASTLE_KINGSIDE.is_capture());
        assert!(!MoveType::CASTLE_KINGSIDE.is_en_passant());
        assert!(!MoveType::CASTLE_KINGSIDE.is_promotion());

        assert!(MoveType::CASTLE_QUEENSIDE.is_castle());
        assert!(!MoveType::CASTLE_QUEENSIDE.is_capture());
        assert!(!MoveType::CASTLE_QUEENSIDE.is_en_passant());
        assert!(!MoveType::CASTLE_QUEENSIDE.is_promotion());

        assert!(!MoveType::CAPTURE.is_castle());
        assert!(MoveType::CAPTURE.is_capture());
        assert!(!MoveType::CAPTURE.is_en_passant());
        assert!(!MoveType::CAPTURE.is_promotion());

        assert!(!MoveType::EN_PASSANT_CAPTURE.is_castle());
        assert!(MoveType::EN_PASSANT_CAPTURE.is_capture());
        assert!(MoveType::EN_PASSANT_CAPTURE.is_en_passant());
        assert!(!MoveType::EN_PASSANT_CAPTURE.is_promotion());

        assert!(!MoveType::PROMOTION_KNIGHT.is_castle());
        assert!(!MoveType::PROMOTION_KNIGHT.is_capture());
        assert!(!MoveType::PROMOTION_KNIGHT.is_en_passant());
        assert!(MoveType::PROMOTION_KNIGHT.is_promotion());
        assert_eq!(
            piece::Type::Knight,
            MoveType::PROMOTION_KNIGHT.promo_piece_unchecked()
        );

        assert!(!MoveType::PROMOTION_BISHOP.is_castle());
        assert!(!MoveType::PROMOTION_BISHOP.is_capture());
        assert!(!MoveType::PROMOTION_BISHOP.is_en_passant());
        assert!(MoveType::PROMOTION_BISHOP.is_promotion());
        assert_eq!(
            piece::Type::Bishop,
            MoveType::PROMOTION_BISHOP.promo_piece_unchecked()
        );

        assert!(!MoveType::PROMOTION_ROOK.is_castle());
        assert!(!MoveType::PROMOTION_ROOK.is_capture());
        assert!(!MoveType::PROMOTION_ROOK.is_en_passant());
        assert!(MoveType::PROMOTION_ROOK.is_promotion());
        assert_eq!(
            piece::Type::Rook,
            MoveType::PROMOTION_ROOK.promo_piece_unchecked()
        );

        assert!(!MoveType::PROMOTION_QUEEN.is_castle());
        assert!(!MoveType::PROMOTION_QUEEN.is_capture());
        assert!(!MoveType::PROMOTION_QUEEN.is_en_passant());
        assert!(MoveType::PROMOTION_QUEEN.is_promotion());
        assert_eq!(
            piece::Type::Queen,
            MoveType::PROMOTION_QUEEN.promo_piece_unchecked()
        );

        assert!(!MoveType::PROMOTION_CAPTURE_KNIGHT.is_castle());
        assert!(MoveType::PROMOTION_CAPTURE_KNIGHT.is_capture());
        assert!(!MoveType::PROMOTION_CAPTURE_KNIGHT.is_en_passant());
        assert!(MoveType::PROMOTION_CAPTURE_KNIGHT.is_promotion());
        assert_eq!(
            piece::Type::Knight,
            MoveType::PROMOTION_CAPTURE_KNIGHT.promo_piece_unchecked()
        );

        assert!(!MoveType::PROMOTION_CAPTURE_BISHOP.is_castle());
        assert!(MoveType::PROMOTION_CAPTURE_BISHOP.is_capture());
        assert!(!MoveType::PROMOTION_CAPTURE_BISHOP.is_en_passant());
        assert!(MoveType::PROMOTION_CAPTURE_BISHOP.is_promotion());
        assert_eq!(
            piece::Type::Bishop,
            MoveType::PROMOTION_CAPTURE_BISHOP.promo_piece_unchecked()
        );

        assert!(!MoveType::PROMOTION_CAPTURE_ROOK.is_castle());
        assert!(MoveType::PROMOTION_CAPTURE_ROOK.is_capture());
        assert!(!MoveType::PROMOTION_CAPTURE_ROOK.is_en_passant());
        assert!(MoveType::PROMOTION_CAPTURE_ROOK.is_promotion());
        assert_eq!(
            piece::Type::Rook,
            MoveType::PROMOTION_CAPTURE_ROOK.promo_piece_unchecked()
        );

        assert!(!MoveType::PROMOTION_CAPTURE_QUEEN.is_castle());
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
        let m = Move::new(Square::E2, Square::E3, MoveType::QUIET);
        assert_eq!(Square::E2, m.origin());
        assert_eq!(Square::E3, m.target());
        assert_eq!(MoveType::QUIET, m.move_type());

        let m = Move::new(Square::E2, Square::E4, MoveType::DOUBLE_PAWN_PUSH);
        assert_eq!(Square::E2, m.origin());
        assert_eq!(Square::E4, m.target());
        assert_eq!(MoveType::DOUBLE_PAWN_PUSH, m.move_type());

        let m = Move::new(Square::E1, Square::G1, MoveType::CASTLE_KINGSIDE);
        assert_eq!(Square::E1, m.origin());
        assert_eq!(Square::G1, m.target());
        assert_eq!(MoveType::CASTLE_KINGSIDE, m.move_type());

        let m = Move::new(Square::E8, Square::C8, MoveType::CASTLE_QUEENSIDE);
        assert_eq!(Square::E8, m.origin());
        assert_eq!(Square::C8, m.target());
        assert_eq!(MoveType::CASTLE_QUEENSIDE, m.move_type());

        let m = Move::new(Square::C4, Square::D5, MoveType::CAPTURE);
        assert_eq!(Square::C4, m.origin());
        assert_eq!(Square::D5, m.target());
        assert_eq!(MoveType::CAPTURE, m.move_type());

        let m = Move::new(Square::D6, Square::E5, MoveType::EN_PASSANT_CAPTURE);
        assert_eq!(Square::D6, m.origin());
        assert_eq!(Square::E5, m.target());
        assert_eq!(MoveType::EN_PASSANT_CAPTURE, m.move_type());

        let m = Move::new(Square::A7, Square::A8, MoveType::PROMOTION_KNIGHT);
        assert_eq!(Square::A7, m.origin());
        assert_eq!(Square::A8, m.target());
        assert_eq!(MoveType::PROMOTION_KNIGHT, m.move_type());

        let m = Move::new(Square::A7, Square::A8, MoveType::PROMOTION_BISHOP);
        assert_eq!(Square::A7, m.origin());
        assert_eq!(Square::A8, m.target());
        assert_eq!(MoveType::PROMOTION_BISHOP, m.move_type());

        let m = Move::new(Square::A7, Square::A8, MoveType::PROMOTION_ROOK);
        assert_eq!(Square::A7, m.origin());
        assert_eq!(Square::A8, m.target());
        assert_eq!(MoveType::PROMOTION_ROOK, m.move_type());

        let m = Move::new(Square::A7, Square::A8, MoveType::PROMOTION_QUEEN);
        assert_eq!(Square::A7, m.origin());
        assert_eq!(Square::A8, m.target());
        assert_eq!(MoveType::PROMOTION_QUEEN, m.move_type());

        let m = Move::new(Square::G2, Square::H1, MoveType::PROMOTION_CAPTURE_KNIGHT);
        assert_eq!(Square::G2, m.origin());
        assert_eq!(Square::H1, m.target());
        assert_eq!(MoveType::PROMOTION_CAPTURE_KNIGHT, m.move_type());

        let m = Move::new(Square::G2, Square::H1, MoveType::PROMOTION_CAPTURE_BISHOP);
        assert_eq!(Square::G2, m.origin());
        assert_eq!(Square::H1, m.target());
        assert_eq!(MoveType::PROMOTION_CAPTURE_BISHOP, m.move_type());

        let m = Move::new(Square::G2, Square::H1, MoveType::PROMOTION_CAPTURE_ROOK);
        assert_eq!(Square::G2, m.origin());
        assert_eq!(Square::H1, m.target());
        assert_eq!(MoveType::PROMOTION_CAPTURE_ROOK, m.move_type());

        let m = Move::new(Square::G2, Square::H1, MoveType::PROMOTION_CAPTURE_QUEEN);
        assert_eq!(Square::G2, m.origin());
        assert_eq!(Square::H1, m.target());
        assert_eq!(MoveType::PROMOTION_CAPTURE_QUEEN, m.move_type());
    }

    #[test]
    fn fmt_move() {
        assert_eq!("null", format!("{}", Move::NULL));

        assert_eq!(
            "d2d3",
            format!("{}", Move::new(Square::D2, Square::D3, MoveType::QUIET))
        );
        assert_eq!(
            "d7d5",
            format!(
                "{}",
                Move::new(Square::D7, Square::D5, MoveType::DOUBLE_PAWN_PUSH)
            )
        );
        assert_eq!(
            "0-0",
            format!(
                "{}",
                Move::new(Square::E1, Square::G1, MoveType::CASTLE_KINGSIDE)
            )
        );
        assert_eq!(
            "0-0-0",
            format!(
                "{}",
                Move::new(Square::E8, Square::C8, MoveType::CASTLE_QUEENSIDE)
            )
        );
        assert_eq!(
            "c4xd5",
            format!("{}", Move::new(Square::C4, Square::D5, MoveType::CAPTURE))
        );
        assert_eq!(
            "c4xd3",
            format!(
                "{}",
                Move::new(Square::C4, Square::D3, MoveType::EN_PASSANT_CAPTURE)
            )
        );
        assert_eq!(
            "a7a8=N",
            format!(
                "{}",
                Move::new(Square::A7, Square::A8, MoveType::PROMOTION_KNIGHT)
            )
        );
        assert_eq!(
            "b2b1=B",
            format!(
                "{}",
                Move::new(Square::B2, Square::B1, MoveType::PROMOTION_BISHOP)
            )
        );
        assert_eq!(
            "e7e8=R",
            format!(
                "{}",
                Move::new(Square::E7, Square::E8, MoveType::PROMOTION_ROOK)
            )
        );
        assert_eq!(
            "g2g1=Q",
            format!(
                "{}",
                Move::new(Square::G2, Square::G1, MoveType::PROMOTION_QUEEN)
            )
        );
        assert_eq!(
            "a7xb8=N",
            format!(
                "{}",
                Move::new(Square::A7, Square::B8, MoveType::PROMOTION_CAPTURE_KNIGHT)
            )
        );
        assert_eq!(
            "b2xc1=B",
            format!(
                "{}",
                Move::new(Square::B2, Square::C1, MoveType::PROMOTION_CAPTURE_BISHOP)
            )
        );
        assert_eq!(
            "e7xf8=R",
            format!(
                "{}",
                Move::new(Square::E7, Square::F8, MoveType::PROMOTION_CAPTURE_ROOK)
            )
        );
        assert_eq!(
            "g2xh1=Q",
            format!(
                "{}",
                Move::new(Square::G2, Square::H1, MoveType::PROMOTION_CAPTURE_QUEEN)
            )
        );
    }

    #[test]
    fn truncate_at_null_move() {
        let mut move_list = MoveList::new();
        move_list.push(Move::new(
            Square::D2,
            Square::D4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));
        move_list.push(Move::NULL);
        move_list.push(Move::new(
            Square::C2,
            Square::C4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));

        assert_eq!(3, move_list.len());
        move_list.truncate_at_null_move();
        assert_eq!(1, move_list.len());
        // Expected: Calling this method on a move list without a null move has no effect
        move_list.truncate_at_null_move();
        assert_eq!(1, move_list.len());
    }

    #[test]
    fn fmt_movelist() {
        let mut move_list = MoveList::new();
        move_list.push(Move::new(
            Square::D2,
            Square::D4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));
        move_list.push(Move::new(
            Square::D7,
            Square::D5,
            MoveType::DOUBLE_PAWN_PUSH,
        ));
        move_list.push(Move::new(
            Square::C2,
            Square::C4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));

        assert_eq!("d2d4 d7d5 c2c4", format!("{}", move_list));
    }
}
