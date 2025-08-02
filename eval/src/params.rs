use movegen::{file::File, rank::Rank, square::Square};

use crate::{score_pair::ScorePair, Score};

pub type PieceSquareTable = [ScorePair; 64];

pub const KNIGHT_MOB_LEN: usize = 9;
pub const BISHOP_MOB_LEN: usize = 14;
pub const ROOK_MOB_LEN: usize = 15;
pub const QUEEN_MOB_LEN: usize = 28;
pub const MOB_LEN: usize = KNIGHT_MOB_LEN + BISHOP_MOB_LEN + ROOK_MOB_LEN + QUEEN_MOB_LEN;

pub const DISTANCE_LEN: usize = 8;

// (middlegame, endgame)
const MATERIAL_KING: ScorePair = ScorePair(0, 0);
const MATERIAL_QUEEN: ScorePair = ScorePair(0, 0);
const MATERIAL_ROOK: ScorePair = ScorePair(0, 0);
const MATERIAL_BISHOP: ScorePair = ScorePair(0, 0);
const MATERIAL_KNIGHT: ScorePair = ScorePair(0, 0);
const MATERIAL_PAWN: ScorePair = ScorePair(0, 0);

// The side to move gets a small bonus
pub const TEMPO: ScorePair = ScorePair(40, 23);

#[rustfmt::skip]
const PASSED_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
          36,   19,   19,   -6,
          -1,   45,   23,    0,
          20,   -7,  -28,  -17,
          30,   27,    5,  -35,
           6,    2,   10,  -24,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
         161,  138,   90,   70,
          86,   64,   51,   43,
          38,   41,   26,   21,
           7,   -3,    3,   10,
          20,   10,   -2,  -16,
           0,    0,    0,    0,
    ],
);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-19, -4);
pub const BACKWARD_PAWN: ScorePair = ScorePair(-19, -1);
pub const DOUBLED_PAWN: ScorePair = ScorePair(-8, -19);

pub const BISHOP_PAIR: ScorePair = ScorePair(39, 17);

const MOBILITY_KNIGHT_MG_EG: ([Score; KNIGHT_MOB_LEN], [Score; KNIGHT_MOB_LEN]) = (
    [-35, 35, 53, 57, 66, 67, 69, 70, 73],
    [1, -5, -24, -17, -6, -4, -3, 2, 6],
);
const MOBILITY_BISHOP_MG_EG: ([Score; BISHOP_MOB_LEN], [Score; BISHOP_MOB_LEN]) = (
    [1, 9, 25, 27, 34, 43, 45, 57, 68, 64, 79, 85, 38, 35],
    [-27, -21, -30, -27, -10, 0, 10, 8, 9, 9, -1, 14, 21, 40],
);
const MOBILITY_ROOK_MG_EG: ([Score; ROOK_MOB_LEN], [Score; ROOK_MOB_LEN]) = (
    [5, 10, 6, 17, 18, 22, 26, 39, 41, 52, 52, 59, 78, 83, 64],
    [-19, -27, 5, 1, 3, 8, 2, 6, 13, 11, 15, 14, 16, 24, 18],
);
const MOBILITY_QUEEN_MG_EG: ([Score; QUEEN_MOB_LEN], [Score; QUEEN_MOB_LEN]) = (
    [
        -23, -13, 3, -3, 13, 18, 23, 27, 20, 36, 31, 31, 34, 36, 42, 54, 49, 60, 69, 61, 56, 76,
        66, 50, 19, 37, 6, 11,
    ],
    [
        -2, -1, -6, 0, -15, -15, -19, -17, 10, 30, 14, 53, 51, 49, 36, 41, 36, 67, 53, 72, 67, 63,
        58, 49, 22, 51, 6, 19,
    ],
);

const DISTANCE_FRIENDLY_PAWN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) =
    ([0, 60, 39, 21, 17, -9, 3, 5], [0, 4, 12, 10, 5, 16, 15, 13]);
