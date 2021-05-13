use crate::bitboard::Bitboard;
use crate::piece::Piece;
use crate::square::Square;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PieceTargets {
    piece: Piece,
    origin: Square,
    targets: Bitboard,
}

impl PieceTargets {
    pub fn new(piece: Piece, origin: Square, targets: Bitboard) -> PieceTargets {
        PieceTargets {
            piece,
            origin,
            targets,
        }
    }

    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn origin(&self) -> Square {
        self.origin
    }

    pub fn targets(&self) -> Bitboard {
        self.targets
    }
}
