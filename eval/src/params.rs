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
pub const TEMPO: ScorePair = ScorePair(26, 21);

pub const PASSED_PAWN: ScorePair = ScorePair(-10, 30);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-14, -6);
pub const BACKWARD_PAWN: ScorePair = ScorePair(-10, -2);
pub const DOUBLED_PAWN: ScorePair = ScorePair(-6, -5);

pub const BISHOP_PAIR: ScorePair = ScorePair(32, 28);

pub const KNIGHT_MOB_LEN: usize = 9;
pub const BISHOP_MOB_LEN: usize = 14;
pub const ROOK_MOB_LEN: usize = 15;
pub const QUEEN_MOB_LEN: usize = 28;
pub const MOB_LEN: usize = KNIGHT_MOB_LEN + BISHOP_MOB_LEN + ROOK_MOB_LEN + QUEEN_MOB_LEN;

const MOBILITY_KNIGHT_MG_EG: ([Score; KNIGHT_MOB_LEN], [Score; KNIGHT_MOB_LEN]) = (
    [-16, 26, 35, 41, 49, 50, 52, 48, 50],
    [0, -37, -42, -40, -38, -39, -36, -34, -40],
);
const MOBILITY_BISHOP_MG_EG: ([Score; BISHOP_MOB_LEN], [Score; BISHOP_MOB_LEN]) = (
    [15, 26, 37, 37, 44, 48, 47, 52, 63, 58, 73, 74, 53, 32],
    [
        -54, -57, -41, -41, -40, -25, -21, -16, -22, -13, -13, -14, -13, 6,
    ],
);
const MOBILITY_ROOK_MG_EG: ([Score; ROOK_MOB_LEN], [Score; ROOK_MOB_LEN]) = (
    [
        -32, -26, -22, -19, -17, -13, -4, -4, 0, 9, 16, 15, 33, 28, 26,
    ],
    [
        -42, -37, -22, -28, -20, -18, -23, -20, -12, -8, -15, -8, -8, -4, -1,
    ],
);
const MOBILITY_QUEEN_MG_EG: ([Score; QUEEN_MOB_LEN], [Score; QUEEN_MOB_LEN]) = (
    [
        -19, -28, -15, -12, -4, 5, 11, 17, 13, 11, 26, 16, 25, 26, 23, 30, 35, 28, 38, 39, 33, 60,
        51, 67, 37, 38, 12, 10,
    ],
    [
        -5, -4, -13, -4, -50, -10, -37, -32, -9, 14, 19, 34, 23, 30, 41, 47, 47, 63, 75, 70, 57,
        55, 39, 54, 44, 48, 17, 11,
    ],
);

// Piece square tables:
// We only define values for the queenside (left side) and mirror them to the
// kingside (right side) so that we end up with symmetrical PSTs.
#[rustfmt::skip]
const PST_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
         115,  147,  137,  175,
          77,   95,  128,  107,
          90,  101,   99,  124,
          68,   83,  100,  118,
          84,   96,  104,  112,
          79,  104,  103,  106,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         266,  249,  244,  191,
         165,  149,  121,  125,
          91,   79,   77,   60,
          75,   69,   58,   54,
          70,   62,   58,   57,
          71,   63,   65,   65,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         197,  285,  238,  264,
         282,  277,  369,  351,
         293,  323,  364,  378,
         319,  317,  335,  340,
         299,  322,  336,  334,
         299,  323,  328,  340,
         304,  299,  307,  333,
         278,  307,  300,  289,
    ],
    [
         210,  281,  305,  294,
         278,  298,  277,  288,
         277,  293,  318,  300,
         293,  313,  326,  319,
         298,  289,  309,  310,
         281,  285,  284,  301,
         265,  281,  289,  289,
         278,  268,  295,  303,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         276,  300,  271,  246,
         320,  313,  303,  306,
         345,  336,  337,  351,
         309,  313,  303,  346,
         320,  312,  314,  343,
         334,  340,  346,  331,
         330,  352,  332,  335,
         328,  318,  323,  329,
    ],
    [
         297,  302,  256,  276,
         270,  294,  289,  287,
         280,  281,  292,  288,
         279,  299,  300,  310,
         278,  297,  297,  302,
         281,  284,  286,  292,
         282,  287,  288,  291,
         275,  305,  290,  301,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         485,  489,  523,  513,
         495,  483,  512,  527,
         519,  528,  506,  523,
         477,  506,  486,  504,
         476,  487,  465,  492,
         472,  504,  500,  496,
         456,  506,  498,  510,
         502,  505,  519,  527,
    ],
    [
         514,  519,  493,  493,
         503,  514,  515,  500,
         494,  498,  494,  490,
         496,  483,  498,  492,
         482,  495,  507,  491,
         474,  476,  481,  487,
         503,  485,  476,  491,
         469,  476,  473,  471,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         935,  914,  937,  918,
         908,  879,  926,  868,
         936,  931,  931,  941,
         919,  886,  897,  900,
         905,  909,  917,  904,
         914,  919,  918,  914,
         922,  936,  940,  931,
         913,  928,  936,  941,
    ],
    [
         921,  933,  938,  924,
         925,  939,  948,  965,
         912,  920,  936,  953,
         900,  935,  935,  960,
         941,  911,  917,  961,
         874,  894,  923,  912,
         913,  902,  884,  917,
         891,  879,  885,  875,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           9,   -4,    7,    6,
           5,    1,   30,   23,
           8,   14,    9,   19,
           3,    5,  -11,  -10,
         -23,    6,  -24,  -43,
          -7,  -13,  -50,  -69,
          28,   23,  -25,  -48,
          43,   60,   11,   15,
    ],
    [
         -30,   -2,  -17,   -1,
           0,   10,    6,   17,
          -1,   25,   30,   12,
         -12,    7,   18,   24,
         -15,    0,   24,   28,
         -20,    5,   24,   32,
         -24,   -4,   10,   21,
         -71,  -44,  -18,  -33,
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
