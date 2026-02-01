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
pub const TEMPO: ScorePair = ScorePair(29, 30);

#[rustfmt::skip]
const PASSED_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
         -31,  -15,  -19,  -27,
         -34,  -15,  -12,  -22,
         -23,  -49,  -41,  -49,
          -7,   -5,  -24,  -42,
           2,   10,  -15,  -36,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
         130,  115,  106,   87,
          62,   62,   44,   49,
          27,   35,   21,   24,
          -2,   -1,   -5,   -7,
          -8,   -8,  -22,  -23,
           0,    0,    0,    0,
    ],
);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-13, -10);
pub const BACKWARD_PAWN: ScorePair = ScorePair(-10, -3);
pub const DOUBLED_PAWN: ScorePair = ScorePair(-8, -7);

pub const UNDEFENDED_KNIGHT_OUTPOST: ScorePair = ScorePair(13, -2);
pub const DEFENDED_KNIGHT_OUTPOST: ScorePair = ScorePair(24, 13);
pub const UNDEFENDED_BISHOP_OUTPOST: ScorePair = ScorePair(5, 3);
pub const DEFENDED_BISHOP_OUTPOST: ScorePair = ScorePair(35, 5);

pub const BISHOP_PAIR: ScorePair = ScorePair(30, 41);

const MOBILITY_KNIGHT_MG_EG: ([Score; KNIGHT_MOB_LEN], [Score; KNIGHT_MOB_LEN]) = (
    [16, 43, 53, 58, 65, 66, 67, 69, 67],
    [-19, -23, -28, -31, -31, -29, -31, -35, -35],
);
const MOBILITY_BISHOP_MG_EG: ([Score; BISHOP_MOB_LEN], [Score; BISHOP_MOB_LEN]) = (
    [21, 32, 43, 48, 53, 58, 63, 65, 69, 70, 79, 84, 94, 69],
    [-46, -32, -33, -23, -15, -6, -3, 4, 7, 11, 6, 10, 9, 21],
);
const MOBILITY_ROOK_MG_EG: ([Score; ROOK_MOB_LEN], [Score; ROOK_MOB_LEN]) = (
    [-1, 8, 9, 14, 16, 25, 29, 33, 38, 45, 51, 53, 62, 80, 88],
    [-38, -26, -15, -11, -7, -6, -7, -1, 3, 2, 6, 11, 14, 9, 2],
);
const MOBILITY_QUEEN_MG_EG: ([Score; QUEEN_MOB_LEN], [Score; QUEEN_MOB_LEN]) = (
    [
        38, 50, 53, 54, 61, 65, 70, 70, 68, 75, 78, 78, 78, 81, 85, 89, 94, 91, 101, 101, 95, 112,
        115, 128, 92, 76, 74, 58,
    ],
    [
        6, -6, -17, -5, -22, 6, 0, 7, 28, 36, 40, 50, 57, 63, 65, 59, 61, 77, 73, 72, 87, 72, 69,
        66, 77, 95, 90, 95,
    ],
);

