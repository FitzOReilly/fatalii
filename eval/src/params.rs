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
pub const TEMPO: ScorePair = ScorePair(29, 29);

#[rustfmt::skip]
const PASSED_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
         -30,  -13,  -18,  -27,
         -32,  -12,  -10,  -22,
         -21,  -46,  -40,  -49,
          -7,   -3,  -23,  -41,
           2,   13,  -13,  -36,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
           0,    0,    0,    0,
         129,  114,  106,   86,
          61,   61,   44,   49,
          26,   34,   20,   24,
          -2,   -2,   -6,   -7,
          -8,   -9,  -23,  -23,
           0,    0,    0,    0,
    ],
);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-16, -9);
pub const BACKWARD_PAWN: ScorePair = ScorePair(-11, -3);
pub const DOUBLED_PAWN: ScorePair = ScorePair(-8, -7);

pub const BISHOP_PAIR: ScorePair = ScorePair(30, 40);

const MOBILITY_KNIGHT_MG_EG: ([Score; KNIGHT_MOB_LEN], [Score; KNIGHT_MOB_LEN]) = (
    [19, 46, 56, 61, 68, 69, 70, 72, 70],
    [-19, -25, -30, -32, -32, -29, -31, -34, -33],
);
const MOBILITY_BISHOP_MG_EG: ([Score; BISHOP_MOB_LEN], [Score; BISHOP_MOB_LEN]) = (
    [22, 33, 44, 49, 54, 59, 64, 65, 69, 70, 79, 84, 95, 68],
    [-44, -30, -31, -21, -13, -4, -1, 5, 9, 12, 7, 9, 7, 19],
);
const MOBILITY_ROOK_MG_EG: ([Score; ROOK_MOB_LEN], [Score; ROOK_MOB_LEN]) = (
    [-3, 6, 8, 12, 15, 23, 27, 31, 36, 43, 49, 51, 61, 78, 85],
    [-39, -27, -16, -12, -8, -8, -8, -3, 2, 1, 4, 10, 12, 8, 0],
);
const MOBILITY_QUEEN_MG_EG: ([Score; QUEEN_MOB_LEN], [Score; QUEEN_MOB_LEN]) = (
    [
        36, 48, 52, 53, 60, 64, 69, 69, 67, 73, 76, 76, 77, 79, 84, 88, 92, 89, 99, 99, 92, 110,
        113, 126, 91, 74, 75, 58,
    ],
    [
        6, -6, -18, -8, -23, 4, -3, 4, 26, 34, 38, 48, 54, 60, 63, 57, 59, 75, 71, 70, 85, 69, 67,
        63, 74, 93, 89, 93,
    ],
);

