use crate::bitboard::Bitboard;
use crate::file::File;
use crate::rank::Rank;
use crate::square::Square;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CastlingSquaresInner {
    pub non_blocked: Bitboard,  // Squares passed by king or rook
    pub non_attacked: Bitboard, // Squares passed by the king (including start and end square)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CastlingSquares {
    white_kingside: CastlingSquaresInner,
    white_queenside: CastlingSquaresInner,
    black_kingside: CastlingSquaresInner,
    black_queenside: CastlingSquaresInner,
}

impl CastlingSquares {
    pub fn new(queen_rook_file: File, king_file: File, king_rook_file: File) -> CastlingSquares {
        let mut cs = CastlingSquares {
            ..Default::default()
        };
        const KINGSIDE_KING_TARGET_FILE: File = File::G;
        const KINGSIDE_ROOK_TARGET_FILE: File = File::F;
        const QUEENSIDE_KING_TARGET_FILE: File = File::C;
        const QUEENSIDE_ROOK_TARGET_FILE: File = File::D;

        // Kingside
        let min_kingside_file = king_file.idx();
        let max_kingside_file = KINGSIDE_KING_TARGET_FILE.idx();
        for file in min_kingside_file..=max_kingside_file {
            cs.white_kingside.non_attacked |=
                Bitboard::from_square(Square::from((File::from_idx(file), Rank::R1)));
            cs.black_kingside.non_attacked |=
                Bitboard::from_square(Square::from((File::from_idx(file), Rank::R8)));
        }

        let min_kingside_file = min_kingside_file.min(KINGSIDE_ROOK_TARGET_FILE.idx());
        let max_kingside_file = max_kingside_file.max(king_rook_file.idx());
        for idx in min_kingside_file..=max_kingside_file {
            let file = File::from_idx(idx);
            if file == king_file || file == king_rook_file {
                continue;
            }
            cs.white_kingside.non_blocked |= Bitboard::from_square(Square::from((file, Rank::R1)));
            cs.black_kingside.non_blocked |= Bitboard::from_square(Square::from((file, Rank::R8)));
        }

        // Queenside
        let min_queenside_file = king_file.idx().min(QUEENSIDE_KING_TARGET_FILE.idx());
        let max_queenside_file = king_file.idx().max(QUEENSIDE_KING_TARGET_FILE.idx());
        for file in min_queenside_file..=max_queenside_file {
            cs.white_queenside.non_attacked |=
                Bitboard::from_square(Square::from((File::from_idx(file), Rank::R1)));
            cs.black_queenside.non_attacked |=
                Bitboard::from_square(Square::from((File::from_idx(file), Rank::R8)));
        }

        let min_queenside_file = min_queenside_file.min(queen_rook_file.idx());
        let max_queenside_file = max_queenside_file.max(QUEENSIDE_ROOK_TARGET_FILE.idx());
        for idx in min_queenside_file..=max_queenside_file {
            let file = File::from_idx(idx);
            if file == king_file || file == queen_rook_file {
                continue;
            }
            cs.white_queenside.non_blocked |= Bitboard::from_square(Square::from((file, Rank::R1)));
            cs.black_queenside.non_blocked |= Bitboard::from_square(Square::from((file, Rank::R8)));
        }

        cs
    }

    pub fn white_kingside(&self) -> &CastlingSquaresInner {
        &self.white_kingside
    }

    pub fn white_queenside(&self) -> &CastlingSquaresInner {
        &self.white_queenside
    }

    pub fn black_kingside(&self) -> &CastlingSquaresInner {
        &self.black_kingside
    }

    pub fn black_queenside(&self) -> &CastlingSquaresInner {
        &self.black_queenside
    }
}
