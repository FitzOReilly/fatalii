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
pub const TEMPO: ScorePair = ScorePair(25, 25);

// Piece square tables:
// We only define values for the queenside (left side) and mirror them to the
// kingside (right side) so that we end up with symmetrical PSTs.
#[rustfmt::skip]
const PST_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
          90,  122,  139,  171,
          53,  105,  131,  113,
          79,  102,   88,  110,
          51,   84,   85,  107,
          60,   98,   81,   93,
          56,  104,   89,   84,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         298,  302,  251,  229,
         204,  183,  140,  157,
         119,  108,   99,   81,
         102,   97,   82,   73,
          94,   91,   83,   87,
          95,   92,   96,   79,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         171,  233,  277,  288,
         346,  377,  483,  413,
         362,  410,  414,  476,
         394,  385,  390,  394,
         355,  374,  391,  388,
         350,  375,  383,  392,
         370,  368,  373,  371,
         279,  349,  352,  353,
    ],
    [
         212,  244,  267,  278,
         227,  248,  239,  260,
         245,  272,  279,  263,
         246,  283,  300,  297,
         263,  280,  290,  295,
         239,  259,  265,  282,
         228,  258,  255,  275,
         241,  227,  268,  268,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         369,  370,  323,  299,
         415,  444,  400,  426,
         447,  450,  454,  476,
         401,  404,  414,  429,
         404,  434,  416,  431,
         420,  419,  432,  423,
         413,  436,  411,  413,
         407,  392,  395,  389,
    ],
    [
         280,  266,  283,  288,
         264,  276,  284,  289,
         262,  267,  289,  277,
         280,  286,  292,  312,
         267,  273,  298,  289,
         253,  276,  275,  291,
         279,  264,  277,  276,
         255,  274,  254,  284,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         556,  521,  593,  611,
         558,  544,  574,  577,
         549,  532,  530,  568,
         542,  529,  518,  541,
         514,  521,  501,  534,
         510,  525,  514,  518,
         456,  529,  503,  517,
         515,  513,  519,  537,
    ],
    [
         509,  520,  493,  492,
         495,  515,  508,  497,
         496,  503,  513,  496,
         501,  499,  493,  508,
         479,  480,  514,  494,
         482,  494,  487,  483,
         513,  502,  486,  497,
         474,  485,  486,  490,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         986,  975, 1016, 1024,
         968,  967, 1002,  941,
        1021, 1019, 1011, 1046,
        1001,  964,  997,  980,
         962, 1008,  967,  980,
         986,  979,  978,  985,
         954,  992,  996,  986,
         980,  956,  963,  987,
    ],
    [
         997, 1003, 1021, 1010,
         976, 1010, 1041, 1072,
         967,  976, 1040, 1035,
         966, 1015, 1022, 1045,
         981,  989, 1024, 1020,
         946,  984, 1001,  994,
        1005,  980,  954, 1002,
         953,  981,  942,  926,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
          32,    8,   75,   34,
          -2,    0,   31,   40,
          27,   85,   41,   55,
         -14,   31,    1,    7,
         -39,  -25,  -66,  -86,
         -23,  -23,  -73, -110,
          14,   27,  -39,  -70,
          27,   48,  -14,    3,
    ],
    [
         -37,  -19,  -15,  -12,
           0,   14,   22,    8,
         -14,   26,   23,    8,
          -4,   12,   26,   22,
         -19,   -3,   31,   42,
         -11,   10,   23,   35,
         -32,  -14,   14,   24,
         -64,  -48,  -15,  -32,
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
