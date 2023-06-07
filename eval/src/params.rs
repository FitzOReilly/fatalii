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
pub const TEMPO: ScorePair = ScorePair(33, 21);

pub const PASSED_PAWN: ScorePair = ScorePair(-9, 33);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-20, -5);
pub const BACKWARD_PAWN: ScorePair = ScorePair(-9, -5);

pub const KNIGHT_MOB_LEN: usize = 9;
pub const BISHOP_MOB_LEN: usize = 14;
pub const ROOK_MOB_LEN: usize = 15;
pub const QUEEN_MOB_LEN: usize = 28;
pub const MOB_LEN: usize = KNIGHT_MOB_LEN + BISHOP_MOB_LEN + ROOK_MOB_LEN + QUEEN_MOB_LEN;

const MOBILITY_KNIGHT_MG_EG: ([Score; KNIGHT_MOB_LEN], [Score; KNIGHT_MOB_LEN]) = (
    [-18, 26, 42, 40, 60, 54, 56, 48, 53],
    [0, -14, -29, -24, -25, -29, -22, -14, -18],
);
const MOBILITY_BISHOP_MG_EG: ([Score; BISHOP_MOB_LEN], [Score; BISHOP_MOB_LEN]) = (
    [29, 39, 54, 62, 67, 69, 77, 78, 88, 84, 98, 96, 46, 29],
    [-33, -29, -42, -39, -27, -15, -18, -7, -7, 4, -3, 8, 20, 22],
);
const MOBILITY_ROOK_MG_EG: ([Score; ROOK_MOB_LEN], [Score; ROOK_MOB_LEN]) = (
    [
        -37, -24, -22, -20, -19, -15, -1, 5, 4, 20, 20, 16, 34, 41, 19,
    ],
    [-33, -23, 0, -10, -5, 2, -5, -1, 5, 5, 2, 10, 14, 15, 22],
);
const MOBILITY_QUEEN_MG_EG: ([Score; QUEEN_MOB_LEN], [Score; QUEEN_MOB_LEN]) = (
    [
        -25, -33, -18, -27, -12, 1, 7, 11, 12, 10, 17, 13, 17, 25, 17, 34, 31, 24, 29, 32, 32, 40,
        31, 32, 25, 17, 3, 5,
    ],
    [
        -3, -1, -7, -8, -36, -10, -37, -35, -24, 17, 28, 28, 16, 22, 34, 21, 37, 47, 50, 55, 42,
        38, 24, 43, 33, 27, 5, 4,
    ],
);

// Piece square tables:
// We only define values for the queenside (left side) and mirror them to the
// kingside (right side) so that we end up with symmetrical PSTs.
#[rustfmt::skip]
const PST_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
         131,  167,  146,  177,
          94,  113,  163,  126,
         111,  112,  122,  140,
          77,   91,  113,  137,
          94,  106,  116,  128,
          89,  119,  120,  122,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         264,  236,  248,  196,
         168,  154,  128,  125,
          89,   81,   78,   53,
          77,   66,   61,   45,
          68,   63,   62,   59,
          67,   56,   63,   66,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         237,  295,  259,  276,
         277,  300,  387,  342,
         290,  328,  361,  381,
         331,  301,  337,  333,
         288,  310,  339,  336,
         277,  311,  325,  341,
         309,  302,  305,  328,
         278,  304,  289,  282,
    ],
    [
         234,  286,  301,  284,
         286,  297,  283,  292,
         287,  299,  307,  288,
         292,  320,  324,  322,
         306,  310,  310,  311,
         288,  288,  294,  306,
         292,  285,  281,  281,
         277,  280,  310,  307,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         286,  298,  272,  270,
         321,  339,  305,  306,
         360,  343,  356,  358,
         312,  325,  314,  346,
         310,  339,  329,  362,
         338,  328,  356,  340,
         336,  370,  340,  346,
         320,  333,  331,  329,
    ],
    [
         294,  295,  261,  296,
         283,  298,  300,  312,
         288,  310,  302,  293,
         295,  292,  308,  310,
         288,  287,  290,  299,
         292,  302,  289,  298,
         299,  279,  285,  292,
         291,  307,  294,  305,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         491,  500,  516,  511,
         514,  498,  515,  525,
         520,  522,  501,  524,
         482,  507,  495,  509,
         466,  479,  471,  498,
         472,  493,  486,  506,
         432,  492,  484,  529,
         508,  509,  527,  540,
    ],
    [
         506,  521,  518,  499,
         511,  511,  506,  517,
         517,  504,  511,  496,
         500,  497,  506,  498,
         497,  496,  508,  508,
         487,  488,  489,  485,
         497,  500,  504,  497,
         477,  491,  480,  475,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         908,  905,  921,  926,
         895,  870,  925,  874,
         936,  919,  943,  958,
         920,  889,  908,  904,
         874,  912,  897,  895,
         889,  909,  903,  913,
         902,  928,  929,  923,
         908,  919,  908,  938,
    ],
    [
         914,  907,  925,  925,
         927,  937,  950,  924,
         896,  913,  920,  941,
         903,  937,  919,  948,
         926,  904,  921,  945,
         877,  890,  919,  914,
         890,  898,  892,  908,
         904,  897,  882,  858,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           3,    0,   -1,    4,
           3,   10,   17,   10,
           4,   11,   13,   10,
          -2,    5,   -6,    0,
         -12,    0,   -7,  -21,
         -16,   -7,  -33,  -70,
          30,   21,  -14,  -58,
          41,   64,   -6,    6,
    ],
    [
         -21,    0,   -9,    4,
          -7,   15,   12,   20,
          -5,   16,   26,   22,
         -15,    5,   15,   14,
         -27,    3,   24,   30,
          -4,    8,   16,   26,
         -19,  -10,    4,   24,
         -69,  -46,  -20,  -33,
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