const PAWN_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        0, 0, 0, 0, 0, 0, 0, 0, 2, -11, -3, -3, -1, -3, 1, -1, 4, -4, 8, -6, -3, -17, -12, 3, -25,
        -25, -30, -14, -12, -39, -26, -5, -23, -36, -36, -68, -36, -60, -81, -11, -4, -36, -31,
        -75, -61, -69, -61, -28, 19, -11, 10, -13, -12, -30, -42, -19, 0, 44, 16, 6, -3, -14, -28,
        -10, 66, 46, 15, 14, 0, -19, -5, 2, 65, 35, 29, 11, 8, -8, -9, -1, 43, 32, 16, 12, 14, 0,
        4, 8, 37, 26, 21, 22, 20, 18, 10, 23, 24, 30, 7, -1, 11, 9, 0, 5, -4, -14, -5, -11, 0, 7,
        43, -19, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 34, 14, 18, 7, 23, -13, 20, 7, 35, 31, 19, 4, 11, 25, 12, 13, 17,
        35, 6, 2, 4, 14, 22, 18, 14, 23, 14, 8, 8, 11, 27, 21, 11, 26, 15, 21, 15, 20, 22, 31, 10,
        14, 11, 12, 8, 12, 17, 19, 0, 10, 10, 4, 5, 4, 15, 6, -7, -8, 7, 4, 3, 11, 6, 8, -3, 1, 0,
        1, 0, 4, 4, 11, -12, -12, -11, -6, -8, 3, -2, 3, -20, -19, -25, -24, -11, -14, -7, -15, 8,
        -14, -17, -10, -12, -13, -4, 2, -16, -10, -17, -3, -5, -14, -12, 26, 0, 0, 0, 0, 0, 0, 0,
        0,
    ],
);
const PAWN_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 3, -3, -7, 0, -3, -2, 1, 3, 2, 3, -1, 10, -2, -1, 10, 26,
        28, 17, 4, 12, 6, 9, 20, 60, 36, 38, 51, 41, 25, 4, 68, 74, 72, 80, 60, 48, 30, 34, 49, 72,
        -20, 24, 46, 39, 27, 27, 0, 12, -60, 19, 27, 30, 34, 23, -7, -85, -29, 13, -26, 3, 8, -2,
        -33, -26, -18, -11, -9, 5, -2, 5, -10, -23, -22, -7, -13, -15, -11, -4, -9, -21, -21, -23,
        -18, -19, -27, -13, -13, -21, -26, -27, -21, -24, -25, -5, -18, -22, -26, -25, -23, -21,
        -33, -11, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, -13, -9, -2, -31, -40, -29, -32, -4, -42, -41, -40, -41, -44, -42,
        -50, -46, -40, -31, -36, -38, -22, -27, -44, -24, 1, -7, 11, 4, -6, -29, -22, -8, 26, 22,
        19, 14, 1, -13, -20, -10, 37, 30, 36, 16, 3, -13, -16, -20, 0, 41, 38, 18, -1, -8, -20,
        -22, 34, -2, 28, 11, 13, -1, -10, -5, 14, 17, 12, 10, 6, -6, -10, -19, 15, 14, 12, 6, 0,
        -3, -8, -27, 10, 11, 10, 8, 3, -4, -4, -21, 9, 7, 7, 7, 1, -5, -6, -25, 11, 5, 5, 3, 1, -5,
        -8, -28, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
);
const KNIGHT_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        -1, -1, -1, 0, 0, 0, 0, 0, -2, -5, -5, -4, -1, 0, -2, 0, -6, -2, -9, -5, -6, -2, -2, 0, -5,
        -6, -14, -8, -22, -4, -14, 0, -5, -28, -1, -20, -14, -9, -3, 0, -27, 1, 3, -13, -7, -40, 2,
        -5, 42, 11, 32, 21, -8, 7, 13, -1, 0, 26, 19, 27, 1, 17, 16, 3, 47, 36, 28, 21, 15, 8, 20,
        -25, 37, 35, 35, 24, 11, 16, 7, -25, 26, 27, 26, 21, 29, 28, 15, 0, 1, 20, 9, 21, 30, 10,
        21, -35, 30, -10, -1, -19, 16, 14, 28, 24, 10, 5, -8, 1, -49, 18, 2, 16, 27, -1, 23, -16,
        -52, -59, 16, 19,
    ],
    [
        -11, -4, -4, 0, 0, 0, -7, 0, -16, -30, -24, -10, -5, -4, -3, -1, -25, 3, -7, -21, -8, -6,
        2, -2, -35, -26, -36, -34, -39, -16, -15, -2, -32, -25, -19, -32, -10, -21, -13, 1, -17,
        -13, -8, 4, -14, -24, -23, -11, -18, -8, -20, -12, -4, -24, -41, 4, 0, -8, -12, -16, -7,
        -22, -32, -35, -5, -12, -8, -6, -9, -13, -15, -11, -1, -11, -3, -2, -3, -3, 6, 1, -1, 1, 8,
        -1, 2, -4, 11, 19, 18, 22, 22, 28, 3, 20, 12, -8, 18, 33, 24, 41, 32, 11, 22, 20, 20, 23,
        33, 40, 42, 23, 30, 13, 33, -14, 12, 29, 27, 24, 29, 19,
    ],
);
const KNIGHT_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        0, -1, 0, 1, 0, 0, -1, 0, 0, 2, 5, 1, 2, 0, 1, 3, 13, 13, 9, 5, 8, 6, 1, -1, 4, 19, 0, 6,
        9, 3, 1, 0, 23, -1, -4, 1, 10, 16, -4, 2, -14, -13, 4, 38, -2, 16, 14, 5, 13, 6, -73, 10,
        -8, 29, 11, -2, 0, 26, -14, -2, -33, 14, 69, -9, -42, -14, -154, -17, 9, -2, 18, 14, -28,
        -122, -12, -4, -13, 14, 19, 15, -13, -24, -4, -16, 6, 6, -6, -20, -16, -3, -11, 4, -2, -6,
        -14, 34, -10, -8, -8, -1, -17, -3, -12, 32, -11, -19, -16, -25, -18, 6, -3, -21, -29, -13,
        -28, -20, -21, -35, -36, -17,
    ],
    [
        0, 0, 0, 7, 7, 1, 2, 0, -2, 16, 20, 9, 16, 0, 1, 6, 19, 44, 18, 36, 54, 33, 12, -2, 14, 61,
        17, 33, 47, 48, 6, 8, 32, 31, 49, -9, 35, 49, 2, 6, 16, 1, 46, 24, 13, 25, 39, 7, 49, 17,
        0, -7, 11, 12, 13, -1, 0, 29, -3, 4, -6, -1, -34, 9, 31, -2, -3, -16, -3, -2, -2, 11, -10,
        -36, -5, -12, -27, -16, -8, -11, -16, -32, -13, -50, -30, -20, -18, 20, -23, -20, -28, -23,
        -22, -15, -15, -29, -14, -20, -22, -27, -18, -21, -13, 6, -2, -18, -14, -7, 0, -7, -20, -6,
        -2, -26, -14, -28, -19, 8, 6, -3,
    ],
);
const BISHOP_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        -5, -5, -4, -1, -5, 1, 0, 0, 0, -4, -9, -2, -3, 1, 0, -1, -14, 4, -2, -7, 1, 0, 4, -7, -12,
        -25, -22, -31, -18, -1, -2, -1, -21, -33, -11, -19, -46, -23, -1, -1, 20, 2, -10, -18, 22,
        -27, -12, 1, -6, 13, 7, 21, 14, 20, -39, -5, 0, 16, 33, 27, 33, 16, -2, -10, 47, 48, 34,
        29, 13, 15, 17, -1, 47, 25, 36, 16, 25, 20, 27, 13, 30, 35, 24, 30, 16, 27, 29, 10, 22, -2,
        32, 21, 26, 16, 9, -23, 34, 25, 15, 21, 1, 27, 12, -5, -11, 9, 0, 11, 14, 30, 63, 44, 15,
        17, -30, 1, 9, 24, 27, -12,
    ],
    [
        -17, -11, -1, -1, -5, 6, 0, 2, 16, -22, -18, -6, -18, 22, 0, -4, -11, -9, 0, 0, -3, -6, 12,
        -20, -24, -4, 0, 1, 3, 9, -3, -4, -26, -19, 0, -11, -8, -21, -1, 15, -18, -13, -18, -9,
        -18, -3, -4, 31, 3, -14, -7, -14, -10, -15, -11, -10, 0, -5, -14, -19, -31, -25, -3, -7,
        -9, -12, -6, -8, -2, -8, -15, 0, -12, -2, 2, 4, -10, 1, -7, -4, -1, -3, 1, -2, 2, -1, -1,
        -6, 1, 15, -3, 8, 9, 8, 7, 13, 7, 14, 13, 13, 16, 10, 4, 14, 35, 11, 25, 27, 10, 3, 15, 28,
        10, 21, 20, 9, 13, 6, 30, 14,
    ],
);
const BISHOP_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        1, 1, 0, 2, 2, 0, 0, 0, 3, 3, 7, 1, 2, 2, -1, 0, 7, 4, 1, -2, 2, -2, 3, 3, 11, 19, 0, 10,
        -5, 2, -2, 2, 25, 12, 26, -8, 10, 0, -1, 6, 17, 42, -7, 13, -23, 21, 5, 7, 14, -15, 48, 26,
        -21, 10, 60, -8, 0, -30, -36, 19, -2, 43, 2, 26, -40, -177, -5, -10, 7, 21, 32, 32, -36,
        -20, -95, -15, -17, 16, 6, -4, -31, -28, -17, -23, -11, -17, -2, -3, -15, -9, -26, -14,
        -33, -14, -32, 24, -9, -23, -14, -26, -20, -34, -19, -26, -23, -20, -25, -24, -35, -28,
        -40, -16, -19, -35, -31, -40, -36, -30, -51, -24,
    ],
    [
        10, 0, 1, 8, 11, 1, -1, 0, 22, 12, 4, -1, -4, 11, -5, 0, 27, -3, 10, 0, 2, -10, -8, 12, 7,
        29, -6, 28, -27, 6, -22, 9, 22, -5, 13, -9, 9, 4, 7, 26, 3, 21, -33, 17, 3, 10, 11, 0, 26,
        -30, 13, -9, 14, 0, 6, 6, 0, 39, 15, 12, -11, 5, 3, 4, 41, -10, 18, -7, 6, -7, -10, -10, 5,
        10, -38, 8, -8, 6, -13, 15, 16, -4, 6, -48, 2, -12, -6, 0, -17, 4, -9, 2, -22, 4, -9, -9,
        4, -11, -1, -6, -10, -24, -12, -11, -4, 4, -16, 1, -4, 6, -19, 16, -9, -14, 1, -12, 8, -11,
        16, -20,
    ],
);
const ROOK_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        -3, -4, -11, 5, 2, -4, -1, -2, -4, -10, -7, -11, -4, 0, -3, -3, -8, -4, -15, -10, -23, -3,
        -4, 1, -16, -17, -43, -43, -1, -41, -29, -21, -21, 2, -24, -15, -31, -12, -56, -10, -18,
        -10, -10, -1, -26, 1, -9, -26, -11, 0, -3, -16, -13, -21, -18, -15, 0, 19, 33, 42, 34, 37,
        29, 37, 91, 35, 29, 30, 51, 24, 39, -24, 55, 36, 32, 22, 37, 17, 13, -54, 28, 17, 35, 20,
        27, 23, 15, -6, 18, 30, 23, 56, 26, 26, 22, -18, 32, 30, 36, 14, 34, 25, 4, -16, 43, 24,
        -30, -36, -19, 2, -19, -12, -11, 41, 6, 28, 23, 11, 39, -4,
    ],
    [
        -9, -3, -32, -5, -4, -18, -12, -10, -20, -41, -46, -29, 9, -11, -1, -19, -40, -37, -37,
        -41, -40, -16, 3, -6, -47, -30, -23, -38, -45, -33, -18, -7, -40, -31, -15, -31, -5, -8,
        -3, 2, -28, -19, -20, -25, -16, -16, -5, 3, -25, -25, -15, -13, -10, -2, -4, 1, 0, -11,
        -20, -26, -15, -14, -12, -14, -31, -16, -2, -6, -12, -3, -9, 10, -6, -2, 5, 7, 3, 6, 7, 22,
        -5, 14, 8, 10, 12, 17, 20, 25, 8, 19, 21, 11, 16, 18, 17, 27, 16, 23, 22, 29, 28, 25, 33,
        37, 28, 30, 54, 55, 50, 47, 48, 44, 42, 42, 55, 48, 44, 46, 41, 4,
    ],
);
const ROOK_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        5, 19, 9, 4, 4, -1, -1, 0, 10, 18, 21, 14, 19, 9, 10, 2, 18, 6, -8, 18, 13, 17, 11, 1, 2,
        4, 13, 4, 27, 17, 9, 1, -8, 5, 12, 24, 22, 6, 15, 11, -30, -3, 43, 7, 17, 36, 48, 23, -63,
        -69, -9, -16, -12, -1, 10, 10, 0, -175, -62, -69, -41, -25, -9, 18, -188, -77, -20, -25,
        -5, 21, 20, 28, -57, -30, -14, -9, 25, 19, -1, 12, -34, -17, -12, -4, 1, 8, 14, 55, -18,
        -24, -6, -10, 3, 12, 2, 32, -23, -14, -3, -5, 13, 2, 14, 13, -15, -25, -16, -5, 8, 3, -3,
        22, -37, -39, -34, -31, -25, -30, -25, -4,
    ],
    [
        25, 65, 28, 19, 18, -1, -3, 2, 42, 86, 66, 54, 54, 40, 35, 9, 59, 74, 64, 65, 78, 69, 42,
        16, 44, 57, 52, 56, 57, 56, 47, 23, 21, 40, 41, 40, 42, 44, 27, 30, 14, 35, 25, 28, 29, 27,
        31, 24, -68, 64, 29, 29, 31, 30, 32, 29, 0, -35, 12, 0, 0, 2, 8, 0, -25, 36, -3, -8, -7,
        -15, -16, -8, -19, -3, -12, -21, -31, -29, -22, -17, -24, -15, -26, -27, -32, -29, -41,
        -31, -26, -22, -35, -36, -38, -47, -42, -46, -25, -34, -42, -46, -53, -52, -62, -47, -43,
        -39, -51, -58, -66, -61, -65, -86, -47, -42, -52, -57, -58, -57, -70, -69,
    ],
);
const QUEEN_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        4, -4, 2, 2, -1, 0, -4, 1, 2, -1, 1, 9, 4, -4, -3, 4, -3, -1, 7, 5, -11, 0, -4, 1, -1, -22,
        -1, -10, -6, -1, 6, -1, -6, 1, 4, -13, -16, -7, -7, 0, -8, -21, 32, -20, -9, -24, 9, 0, -1,
        -6, 8, 22, 35, -1, -12, -26, 0, 33, 20, 33, 22, 28, 7, 4, 53, 52, 45, 35, 30, 33, 41, 9,
        57, 49, 53, 41, 37, 35, 40, 41, 47, 43, 47, 33, 56, 63, 53, 7, 28, 39, 32, 46, 45, 59, 45,
        23, 7, 39, 23, 33, 46, 13, 38, 27, 63, 48, 17, 36, 53, 57, 64, 16, 43, 63, 34, -27, 14, 42,
        102, 6,
    ],
    [
        8, -7, 4, 3, -1, 0, -5, 2, 5, -1, -1, 17, 5, -7, -6, 5, 1, -4, 16, 9, -19, 4, -10, 0, 3,
        -27, 4, -9, 1, -1, 6, -3, -13, 8, 10, -20, -10, 0, -13, 0, 1, 32, -29, 15, 19, 3, 3, -16,
        -5, 8, 9, -5, -23, 13, 1, 4, 0, -6, 16, 8, 11, 1, 34, -21, -3, 10, -1, 16, 27, 23, -6, -15,
        3, 14, -2, 17, 10, 18, -8, 8, -11, 23, 17, 20, -7, -5, 5, 46, 13, 21, 27, 28, 25, -7, 22,
        26, 41, 41, 71, 30, 15, 47, 41, 5, 31, 30, 57, 59, 35, 31, 56, 41, 39, 27, 27, 79, 42, 26,
        43, 9,
    ],
);
const QUEEN_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        3, 22, 4, 9, 26, 8, 12, 6, 5, 21, 23, 18, 15, 25, 7, 7, 0, 8, 23, 0, 26, 2, 26, 4, -15, 31,
        -4, 16, -2, 19, 17, 8, -27, -41, -8, -35, 28, 2, 14, 6, -46, -28, -58, -40, -4, -5, 7, -21,
        -29, -40, -9, -59, 9, 22, 42, 47, 0, -144, -134, -39, -12, 18, 69, 25, -415, -372, -58,
        -23, -1, 35, 44, 72, -102, -78, -109, -49, -13, 26, 31, 31, -82, -54, -54, -41, -33, -35,
        13, -4, -56, -41, -43, -34, -18, -12, -3, 10, -35, -35, -21, -36, -26, -24, -22, -14, -39,
        -30, -24, -33, -23, -9, -33, -11, -45, -29, -33, -29, -33, -37, -16, 1,
    ],
    [
        5, 29, 10, 18, 38, 18, 21, 3, 10, 43, 32, 40, 36, 33, 14, 9, 17, 31, 32, 14, 48, 1, 35, 6,
        -7, 43, 2, 42, 9, 53, 26, 12, -28, 12, 12, -29, 39, 2, 21, 14, -67, -2, -64, 1, -3, -3, 15,
        -24, -37, -56, -16, 18, 8, -2, 28, 42, 0, -135, -70, -50, -51, -47, -49, 4, -199, -146,
        -22, -49, -19, -44, -7, 4, -90, -17, -105, -9, -47, -47, -52, -8, -41, -39, -23, -52, -10,
        -31, -52, -5, -28, -17, -24, -35, -51, -21, -53, -24, -4, -20, -46, -18, -37, -1, 13, -33,
        1, 2, -24, -4, -30, -31, 3, 9, 45, 32, -6, -13, 6, -5, 5, 10,
    ],
);

