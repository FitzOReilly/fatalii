use movegen::{file::File, rank::Rank, square::Square};

use crate::{score_pair::ScorePair, Score};

pub type PieceSquareTable = [ScorePair; 64];

pub const KNIGHT_MOB_LEN: usize = 9;
pub const BISHOP_MOB_LEN: usize = 14;
pub const ROOK_MOB_LEN: usize = 15;
pub const QUEEN_MOB_LEN: usize = 28;
pub const MOB_LEN: usize = KNIGHT_MOB_LEN + BISHOP_MOB_LEN + ROOK_MOB_LEN + QUEEN_MOB_LEN;

pub const PIECE_RELATIVE_TO_KING_LEN: usize = (2 * Rank::NUM_RANKS - 1) * File::NUM_FILES;
// Pawns are never on ranks 1 or 8, so we can ignore them
pub const PASSED_PAWNS_RELATIVE_TO_KING_LEN: usize =
    (2 * (Rank::NUM_RANKS - 1) - 1) * File::NUM_FILES;

// (middlegame, endgame)
const MATERIAL_KING: ScorePair = ScorePair(0, 0);
const MATERIAL_QUEEN: ScorePair = ScorePair(0, 0);
const MATERIAL_ROOK: ScorePair = ScorePair(0, 0);
const MATERIAL_BISHOP: ScorePair = ScorePair(0, 0);
const MATERIAL_KNIGHT: ScorePair = ScorePair(0, 0);
const MATERIAL_PAWN: ScorePair = ScorePair(0, 0);

// The side to move gets a small bonus
pub const TEMPO: ScorePair = ScorePair(30, 29);

#[rustfmt::skip]
const PASSED_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
          16,    8,   -9,  -17,
          19,   22,   15,    6,
          23,   -3,  -10,   -7,
          18,   22,    5,  -16,
           9,   11,   10,  -11,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
         138,  121,  107,   82,
          64,   63,   43,   43,
          30,   35,   20,   18,
          10,    9,    4,    1,
          13,   18,   -3,   -4,
           0,    0,    0,    0,
    ],
);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-17, -8);
pub const BACKWARD_PAWN: ScorePair = ScorePair(-11, -3);
pub const DOUBLED_PAWN: ScorePair = ScorePair(-7, -9);

pub const BISHOP_PAIR: ScorePair = ScorePair(31, 41);

const MOBILITY_KNIGHT_MG_EG: ([Score; KNIGHT_MOB_LEN], [Score; KNIGHT_MOB_LEN]) = (
    [19, 48, 59, 64, 70, 71, 73, 74, 73],
    [-18, -24, -29, -32, -32, -29, -30, -34, -33],
);
const MOBILITY_BISHOP_MG_EG: ([Score; BISHOP_MOB_LEN], [Score; BISHOP_MOB_LEN]) = (
    [24, 35, 47, 51, 56, 61, 66, 68, 72, 73, 82, 87, 94, 68],
    [-46, -31, -32, -23, -15, -6, -3, 4, 8, 11, 5, 9, 9, 18],
);
const MOBILITY_ROOK_MG_EG: ([Score; ROOK_MOB_LEN], [Score; ROOK_MOB_LEN]) = (
    [-1, 8, 10, 15, 17, 26, 30, 35, 39, 47, 53, 55, 65, 82, 91],
    [
        -40, -29, -18, -13, -10, -10, -11, -5, -1, -2, 1, 6, 10, 5, -3,
    ],
);
const MOBILITY_QUEEN_MG_EG: ([Score; QUEEN_MOB_LEN], [Score; QUEEN_MOB_LEN]) = (
    [
        37, 50, 54, 55, 62, 66, 71, 71, 69, 76, 79, 78, 79, 82, 86, 91, 95, 92, 102, 102, 95, 112,
        116, 133, 92, 77, 73, 57,
    ],
    [
        5, -7, -19, -8, -25, 3, -3, 4, 26, 34, 38, 49, 55, 61, 64, 57, 58, 75, 72, 71, 85, 71, 67,
        62, 76, 94, 89, 91,
    ],
);

