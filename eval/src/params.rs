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
          11,   10,  -12,  -20,
          14,   21,   11,   -2,
          21,   -5,  -15,  -10,
          19,   19,    3,  -24,
          11,   10,    7,  -14,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
         146,  127,  113,   93,
          69,   68,   48,   52,
          34,   39,   24,   25,
          11,   13,    8,    8,
          13,   19,   -1,   -2,
           0,    0,    0,    0,
    ],
);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-18, -7);
pub const BACKWARD_PAWN: ScorePair = ScorePair(-12, -4);
pub const DOUBLED_PAWN: ScorePair = ScorePair(-3, -12);

pub const BISHOP_PAIR: ScorePair = ScorePair(32, 45);

const MOBILITY_KNIGHT_MG_EG: ([Score; KNIGHT_MOB_LEN], [Score; KNIGHT_MOB_LEN]) = (
    [-7, 20, 32, 38, 46, 47, 47, 47, 46],
    [-14, -3, -10, -13, -13, -8, -8, -10, -10],
);
const MOBILITY_BISHOP_MG_EG: ([Score; BISHOP_MOB_LEN], [Score; BISHOP_MOB_LEN]) = (
    [-7, 4, 15, 18, 22, 28, 33, 34, 38, 40, 50, 54, 58, 45],
    [-47, -34, -33, -20, -11, -2, 1, 9, 12, 14, 8, 11, 16, 15],
);
const MOBILITY_ROOK_MG_EG: ([Score; ROOK_MOB_LEN], [Score; ROOK_MOB_LEN]) = (
    [-9, 0, 2, 7, 7, 14, 20, 25, 29, 37, 41, 45, 55, 73, 80],
    [-35, -23, -15, -11, -3, -3, -4, 1, 7, 6, 9, 14, 17, 13, 5],
);
const MOBILITY_QUEEN_MG_EG: ([Score; QUEEN_MOB_LEN], [Score; QUEEN_MOB_LEN]) = (
    [
        2, 15, 18, 19, 25, 29, 32, 30, 27, 34, 36, 35, 36, 39, 43, 48, 51, 49, 60, 60, 54, 67, 71,
        95, 59, 56, 52, 34,
    ],
    [
        1, -39, -70, -62, -74, -48, -49, -35, -10, -4, 3, 15, 22, 30, 32, 26, 29, 47, 43, 44, 58,
        45, 41, 27, 41, 50, 53, 49,
    ],
);

const VIRTUAL_MOBILITY_MG_EG: ([Score; VIRTUAL_MOB_LEN], [Score; VIRTUAL_MOB_LEN]) = (
    [
        60, 58, 55, 42, 35, 31, 32, 30, 16, 12, 3, -10, -16, -20, -39, -55, -54, -47, -41, -28,
        -12, -17, -28, 1, -29, 23, -2, -2,
    ],
    [
        -13, -15, -13, -10, -6, -9, -13, -19, 1, -2, -1, 7, 7, 9, 13, 18, 14, 15, 10, 7, 2, 0, 1,
        -4, 0, -4, -2, 7,
    ],
);

const DISTANCE_FRIENDLY_PAWN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) =
    ([0, 27, 24, 13, 4, -14, -3, -7], [0, 6, 10, 4, 5, 11, 9, 9]);
