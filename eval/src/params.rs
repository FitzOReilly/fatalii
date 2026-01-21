use movegen::{file::File, rank::Rank, square::Square};

use crate::{score_pair::ScorePair, Score};

pub type PieceSquareTable = [ScorePair; 64];

pub const KNIGHT_MOB_LEN: usize = 9;
pub const BISHOP_MOB_LEN: usize = 14;
pub const ROOK_MOB_LEN: usize = 15;
pub const QUEEN_MOB_LEN: usize = 28;
pub const VIRTUAL_MOB_LEN: usize = QUEEN_MOB_LEN;
pub const MOB_LEN: usize =
    KNIGHT_MOB_LEN + BISHOP_MOB_LEN + ROOK_MOB_LEN + QUEEN_MOB_LEN + VIRTUAL_MOB_LEN;

pub const DISTANCE_LEN: usize = 8;

// (middlegame, endgame)
const MATERIAL_KING: ScorePair = ScorePair(0, 0);
const MATERIAL_QUEEN: ScorePair = ScorePair(0, 0);
const MATERIAL_ROOK: ScorePair = ScorePair(0, 0);
const MATERIAL_BISHOP: ScorePair = ScorePair(0, 0);
const MATERIAL_KNIGHT: ScorePair = ScorePair(0, 0);
const MATERIAL_PAWN: ScorePair = ScorePair(0, 0);

// The side to move gets a small bonus
pub const TEMPO: ScorePair = ScorePair(29, 27);

#[rustfmt::skip]
const PASSED_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
          13,   12,  -11,  -20,
          14,   21,   11,   -2,
          20,   -6,  -16,  -11,
          19,   19,    1,  -25,
          12,   10,    7,  -12,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
         144,  125,  111,   92,
          68,   67,   48,   51,
          33,   39,   25,   25,
          11,   13,    9,    9,
          12,   19,   -1,   -3,
           0,    0,    0,    0,
    ],
);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-19, -6);
pub const BACKWARD_PAWN: ScorePair = ScorePair(-11, -4);
pub const DOUBLED_PAWN: ScorePair = ScorePair(-4, -12);

pub const BISHOP_PAIR: ScorePair = ScorePair(32, 44);

const MOBILITY_KNIGHT_MG_EG: ([Score; KNIGHT_MOB_LEN], [Score; KNIGHT_MOB_LEN]) = (
    [-9, 20, 32, 36, 44, 45, 45, 46, 45],
    [-15, -7, -11, -13, -13, -9, -7, -9, -9],
);
const MOBILITY_BISHOP_MG_EG: ([Score; BISHOP_MOB_LEN], [Score; BISHOP_MOB_LEN]) = (
    [-8, 3, 13, 17, 22, 27, 32, 34, 38, 40, 48, 53, 55, 42],
    [-45, -34, -32, -21, -12, -3, 1, 8, 11, 13, 7, 11, 16, 15],
);
const MOBILITY_ROOK_MG_EG: ([Score; ROOK_MOB_LEN], [Score; ROOK_MOB_LEN]) = (
    [-8, 1, 2, 5, 6, 13, 18, 24, 28, 35, 40, 43, 53, 70, 78],
    [-38, -25, -15, -12, -3, -3, -5, 0, 7, 6, 9, 14, 17, 12, 4],
);
const MOBILITY_QUEEN_MG_EG: ([Score; QUEEN_MOB_LEN], [Score; QUEEN_MOB_LEN]) = (
    [
        -3, 7, 10, 13, 19, 24, 28, 27, 26, 33, 35, 35, 36, 38, 42, 48, 50, 48, 59, 61, 54, 67, 72,
        101, 58, 56, 52, 35,
    ],
    [
        0, -32, -58, -60, -73, -45, -46, -37, -12, -6, 1, 12, 19, 28, 29, 23, 27, 43, 40, 40, 54,
        43, 37, 21, 39, 48, 52, 50,
    ],
);

const VIRTUAL_MOBILITY_MG_EG: ([Score; VIRTUAL_MOB_LEN], [Score; VIRTUAL_MOB_LEN]) = (
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
);

const DISTANCE_FRIENDLY_PAWN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) =
    ([0, 38, 23, 11, 1, -17, -4, -7], [0, 5, 10, 4, 4, 11, 8, 8]);