const PASSED_PAWN_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
    [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
) = (
    [
        -4, -5, 3, -2, -3, 0, 0, 0, 7, 2, 7, 1, 5, -1, -1, 1, 8, -5, 9, -5, 12, -4, 3, -4, 12, 20,
        3, 13, 19, 12, -13, -15, 71, -19, -15, 16, 9, -8, -23, -10, 36, 15, -6, -22, -18, -18, -19,
        -38, 0, -16, -7, -47, -33, -56, -24, -34, -15, -28, -6, -29, -21, -7, -10, -36, -6, -16,
        -13, -16, -7, 9, 13, -13, 5, -4, -7, 11, 1, 21, 29, -14, -3, -31, -26, -1, -2, 15, 52, 21,
        -35, -70, -13, 5, 22, 32, 93, 13, -4, -14, -5, -11, 0, 7, 43, -19,
    ],
    [
        2, 9, 10, -10, 7, 2, 2, -2, 24, 22, 13, 14, 5, 0, 4, -3, 56, 27, 32, 37, 38, 14, 18, -4,
        60, 49, 54, 24, 9, 16, 11, 12, 48, 61, 46, 18, 21, 5, -2, -16, 45, 57, 41, 28, 21, 7, 10,
        -15, 0, 56, 38, 27, 20, 24, 9, 1, 52, 48, 33, 25, 17, 3, 4, -2, 30, 30, 21, 17, 8, 0, -4,
        -8, 13, 18, 14, 1, 10, 0, -3, -8, 0, 29, 19, 11, 8, 2, 0, -6, -25, 23, 11, -7, -5, 4, -9,
        -13, -16, -10, -17, -3, -5, -14, -12, 26,
    ],
);
const PASSED_PAWN_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
    [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
) = (
    [
        -4, 0, 3, -3, -7, 0, -3, -2, -2, 2, 3, 3, -1, 10, -3, -1, 5, 27, 24, 23, -2, 10, 11, 10,
        17, 53, 41, 39, 40, 48, 24, 10, 60, 85, 85, 80, 68, 77, 39, 27, 11, 85, 26, 36, 58, 60, 18,
        36, 0, -72, -33, 6, 47, 47, 54, 32, 20, -17, -46, -32, 34, 30, 19, 20, -14, -65, -40, -41,
        -3, -16, -10, 29, -38, -56, -45, -36, -32, -21, -30, 3, -45, -46, -41, -41, -23, -30, -31,
        22, -54, -50, -56, -22, -32, -11, -18, 29, -64, -67, -30, -21, -23, 7, -12, 22,
    ],
    [
        -13, -9, -2, -31, -40, -29, -32, -4, -51, -47, -39, -52, -39, -33, -52, -44, -54, -46, -49,
        -35, -28, -61, -41, -20, -70, -57, -64, -60, -65, -53, -35, -22, -62, -52, -64, -77, -68,
        -50, -32, -30, -4, -10, 5, -30, -48, -41, -21, -27, 0, 71, 40, -7, -22, -28, -34, -9, 71,
        74, 40, 9, -21, -31, -27, -39, 63, 56, 36, 20, -9, -13, -15, -26, 42, 38, 29, 23, 11, -7,
        -11, -18, 34, 32, 23, 18, 6, 1, -7, -29, 29, 23, 23, 4, -2, -1, -9, -18, 37, 31, 12, 4, -4,
        -17, -16, -3,
    ],
);

