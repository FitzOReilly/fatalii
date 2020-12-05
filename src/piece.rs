#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl Piece {
    pub fn from_ascii(c: u8) -> Result<Self, String> {
        match c {
            b'P' => Ok(Piece::WhitePawn),
            b'N' => Ok(Piece::WhiteKnight),
            b'B' => Ok(Piece::WhiteBishop),
            b'R' => Ok(Piece::WhiteRook),
            b'Q' => Ok(Piece::WhiteQueen),
            b'K' => Ok(Piece::WhiteKing),
            b'p' => Ok(Piece::BlackPawn),
            b'n' => Ok(Piece::BlackKnight),
            b'b' => Ok(Piece::BlackBishop),
            b'r' => Ok(Piece::BlackRook),
            b'q' => Ok(Piece::BlackQueen),
            b'k' => Ok(Piece::BlackKing),
            _ => Err(format!("Invalid piece `{}`", c as char)),
        }
    }

    pub fn to_ascii(&self) -> u8 {
        match self {
            Piece::WhitePawn => b'P',
            Piece::WhiteKnight => b'N',
            Piece::WhiteBishop => b'B',
            Piece::WhiteRook => b'R',
            Piece::WhiteQueen => b'Q',
            Piece::WhiteKing => b'K',
            Piece::BlackPawn => b'p',
            Piece::BlackKnight => b'n',
            Piece::BlackBishop => b'b',
            Piece::BlackRook => b'r',
            Piece::BlackQueen => b'q',
            Piece::BlackKing => b'k',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_ascii() {
        assert_eq!(Ok(Piece::WhitePawn), Piece::from_ascii(b'P'));
        assert_eq!(Ok(Piece::WhiteKnight), Piece::from_ascii(b'N'));
        assert_eq!(Ok(Piece::WhiteBishop), Piece::from_ascii(b'B'));
        assert_eq!(Ok(Piece::WhiteRook), Piece::from_ascii(b'R'));
        assert_eq!(Ok(Piece::WhiteQueen), Piece::from_ascii(b'Q'));
        assert_eq!(Ok(Piece::WhiteKing), Piece::from_ascii(b'K'));
        assert_eq!(Ok(Piece::BlackPawn), Piece::from_ascii(b'p'));
        assert_eq!(Ok(Piece::BlackKnight), Piece::from_ascii(b'n'));
        assert_eq!(Ok(Piece::BlackBishop), Piece::from_ascii(b'b'));
        assert_eq!(Ok(Piece::BlackRook), Piece::from_ascii(b'r'));
        assert_eq!(Ok(Piece::BlackQueen), Piece::from_ascii(b'q'));
        assert_eq!(Ok(Piece::BlackKing), Piece::from_ascii(b'k'));
        assert_eq!(
            Err(String::from("Invalid piece `!`")),
            Piece::from_ascii(b'!')
        );
    }

    #[test]
    fn to_ascii() {
        assert_eq!(b'P', Piece::WhitePawn.to_ascii());
        assert_eq!(b'N', Piece::WhiteKnight.to_ascii());
        assert_eq!(b'B', Piece::WhiteBishop.to_ascii());
        assert_eq!(b'R', Piece::WhiteRook.to_ascii());
        assert_eq!(b'Q', Piece::WhiteQueen.to_ascii());
        assert_eq!(b'K', Piece::WhiteKing.to_ascii());
        assert_eq!(b'p', Piece::BlackPawn.to_ascii());
        assert_eq!(b'n', Piece::BlackKnight.to_ascii());
        assert_eq!(b'b', Piece::BlackBishop.to_ascii());
        assert_eq!(b'r', Piece::BlackRook.to_ascii());
        assert_eq!(b'q', Piece::BlackQueen.to_ascii());
        assert_eq!(b'k', Piece::BlackKing.to_ascii());
    }
}
