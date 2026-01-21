use movegen::{file::File, rank::Rank, square::Square};

use crate::{score_pair::ScorePair, Score};

pub type PieceSquareTable = [ScorePair; 64];

pub const KNIGHT_MOB_LEN: usize = 9;
pub const BISHOP_MOB_LEN: usize = 14;
pub const ROOK_MOB_LEN: usize = 15;
pub const QUEEN_MOB_LEN: usize = 28;
pub const VIRTUAL_DIAG_MOB_LEN: usize = 8;
pub const VIRTUAL_LINE_MOB_LEN: usize = 8;
pub const VIRTUAL_MOB_LEN: usize = 2 * (VIRTUAL_DIAG_MOB_LEN + VIRTUAL_LINE_MOB_LEN);
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
pub const TEMPO: ScorePair = ScorePair(39, 23);

#[rustfmt::skip]
const PASSED_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
          27,   26,   19,   -9,
          18,   42,   31,  -12,
          28,   -9,  -22,  -11,
          24,    0,    4,  -41,
          14,   -5,    5,  -11,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
         169,  140,   89,   44,
          78,   71,   56,   51,
          34,   47,   27,   30,
           6,   11,    6,    8,
          13,   15,   -7,  -18,
           0,    0,    0,    0,
    ],
);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-20, -3);
pub const BACKWARD_PAWN: ScorePair = ScorePair(-20, -1);
pub const DOUBLED_PAWN: ScorePair = ScorePair(-5, -15);

pub const BISHOP_PAIR: ScorePair = ScorePair(42, 23);

const MOBILITY_KNIGHT_MG_EG: ([Score; KNIGHT_MOB_LEN], [Score; KNIGHT_MOB_LEN]) = (
    [-26, 36, 51, 56, 69, 70, 67, 72, 67],
    [-3, 15, -33, -17, -14, -3, 1, -1, 3],
);
const MOBILITY_BISHOP_MG_EG: ([Score; BISHOP_MOB_LEN], [Score; BISHOP_MOB_LEN]) = (
    [2, 2, 24, 27, 44, 43, 48, 52, 66, 67, 82, 76, 42, 22],
    [-31, -23, -29, -19, -9, 0, 3, 7, 12, 11, 3, 24, 23, 18],
);
const MOBILITY_ROOK_MG_EG: ([Score; ROOK_MOB_LEN], [Score; ROOK_MOB_LEN]) = (
    [3, 5, 6, 12, 20, 25, 28, 37, 39, 56, 60, 57, 67, 69, 71],
    [-23, -16, 1, -4, -2, 3, 5, 8, 19, 15, 12, 22, 21, 27, 28],
);
const MOBILITY_QUEEN_MG_EG: ([Score; QUEEN_MOB_LEN], [Score; QUEEN_MOB_LEN]) = (
    [
        -42, 4, -1, 4, 16, 26, 34, 31, 18, 32, 36, 32, 34, 45, 46, 57, 54, 54, 54, 65, 43, 70, 61,
        55, 22, 19, 5, 12,
    ],
    [
        1, 8, -3, 13, -9, -16, -43, -12, 14, 35, 30, 57, 44, 38, 49, 41, 53, 60, 64, 70, 64, 60,
        41, 57, 21, 35, 6, 21,
    ],
);
const VIRTUAL_DIAG_MOBILITY_MG_EG: ([Score; VIRTUAL_DIAG_MOB_LEN], [Score; VIRTUAL_DIAG_MOB_LEN]) = (
    [36, 11, -7, 8, 0, -5, -27, -16],
    [-20, -3, 0, -1, -2, 3, 8, 15],
);
const VIRTUAL_FILE_MOBILITY_MG_EG: ([Score; VIRTUAL_LINE_MOB_LEN], [Score; VIRTUAL_LINE_MOB_LEN]) = (
    [40, 25, 0, -3, -13, -8, -10, -30],
    [-17, 0, 4, 8, 3, 1, 0, 1],
);
const VIRTUAL_RANK_MOBILITY_MG_EG: ([Score; VIRTUAL_LINE_MOB_LEN], [Score; VIRTUAL_LINE_MOB_LEN]) =
    ([2, 3, 14, 3, -3, 10, 1, -30], [18, 6, -5, 0, -7, -7, -5, 0]);

