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
