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
pub const TEMPO: ScorePair = ScorePair(40, 25);

#[rustfmt::skip]
const PASSED_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
          24,   23,   -2,   -1,
          20,   36,   24,    1,
          34,  -16,  -28,  -20,
          36,    6,    6,  -46,
           6,   -9,    2,  -12,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
         166,  128,  109,   62,
          76,   67,   54,   39,
          34,   37,   26,   24,
           4,    0,    7,   16,
          30,   29,   -8,  -22,
           0,    0,    0,    0,
    ],
);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-22, -3);
pub const BACKWARD_PAWN: ScorePair = ScorePair(-18, 0);
pub const DOUBLED_PAWN: ScorePair = ScorePair(-2, -15);

pub const BISHOP_PAIR: ScorePair = ScorePair(45, 23);

const MOBILITY_KNIGHT_MG_EG: ([Score; KNIGHT_MOB_LEN], [Score; KNIGHT_MOB_LEN]) = (
    [-12, 27, 48, 53, 65, 71, 75, 71, 86],
    [-22, -21, -24, -19, -11, 1, 7, 15, 4],
);
const MOBILITY_BISHOP_MG_EG: ([Score; BISHOP_MOB_LEN], [Score; BISHOP_MOB_LEN]) = (
    [0, 8, 28, 37, 48, 51, 69, 66, 67, 69, 76, 64, 14, 17],
    [-39, -36, -24, -20, -10, 1, -2, 6, 3, 6, 19, 7, 28, 18],
);
const MOBILITY_ROOK_MG_EG: ([Score; ROOK_MOB_LEN], [Score; ROOK_MOB_LEN]) = (
    [-2, 10, 8, 13, 24, 28, 35, 46, 49, 57, 64, 77, 82, 36, 52],
    [-28, -18, 3, -11, -7, 0, 5, 6, 15, 15, 13, 13, 17, 28, 29],
);
const MOBILITY_QUEEN_MG_EG: ([Score; QUEEN_MOB_LEN], [Score; QUEEN_MOB_LEN]) = (
    [
        -33, -1, 4, 1, 20, 26, 29, 34, 39, 38, 32, 56, 56, 48, 63, 62, 54, 54, 64, 74, 56, 35, 19,
        24, 20, 7, -1, 7,
    ],
    [
        -1, 6, -9, -2, -21, 13, 8, 23, 15, 34, 58, 29, 33, 41, 49, 39, 51, 57, 62, 67, 58, 46, 42,
        26, 23, 18, 0, 13,
    ],
);

const DISTANCE_FRIENDLY_PAWN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) =
    ([0, 59, 36, 20, 13, -14, -5, 12], [0, 2, 8, 4, 5, 15, 14, 6]);
const DISTANCE_ENEMY_PAWN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -39, -54, -7, -12, -15, -12, 18],
    [0, 35, 6, -11, -16, -18, -21, -30],
);
const DISTANCE_FRIENDLY_KNIGHT_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 97, 90, 87, 73, 81, 72, -16],
    [0, -15, -17, -9, -6, -16, -12, 5],
);
const DISTANCE_ENEMY_KNIGHT_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -95, -127, -74, -61, -52, -49, -26],
    [0, 27, 20, 8, 7, 2, 12, -5],
);
const DISTANCE_FRIENDLY_BISHOP_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 105, 105, 94, 102, 91, 86, 32],
    [0, -2, -5, -1, -13, -4, -7, -11],
);
const DISTANCE_ENEMY_BISHOP_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -185, -100, -71, -76, -60, -62, -61],
    [0, 46, 17, 0, 5, -6, -9, -11],
);
const DISTANCE_FRIENDLY_ROOK_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 70, 75, 84, 82, 94, 81, 94],
    [0, 5, 16, 8, 12, 7, 17, 16],
);
const DISTANCE_ENEMY_ROOK_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -122, -86, -86, -93, -86, -57, -50],
    [0, -13, -5, -12, -6, -6, -17, -21],
);
const DISTANCE_FRIENDLY_QUEEN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 145, 144, 144, 127, 121, 123, 83],
    [0, 81, 111, 112, 120, 117, 136, 104],
);
const DISTANCE_ENEMY_QUEEN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -270, -168, -124, -94, -90, -79, -61],
    [0, -146, -132, -127, -119, -102, -87, -67],
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
         146,  131,  145,  175,
          85,  103,  170,  119,
          87,   93,  107,  124,
          64,   71,  103,  112,
          72,   91,   87,   95,
          68,  107,   85,   79,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         306,  311,  293,  253,
          78,   90,   74,  123,
          69,   65,   63,   57,
          62,   65,   57,   52,
          53,   53,   55,   58,
          48,   39,   65,   65,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         247,  298,  259,  310,
         282,  307,  374,  332,
         314,  290,  338,  342,
         348,  318,  322,  335,
         304,  315,  339,  316,
         303,  316,  332,  330,
         310,  315,  326,  337,
         298,  318,  303,  308,
    ],
    [
         242,  271,  296,  291,
         279,  284,  274,  290,
         299,  310,  323,  304,
         311,  315,  314,  314,
         320,  293,  318,  327,
         292,  300,  282,  299,
         294,  303,  293,  293,
         297,  292,  304,  305,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         348,  305,  295,  254,
         274,  304,  296,  276,
         330,  334,  356,  339,
         291,  310,  284,  331,
         295,  327,  311,  356,
         342,  339,  354,  329,
         336,  369,  332,  335,
         314,  310,  316,  324,
    ],
    [
         294,  299,  269,  296,
         280,  313,  310,  303,
         293,  302,  302,  304,
         304,  313,  319,  323,
         312,  294,  309,  300,
         283,  297,  296,  305,
         283,  269,  295,  289,
         294,  311,  298,  301,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         507,  500,  514,  519,
         528,  522,  555,  537,
         535,  537,  504,  528,
         509,  496,  520,  526,
         494,  479,  470,  487,
         486,  500,  493,  515,
         452,  517,  545,  553,
         543,  541,  577,  590,
    ],
    [
         519,  526,  507,  514,
         510,  515,  513,  503,
         501,  504,  512,  516,
         496,  514,  510,  498,
         493,  507,  522,  524,
         488,  493,  499,  500,
         515,  495,  486,  496,
         464,  484,  477,  479,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         958,  915,  915,  922,
         939,  884,  902,  844,
         920,  914,  926,  938,
         924,  903,  926,  897,
         902,  919,  919,  907,
         933,  936,  927,  930,
         934,  968,  957,  959,
         960,  958,  974,  978,
    ],
    [
         937,  910,  931,  925,
         924,  945,  956,  934,
         899,  913,  909,  946,
         911,  917,  917,  963,
         944,  927,  936,  949,
         896,  907,  919,  913,
         929,  926,  908,  922,
         942,  912,  908,  904,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           4,    3,    5,    0,
          -4,   -1,   15,   -2,
           2,   26,   25,   24,
           3,   21,   19,   15,
         -19,   17,  -25,  -20,
         -35,    7,  -16,  -70,
           7,   47,  -31,  -69,
          38,   65,  -18,  -33,
    ],
    [
         -33,  -12,  -15,  -22,
         -19,   -5,    1,   11,
           3,    7,   19,    6,
         -14,    8,   15,   14,
         -10,  -16,   13,   23,
          -5,    6,   11,   27,
          -8,   -7,   20,   33,
         -51,  -28,   12,   15,
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
