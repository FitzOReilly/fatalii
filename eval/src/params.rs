use movegen::{file::File, rank::Rank, square::Square};

use crate::{score_pair::ScorePair, Score};

pub type PieceSquareTable = [ScorePair; 64];

// (middlegame, endgame)
const MATERIAL_KING: ScorePair = ScorePair(0, 0);
const MATERIAL_QUEEN: ScorePair = ScorePair(0, 0);
const MATERIAL_ROOK: ScorePair = ScorePair(0, 0);
const MATERIAL_BISHOP: ScorePair = ScorePair(0, 0);
const MATERIAL_KNIGHT: ScorePair = ScorePair(0, 0);
const MATERIAL_PAWN: ScorePair = ScorePair(0, 0);

// The side to move gets a small bonus
pub const TEMPO: ScorePair = ScorePair(29, 21);

pub const PASSED_PAWN: ScorePair = ScorePair(-8, 32);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-17, -7);
pub const BACKWARD_PAWN: ScorePair = ScorePair(-12, -3);

pub const BISHOP_PAIR: ScorePair = ScorePair(28, 15);

pub const KNIGHT_MOB_LEN: usize = 9;
pub const BISHOP_MOB_LEN: usize = 14;
pub const ROOK_MOB_LEN: usize = 15;
pub const QUEEN_MOB_LEN: usize = 28;
pub const MOB_LEN: usize = KNIGHT_MOB_LEN + BISHOP_MOB_LEN + ROOK_MOB_LEN + QUEEN_MOB_LEN;

const MOBILITY_KNIGHT_MG_EG: ([Score; KNIGHT_MOB_LEN], [Score; KNIGHT_MOB_LEN]) = (
    [-14, 33, 39, 47, 53, 53, 53, 49, 43],
    [1, -35, -36, -33, -37, -35, -34, -27, -28],
);
const MOBILITY_BISHOP_MG_EG: ([Score; BISHOP_MOB_LEN], [Score; BISHOP_MOB_LEN]) = (
    [22, 28, 40, 48, 49, 54, 53, 58, 67, 66, 81, 80, 55, 30],
    [
        -42, -46, -41, -42, -35, -25, -17, -16, -18, -12, -10, -13, 5, 11,
    ],
);
const MOBILITY_ROOK_MG_EG: ([Score; ROOK_MOB_LEN], [Score; ROOK_MOB_LEN]) = (
    [
        -34, -23, -18, -20, -15, -13, -3, 6, 0, 13, 8, 18, 23, 29, 32,
    ],
    [
        -32, -37, -17, -20, -15, -7, -8, -20, -5, -2, -5, -4, 4, 4, 1,
    ],
);
const MOBILITY_QUEEN_MG_EG: ([Score; QUEEN_MOB_LEN], [Score; QUEEN_MOB_LEN]) = (
    [
        -35, -27, -11, -12, -5, 3, 17, 15, 12, 17, 19, 19, 18, 29, 24, 28, 30, 24, 41, 29, 38, 55,
        44, 50, 32, 31, 8, 7,
    ],
    [
        -5, -1, -14, -6, -43, -10, -39, -28, -9, 13, 19, 32, 23, 29, 38, 34, 49, 64, 67, 59, 52,
        49, 36, 52, 39, 41, 11, 6,
    ],
);

