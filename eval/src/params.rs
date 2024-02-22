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
pub const TEMPO: ScorePair = ScorePair(31, 22);

pub const PASSED_PAWN: ScorePair = ScorePair(-12, 32);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-16, -6);
pub const BACKWARD_PAWN: ScorePair = ScorePair(-13, -2);
pub const DOUBLED_PAWN: ScorePair = ScorePair(-13, -4);

pub const BISHOP_PAIR: ScorePair = ScorePair(44, 22);
pub const ROOK_ON_OPEN_FILE: ScorePair = ScorePair(41, -5);
pub const ROOK_ON_SEMI_OPEN_FILE: ScorePair = ScorePair(13, 1);

pub const KNIGHT_MOB_LEN: usize = 9;
pub const BISHOP_MOB_LEN: usize = 14;
pub const ROOK_MOB_LEN: usize = 15;
pub const QUEEN_MOB_LEN: usize = 28;
pub const MOB_LEN: usize = KNIGHT_MOB_LEN + BISHOP_MOB_LEN + ROOK_MOB_LEN + QUEEN_MOB_LEN;

const MOBILITY_KNIGHT_MG_EG: ([Score; KNIGHT_MOB_LEN], [Score; KNIGHT_MOB_LEN]) = (
    [-14, 69, 84, 90, 105, 100, 96, 95, 98],
    [4, -20, -35, -42, -40, -39, -30, -31, -35],
);
const MOBILITY_BISHOP_MG_EG: ([Score; BISHOP_MOB_LEN], [Score; BISHOP_MOB_LEN]) = (
    [53, 63, 75, 77, 84, 92, 92, 94, 108, 106, 108, 116, 97, 63],
    [
        -40, -43, -41, -37, -35, -28, -19, -12, -17, -13, -14, -3, 0, 2,
    ],
);
const MOBILITY_ROOK_MG_EG: ([Score; ROOK_MOB_LEN], [Score; ROOK_MOB_LEN]) = (
    [30, 38, 36, 39, 39, 45, 49, 57, 51, 56, 47, 51, 61, 69, 48],
    [
        -40, -36, -17, -25, -12, -10, -12, -17, -4, -8, -5, -3, 3, 8, 7,
    ],
);
const MOBILITY_QUEEN_MG_EG: ([Score; QUEEN_MOB_LEN], [Score; QUEEN_MOB_LEN]) = (
    [
        -2, -4, 24, 19, 33, 37, 43, 49, 48, 49, 64, 55, 58, 63, 61, 68, 63, 66, 64, 70, 66, 104,
        92, 126, 68, 70, 35, 23,
    ],
    [
        -6, -12, -3, 19, -44, -16, -10, 0, 34, 58, 44, 78, 71, 72, 81, 80, 92, 117, 116, 107, 104,
        80, 64, 69, 72, 82, 50, 37,
    ],
);

// Piece square tables:
// We only define values for the queenside (left side) and mirror them to the
// kingside (right side) so that we end up with symmetrical PSTs.
#[rustfmt::skip]
const PST_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
         143,  156,  167,  215,
          99,  120,  162,  145,
         116,  117,  118,  150,
          85,   99,  122,  138,
         106,  114,  123,  135,
          96,  123,  122,  126,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         277,  269,  245,  205,
         178,  147,  124,  133,
          91,   79,   79,   52,
          75,   68,   61,   52,
          68,   62,   56,   55,
          68,   58,   66,   58,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         177,  274,  225,  271,
         289,  287,  405,  375,
         307,  341,  394,  407,
         331,  331,  352,  356,
         299,  338,  358,  351,
         307,  335,  341,  362,
         327,  300,  329,  343,
         285,  311,  308,  302,
    ],
    [
         220,  283,  310,  296,
         282,  291,  265,  281,
         273,  294,  321,  295,
         301,  314,  332,  320,
         304,  298,  310,  317,
         280,  287,  290,  302,
         272,  285,  286,  290,
         275,  269,  289,  299,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         289,  290,  268,  233,
         361,  325,  325,  315,
         370,  375,  361,  376,
         327,  334,  312,  358,
         331,  343,  335,  360,
         359,  361,  370,  353,
         356,  377,  355,  353,
         345,  330,  335,  347,
    ],
    [
         294,  297,  276,  291,
         267,  298,  303,  289,
         285,  285,  288,  291,
         289,  292,  311,  309,
         293,  285,  300,  303,
         286,  283,  282,  292,
         275,  279,  280,  287,
         287,  307,  292,  303,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         507,  502,  542,  528,
         524,  504,  548,  552,
         541,  546,  519,  528,
         517,  522,  528,  528,
         499,  516,  499,  515,
         502,  513,  512,  531,
         464,  531,  518,  536,
         528,  530,  542,  546,
    ],
    [
         516,  525,  490,  496,
         503,  516,  516,  499,
         502,  503,  494,  498,
         486,  491,  493,  500,
         485,  500,  511,  494,
         481,  479,  478,  481,
         517,  489,  491,  489,
         467,  482,  479,  478,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         951,  925,  964,  950,
         941,  905,  954,  881,
         985,  977,  966,  960,
         960,  929,  920,  924,
         924,  934,  947,  928,
         952,  949,  943,  954,
         956,  974,  970,  962,
         943,  964,  954,  966,
    ],
    [
         935,  941,  967,  958,
         942,  972,  979, 1016,
         929,  914,  965,  996,
         918,  952,  978,  993,
         949,  960,  952, 1004,
         884,  925,  954,  945,
         938,  906,  908,  940,
         911,  892,  908,  904,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
          20,    6,   17,   17,
           3,    2,   42,   32,
          26,   30,   16,   39,
           7,   22,  -11,    4,
         -37,  -20,  -41,  -75,
          -8,  -16,  -52,  -85,
          27,   18,  -30,  -60,
          39,   59,   -1,    5,
    ],
    [
         -29,  -15,  -12,  -15,
         -15,    8,    7,   25,
          -3,   25,   16,   14,
          -7,   14,   24,   22,
         -19,    5,   29,   36,
         -16,    6,   23,   34,
         -28,   -4,    8,   25,
         -68,  -45,  -14,  -29,
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
