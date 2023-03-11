use movegen::{file::File, rank::Rank, square::Square};

use crate::{score_pair::ScorePair, Score};

pub type PieceSquareTable = [ScorePair; 64];

// (middlegame, endgame)
const MATERIAL_KING: ScorePair = ScorePair(0, 0);
const MATERIAL_QUEEN: ScorePair = ScorePair(900, 910);
const MATERIAL_ROOK: ScorePair = ScorePair(500, 520);
const MATERIAL_BISHOP: ScorePair = ScorePair(330, 310);
const MATERIAL_KNIGHT: ScorePair = ScorePair(320, 300);
const MATERIAL_PAWN: ScorePair = ScorePair(100, 120);

// The side to move gets a small bonus
pub const TEMPO: ScorePair = ScorePair(10, 10);

// Piece square tables:
// We only define values for the queenside (left side) and mirror them to the
// kingside (right side) so that we end up with symmetrical PSTs.
#[rustfmt::skip]
const PST_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
          0,   0,   0,   0,
         30,  40,  45,  50,
         10,  20,  25,  30,
          5,  10,  15,  25,
          0,  -5,   5,  20,
          5,   0,   0,   0,
          5,  10,  10, -20,
          0,   0,   0,   0,
    ],
    [
          0,   0,   0,   0,
         50,  60,  65,  70,
         25,  35,  40,  45,
         10,  15,  20,  25,
          5,  10,  15,  20,
          0,   5,   5,  10,
          0,   0,   0,   0,
          0,   0,   0,   0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
        -40, -25, -20, -20,
        -25, -10,   0,   0,
        -20,   0,  10,  15,
        -20,   5,  15,  20,
        -20,   0,  15,  20,
        -20,   5,  10,  15,
        -25, -10,   0,   5,
        -40, -25, -20, -20,
    ],
    [
        -40, -25, -20, -20,
        -25, -10,   0,   0,
        -20,   0,   5,  10,
        -20,   0,  10,  15,
        -20,   0,  10,  15,
        -20,   0,   5,  10,
        -25, -10,   0,   0,
        -40, -25, -20, -20,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
        -20, -10, -10, -10,
        -10,   0,   0,   0,
        -10,   0,   5,  10,
        -10,  10,   5,  10,
        -10,   5,  15,  10,
        -10,  10,  10,  10,
        -10,  15,  10,  10,
        -20, -10, -10, -10,
    ],
    [
        -10,  -5,  -5,  -5,
         -5,   0,   0,   0,
         -5,   0,   5,   5,
         -5,   0,   5,  10,
         -5,   0,   5,  10,
         -5,   0,   5,   5,
         -5,   0,   0,   0,
        -10,  -5,  -5,  -5,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
          0,   0,   0,   0,
          5,  10,  10,  10,
         -5,   0,   0,   5,
         -5,   0,   0,   5,
         -5,   0,   0,   5,
         -5,   0,   0,   5,
         -5,  -5,   0,   5,
        -10,  -5,   5,  10,
    ],
    [
        -10,  -5,  -5,  -5,
         -5,   0,   0,   0,
         -5,   0,   5,   5,
         -5,   0,   5,   5,
         -5,   0,   5,   5,
         -5,   0,   5,   5,
         -5,   0,   0,   0,
        -10,  -5,  -5,  -5,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
        -20, -10, -10,  -5,
        -10,   0,   0,   0,
        -10,   0,   5,   5,
         -5,   0,   5,   5,
         -5,   0,   5,   5,
        -10,   0,   5,   5,
        -10,   0,   0,   0,
        -20, -10, -10,  -5,
    ],
    [
         -5,  -5,  -5,  -5,
         -5,   0,   0,   0,
         -5,   0,   5,   5,
         -5,   0,   5,   5,
         -5,   0,   5,   5,
         -5,   0,   5,   5,
         -5,   0,   0,   0,
         -5,  -5,  -5,  -5,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
        -30, -40, -40, -50,
        -30, -40, -40, -50,
        -30, -40, -40, -50,
        -30, -40, -40, -50,
        -30, -30, -30, -40,
        -20, -20, -25, -25,
         10,  10, -10, -10,
         20,  30,  -5,  -5,
    ],
    [
        -50, -35, -25, -20,
        -35, -15,  -5,   0,
        -25,  -5,  10,  15,
        -20,   0,  15,  25,
        -20,   0,  15,  25,
        -25,  -5,  10,  15,
        -35, -15,  -5,   0,
        -50, -35, -25, -20,
    ],
);

