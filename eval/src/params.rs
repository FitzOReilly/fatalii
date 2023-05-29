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
pub const TEMPO: ScorePair = ScorePair(22, 26);

pub const PASSED_PAWN: ScorePair = ScorePair(-9, 19);

// Piece square tables:
// We only define values for the queenside (left side) and mirror them to the
// kingside (right side) so that we end up with symmetrical PSTs.
#[rustfmt::skip]
const PST_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
          88,  104,  128,  172,
          50,   79,  120,  104,
          67,   90,   80,  101,
          48,   76,   80,   97,
          55,   90,   75,   84,
          52,   96,   79,   74,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         288,  297,  234,  209,
         186,  161,  130,  151,
         109,   95,   90,   71,
          88,   87,   69,   67,
          83,   82,   71,   81,
          86,   81,   82,   69,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         164,  227,  290,  286,
         348,  361,  475,  420,
         353,  413,  415,  474,
         399,  380,  384,  401,
         366,  374,  393,  387,
         359,  379,  387,  397,
         366,  374,  373,  378,
         299,  359,  358,  358,
    ],
    [
         210,  238,  279,  275,
         237,  249,  244,  259,
         251,  254,  275,  267,
         240,  289,  305,  291,
         271,  278,  294,  292,
         243,  254,  263,  278,
         239,  257,  259,  270,
         244,  224,  267,  271,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         364,  373,  318,  296,
         412,  440,  400,  427,
         440,  445,  436,  471,
         409,  407,  415,  430,
         400,  412,  414,  430,
         417,  414,  430,  419,
         417,  432,  411,  410,
         411,  398,  397,  386,
    ],
    [
         278,  269,  292,  292,
         262,  280,  284,  284,
         275,  264,  293,  279,
         273,  287,  298,  308,
         275,  281,  299,  289,
         252,  279,  275,  290,
         277,  268,  271,  279,
         254,  281,  256,  274,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         554,  514,  596,  623,
         565,  531,  561,  565,
         536,  525,  507,  565,
         534,  525,  513,  524,
         528,  507,  495,  523,
         494,  521,  495,  518,
         453,  504,  496,  509,
         506,  502,  509,  525,
    ],
    [
         504,  520,  494,  485,
         496,  507,  509,  496,
         492,  500,  511,  484,
         498,  502,  498,  504,
         465,  474,  501,  493,
         474,  488,  493,  484,
         513,  498,  481,  496,
         464,  493,  482,  490,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         977,  977, 1008, 1024,
         963,  975,  999,  956,
        1012, 1020, 1009, 1037,
        1003,  971,  989,  980,
         968,  998,  975,  984,
         985,  985,  979,  983,
         956,  997,  995,  986,
         990,  970,  975,  991,
    ],
    [
        1003, 1005, 1020, 1007,
         976, 1002, 1048, 1064,
         980,  975, 1056, 1037,
         957, 1016, 1030, 1050,
         978,  994, 1018, 1017,
         949,  971,  996,  993,
        1000,  982,  965,  995,
         958,  980,  944,  916,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
          35,    8,   82,   46,
          -3,    1,   29,   41,
          29,   84,   42,   42,
         -20,   23,  -11,   15,
         -35,  -27,  -71,  -98,
         -15,  -21,  -68, -111,
          14,   14,  -32,  -59,
          28,   42,   -7,    5,
    ],
    [
         -44,  -14,  -19,  -15,
          -4,   16,   11,    7,
          -9,   21,   27,    6,
          -9,   11,   28,   27,
         -15,    3,   29,   38,
         -19,    7,   24,   38,
         -31,    2,   17,   21,
         -61,  -42,  -15,  -34,
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