// Piece square tables:
// We only define values for the queenside (left side) and mirror them to the
// kingside (right side) so that we end up with symmetrical PSTs.
#[rustfmt::skip]
const PST_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
         126,  154,  141,  170,
          87,   97,  134,  110,
          87,   96,  102,  124,
          70,   78,  102,  119,
          84,   92,  106,  109,
          79,  104,  104,  108,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         269,  258,  239,  186,
         164,  152,  119,  126,
          89,   79,   76,   56,
          73,   66,   56,   50,
          66,   61,   56,   59,
          67,   59,   65,   56,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         217,  291,  249,  271,
         277,  278,  376,  345,
         294,  327,  367,  378,
         332,  315,  335,  338,
         293,  314,  340,  334,
         292,  320,  328,  345,
         298,  299,  301,  328,
         278,  307,  292,  293,
    ],
    [
         219,  282,  302,  287,
         281,  296,  277,  292,
         282,  293,  316,  300,
         292,  318,  320,  315,
         302,  293,  308,  309,
         281,  292,  290,  301,
         275,  289,  291,  284,
         279,  266,  299,  309,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         278,  296,  268,  261,
         311,  322,  296,  311,
         342,  340,  349,  347,
         313,  321,  301,  343,
         321,  319,  326,  349,
         329,  338,  349,  333,
         334,  358,  325,  340,
         340,  316,  324,  335,
    ],
    [
         295,  296,  260,  288,
         275,  295,  295,  297,
         288,  290,  290,  287,
         285,  296,  305,  313,
         281,  296,  296,  300,
         282,  283,  287,  293,
         286,  282,  282,  291,
         285,  303,  294,  303,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         494,  496,  521,  510,
         505,  487,  518,  517,
         523,  526,  505,  528,
         474,  505,  484,  512,
         480,  481,  477,  489,
         473,  496,  492,  497,
         456,  501,  485,  510,
         505,  509,  519,  530,
    ],
    [
         517,  519,  502,  493,
         505,  508,  510,  504,
         513,  502,  499,  489,
         492,  488,  497,  497,
         491,  503,  508,  494,
         484,  483,  477,  482,
         508,  489,  484,  500,
         470,  478,  476,  473,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         926,  916,  931,  918,
         903,  879,  922,  875,
         937,  922,  939,  940,
         926,  886,  899,  900,
         884,  907,  918,  896,
         906,  916,  916,  913,
         915,  940,  935,  929,
         915,  927,  926,  936,
    ],
    [
         914,  928,  934,  919,
         928,  931,  953,  949,
         905,  919,  932,  941,
         903,  945,  932,  957,
         942,  904,  921,  964,
         877,  890,  921,  915,
         906,  897,  883,  910,
         902,  883,  879,  874,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           5,   -3,    6,    7,
           4,    6,   25,   13,
           2,   12,   13,   13,
           1,    6,   -6,   -4,
         -18,    1,  -17,  -35,
          -7,  -12,  -40,  -77,
          22,   21,  -25,  -43,
          42,   59,   13,   13,
    ],
    [
         -25,   -3,   -6,    8,
          -1,    9,    5,   20,
         -11,   15,   29,   13,
         -13,   10,   12,   26,
         -17,    0,   22,   32,
         -17,    3,   21,   30,
         -22,   -5,   11,   20,
         -63,  -45,  -23,  -34,
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

pub const MOBILITY_KNIGHT: [ScorePair; 9] = {
    let mg = MOBILITY_KNIGHT_MG_EG.0;
    let eg = MOBILITY_KNIGHT_MG_EG.1;
    let mut table = [ScorePair(0, 0); 9];
    let mut idx = 0;
    while idx < 9 {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const MOBILITY_BISHOP: [ScorePair; 14] = {
    let mg = MOBILITY_BISHOP_MG_EG.0;
    let eg = MOBILITY_BISHOP_MG_EG.1;
    let mut table = [ScorePair(0, 0); 14];
    let mut idx = 0;
    while idx < 14 {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const MOBILITY_ROOK: [ScorePair; 15] = {
    let mg = MOBILITY_ROOK_MG_EG.0;
    let eg = MOBILITY_ROOK_MG_EG.1;
    let mut table = [ScorePair(0, 0); 15];
    let mut idx = 0;
    while idx < 15 {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const MOBILITY_QUEEN: [ScorePair; 28] = {
    let mg = MOBILITY_QUEEN_MG_EG.0;
    let eg = MOBILITY_QUEEN_MG_EG.1;
    let mut table = [ScorePair(0, 0); 28];
    let mut idx = 0;
    while idx < 28 {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

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