const DISTANCE_ENEMY_PAWN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -25, -57, -13, -20, -23, -20, 21],
    [0, 34, 3, -14, -18, -21, -21, -36],
);
const DISTANCE_FRIENDLY_KNIGHT_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 90, 88, 82, 65, 75, 74, -17],
    [0, -5, -8, -7, 1, -8, -13, -10],
);
const DISTANCE_ENEMY_KNIGHT_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -91, -128, -66, -59, -47, -50, -15],
    [0, 26, 18, 4, 6, -2, 8, -10],
);
const DISTANCE_FRIENDLY_BISHOP_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 101, 102, 96, 100, 85, 89, 38],
    [0, 3, 0, -1, -10, 1, -3, 4],
);
const DISTANCE_ENEMY_BISHOP_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -185, -98, -76, -76, -60, -60, -55],
    [0, 46, 8, -2, -2, -15, -14, -16],
);
const DISTANCE_FRIENDLY_ROOK_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 72, 74, 85, 75, 88, 77, 98],
    [0, 2, 21, 8, 12, 12, 19, 15],
);
const DISTANCE_ENEMY_ROOK_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -119, -90, -92, -81, -86, -55, -47],
    [0, -15, -5, -15, -15, -8, -15, -18],
);
const DISTANCE_FRIENDLY_QUEEN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 140, 140, 135, 129, 125, 126, 93],
    [0, 83, 112, 118, 118, 111, 130, 102],
);
const DISTANCE_ENEMY_QUEEN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -284, -175, -109, -92, -89, -74, -65],
    [0, -150, -129, -129, -110, -103, -91, -62],
);
const DISTANCE_FRIENDLY_KING_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) =
    ([0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]);
const DISTANCE_ENEMY_KING_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) =
    ([0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]);