const fn human_readable_to_file_rank(piece_value: Score, pst: [Score; 32]) -> [Score; 64] {
    let mut res = [0; 64];
    let mut idx = 0;
    while idx < 32 {
        let rank = 7 - idx / 4;
        let file = idx % 4;
        let new_idx = Square::from_file_and_rank(File::from_idx(file), Rank::from_idx(rank)).idx();
        let mirrored_idx = Square::from_idx(new_idx).mirror_horizontal().idx();
        res[new_idx] = piece_value + pst[idx];
        res[mirrored_idx] = piece_value + pst[idx];
        idx += 1;
    }
    res
}

pub const PST_PAWN: PieceSquareTable = {
    let mg = human_readable_to_file_rank(MATERIAL_PAWN.0, PST_PAWN_MG_EG.0);
    let eg = human_readable_to_file_rank(MATERIAL_PAWN.1, PST_PAWN_MG_EG.1);
    let mut table = [ScorePair(0, 0); 64];
    let mut idx = 0;
    while idx < 64 {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const PST_KNIGHT: PieceSquareTable = {
    let mg = human_readable_to_file_rank(MATERIAL_KNIGHT.0, PST_KNIGHT_MG_EG.0);
    let eg = human_readable_to_file_rank(MATERIAL_KNIGHT.1, PST_KNIGHT_MG_EG.1);
    let mut table = [ScorePair(0, 0); 64];
    let mut idx = 0;
    while idx < 64 {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const PST_BISHOP: PieceSquareTable = {
    let mg = human_readable_to_file_rank(MATERIAL_BISHOP.0, PST_BISHOP_MG_EG.0);
    let eg = human_readable_to_file_rank(MATERIAL_BISHOP.1, PST_BISHOP_MG_EG.1);
    let mut table = [ScorePair(0, 0); 64];
    let mut idx = 0;
    while idx < 64 {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const PST_ROOK: PieceSquareTable = {
    let mg = human_readable_to_file_rank(MATERIAL_ROOK.0, PST_ROOK_MG_EG.0);
    let eg = human_readable_to_file_rank(MATERIAL_ROOK.1, PST_ROOK_MG_EG.1);
    let mut table = [ScorePair(0, 0); 64];
    let mut idx = 0;
    while idx < 64 {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const PST_QUEEN: PieceSquareTable = {
    let mg = human_readable_to_file_rank(MATERIAL_QUEEN.0, PST_QUEEN_MG_EG.0);
    let eg = human_readable_to_file_rank(MATERIAL_QUEEN.1, PST_QUEEN_MG_EG.1);
    let mut table = [ScorePair(0, 0); 64];
    let mut idx = 0;
    while idx < 64 {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const PST_KING: PieceSquareTable = {
    let mg = human_readable_to_file_rank(MATERIAL_KING.0, PST_KING_MG_EG.0);
    let eg = human_readable_to_file_rank(MATERIAL_KING.1, PST_KING_MG_EG.1);
    let mut table = [ScorePair(0, 0); 64];
    let mut idx = 0;
    while idx < 64 {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

#[cfg(test)]
mod tests {
    use movegen::square::Square;

    #[test]
    fn human_readable_to_file_rank() {
        #[rustfmt::skip]
        let arr = [
             0,  1,  2,  3,
             8,  9, 10, 11,
            16, 17, 18, 19,
            24, 25, 26, 27,
            32, 33, 34, 35,
            40, 41, 42, 43,
            48, 49, 50, 51,
            56, 57, 58, 59,
        ];

        let res = super::human_readable_to_file_rank(100, arr);
        assert_eq!(156, res[Square::A1.idx()]);
        assert_eq!(148, res[Square::A2.idx()]);
        assert_eq!(100, res[Square::A8.idx()]);
        assert_eq!(157, res[Square::B1.idx()]);
        assert_eq!(149, res[Square::B2.idx()]);
        assert_eq!(142, res[Square::C3.idx()]);
        assert_eq!(135, res[Square::D4.idx()]);
        assert_eq!(127, res[Square::E5.idx()]);
        assert_eq!(118, res[Square::F6.idx()]);
        assert_eq!(109, res[Square::G7.idx()]);
        assert_eq!(101, res[Square::G8.idx()]);
        assert_eq!(156, res[Square::H1.idx()]);
        assert_eq!(108, res[Square::H7.idx()]);
        assert_eq!(100, res[Square::H8.idx()]);
    }
}