const DISTANCE_ENEMY_PAWN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -12, -35, -8, -5, -9, -3, 27],
    [0, 42, 11, -7, -14, -18, -23, -41],
);
const DISTANCE_FRIENDLY_KNIGHT_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 61, 56, 50, 36, 45, 42, 14],
    [0, -16, -18, -17, -10, -15, -11, -8],
);
const DISTANCE_ENEMY_KNIGHT_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -67, -97, -43, -25, -24, -26, -22],
    [0, 33, 22, 5, 2, 8, 14, 9],
);
const DISTANCE_FRIENDLY_BISHOP_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 68, 68, 59, 69, 55, 60, 37],
    [0, -8, -9, -10, -16, -9, -11, -2],
);
const DISTANCE_ENEMY_BISHOP_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -155, -70, -43, -39, -37, -35, -37],
    [0, 45, 12, 1, 0, 0, 1, 6],
);
const DISTANCE_FRIENDLY_ROOK_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 55, 63, 65, 59, 59, 49, 59],
    [0, -10, -7, -10, -6, -3, 4, 0],
);
const DISTANCE_ENEMY_ROOK_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -245, -68, -53, -35, -19, -1, 13],
    [0, 67, 9, 0, -4, -7, -13, -20],
);
const DISTANCE_FRIENDLY_QUEEN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 175, 172, 168, 160, 158, 163, 135],
    [0, 14, 31, 36, 35, 37, 40, 46],
);
const DISTANCE_ENEMY_QUEEN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -553, -172, -122, -93, -80, -65, -48],
    [0, -212, -20, -16, -7, -5, 5, 16],
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
         125,  127,  147,  150,
          89,  104,  152,  130,
          85,   99,   96,  118,
          67,   82,   96,  109,
          76,   87,   88,   89,
          68,   99,   84,   77,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         284,  289,  264,  244,
          78,   87,   85,   89,
          71,   74,   78,   65,
          64,   66,   68,   62,
          52,   58,   65,   68,
          50,   50,   68,   72,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         136,  268,  212,  273,
         266,  288,  373,  341,
         286,  330,  342,  358,
         340,  325,  338,  342,
         309,  321,  344,  338,
         305,  325,  333,  341,
         314,  313,  323,  329,
         267,  308,  307,  309,
    ],
    [
         255,  260,  290,  292,
         283,  295,  282,  292,
         291,  295,  322,  313,
         293,  314,  327,  332,
         295,  305,  317,  327,
         282,  298,  302,  318,
         279,  291,  298,  300,
         272,  282,  301,  302,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         283,  290,  228,  194,
         275,  304,  296,  282,
         319,  331,  353,  334,
         302,  311,  305,  328,
         313,  325,  319,  347,
         338,  331,  339,  325,
         326,  361,  334,  332,
         331,  322,  316,  322,
    ],
    [
         300,  293,  304,  312,
         299,  303,  301,  300,
         301,  301,  302,  304,
         301,  304,  307,  312,
         294,  291,  304,  299,
         285,  293,  294,  303,
         289,  280,  287,  291,
         285,  295,  299,  302,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         535,  557,  557,  552,
         533,  500,  519,  504,
         524,  512,  488,  487,
         496,  488,  483,  478,
         486,  476,  465,  478,
         499,  500,  501,  514,
         497,  519,  526,  530,
         541,  538,  559,  565,
    ],
    [
         499,  493,  491,  490,
         497,  514,  510,  510,
         499,  506,  512,  513,
         500,  505,  514,  513,
         498,  505,  513,  513,
         490,  492,  496,  497,
         491,  492,  497,  494,
         474,  485,  482,  483,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         915,  935,  939,  966,
         926,  881,  875,  791,
         925,  942,  906,  895,
         927,  906,  899,  887,
         934,  934,  927,  919,
         952,  958,  950,  947,
         968,  984,  980,  979,
         999,  995,  995,  998,
    ],
    [
         931,  904,  914,  897,
         919,  941,  963,  997,
         899,  895,  931,  946,
         907,  925,  930,  943,
         905,  902,  899,  923,
         881,  877,  899,  885,
         896,  874,  875,  882,
         879,  869,  877,  874,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
          21,   28,   48,   44,
         -11,   39,   85,   50,
         -10,   95,   70,   50,
         -49,    6,   37,   10,
         -74,  -18,  -31,  -47,
         -36,   -3,  -40,  -87,
           7,   25,  -41,  -85,
           5,   25,  -44,  -70,
    ],
    [
         -47,  -21,  -21,  -20,
          -2,    5,   -6,   -9,
           2,    4,    1,   -2,
           4,    1,    1,    3,
          -1,   -1,    5,   12,
          -2,    1,   13,   24,
          -6,    3,   22,   32,
         -29,   -8,   20,   19,
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

pub const MOBILITY_KNIGHT: [ScorePair; KNIGHT_MOB_LEN] = {
    let mg = MOBILITY_KNIGHT_MG_EG.0;
    let eg = MOBILITY_KNIGHT_MG_EG.1;
    let mut table = [ScorePair(0, 0); KNIGHT_MOB_LEN];
    let mut idx = 0;
    while idx < KNIGHT_MOB_LEN {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const MOBILITY_BISHOP: [ScorePair; BISHOP_MOB_LEN] = {
    let mg = MOBILITY_BISHOP_MG_EG.0;
    let eg = MOBILITY_BISHOP_MG_EG.1;
    let mut table = [ScorePair(0, 0); BISHOP_MOB_LEN];
    let mut idx = 0;
    while idx < BISHOP_MOB_LEN {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const MOBILITY_ROOK: [ScorePair; ROOK_MOB_LEN] = {
    let mg = MOBILITY_ROOK_MG_EG.0;
    let eg = MOBILITY_ROOK_MG_EG.1;
    let mut table = [ScorePair(0, 0); ROOK_MOB_LEN];
    let mut idx = 0;
    while idx < ROOK_MOB_LEN {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const MOBILITY_QUEEN: [ScorePair; QUEEN_MOB_LEN] = {
    let mg = MOBILITY_QUEEN_MG_EG.0;
    let eg = MOBILITY_QUEEN_MG_EG.1;
    let mut table = [ScorePair(0, 0); QUEEN_MOB_LEN];
    let mut idx = 0;
    while idx < QUEEN_MOB_LEN {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const VIRTUAL_MOBILITY: [ScorePair; VIRTUAL_MOB_LEN] = {
    let mg = VIRTUAL_MOBILITY_MG_EG.0;
    let eg = VIRTUAL_MOBILITY_MG_EG.1;
    let mut table = [ScorePair(0, 0); VIRTUAL_MOB_LEN];
    let mut idx = 0;
    while idx < VIRTUAL_MOB_LEN {
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