const PAWN_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        0, 0, 0, 0, 0, 0, 0, 0, 2, -11, -3, -3, -1, -4, 1, -1, 4, -5, 8, -7, -3, -17, -12, 3, -24,
        -25, -29, -15, -12, -40, -26, -4, -25, -36, -35, -69, -36, -61, -82, -10, -4, -35, -30,
        -74, -61, -69, -62, -28, 19, -11, 9, -13, -11, -30, -40, -18, 0, 44, 16, 6, -2, -14, -28,
        -10, 65, 46, 15, 14, 0, -19, -5, 2, 64, 34, 29, 10, 9, -8, -9, 0, 43, 31, 16, 12, 14, -1,
        3, 8, 36, 26, 20, 22, 20, 18, 10, 22, 25, 31, 7, -1, 11, 9, -1, 4, -6, -16, -5, -11, 1, 8,
        42, -19, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 33, 14, 17, 8, 22, -15, 20, 6, 34, 32, 19, 4, 11, 24, 12, 13, 16,
        34, 5, 1, 3, 13, 22, 18, 14, 23, 14, 8, 8, 11, 27, 22, 11, 26, 15, 21, 15, 19, 22, 30, 9,
        13, 11, 12, 8, 12, 17, 19, 0, 9, 10, 4, 4, 4, 15, 6, -8, -8, 7, 4, 3, 11, 6, 8, -3, 1, 0,
        1, 0, 4, 4, 11, -12, -11, -11, -6, -8, 3, -2, 3, -20, -19, -24, -24, -10, -13, -7, -15, 9,
        -14, -17, -10, -11, -12, -4, 0, -15, -10, -17, -2, -6, -14, -11, 26, 0, 0, 0, 0, 0, 0, 0,
        0,
    ],
);
const PAWN_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 3, -3, -7, 0, -3, -2, 1, 3, 3, 4, -1, 11, -2, -2, 9, 25, 28,
        17, 3, 11, 5, 8, 19, 61, 33, 38, 51, 43, 23, 4, 67, 76, 72, 79, 60, 48, 30, 34, 49, 71,
        -19, 23, 47, 38, 28, 28, 0, 11, -61, 19, 26, 30, 32, 24, -8, -85, -29, 14, -24, 4, 9, -2,
        -33, -25, -18, -10, -8, 5, -2, 3, -10, -23, -22, -7, -13, -15, -11, -4, -9, -20, -21, -23,
        -18, -19, -27, -13, -12, -20, -25, -27, -21, -23, -25, -6, -17, -21, -26, -25, -23, -21,
        -33, -11, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, -13, -7, 2, -29, -41, -23, -24, -5, -44, -39, -39, -43, -43, -43,
        -50, -47, -39, -31, -36, -37, -22, -26, -43, -24, 2, -8, 11, 5, -6, -29, -22, -9, 26, 22,
        19, 14, 1, -13, -19, -11, 38, 30, 36, 16, 3, -13, -16, -21, 0, 41, 38, 17, -1, -8, -20,
        -23, 34, -3, 28, 11, 13, -1, -11, -5, 14, 16, 12, 10, 6, -6, -10, -18, 15, 13, 12, 6, 0,
        -3, -8, -27, 10, 11, 10, 8, 3, -5, -4, -21, 9, 7, 6, 7, 1, -5, -6, -24, 10, 5, 5, 3, 0, -5,
        -8, -28, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
);
const KNIGHT_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        -1, -1, -1, 0, 0, 0, 0, 0, -2, -4, -5, -4, -1, 0, -1, 0, -6, -2, -9, -5, -6, -2, -2, 0, -5,
        -6, -14, -9, -22, -4, -15, 0, -6, -29, -1, -21, -14, -10, -2, 0, -28, -1, 7, -11, -9, -42,
        1, -5, 41, 12, 34, 22, -7, 6, 14, 0, 0, 27, 19, 28, 2, 18, 17, 5, 48, 37, 29, 23, 16, 8,
        22, -25, 38, 35, 35, 25, 12, 17, 8, -23, 24, 26, 24, 20, 29, 28, 17, 3, -3, 18, 6, 18, 30,
        9, 22, -36, 27, -11, -3, -21, 16, 15, 26, 25, 13, 6, -7, 3, -48, 19, 3, 14, 26, 2, 26, -15,
        -46, -57, 18, 22,
    ],
    [
        -7, -4, -4, 1, 0, 0, 0, 0, -16, -31, -25, -9, -6, -4, -3, -1, -27, 3, -10, -18, -8, -6, 2,
        -2, -34, -27, -37, -34, -36, -17, -17, 3, -33, -26, -19, -34, -12, -24, -14, 1, -17, -13,
        -9, 3, -15, -24, -24, -12, -18, -9, -21, -13, -4, -24, -43, 4, 0, -8, -12, -16, -7, -23,
        -33, -33, -5, -13, -8, -7, -9, -13, -14, -10, 0, -10, -2, -2, -3, -3, 7, -1, 0, 2, 8, 0, 3,
        -3, 13, 19, 19, 22, 22, 30, 4, 22, 12, -8, 18, 32, 24, 41, 30, 10, 23, 19, 18, 22, 33, 39,
        42, 23, 30, 13, 33, -15, 11, 27, 25, 24, 29, 21,
    ],
);
const KNIGHT_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        0, -1, 0, 1, 0, 0, -1, 0, -1, 2, 5, 1, 1, 0, 1, 3, 13, 12, 9, 5, 8, 7, 1, -1, 4, 19, -1, 6,
        8, 3, 1, 0, 24, -3, -3, 1, 11, 16, -4, 2, -15, -13, 4, 39, -2, 16, 15, 5, 13, 6, -76, 10,
        -10, 26, 10, -2, 0, 22, -17, -5, -36, 12, 68, -14, -43, -19, -158, -21, 5, -5, 12, 12, -30,
        -124, -15, -7, -18, 9, 14, 15, -11, -23, -3, -17, 4, 5, -7, -18, -13, -1, -9, 5, -1, -5,
        -12, 35, -8, -6, -6, 0, -15, -2, -10, 34, -9, -17, -14, -23, -16, 7, -2, -21, -27, -10,
        -25, -18, -19, -33, -35, -18,
    ],
    [
        0, 0, 0, 7, 6, 1, 2, 0, -3, 15, 20, 9, 16, 0, 1, 6, 19, 39, 17, 37, 54, 34, 12, -2, 15, 61,
        18, 33, 48, 47, 6, 8, 32, 32, 49, -10, 35, 48, 3, 6, 18, 2, 46, 24, 14, 25, 40, 8, 48, 17,
        2, -6, 12, 13, 15, -1, 0, 26, -2, 5, -5, 0, -34, 9, 28, -4, -1, -15, -2, -1, 0, 14, -9,
        -36, -4, -12, -26, -15, -6, -11, -15, -32, -13, -51, -30, -22, -17, 20, -23, -20, -28, -24,
        -23, -17, -16, -29, -14, -19, -22, -28, -19, -22, -15, 6, -2, -19, -14, -8, -1, -8, -20,
        -5, -3, -27, -14, -29, -20, 7, 6, -3,
    ],
);
const BISHOP_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        -5, -4, -4, -1, -5, 1, 0, 0, -1, -4, -9, -2, -3, 1, 0, -1, -15, 5, -2, -7, 1, 0, 4, -7,
        -11, -25, -21, -32, -18, -1, -1, -1, -21, -34, -11, -20, -45, -24, 0, 0, 21, 3, -12, -19,
        24, -28, -10, 1, -6, 13, 7, 20, 15, 20, -40, -6, 0, 16, 32, 27, 33, 16, -2, -9, 48, 49, 35,
        29, 14, 16, 18, -2, 47, 26, 36, 16, 25, 22, 28, 13, 30, 35, 24, 30, 16, 28, 30, 12, 20, -3,
        31, 20, 25, 17, 9, -22, 31, 24, 14, 21, 0, 26, 12, -8, -12, 10, 0, 12, 13, 30, 62, 42, 16,
        19, -28, 4, 10, 26, 29, -12,
    ],
    [
        -17, -12, -2, -1, -5, 6, 0, 2, 18, -20, -15, -5, -19, 20, 0, -4, -11, -7, 1, -1, 0, -7, 17,
        -20, -23, -3, 0, 1, 2, 10, -3, -2, -25, -18, 1, -10, -7, -19, 0, 15, -17, -13, -18, -9,
        -18, -4, -4, 30, 3, -14, -6, -13, -10, -15, -10, -10, 0, -6, -14, -19, -30, -25, -2, -7,
        -9, -13, -6, -8, -2, -9, -14, 2, -12, -2, 1, 4, -10, 1, -7, -2, -1, -4, 0, -3, 2, -1, 0,
        -7, 0, 14, -4, 8, 8, 8, 7, 11, 7, 14, 13, 12, 16, 9, 3, 14, 34, 10, 25, 27, 10, 3, 15, 29,
        12, 20, 19, 8, 13, 6, 30, 13,
    ],
);
const BISHOP_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        1, 1, 0, 2, 2, 0, 0, 0, 3, 3, 7, 1, 2, 2, -1, 0, 7, 4, 1, -1, 3, -2, 3, 2, 12, 19, 0, 10,
        -5, 2, -2, 2, 26, 13, 26, -7, 9, 0, -1, 6, 17, 41, -7, 11, -23, 20, 5, 6, 13, -15, 46, 25,
        -22, 9, 59, -8, 0, -32, -37, 19, -2, 42, 1, 26, -43, -178, -8, -12, 5, 19, 29, 29, -35,
        -22, -94, -16, -17, 12, 5, -5, -30, -26, -15, -22, -11, -18, -3, -4, -13, -7, -24, -14,
        -32, -13, -31, 26, -7, -22, -12, -25, -19, -33, -17, -27, -23, -20, -24, -23, -34, -27,
        -39, -16, -19, -35, -30, -39, -36, -29, -49, -25,
    ],
    [
        10, 0, 0, 8, 11, 0, -1, 0, 22, 12, 4, -1, -4, 12, -5, 1, 25, -3, 11, 1, -2, -15, -8, 11, 4,
        29, -5, 27, -26, 3, -23, 9, 23, -6, 13, -9, 8, 4, 7, 26, 2, 21, -34, 17, 3, 9, 12, -2, 24,
        -29, 13, -9, 14, 1, 6, 3, 0, 37, 14, 12, -11, 4, 3, 4, 40, -9, 18, -8, 6, -7, -9, -9, 4,
        10, -36, 8, -8, 7, -13, 16, 17, -3, 6, -48, 2, -12, -6, 0, -16, 5, -8, 2, -21, 4, -9, -12,
        4, -10, 0, -6, -10, -23, -12, -12, -3, 4, -15, 2, -4, 6, -20, 17, -7, -13, 2, -12, 8, -10,
        15, -20,
    ],
);
const ROOK_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        -3, -4, -11, 5, 2, -4, -2, -2, -4, -11, -7, -11, -4, -1, -3, -3, -9, -4, -15, -10, -24, -3,
        -4, 1, -17, -17, -44, -43, -1, -41, -28, -21, -19, 1, -24, -14, -31, -15, -57, -9, -18,
        -11, -11, 0, -27, 2, -8, -28, -13, -1, -4, -17, -13, -22, -19, -15, 0, 19, 32, 41, 33, 36,
        28, 36, 91, 35, 29, 30, 50, 24, 38, -24, 54, 36, 33, 22, 36, 17, 13, -57, 27, 18, 35, 22,
        28, 23, 16, -6, 20, 29, 23, 56, 25, 25, 21, -19, 30, 30, 35, 14, 33, 25, 4, -16, 42, 23,
        -30, -37, -19, 2, -18, -12, -10, 41, 5, 28, 23, 12, 40, -5,
    ],
    [
        -10, -3, -33, -6, -5, -19, -13, -10, -20, -43, -46, -28, 8, -11, -1, -19, -41, -37, -38,
        -42, -41, -17, 2, -5, -47, -30, -23, -39, -46, -33, -19, -8, -41, -32, -15, -32, -6, -8,
        -4, 2, -28, -19, -20, -26, -16, -17, -6, 3, -24, -25, -15, -13, -11, -2, -4, 0, 0, -11,
        -20, -26, -15, -14, -12, -14, -31, -16, -2, -6, -12, -3, -9, 10, -6, -2, 5, 7, 2, 5, 7, 23,
        -4, 14, 8, 10, 12, 17, 20, 25, 8, 20, 21, 11, 16, 19, 17, 27, 16, 23, 22, 29, 29, 25, 34,
        37, 28, 31, 55, 55, 50, 47, 48, 44, 43, 42, 56, 49, 45, 47, 41, 3,
    ],
);
const ROOK_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        5, 19, 9, 5, 4, -1, -1, 0, 9, 19, 20, 15, 18, 8, 10, 2, 18, 6, -8, 18, 12, 17, 11, 1, 1, 4,
        14, 4, 27, 18, 9, 1, -6, 7, 12, 25, 23, 6, 16, 10, -30, -2, 45, 10, 18, 35, 49, 24, -62,
        -70, -9, -16, -11, 0, 9, 11, 0, -176, -62, -68, -42, -23, -7, 18, -188, -76, -21, -25, -4,
        20, 21, 28, -56, -29, -14, -9, 25, 19, 0, 13, -35, -16, -11, -3, 1, 8, 14, 51, -18, -24,
        -6, -10, 3, 13, 2, 33, -22, -15, -3, -5, 12, 2, 13, 15, -15, -24, -15, -5, 8, 4, -4, 22,
        -36, -38, -33, -31, -24, -30, -24, -3,
    ],
    [
        25, 65, 29, 20, 19, -2, -2, 2, 41, 86, 65, 54, 53, 39, 35, 9, 60, 74, 64, 65, 79, 69, 42,
        16, 44, 57, 52, 56, 58, 56, 47, 23, 21, 40, 41, 40, 42, 44, 27, 29, 14, 35, 25, 28, 29, 27,
        31, 24, -67, 65, 30, 30, 31, 30, 32, 28, 0, -34, 13, 0, 1, 2, 8, 1, -23, 36, -2, -7, -7,
        -15, -16, -7, -19, -3, -12, -20, -31, -29, -22, -17, -24, -15, -26, -27, -31, -29, -41,
        -30, -25, -22, -35, -36, -38, -47, -42, -45, -25, -34, -41, -46, -52, -52, -61, -48, -43,
        -39, -51, -58, -66, -61, -65, -86, -47, -42, -51, -57, -58, -57, -70, -69,
    ],
);
const QUEEN_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        4, -4, 2, 3, -1, 0, -4, 1, 2, 0, 1, 9, 4, -4, -3, 4, -2, -1, 6, 5, -11, 0, -4, 1, -2, -22,
        -1, -10, -6, -1, 5, -1, -8, 0, 3, -13, -16, -8, -8, 1, -6, -20, 32, -19, -10, -24, 8, 1,
        -2, -7, 6, 23, 35, -1, -10, -26, 0, 33, 19, 33, 22, 28, 7, 4, 52, 51, 44, 35, 29, 33, 42,
        9, 56, 48, 52, 40, 36, 35, 40, 40, 47, 43, 46, 31, 56, 62, 52, 6, 28, 39, 31, 45, 45, 58,
        44, 21, 5, 37, 24, 32, 46, 14, 38, 25, 64, 47, 17, 36, 51, 57, 62, 15, 41, 60, 35, -24, 16,
        43, 101, 6,
    ],
    [
        8, -8, 4, 3, -1, 0, -4, 2, 5, -1, -2, 16, 5, -7, -6, 5, 1, -4, 15, 10, -20, 4, -10, 0, 2,
        -26, 3, -9, 1, -2, 6, -3, -14, 8, 8, -21, -9, -1, -14, 0, 0, 31, -30, 14, 18, 2, 2, -15,
        -3, 8, 8, -7, -23, 13, -1, 3, 0, -7, 16, 7, 10, 1, 32, -20, -4, 9, -1, 16, 26, 23, -7, -15,
        2, 14, -4, 17, 9, 17, -9, 7, -12, 22, 17, 20, -8, -5, 5, 44, 12, 20, 27, 28, 24, -7, 22,
        27, 41, 40, 68, 29, 14, 46, 41, 5, 30, 30, 56, 57, 35, 29, 55, 41, 37, 27, 27, 78, 41, 25,
        43, 8,
    ],
);
const QUEEN_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PIECE_RELATIVE_TO_KING_LEN],
    [Score; PIECE_RELATIVE_TO_KING_LEN],
) = (
    [
        3, 22, 4, 10, 26, 8, 12, 6, 6, 21, 24, 17, 17, 25, 7, 7, 1, 8, 22, 0, 25, 2, 26, 4, -16,
        31, -4, 17, -3, 19, 16, 8, -27, -39, -9, -35, 29, 1, 14, 6, -45, -26, -58, -40, -4, -6, 7,
        -21, -29, -41, -11, -56, 9, 22, 43, 46, 0, -143, -134, -36, -11, 19, 67, 26, -415, -369,
        -58, -23, -1, 34, 43, 72, -102, -78, -109, -49, -13, 27, 32, 32, -81, -54, -54, -41, -32,
        -35, 12, -5, -55, -41, -42, -34, -18, -11, -3, 9, -35, -35, -21, -36, -25, -23, -22, -15,
        -38, -29, -23, -32, -22, -8, -31, -11, -44, -28, -32, -27, -31, -35, -14, -1,
    ],
    [
        5, 29, 10, 18, 39, 18, 21, 3, 11, 43, 32, 39, 38, 33, 15, 9, 18, 31, 31, 13, 48, 0, 36, 6,
        -7, 43, 1, 42, 8, 52, 26, 13, -27, 12, 13, -28, 39, 2, 22, 15, -66, -2, -64, 2, -2, -3, 15,
        -24, -37, -56, -14, 17, 9, -1, 27, 42, 0, -135, -69, -50, -50, -47, -48, 4, -199, -144,
        -21, -47, -18, -43, -5, 5, -90, -16, -104, -8, -45, -47, -51, -7, -40, -38, -22, -52, -10,
        -30, -50, -4, -27, -16, -23, -34, -50, -21, -53, -25, -3, -19, -45, -16, -35, 0, 14, -33,
        2, 3, -24, -2, -29, -30, 3, 9, 44, 32, -5, -13, 6, -5, 5, 10,
    ],
);