const DISTANCE_ENEMY_PAWN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -15, -34, -6, -3, -7, -4, 26],
    [0, 41, 10, -8, -14, -19, -24, -41],
);
const DISTANCE_FRIENDLY_KNIGHT_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 53, 58, 55, 41, 48, 45, 17],
    [0, -14, -17, -16, -9, -14, -10, -7],
);
const DISTANCE_ENEMY_KNIGHT_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -72, -99, -43, -25, -25, -27, -25],
    [0, 32, 21, 3, 1, 7, 13, 11],
);
const DISTANCE_FRIENDLY_BISHOP_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 60, 71, 63, 73, 59, 64, 41],
    [0, -6, -9, -10, -16, -9, -10, -2],
);
const DISTANCE_ENEMY_BISHOP_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -163, -73, -44, -40, -37, -36, -40],
    [0, 44, 12, 0, -1, -1, 0, 7],
);
const DISTANCE_FRIENDLY_ROOK_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 47, 61, 69, 64, 65, 55, 65],
    [0, -8, -5, -9, -4, -2, 5, 2],
);
const DISTANCE_ENEMY_ROOK_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -252, -68, -53, -35, -19, -5, 5],
    [0, 65, 6, -2, -6, -9, -13, -18],
);
const DISTANCE_FRIENDLY_QUEEN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 172, 178, 176, 169, 167, 170, 141],
    [0, 21, 33, 37, 37, 39, 44, 52],
);
const DISTANCE_ENEMY_QUEEN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -561, -177, -126, -98, -85, -71, -55],
    [0, -219, -24, -20, -10, -8, 2, 16],
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
         124,  128,  149,  153,
          89,  106,  154,  132,
          84,   99,   97,  118,
          66,   81,   96,  108,
          76,   86,   88,   87,
          68,   98,   83,   73,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         287,  292,  266,  245,
          78,   86,   84,   89,
          71,   73,   78,   65,
          63,   66,   67,   61,
          51,   57,   64,   68,
          50,   51,   68,   73,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         136,  270,  214,  274,
         266,  289,  377,  343,
         286,  332,  344,  361,
         339,  326,  339,  343,
         308,  320,  346,  337,
         305,  325,  332,  340,
         317,  313,  322,  326,
         270,  307,  303,  308,
    ],
    [
         253,  259,  289,  291,
         283,  295,  281,  292,
         291,  295,  323,  314,
         293,  315,  327,  333,
         296,  306,  318,  329,
         282,  299,  303,  319,
         279,  291,  299,  302,
         270,  282,  302,  302,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         283,  290,  230,  196,
         274,  304,  298,  284,
         320,  332,  354,  336,
         303,  313,  307,  329,
         314,  324,  320,  346,
         339,  332,  339,  324,
         330,  363,  333,  331,
         331,  321,  315,  319,
    ],
    [
         301,  293,  304,  312,
         299,  303,  301,  299,
         301,  300,  303,  303,
         301,  303,  307,  312,
         294,  291,  303,  299,
         286,  292,  294,  303,
         290,  281,  287,  292,
         286,  296,  299,  303,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         533,  557,  559,  556,
         530,  501,  521,  509,
         522,  515,  490,  493,
         496,  490,  487,  483,
         485,  477,  469,  480,
         499,  502,  507,  519,
         495,  519,  526,  533,
         533,  531,  552,  557,
    ],
    [
         500,  494,  491,  490,
         498,  515,  510,  509,
         500,  505,  512,  511,
         501,  505,  514,  512,
         498,  505,  512,  513,
         490,  492,  495,  496,
         492,  492,  498,  494,
         478,  488,  484,  485,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         921,  937,  941,  972,
         926,  885,  878,  795,
         926,  945,  907,  897,
         929,  909,  901,  889,
         936,  935,  929,  918,
         952,  960,  951,  948,
         968,  984,  980,  979,
         998,  992,  992,  997,
    ],
    [
         929,  905,  916,  896,
         920,  940,  963,  995,
         899,  895,  932,  948,
         907,  925,  931,  945,
         905,  903,  900,  927,
         883,  877,  901,  886,
         898,  875,  874,  884,
         882,  873,  878,  872,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
          23,   30,   51,   46,
          -5,   38,   80,   47,
           1,   99,   62,   41,
         -39,   16,   32,   -2,
         -68,   -7,  -26,  -50,
         -39,    8,  -31,  -77,
         -10,   28,  -33,  -81,
         -23,   11,  -49,  -75,
    ],
    [
         -52,  -23,  -24,  -21,
          -6,    5,   -4,   -7,
          -3,    3,    5,    3,
          -1,    0,    6,   10,
          -5,   -3,    8,   18,
          -3,   -1,   13,   25,
          -4,    2,   21,   32,
         -26,   -7,   19,   19,
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