const DISTANCE_FRIENDLY_PAWN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 36, 42, 25, 15, -6, 5, 14],
    [0, 10, 10, 8, 6, 19, 15, 15],
);
const DISTANCE_ENEMY_PAWN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -39, -46, -16, -15, -19, -13, 17],
    [0, 33, 3, -13, -18, -21, -24, -41],
);
const DISTANCE_FRIENDLY_KNIGHT_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 72, 89, 86, 67, 79, 83, -13],
    [0, -1, -9, -8, -3, -10, -10, -11],
);
const DISTANCE_ENEMY_KNIGHT_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -93, -129, -69, -50, -46, -43, -32],
    [0, 25, 25, 6, 0, -4, 7, -6],
);
const DISTANCE_FRIENDLY_BISHOP_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 85, 101, 98, 101, 93, 91, 27],
    [0, 9, -4, 0, -10, -5, 1, 1],
);
const DISTANCE_ENEMY_BISHOP_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -190, -92, -73, -69, -54, -64, -55],
    [0, 35, 7, -6, 3, -13, -7, -11],
);
const DISTANCE_FRIENDLY_ROOK_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 56, 73, 90, 79, 85, 77, 94],
    [0, 11, 17, 7, 19, 18, 20, 24],
);
const DISTANCE_ENEMY_ROOK_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -118, -85, -84, -62, -83, -60, -63],
    [0, -17, -10, -18, -21, -12, -22, -17],
);
const DISTANCE_FRIENDLY_QUEEN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, 125, 143, 135, 131, 134, 123, 96],
    [0, 97, 113, 130, 118, 119, 125, 99],
);
const DISTANCE_ENEMY_QUEEN_MG_EG: ([Score; DISTANCE_LEN], [Score; DISTANCE_LEN]) = (
    [0, -277, -165, -108, -95, -85, -79, -78],
    [0, -144, -147, -131, -111, -113, -91, -63],
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
         171,  148,  158,  185,
          66,  114,  148,  129,
          90,  105,  103,  137,
          53,   77,  105,  114,
          68,   84,   89,   88,
          57,  101,   83,   61,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         305,  312,  305,  259,
          82,   83,   89,  136,
          71,   63,   66,   51,
          66,   57,   54,   49,
          47,   49,   54,   53,
          52,   43,   61,   74,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         244,  300,  265,  313,
         283,  286,  380,  338,
         276,  304,  341,  376,
         346,  325,  350,  343,
         291,  313,  339,  325,
         287,  317,  332,  338,
         307,  297,  316,  328,
         285,  304,  318,  298,
    ],
    [
         235,  267,  289,  307,
         271,  297,  288,  300,
         279,  307,  333,  308,
         295,  320,  321,  321,
         324,  298,  316,  328,
         301,  296,  298,  310,
         280,  305,  282,  297,
         277,  286,  306,  307,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         316,  312,  286,  268,
         272,  310,  294,  279,
         320,  336,  347,  333,
         310,  311,  293,  331,
         295,  317,  310,  350,
         342,  342,  352,  318,
         326,  368,  331,  327,
         322,  318,  331,  329,
    ],
    [
         299,  285,  273,  293,
         298,  302,  298,  306,
         298,  309,  310,  309,
         298,  310,  313,  328,
         303,  303,  310,  304,
         288,  296,  288,  311,
         290,  280,  291,  297,
         307,  308,  286,  301,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         515,  514,  509,  525,
         530,  514,  544,  554,
         542,  514,  502,  515,
         513,  504,  511,  530,
         492,  469,  503,  501,
         488,  503,  512,  518,
         462,  524,  527,  544,
         522,  527,  567,  559,
    ],
    [
         518,  517,  521,  512,
         516,  526,  526,  505,
         513,  509,  508,  510,
         500,  507,  513,  499,
         494,  508,  511,  513,
         492,  490,  491,  494,
         506,  505,  487,  493,
         483,  485,  475,  488,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         962,  917,  924,  932,
         932,  888,  927,  851,
         917,  926,  916,  917,
         944,  895,  922,  895,
         911,  918,  927,  912,
         946,  936,  925,  924,
         943,  948,  959,  946,
         952,  943,  958,  974,
    ],
    [
         940,  902,  937,  931,
         931,  942,  940,  936,
         899,  894,  927,  938,
         917,  934,  935,  951,
         950,  918,  921,  951,
         900,  906,  935,  921,
         914,  943,  895,  932,
         935,  920,  913,  894,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           4,   -2,    0,   -6,
          -1,   10,   17,   10,
          -4,   25,   22,   20,
         -10,   26,   15,    9,
         -33,    9,  -19,    2,
          -3,   36,    3,  -57,
           3,   71,  -12,  -61,
         -31,   41,  -36,  -48,
    ],
    [
         -21,  -20,  -23,  -29,
           7,   -2,   11,   16,
           4,   15,    8,   19,
         -10,    3,    5,    1,
         -16,   -5,    2,    8,
         -14,   -2,    9,   22,
         -14,  -20,   14,   36,
         -24,  -11,   21,   10,
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

pub const VIRTUAL_DIAG_MOBILITY: [ScorePair; VIRTUAL_DIAG_MOB_LEN] = {
    let mg = VIRTUAL_DIAG_MOBILITY_MG_EG.0;
    let eg = VIRTUAL_DIAG_MOBILITY_MG_EG.1;
    let mut table = [ScorePair(0, 0); VIRTUAL_DIAG_MOB_LEN];
    let mut idx = 0;
    while idx < VIRTUAL_DIAG_MOB_LEN {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const VIRTUAL_FILE_MOBILITY: [ScorePair; VIRTUAL_LINE_MOB_LEN] = {
    let mg = VIRTUAL_FILE_MOBILITY_MG_EG.0;
    let eg = VIRTUAL_FILE_MOBILITY_MG_EG.1;
    let mut table = [ScorePair(0, 0); VIRTUAL_LINE_MOB_LEN];
    let mut idx = 0;
    while idx < VIRTUAL_LINE_MOB_LEN {
        table[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

pub const VIRTUAL_RANK_MOBILITY: [ScorePair; VIRTUAL_LINE_MOB_LEN] = {
    let mg = VIRTUAL_RANK_MOBILITY_MG_EG.0;
    let eg = VIRTUAL_RANK_MOBILITY_MG_EG.1;
    let mut table = [ScorePair(0, 0); VIRTUAL_LINE_MOB_LEN];
    let mut idx = 0;
    while idx < VIRTUAL_LINE_MOB_LEN {
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