const PAWN_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        0, 0, 0, 0, 0, 0, 0, 0, 2, -9, -1, 0, 2, -1, 4, 0, 5, -8, 11, -2, -3, -14, -9, 3, -21, -20,
        -31, -15, -9, -36, -24, -5, -21, -29, -44, -73, -45, -72, -88, -9, 9, -47, -27, -79, -66,
        -80, -64, -32, 17, -8, 16, -16, -21, -35, -47, -23, 0, 45, 21, 6, -4, -18, -32, -24, 69,
        48, 17, 15, -1, -20, -6, -12, 68, 37, 32, 13, 10, -7, -8, -12, 48, 35, 19, 15, 16, 2, 6,
        -2, 39, 24, 20, 23, 20, 18, 15, 15, 12, 5, 0, 9, 21, 18, 23, -1, -7, -22, 5, -12, -9, 3,
        54, -24, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 35, 30, 26, 26, 45, 9, 36, 10, 56, 55, 45, 35, 42, 64, 36, 24, 40,
        49, 31, 33, 39, 45, 41, 23, 28, 32, 31, 30, 31, 33, 44, 33, 14, 34, 17, 29, 31, 35, 30, 37,
        17, 12, 11, 17, 19, 21, 27, 17, 0, 12, 8, 5, 9, 11, 21, 10, -8, -8, 6, 3, 4, 13, 9, 8, -7,
        -3, -4, -4, -4, 2, 3, 7, -21, -19, -19, -14, -13, -2, -6, -5, -35, -24, -32, -31, -16, -19,
        -8, -23, -38, -21, -28, -31, -27, -14, -4, -12, -61, -48, -62, -26, -22, -28, -16, 24, 0,
        0, 0, 0, 0, 0, 0, 0,
    ],
);
const PAWN_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        0, 0, 0, 0, 0, 0, 0, 0, -4, -2, 4, -3, -7, 0, -3, -1, -2, -2, 0, -1, -3, 9, -5, -1, 1, 16,
        24, 17, -1, 7, 4, 10, 15, 57, 30, 33, 51, 48, 28, 3, 66, 86, 76, 88, 65, 61, 38, 34, 53,
        113, 7, 43, 57, 52, 32, 29, 0, -15, -58, 28, 56, 44, 46, 29, 24, -88, -47, 0, -6, 18, 11,
        1, -42, -45, -25, -19, -4, 0, -6, 18, -16, -31, -27, -12, -17, -19, -17, 5, -14, -26, -26,
        -28, -22, -25, -35, -5, -17, -25, -30, -31, -26, -29, -33, 1, -21, -26, -30, -29, -27, -26,
        -41, -5, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, -16, -18, 9, -33, -43, -24, -32, -6, -67, -73, -70, -63, -69, -61,
        -75, -51, -84, -60, -68, -61, -40, -69, -68, -28, -47, -44, -30, -36, -50, -59, -39, -7,
        -11, -10, -17, -31, -35, -33, -29, -14, 26, 16, 34, -3, -19, -27, -19, -20, 0, 71, 54, 12,
        -10, -13, -24, -12, 68, 33, 44, 16, 9, -7, -12, -7, 34, 35, 24, 21, 9, -1, -5, -15, 26, 24,
        24, 19, 12, 6, 1, -19, 22, 24, 24, 23, 16, 9, 9, -9, 23, 21, 22, 23, 16, 11, 9, -8, 27, 21,
        22, 20, 18, 12, 11, -7, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
);
const KNIGHT_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        -1, -1, 0, 0, 0, 0, 0, 0, -1, -4, -6, -3, -1, 1, -2, 0, -6, -3, -9, -5, -6, -2, -2, 0, -6,
        -7, -15, -10, -25, -3, -16, 0, -6, -29, -2, -22, -14, -11, -2, 0, -30, 2, 8, -11, -10, -39,
        1, -5, 39, 12, 34, 22, -6, 8, 14, 0, 0, 26, 19, 29, 2, 18, 16, 3, 48, 37, 28, 23, 17, 8,
        22, -27, 37, 35, 35, 24, 12, 16, 8, -23, 23, 27, 24, 21, 29, 29, 17, 1, -3, 20, 6, 20, 32,
        11, 27, -36, 26, -11, -1, -21, 17, 16, 28, 23, 14, 7, -5, 5, -46, 20, 6, 16, 28, 5, 25,
        -14, -48, -56, 18, 22,
    ],
    [
        -7, -6, -2, 0, 0, 0, -7, 0, -11, -18, -28, -6, -5, -3, -5, -2, -17, -4, -9, -23, -4, -2, 3,
        -2, -37, -23, -35, -36, -35, -14, -22, -2, -31, -28, -21, -29, -16, -23, -15, -1, -14, -16,
        -10, 1, -12, -29, -28, -13, -16, -8, -20, -13, -6, -28, -46, 1, 0, -5, -11, -16, -8, -24,
        -35, -34, -2, -11, -6, -5, -9, -14, -18, -16, 3, -8, 0, -1, -1, -3, 6, -3, 4, 4, 10, 0, 2,
        -4, 12, 17, 21, 22, 23, 29, 4, 21, 7, -9, 20, 32, 25, 42, 31, 11, 18, 20, 21, 23, 33, 39,
        42, 24, 26, 17, 33, -16, 13, 28, 25, 24, 30, 21,
    ],
);
const KNIGHT_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        0, -1, 0, 1, 0, 0, -1, 0, -1, 2, 5, 1, 2, 0, 1, 3, 13, 13, 10, 5, 9, 7, 1, -1, 5, 21, 2, 8,
        10, 3, 2, 0, 24, 3, -1, 2, 11, 17, -3, 1, -13, -10, 6, 44, -1, 15, 15, 4, 15, 6, -76, 6,
        -7, 30, 6, -2, 0, 21, -20, -4, -34, 14, 66, -16, -46, -20, -162, -22, 6, -5, 15, 12, -33,
        -129, -17, -9, -18, 10, 14, 14, -13, -25, -5, -18, 4, 5, -6, -18, -15, -2, -10, 4, -2, -6,
        -13, 36, -9, -7, -7, -1, -16, -3, -11, 35, -10, -18, -15, -25, -17, 6, -3, -23, -28, -11,
        -27, -19, -20, -35, -36, -19,
    ],
    [
        0, 0, 1, 7, 7, 1, 0, 0, -4, 15, 19, 9, 20, 1, 1, 7, 19, 37, 21, 26, 58, 33, 6, -4, 18, 59,
        22, 31, 39, 46, 9, 7, 33, 32, 50, -10, 32, 50, 9, 5, 20, 6, 46, 23, 15, 26, 38, 6, 49, 18,
        5, -2, 12, 8, 15, -5, 0, 29, 2, 4, -6, -3, -31, 11, 31, -2, 1, -15, -2, -3, -4, 8, -6, -33,
        -3, -11, -27, -18, -9, -14, -14, -31, -12, -52, -31, -23, -21, 17, -21, -19, -27, -24, -25,
        -17, -18, -33, -13, -19, -21, -27, -19, -23, -16, 4, 0, -17, -12, -7, -1, -8, -19, -8, 0,
        -25, -12, -28, -19, 8, 7, -3,
    ],
);
const BISHOP_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        -4, -4, -5, -1, -5, 1, 0, 0, -1, -4, -8, -1, -2, 1, -1, -1, -14, 4, -2, -8, 2, -1, 4, -7,
        -14, -27, -20, -33, -16, -1, -2, -2, -20, -35, -10, -21, -46, -26, -1, -1, 24, 3, -11, -17,
        25, -25, -10, -1, -4, 14, 8, 21, 18, 22, -37, -7, 0, 17, 33, 28, 34, 17, -1, -10, 48, 49,
        35, 30, 14, 16, 17, -2, 48, 26, 37, 17, 26, 21, 29, 11, 30, 35, 25, 30, 16, 29, 30, 9, 20,
        -4, 31, 20, 25, 18, 10, -25, 32, 22, 14, 20, 0, 27, 14, -7, -10, 11, 0, 13, 14, 31, 65, 47,
        17, 16, -29, 5, 10, 25, 30, -11,
    ],
    [
        -15, -8, -2, -1, -6, 8, 0, 2, 11, -21, -16, -1, -19, 16, -4, -4, -6, -13, 5, 0, 8, -9, 18,
        -22, -27, -5, -5, -2, -4, 12, -4, -5, -29, -17, -2, -14, -8, -24, -1, 15, -18, -12, -19,
        -10, -21, -4, -8, 27, 4, -13, -6, -12, -12, -18, -15, -13, 0, -4, -14, -17, -31, -25, -5,
        -10, -7, -11, -4, -8, -2, -9, -15, -2, -12, 0, 2, 5, -10, 1, -8, -1, 1, -3, 2, -2, 3, -2,
        -2, -11, 1, 17, -3, 10, 9, 8, 5, 8, 10, 17, 14, 14, 17, 10, 2, 14, 36, 12, 28, 30, 12, 4,
        12, 28, 16, 24, 22, 10, 15, 7, 29, 14,
    ],
);
const BISHOP_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        1, 1, 1, 1, 2, 0, 0, 0, 3, 3, 7, 1, 2, 2, 0, 0, 8, 7, 2, -1, 4, -1, 3, 2, 12, 20, 0, 12,
        -4, 3, -1, 2, 27, 13, 23, -8, 11, 0, 1, 5, 16, 45, -6, 5, -23, 18, 3, 5, 15, -15, 48, 25,
        -26, 11, 58, -8, 0, -34, -39, 20, -2, 39, 0, 24, -45, -180, -9, -13, 6, 20, 32, 32, -39,
        -25, -97, -18, -18, 14, 6, -3, -31, -28, -16, -22, -11, -18, -2, -1, -14, -8, -26, -15,
        -32, -14, -32, 26, -7, -23, -13, -25, -20, -34, -18, -27, -23, -20, -25, -24, -35, -28,
        -41, -16, -20, -36, -31, -41, -37, -30, -51, -25,
    ],
    [
        11, 3, 1, 8, 12, 1, -1, 0, 27, 13, 7, 7, 0, 10, 0, 1, 25, 4, 15, 4, 4, -7, -8, 9, 5, 34, 2,
        30, -24, 9, -19, 3, 25, -2, 16, -2, 14, 11, 15, 23, 3, 22, -32, 18, 3, 11, 9, 2, 24, -25,
        14, -7, 14, 0, 3, 7, 0, 38, 15, 11, -13, 3, -1, -2, 41, -11, 19, -8, 4, -11, -16, -15, 5,
        12, -38, 8, -10, 2, -19, 8, 16, -4, 5, -50, -1, -16, -11, -4, -17, 3, -10, 1, -25, 2, -11,
        -12, 2, -11, -2, -7, -11, -26, -14, -14, -3, 3, -16, 0, -5, 4, -19, 15, -8, -13, 1, -12, 8,
        -13, 12, -21,
    ],
);
const ROOK_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        -3, -5, -13, 5, 2, -5, -1, -1, -4, -12, -7, -11, -6, 0, -3, -3, -11, -1, -13, -12, -25, -2,
        -6, 0, -15, -16, -47, -45, -2, -45, -30, -22, -17, 8, -22, -13, -29, -16, -56, -8, -19, -7,
        -10, 0, -24, 5, -3, -26, -11, 0, -3, -15, -12, -20, -16, -15, 0, 19, 33, 41, 34, 37, 28,
        35, 88, 35, 30, 32, 52, 25, 40, -22, 50, 37, 34, 23, 38, 20, 14, -57, 28, 18, 36, 24, 30,
        24, 17, -7, 21, 31, 24, 57, 25, 25, 23, -24, 32, 30, 34, 15, 33, 22, 5, -15, 43, 21, -32,
        -38, -20, 1, -18, -13, -8, 43, 4, 32, 23, 12, 38, -3,
    ],
    [
        -10, -8, -45, -10, -6, -27, -13, -6, -25, -53, -50, -36, -3, -15, -7, -18, -48, -48, -38,
        -47, -45, -23, -8, -8, -53, -35, -25, -45, -51, -39, -24, -11, -44, -37, -20, -37, -10,
        -15, -8, -3, -29, -22, -21, -28, -21, -22, -13, -1, -25, -25, -16, -15, -12, -5, -8, 1, 0,
        -10, -20, -26, -16, -15, -13, -15, -27, -13, 0, -6, -13, -4, -10, 7, 0, 2, 8, 9, 3, 5, 6,
        21, 0, 18, 12, 12, 13, 17, 19, 22, 13, 24, 26, 14, 19, 22, 18, 29, 22, 30, 30, 34, 34, 30,
        36, 37, 34, 39, 63, 63, 58, 53, 52, 46, 51, 50, 64, 55, 52, 53, 48, 6,
    ],
);
const ROOK_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        7, 22, 11, 5, 4, 0, -1, 1, 13, 20, 22, 17, 19, 11, 10, 3, 21, 5, -7, 19, 14, 18, 12, 1, 1,
        4, 12, 4, 28, 18, 11, 0, -7, 3, 16, 24, 22, 6, 17, 10, -31, -4, 42, 11, 18, 41, 44, 22,
        -62, -73, -10, -14, -11, 1, 6, 13, 0, -177, -64, -68, -43, -25, -8, 19, -194, -81, -22,
        -24, -5, 22, 21, 29, -59, -33, -16, -11, 25, 17, -3, 16, -40, -18, -14, -6, -1, 7, 11, 52,
        -19, -25, -8, -10, 2, 11, 0, 35, -27, -15, -4, -5, 13, 3, 14, 14, -15, -25, -15, -5, 7, 4,
        -3, 21, -38, -40, -35, -32, -26, -31, -26, -6,
    ],
    [
        36, 72, 37, 24, 19, 0, -1, 2, 59, 98, 75, 62, 58, 50, 40, 10, 69, 82, 67, 74, 89, 77, 50,
        15, 54, 63, 57, 64, 64, 64, 53, 22, 27, 45, 45, 44, 47, 46, 31, 32, 19, 39, 29, 31, 30, 27,
        33, 23, -69, 66, 32, 31, 32, 30, 33, 25, 0, -32, 15, 1, 1, 1, 7, -2, -22, 39, -2, -9, -9,
        -18, -19, -10, -17, -3, -13, -21, -33, -32, -25, -22, -23, -18, -29, -30, -37, -34, -47,
        -37, -29, -26, -40, -43, -43, -52, -48, -53, -26, -39, -47, -51, -59, -59, -68, -53, -48,
        -44, -57, -63, -71, -67, -72, -89, -50, -47, -56, -62, -63, -63, -75, -70,
    ],
);
const QUEEN_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        4, -4, 2, 2, -2, 0, -3, 2, 3, 0, 0, 9, 5, -3, -2, 4, -1, -2, 7, 6, -10, 1, -4, 0, 0, -23,
        2, -10, -5, 0, 7, -2, -5, 1, 5, -11, -16, -7, -7, 0, -3, -17, 39, -19, -11, -27, 8, 0, -3,
        -6, 9, 25, 38, 1, -10, -26, 0, 34, 20, 34, 22, 28, 8, 4, 53, 52, 45, 35, 29, 34, 43, 9, 57,
        49, 53, 41, 37, 35, 40, 36, 47, 43, 47, 31, 57, 63, 53, 4, 29, 39, 32, 46, 44, 59, 47, 17,
        7, 37, 24, 31, 46, 12, 39, 20, 66, 48, 18, 38, 52, 59, 65, 16, 40, 60, 35, -27, 16, 43,
        103, 4,
    ],
    [
        8, -8, 5, 3, -3, 0, -4, 4, 7, -1, -3, 15, 5, -6, -6, 5, 3, -5, 17, 11, -18, 5, -9, 0, 3,
        -29, 9, -8, 2, -1, 7, -4, -10, 8, 11, -20, -7, 0, -14, -1, 5, 34, -32, 17, 17, -1, 1, -17,
        -2, 9, 9, -4, -25, 11, -3, -1, 0, -4, 18, 9, 11, 1, 31, -24, -3, 11, 1, 18, 28, 21, -10,
        -20, 5, 15, -2, 18, 8, 17, -11, 1, -10, 24, 17, 21, -9, -8, -3, 39, 12, 21, 27, 29, 25,
        -10, 16, 22, 39, 42, 68, 31, 13, 46, 40, 3, 29, 33, 57, 56, 36, 30, 51, 40, 36, 28, 28, 79,
        44, 26, 42, 9,
    ],
);
const QUEEN_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        2, 22, 4, 10, 27, 6, 13, 5, 6, 21, 22, 17, 20, 26, 8, 6, 3, 12, 24, 1, 27, 5, 26, 4, -11,
        33, -2, 19, -2, 23, 19, 10, -26, -41, -7, -35, 31, 5, 15, 6, -44, -27, -57, -38, -2, -5, 9,
        -21, -29, -40, -12, -60, 8, 24, 43, 46, 0, -142, -141, -41, -13, 19, 68, 30, -415, -375,
        -61, -24, -4, 35, 46, 76, -106, -81, -114, -52, -16, 27, 34, 35, -85, -57, -57, -44, -35,
        -37, 10, -5, -57, -43, -43, -35, -19, -13, -4, 7, -36, -37, -23, -38, -27, -26, -23, -19,
        -41, -32, -25, -34, -25, -10, -34, -13, -46, -30, -34, -30, -34, -38, -17, -2,
    ],
    [
        4, 29, 8, 18, 40, 15, 23, 2, 10, 44, 30, 41, 42, 33, 16, 9, 23, 34, 34, 17, 50, 6, 36, 6,
        1, 48, 6, 48, 9, 59, 28, 16, -25, 13, 15, -28, 44, 5, 24, 14, -64, -2, -60, 4, -2, -1, 16,
        -24, -37, -55, -13, 20, 9, 0, 29, 39, 0, -134, -66, -48, -49, -49, -49, 1, -202, -145, -20,
        -50, -18, -46, -12, -1, -92, -18, -105, -7, -46, -51, -59, -10, -42, -39, -23, -53, -10,
        -34, -53, -6, -30, -17, -25, -35, -55, -23, -58, -24, -4, -20, -46, -16, -38, -2, 13, -32,
        2, 1, -25, -4, -29, -30, 2, 8, 42, 30, -5, -14, 6, -5, 5, 9,
    ],
);