// Piece square tables:
// We only define values for the queenside (left side) and mirror them to the
// kingside (right side) so that we end up with symmetrical PSTs.
#[rustfmt::skip]
const PST_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
         106,  153,  151,  164,
          74,   90,  128,  102,
          60,   73,   70,   90,
          41,   61,   71,   83,
          57,   62,   67,   63,
          45,   79,   66,   51,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         292,  293,  278,  263,
         108,  112,  109,  124,
          97,   98,  103,   95,
          86,   86,   89,   88,
          71,   77,   85,   94,
          69,   69,   89,   97,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         179,  275,  252,  303,
         297,  292,  395,  322,
         289,  347,  346,  395,
         330,  342,  336,  352,
         310,  319,  341,  341,
         306,  329,  329,  341,
         309,  319,  319,  321,
         265,  293,  308,  303,
    ],
    [
         240,  266,  289,  291,
         268,  290,  277,  292,
         282,  284,  313,  299,
         286,  297,  316,  318,
         281,  300,  312,  318,
         278,  293,  302,  315,
         275,  287,  300,  301,
         276,  295,  299,  300,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         300,  307,  291,  251,
         296,  353,  337,  340,
         341,  339,  360,  332,
         327,  324,  330,  340,
         315,  327,  326,  348,
         340,  333,  346,  329,
         319,  351,  334,  323,
         333,  323,  313,  315,
    ],
    [
         291,  292,  292,  301,
         293,  292,  292,  285,
         297,  298,  297,  300,
         295,  298,  298,  302,
         296,  292,  304,  298,
         285,  297,  297,  305,
         297,  295,  296,  301,
         291,  305,  312,  312,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         541,  567,  563,  559,
         565,  540,  560,  550,
         537,  524,  520,  520,
         507,  503,  511,  499,
         500,  491,  489,  494,
         508,  507,  505,  516,
         474,  506,  496,  510,
         488,  482,  509,  509,
    ],
    [
         517,  508,  506,  505,
         501,  514,  508,  505,
         500,  504,  504,  504,
         496,  499,  504,  505,
         489,  496,  501,  505,
         480,  484,  488,  490,
         486,  484,  498,  493,
         480,  494,  491,  496,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         959,  967,  966,  989,
         985,  959,  948,  884,
         981, 1012,  949,  945,
         965,  931,  936,  928,
         961,  957,  953,  948,
         974,  982,  970,  967,
         978, 1003,  999,  992,
        1029,  995, 1010, 1009,
    ],
    [
         954,  944,  959,  947,
         939,  950,  966,  995,
         922,  912,  963,  965,
         932,  961,  962,  976,
         931,  943,  939,  963,
         925,  926,  938,  927,
         948,  908,  902,  929,
         919,  918,  923,  917,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
          16,   10,   33,    4,
         -13,    2,   35,   -2,
           2,   70,   23,   19,
         -14,   11,   17,   -7,
         -38,    3,  -22,  -28,
           1,   22,  -27,  -53,
          45,   32,  -20,  -50,
           8,   31,  -43,  -68,
    ],
    [
         -43,  -14,  -11,   -8,
         -20,    0,   -6,   -7,
         -15,   -7,   -4,  -12,
         -18,   -9,   -8,  -11,
          -7,   -5,   -2,   -3,
           0,    3,   13,   15,
           6,   15,   27,   29,
           6,   18,   39,   38,
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
