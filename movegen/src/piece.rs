use crate::side::Side;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Type {
    // The pieces are encoded this way to make the 4 possible promotion pieces fit into 2 bits.
    Pawn = 5,
    Knight = 0,
    Bishop = 1,
    Rook = 2,
    Queen = 3,
    King = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece(u8);

impl Piece {
    pub const WHITE_PAWN: Self = Self::new(Side::White, Type::Pawn);
    pub const WHITE_KNIGHT: Self = Self::new(Side::White, Type::Knight);
    pub const WHITE_BISHOP: Self = Self::new(Side::White, Type::Bishop);
    pub const WHITE_ROOK: Self = Self::new(Side::White, Type::Rook);
    pub const WHITE_QUEEN: Self = Self::new(Side::White, Type::Queen);
    pub const WHITE_KING: Self = Self::new(Side::White, Type::King);
    pub const BLACK_PAWN: Self = Self::new(Side::Black, Type::Pawn);
    pub const BLACK_KNIGHT: Self = Self::new(Side::Black, Type::Knight);
    pub const BLACK_BISHOP: Self = Self::new(Side::Black, Type::Bishop);
    pub const BLACK_ROOK: Self = Self::new(Side::Black, Type::Rook);
    pub const BLACK_QUEEN: Self = Self::new(Side::Black, Type::Queen);
    pub const BLACK_KING: Self = Self::new(Side::Black, Type::King);

    pub const fn new(s: Side, t: Type) -> Self {
        // Bit 0: piece side
        // Bits 1-3: piece type
        Piece(s as u8 | (t as u8) << 1)
    }

    pub fn piece_side(&self) -> Side {
        unsafe { std::mem::transmute::<u8, Side>(self.0 & 0x1) }
    }

    pub fn piece_type(&self) -> Type {
        unsafe { std::mem::transmute::<u8, Type>(self.0 >> 1) }
    }

    pub fn is_sliding_piece(&self) -> bool {
        matches!(self.piece_type(), Type::Bishop | Type::Rook | Type::Queen)
    }

    pub fn from_ascii(c: u8) -> Result<Self, String> {
        match c {
            b'P' => Ok(Piece::WHITE_PAWN),
            b'N' => Ok(Piece::WHITE_KNIGHT),
            b'B' => Ok(Piece::WHITE_BISHOP),
            b'R' => Ok(Piece::WHITE_ROOK),
            b'Q' => Ok(Piece::WHITE_QUEEN),
            b'K' => Ok(Piece::WHITE_KING),
            b'p' => Ok(Piece::BLACK_PAWN),
            b'n' => Ok(Piece::BLACK_KNIGHT),
            b'b' => Ok(Piece::BLACK_BISHOP),
            b'r' => Ok(Piece::BLACK_ROOK),
            b'q' => Ok(Piece::BLACK_QUEEN),
            b'k' => Ok(Piece::BLACK_KING),
            _ => Err(format!("Invalid piece `{}`", c as char)),
        }
    }

    pub fn to_ascii(&self) -> u8 {
        match *self {
            Piece::WHITE_PAWN => b'P',
            Piece::WHITE_KNIGHT => b'N',
            Piece::WHITE_BISHOP => b'B',
            Piece::WHITE_ROOK => b'R',
            Piece::WHITE_QUEEN => b'Q',
            Piece::WHITE_KING => b'K',
            Piece::BLACK_PAWN => b'p',
            Piece::BLACK_KNIGHT => b'n',
            Piece::BLACK_BISHOP => b'b',
            Piece::BLACK_ROOK => b'r',
            Piece::BLACK_QUEEN => b'q',
            Piece::BLACK_KING => b'k',
            _ => panic!("Invalid piece encoding `{:?}`", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_representation() {
        let wp = Piece::new(Side::White, Type::Pawn);
        assert_eq!(Side::White, wp.piece_side());
        assert_eq!(Type::Pawn, wp.piece_type());
        let wp = Piece::new(Side::White, Type::Knight);
        assert_eq!(Side::White, wp.piece_side());
        assert_eq!(Type::Knight, wp.piece_type());
        let wp = Piece::new(Side::White, Type::Bishop);
        assert_eq!(Side::White, wp.piece_side());
        assert_eq!(Type::Bishop, wp.piece_type());
        let wp = Piece::new(Side::White, Type::Rook);
        assert_eq!(Side::White, wp.piece_side());
        assert_eq!(Type::Rook, wp.piece_type());
        let wp = Piece::new(Side::White, Type::Queen);
        assert_eq!(Side::White, wp.piece_side());
        assert_eq!(Type::Queen, wp.piece_type());
        let wp = Piece::new(Side::White, Type::King);
        assert_eq!(Side::White, wp.piece_side());
        assert_eq!(Type::King, wp.piece_type());
        let wp = Piece::new(Side::Black, Type::Pawn);
        assert_eq!(Side::Black, wp.piece_side());
        assert_eq!(Type::Pawn, wp.piece_type());
        let wp = Piece::new(Side::Black, Type::Knight);
        assert_eq!(Side::Black, wp.piece_side());
        assert_eq!(Type::Knight, wp.piece_type());
        let wp = Piece::new(Side::Black, Type::Bishop);
        assert_eq!(Side::Black, wp.piece_side());
        assert_eq!(Type::Bishop, wp.piece_type());
        let wp = Piece::new(Side::Black, Type::Rook);
        assert_eq!(Side::Black, wp.piece_side());
        assert_eq!(Type::Rook, wp.piece_type());
        let wp = Piece::new(Side::Black, Type::Queen);
        assert_eq!(Side::Black, wp.piece_side());
        assert_eq!(Type::Queen, wp.piece_type());
        let wp = Piece::new(Side::Black, Type::King);
        assert_eq!(Side::Black, wp.piece_side());
        assert_eq!(Type::King, wp.piece_type());
    }

    #[test]
    fn is_sliding_piece() {
        assert_eq!(false, Piece::WHITE_PAWN.is_sliding_piece());
        assert_eq!(false, Piece::WHITE_KNIGHT.is_sliding_piece());
        assert_eq!(true, Piece::WHITE_BISHOP.is_sliding_piece());
        assert_eq!(true, Piece::WHITE_ROOK.is_sliding_piece());
        assert_eq!(true, Piece::WHITE_QUEEN.is_sliding_piece());
        assert_eq!(false, Piece::WHITE_KING.is_sliding_piece());
        assert_eq!(false, Piece::BLACK_PAWN.is_sliding_piece());
        assert_eq!(false, Piece::BLACK_KNIGHT.is_sliding_piece());
        assert_eq!(true, Piece::BLACK_BISHOP.is_sliding_piece());
        assert_eq!(true, Piece::BLACK_ROOK.is_sliding_piece());
        assert_eq!(true, Piece::BLACK_QUEEN.is_sliding_piece());
        assert_eq!(false, Piece::BLACK_KING.is_sliding_piece());
    }

    #[test]
    fn from_ascii() {
        assert_eq!(Ok(Piece::WHITE_PAWN), Piece::from_ascii(b'P'));
        assert_eq!(Ok(Piece::WHITE_KNIGHT), Piece::from_ascii(b'N'));
        assert_eq!(Ok(Piece::WHITE_BISHOP), Piece::from_ascii(b'B'));
        assert_eq!(Ok(Piece::WHITE_ROOK), Piece::from_ascii(b'R'));
        assert_eq!(Ok(Piece::WHITE_QUEEN), Piece::from_ascii(b'Q'));
        assert_eq!(Ok(Piece::WHITE_KING), Piece::from_ascii(b'K'));
        assert_eq!(Ok(Piece::BLACK_PAWN), Piece::from_ascii(b'p'));
        assert_eq!(Ok(Piece::BLACK_KNIGHT), Piece::from_ascii(b'n'));
        assert_eq!(Ok(Piece::BLACK_BISHOP), Piece::from_ascii(b'b'));
        assert_eq!(Ok(Piece::BLACK_ROOK), Piece::from_ascii(b'r'));
        assert_eq!(Ok(Piece::BLACK_QUEEN), Piece::from_ascii(b'q'));
        assert_eq!(Ok(Piece::BLACK_KING), Piece::from_ascii(b'k'));
        assert_eq!(
            Err(String::from("Invalid piece `!`")),
            Piece::from_ascii(b'!')
        );
    }

    #[test]
    fn to_ascii() {
        assert_eq!(b'P', Piece::WHITE_PAWN.to_ascii());
        assert_eq!(b'N', Piece::WHITE_KNIGHT.to_ascii());
        assert_eq!(b'B', Piece::WHITE_BISHOP.to_ascii());
        assert_eq!(b'R', Piece::WHITE_ROOK.to_ascii());
        assert_eq!(b'Q', Piece::WHITE_QUEEN.to_ascii());
        assert_eq!(b'K', Piece::WHITE_KING.to_ascii());
        assert_eq!(b'p', Piece::BLACK_PAWN.to_ascii());
        assert_eq!(b'n', Piece::BLACK_KNIGHT.to_ascii());
        assert_eq!(b'b', Piece::BLACK_BISHOP.to_ascii());
        assert_eq!(b'r', Piece::BLACK_ROOK.to_ascii());
        assert_eq!(b'q', Piece::BLACK_QUEEN.to_ascii());
        assert_eq!(b'k', Piece::BLACK_KING.to_ascii());
    }

    #[test]
    #[should_panic]
    fn invalid_to_ascii() {
        let p = Piece(0x80);
        p.to_ascii();
    }
}