// Piece square tables:
// We only define values for the queenside (left side) and mirror them to the
// kingside (right side) so that we end up with symmetrical PSTs.
#[rustfmt::skip]
const PST_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
         165,  154,  163,  173,
          90,  108,  165,  129,
          89,  103,  102,  130,
          58,   72,  101,  112,
          70,   85,   85,   91,
          56,   98,   73,   65,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         296,  311,  287,  257,
          76,   91,   83,  112,
          64,   68,   76,   59,
          62,   65,   58,   57,
          50,   54,   58,   59,
          50,   46,   66,   72,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         234,  297,  268,  305,
         283,  291,  378,  337,
         268,  324,  345,  358,
         328,  321,  349,  342,
         284,  320,  341,  320,
         296,  310,  332,  342,
         305,  302,  327,  323,
         296,  317,  314,  299,
    ],
    [
         234,  277,  294,  306,
         280,  286,  288,  295,
         293,  304,  328,  310,
         316,  314,  322,  312,
         318,  296,  318,  329,
         285,  301,  288,  305,
         279,  293,  295,  300,
         284,  278,  308,  313,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         324,  322,  274,  257,
         303,  306,  303,  274,
         324,  323,  353,  345,
         290,  306,  286,  327,
         300,  317,  304,  355,
         339,  339,  352,  324,
         328,  371,  330,  328,
         345,  315,  322,  325,
    ],
    [
         309,  292,  271,  292,
         295,  297,  304,  295,
         304,  313,  313,  304,
         296,  315,  310,  335,
         295,  302,  310,  303,
         295,  295,  288,  309,
         291,  273,  295,  290,
         288,  313,  297,  305,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         506,  507,  519,  523,
         542,  518,  539,  534,
         537,  528,  487,  513,
         507,  492,  513,  520,
         477,  484,  490,  506,
         485,  498,  505,  523,
         483,  516,  544,  550,
         540,  537,  565,  580,
    ],
    [
         529,  528,  516,  501,
         508,  521,  515,  514,
         516,  506,  518,  505,
         503,  504,  503,  503,
         500,  503,  514,  502,
         484,  489,  502,  494,
         499,  493,  490,  496,
         479,  484,  485,  484,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         963,  911,  929,  931,
         937,  891,  911,  848,
         905,  920,  917,  918,
         931,  906,  903,  899,
         908,  910,  920,  904,
         932,  941,  932,  926,
         934,  972,  967,  960,
         955,  960,  963,  984,
    ],
    [
         934,  910,  934,  929,
         921,  928,  946,  941,
         904,  905,  932,  940,
         919,  925,  934,  959,
         944,  921,  921,  952,
         890,  913,  922,  921,
         915,  930,  891,  934,
         927,  912,  913,  905,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           3,    7,    0,    1,
         -12,    8,   13,    8,
          -2,   26,   25,   23,
          -7,   14,   17,   20,
         -28,   11,  -16,  -28,
         -18,   11,  -10,  -63,
          13,   53,  -35,  -73,
          30,   66,  -19,  -37,
    ],
    [
         -29,  -15,  -14,  -27,
          -4,    1,    6,   11,
           0,    9,    7,   15,
         -11,   19,    8,   15,
         -11,  -15,    7,   21,
          -9,   -1,   10,   30,
         -14,   -5,   25,   32,
         -55,  -26,   14,    5,
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

const fn convert_distance(
    mg_eg: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]),
) -> [ScorePair; DISTANCE_LEN] {
    let mg = mg_eg.0;
    let eg = mg_eg.1;
    let mut scores = [ScorePair(0, 0); DISTANCE_LEN];
    let mut idx = 0;
    while idx < DISTANCE_LEN {
        scores[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    scores
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

pub const DISTANCE_FRIENDLY_PAWN: [ScorePair; DISTANCE_LEN] =
    convert_distance(DISTANCE_FRIENDLY_PAWN_MG_EG);
pub const DISTANCE_ENEMY_PAWN: [ScorePair; DISTANCE_LEN] =
    convert_distance(DISTANCE_ENEMY_PAWN_MG_EG);

pub const DISTANCE_FRIENDLY_KNIGHT: [ScorePair; DISTANCE_LEN] =
    convert_distance(DISTANCE_FRIENDLY_KNIGHT_MG_EG);
pub const DISTANCE_ENEMY_KNIGHT: [ScorePair; DISTANCE_LEN] =
    convert_distance(DISTANCE_ENEMY_KNIGHT_MG_EG);

pub const DISTANCE_FRIENDLY_BISHOP: [ScorePair; DISTANCE_LEN] =
    convert_distance(DISTANCE_FRIENDLY_BISHOP_MG_EG);
pub const DISTANCE_ENEMY_BISHOP: [ScorePair; DISTANCE_LEN] =
    convert_distance(DISTANCE_ENEMY_BISHOP_MG_EG);

pub const DISTANCE_FRIENDLY_ROOK: [ScorePair; DISTANCE_LEN] =
    convert_distance(DISTANCE_FRIENDLY_ROOK_MG_EG);
pub const DISTANCE_ENEMY_ROOK: [ScorePair; DISTANCE_LEN] =
    convert_distance(DISTANCE_ENEMY_ROOK_MG_EG);

pub const DISTANCE_FRIENDLY_QUEEN: [ScorePair; DISTANCE_LEN] =
    convert_distance(DISTANCE_FRIENDLY_QUEEN_MG_EG);
pub const DISTANCE_ENEMY_QUEEN: [ScorePair; DISTANCE_LEN] =
    convert_distance(DISTANCE_ENEMY_QUEEN_MG_EG);

// This will always be 0, but is included to avoid branches
pub const DISTANCE_FRIENDLY_KING: [ScorePair; DISTANCE_LEN] =
    convert_distance(DISTANCE_FRIENDLY_KING_MG_EG);
// Both sides will cancel each other out, but it is included to avoid branches
pub const DISTANCE_ENEMY_KING: [ScorePair; DISTANCE_LEN] =
    convert_distance(DISTANCE_ENEMY_KING_MG_EG);

pub const PASSED_PAWN: PieceSquareTable = {
    let mg = human_readable_to_file_rank(0, PASSED_PAWN_MG_EG.0);
    let eg = human_readable_to_file_rank(0, PASSED_PAWN_MG_EG.1);
    let mut table = [ScorePair(0, 0); 64];
    let mut idx = 0;
    while idx < 64 {
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