const PASSED_PAWN_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
    [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
) = (
    [0; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
    [0; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
);
const PASSED_PAWN_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
    [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
) = (
    [0; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
    [0; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
);

// Piece square tables:
// We only define values for the queenside (left side) and mirror them to the
// kingside (right side) so that we end up with symmetrical PSTs.
#[rustfmt::skip]
const PST_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
         117,  155,  151,  170,
          63,   88,  133,  108,
          55,   68,   66,   87,
          36,   55,   65,   77,
          55,   57,   64,   59,
          44,   76,   65,   50,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         311,  314,  297,  280,
         114,  122,  122,  138,
         106,  110,  117,  111,
          97,   99,  104,  105,
          81,   90,   98,  108,
          78,   80,  101,  111,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         186,  278,  252,  308,
         299,  294,  398,  324,
         295,  354,  349,  403,
         333,  342,  336,  356,
         311,  319,  342,  343,
         306,  328,  328,  341,
         309,  317,  318,  321,
         264,  291,  307,  301,
    ],
    [
         239,  266,  290,  292,
         270,  291,  277,  293,
         282,  285,  313,  297,
         286,  299,  318,  318,
         282,  299,  311,  318,
         277,  290,  299,  312,
         276,  287,  298,  300,
         278,  296,  299,  301,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         302,  312,  294,  252,
         295,  355,  337,  340,
         344,  344,  365,  339,
         329,  326,  332,  346,
         315,  327,  327,  350,
         340,  333,  346,  329,
         320,  351,  333,  323,
         332,  321,  312,  313,
    ],
    [
         292,  289,  293,  302,
         295,  293,  294,  287,
         298,  299,  299,  302,
         295,  299,  301,  305,
         295,  292,  303,  300,
         284,  295,  296,  305,
         295,  292,  293,  299,
         290,  303,  312,  311,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         542,  572,  563,  560,
         566,  544,  562,  553,
         538,  525,  520,  520,
         506,  503,  511,  498,
         497,  491,  487,  491,
         510,  507,  505,  516,
         475,  508,  498,  511,
         489,  483,  511,  511,
    ],
    [
         515,  505,  505,  504,
         499,  511,  506,  502,
         499,  503,  503,  503,
         495,  498,  502,  503,
         488,  494,  500,  503,
         477,  482,  485,  487,
         484,  482,  496,  490,
         480,  493,  490,  495,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         962,  968,  968,  992,
         983,  958,  945,  883,
         982, 1014,  947,  946,
         964,  930,  935,  927,
         963,  959,  955,  949,
         975,  983,  971,  968,
         978, 1003, 1000,  994,
        1030,  997, 1012, 1010,
    ],
    [
         953,  944,  959,  948,
         941,  950,  970,  996,
         921,  908,  962,  965,
         932,  958,  961,  975,
         930,  938,  937,  959,
         923,  922,  934,  924,
         947,  906,  897,  924,
         915,  914,  919,  912,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
          19,   13,   32,   12,
          -9,   10,   44,   -1,
           5,   78,   32,   20,
         -17,    9,   27,    4,
         -34,    0,  -16,  -25,
           3,   20,  -29,  -58,
          43,   27,  -33,  -64,
           6,   24,  -56,  -85,
    ],
    [
         -50,  -13,   -9,   -6,
         -23,    0,   -5,   -5,
         -17,   -4,   -3,   -9,
         -15,   -8,   -9,  -11,
         -11,   -5,   -2,    1,
          -5,    2,   14,   20,
           1,   14,   30,   34,
          -2,   15,   40,   41,
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

const fn convert_square_relative_to(
    mg_eg: (
        [Score; PIECE_RELATIVE_TO_KING_LEN],
        [Score; PIECE_RELATIVE_TO_KING_LEN],
    ),
) -> [ScorePair; PIECE_RELATIVE_TO_KING_LEN] {
    let mg = mg_eg.0;
    let eg = mg_eg.1;
    let mut scores = [ScorePair(0, 0); PIECE_RELATIVE_TO_KING_LEN];
    let mut idx = 0;
    while idx < PIECE_RELATIVE_TO_KING_LEN {
        scores[idx] = ScorePair(mg[idx], eg[idx]);
        idx += 1;
    }
    scores
}

const fn convert_passed_pawn_relative_to(
    mg_eg: (
        [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
        [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
    ),
) -> [ScorePair; PASSED_PAWNS_RELATIVE_TO_KING_LEN] {
    let mg = mg_eg.0;
    let eg = mg_eg.1;
    let mut scores = [ScorePair(0, 0); PASSED_PAWNS_RELATIVE_TO_KING_LEN];
    let mut idx = 0;
    while idx < PASSED_PAWNS_RELATIVE_TO_KING_LEN {
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

pub const PAWN_RELATIVE_TO_FRIENDLY_KING: [ScorePair; PIECE_RELATIVE_TO_KING_LEN] =
    convert_square_relative_to(PAWN_RELATIVE_TO_FRIENDLY_KING_MG_EG);
pub const PAWN_RELATIVE_TO_ENEMY_KING: [ScorePair; PIECE_RELATIVE_TO_KING_LEN] =
    convert_square_relative_to(PAWN_RELATIVE_TO_ENEMY_KING_MG_EG);
pub const KNIGHT_RELATIVE_TO_FRIENDLY_KING: [ScorePair; PIECE_RELATIVE_TO_KING_LEN] =
    convert_square_relative_to(KNIGHT_RELATIVE_TO_FRIENDLY_KING_MG_EG);
pub const KNIGHT_RELATIVE_TO_ENEMY_KING: [ScorePair; PIECE_RELATIVE_TO_KING_LEN] =
    convert_square_relative_to(KNIGHT_RELATIVE_TO_ENEMY_KING_MG_EG);
pub const BISHOP_RELATIVE_TO_FRIENDLY_KING: [ScorePair; PIECE_RELATIVE_TO_KING_LEN] =
    convert_square_relative_to(BISHOP_RELATIVE_TO_FRIENDLY_KING_MG_EG);
pub const BISHOP_RELATIVE_TO_ENEMY_KING: [ScorePair; PIECE_RELATIVE_TO_KING_LEN] =
    convert_square_relative_to(BISHOP_RELATIVE_TO_ENEMY_KING_MG_EG);
pub const ROOK_RELATIVE_TO_FRIENDLY_KING: [ScorePair; PIECE_RELATIVE_TO_KING_LEN] =
    convert_square_relative_to(ROOK_RELATIVE_TO_FRIENDLY_KING_MG_EG);
pub const ROOK_RELATIVE_TO_ENEMY_KING: [ScorePair; PIECE_RELATIVE_TO_KING_LEN] =
    convert_square_relative_to(ROOK_RELATIVE_TO_ENEMY_KING_MG_EG);
pub const QUEEN_RELATIVE_TO_FRIENDLY_KING: [ScorePair; PIECE_RELATIVE_TO_KING_LEN] =
    convert_square_relative_to(QUEEN_RELATIVE_TO_FRIENDLY_KING_MG_EG);
pub const QUEEN_RELATIVE_TO_ENEMY_KING: [ScorePair; PIECE_RELATIVE_TO_KING_LEN] =
    convert_square_relative_to(QUEEN_RELATIVE_TO_ENEMY_KING_MG_EG);

pub const PASSED_PAWN_RELATIVE_TO_FRIENDLY_KING: [ScorePair; PASSED_PAWNS_RELATIVE_TO_KING_LEN] =
    convert_passed_pawn_relative_to(PASSED_PAWN_RELATIVE_TO_FRIENDLY_KING_MG_EG);
pub const PASSED_PAWN_RELATIVE_TO_ENEMY_KING: [ScorePair; PASSED_PAWNS_RELATIVE_TO_KING_LEN] =
    convert_passed_pawn_relative_to(PASSED_PAWN_RELATIVE_TO_ENEMY_KING_MG_EG);

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