const PASSED_PAWN_RELATIVE_TO_FRIENDLY_KING_MG_EG: (
    [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
    [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
) = (
    [
        -4, -5, 3, -2, -3, 0, 0, 0, 7, 2, 6, 0, 5, -1, -1, 1, 8, -4, 9, -5, 12, -5, 3, -5, 12, 22,
        4, 14, 19, 12, -14, -15, 70, -19, -15, 16, 10, -7, -23, -10, 37, 16, -3, -20, -17, -15,
        -19, -38, 0, -17, -7, -45, -30, -56, -24, -35, -15, -27, -6, -29, -20, -7, -10, -37, -5,
        -14, -13, -15, -6, 11, 14, -13, 5, -4, -6, 12, 2, 23, 30, -11, -3, -31, -26, -1, -3, 15,
        52, 22, -34, -71, -15, 6, 23, 31, 94, 15, -6, -16, -5, -11, 1, 8, 42, -19,
    ],
    [
        2, 8, 10, -10, 5, 2, 2, -2, 24, 22, 13, 15, 5, 0, 4, -3, 56, 27, 32, 38, 38, 14, 17, -5,
        60, 48, 54, 24, 8, 16, 11, 11, 47, 61, 46, 18, 20, 6, -2, -15, 45, 57, 41, 28, 21, 6, 10,
        -16, 0, 56, 37, 26, 19, 24, 9, 1, 52, 48, 33, 25, 16, 3, 4, -2, 29, 29, 21, 16, 7, 0, -4,
        -7, 13, 17, 13, 1, 11, -1, -4, -9, 1, 28, 19, 11, 8, 1, 0, -7, -26, 24, 10, -7, -7, 4, -9,
        -13, -15, -10, -17, -2, -6, -14, -11, 26,
    ],
);
const PASSED_PAWN_RELATIVE_TO_ENEMY_KING_MG_EG: (
    [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
    [Score; PASSED_PAWNS_RELATIVE_TO_KING_LEN],
) = (
    [
        -4, 0, 3, -3, -7, 0, -3, -2, -2, 2, 4, 3, -1, 10, -4, -1, 4, 27, 24, 22, -2, 10, 10, 10,
        15, 53, 38, 39, 40, 49, 22, 10, 60, 86, 84, 79, 68, 77, 38, 28, 10, 84, 25, 36, 59, 60, 18,
        36, 0, -71, -31, 5, 46, 47, 56, 31, 21, -17, -48, -32, 32, 30, 17, 22, -13, -64, -41, -42,
        -3, -17, -12, 29, -39, -55, -45, -36, -32, -21, -30, 3, -44, -47, -42, -42, -24, -31, -32,
        21, -55, -51, -57, -22, -31, -11, -20, 30, -66, -68, -32, -22, -24, 6, -15, 21,
    ],
    [
        -13, -7, 2, -29, -41, -23, -24, -5, -52, -45, -38, -53, -38, -34, -52, -45, -52, -44, -49,
        -33, -26, -60, -40, -21, -69, -57, -62, -61, -66, -55, -35, -22, -61, -53, -63, -76, -69,
        -50, -33, -30, -4, -10, 5, -30, -48, -41, -22, -27, 0, 70, 39, -7, -22, -29, -35, -9, 70,
        73, 40, 9, -21, -32, -26, -40, 63, 55, 35, 20, -9, -13, -14, -27, 42, 38, 29, 22, 11, -7,
        -11, -18, 34, 31, 23, 19, 6, 1, -7, -29, 29, 23, 23, 4, -3, -1, -9, -19, 37, 31, 12, 4, -4,
        -18, -15, -2,
    ],
);

// Piece square tables:
// We only define values for the queenside (left side) and mirror them to the
// kingside (right side) so that we end up with symmetrical PSTs.
#[rustfmt::skip]
const PST_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
         111,  153,  152,  164,
          75,   87,  128,  102,
          60,   71,   69,   90,
          41,   60,   70,   83,
          57,   61,   68,   64,
          45,   78,   67,   53,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         289,  291,  276,  262,
         107,  113,  108,  124,
          97,   98,  103,   95,
          86,   86,   89,   88,
          71,   77,   85,   93,
          68,   70,   89,   97,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         185,  278,  255,  307,
         300,  294,  398,  325,
         296,  353,  349,  402,
         333,  342,  336,  355,
         310,  318,  340,  342,
         304,  326,  326,  339,
         307,  316,  316,  319,
         262,  291,  305,  301,
    ],
    [
         240,  266,  288,  290,
         268,  290,  275,  291,
         281,  284,  312,  297,
         285,  299,  318,  318,
         283,  300,  312,  318,
         279,  292,  301,  314,
         276,  287,  299,  300,
         278,  295,  300,  300,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         303,  309,  292,  252,
         296,  356,  337,  342,
         343,  342,  364,  338,
         328,  324,  331,  345,
         314,  326,  325,  349,
         339,  331,  344,  328,
         318,  349,  332,  321,
         331,  320,  311,  313,
    ],
    [
         291,  291,  292,  301,
         293,  293,  293,  286,
         298,  299,  299,  302,
         296,  300,  301,  305,
         296,  293,  305,  301,
         285,  297,  298,  306,
         296,  294,  295,  301,
         290,  303,  311,  311,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         540,  567,  560,  560,
         564,  539,  560,  550,
         537,  522,  519,  519,
         506,  502,  511,  499,
         497,  491,  487,  492,
         507,  505,  504,  515,
         473,  505,  496,  508,
         488,  481,  509,  508,
    ],
    [
         515,  507,  506,  504,
         500,  513,  507,  504,
         499,  503,  504,  504,
         495,  499,  503,  504,
         489,  496,  501,  504,
         480,  484,  488,  490,
         485,  484,  498,  492,
         480,  494,  490,  496,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         958,  968,  965,  987,
         983,  957,  946,  883,
         980, 1010,  947,  945,
         963,  930,  935,  927,
         960,  956,  952,  946,
         972,  980,  968,  966,
         977, 1001,  997,  991,
        1027,  994, 1009, 1008,
    ],
    [
         951,  941,  957,  945,
         937,  948,  965,  993,
         920,  909,  961,  963,
         930,  958,  960,  975,
         929,  941,  938,  961,
         924,  924,  937,  925,
         947,  907,  901,  927,
         917,  916,  922,  915,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
          15,    8,   33,    5,
         -13,    3,   35,   -1,
           0,   69,   23,   17,
         -13,   12,   18,   -7,
         -37,    3,  -22,  -29,
           3,   22,  -26,  -53,
          45,   32,  -20,  -50,
           8,   31,  -43,  -68,
    ],
    [
         -41,  -12,  -10,   -8,
         -20,    0,   -6,   -8,
         -15,   -6,   -4,  -11,
         -17,   -9,   -9,  -11,
          -7,   -5,   -2,   -3,
          -1,    3,   12,   15,
           6,   15,   26,   29,
           5,   17,   39,   38,
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
