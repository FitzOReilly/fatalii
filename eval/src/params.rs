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
pub const TEMPO: ScorePair = ScorePair(26, 23);

pub const PASSED_PAWN: ScorePair = ScorePair(-6, 32);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-17, -6);
pub const BACKWARD_PAWN: ScorePair = ScorePair(-8, -7);

// Piece square tables:
// We only define values for the queenside (left side) and mirror them to the
// kingside (right side) so that we end up with symmetrical PSTs.
#[rustfmt::skip]
const PST_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
         130,  137,  137,  176,
          73,   79,  126,  105,
          74,   79,   84,  106,
          58,   71,   79,   98,
          65,   79,   86,   87,
          60,   82,   78,   70,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         247,  255,  239,  187,
         163,  146,  122,  131,
          89,   75,   81,   62,
          74,   67,   58,   54,
          67,   64,   60,   63,
          65,   62,   72,   75,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         185,  270,  261,  276,
         294,  313,  408,  356,
         315,  335,  365,  392,
         345,  330,  339,  347,
         310,  328,  350,  343,
         312,  334,  335,  343,
         309,  316,  325,  328,
         266,  301,  306,  293,
    ],
    [
         216,  260,  284,  281,
         260,  285,  261,  273,
         267,  283,  291,  284,
         268,  308,  316,  311,
         280,  293,  300,  302,
         259,  270,  282,  293,
         252,  272,  270,  287,
         267,  249,  268,  288,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         320,  304,  303,  275,
         359,  368,  361,  377,
         386,  377,  393,  405,
         366,  359,  363,  382,
         342,  361,  367,  381,
         372,  375,  383,  370,
         360,  385,  361,  364,
         359,  339,  343,  339,
    ],
    [
         288,  295,  290,  296,
         277,  291,  307,  295,
         281,  284,  287,  295,
         285,  304,  306,  321,
         284,  288,  301,  308,
         275,  276,  297,  306,
         287,  279,  295,  295,
         266,  297,  275,  302,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         475,  487,  513,  531,
         505,  463,  506,  515,
         505,  485,  481,  508,
         476,  487,  475,  487,
         461,  461,  446,  466,
         431,  489,  446,  462,
         421,  458,  440,  463,
         455,  453,  463,  478,
    ],
    [
         505,  510,  502,  508,
         496,  523,  513,  496,
         504,  495,  495,  492,
         496,  500,  502,  504,
         484,  489,  507,  503,
         487,  474,  502,  487,
         501,  494,  504,  488,
         473,  495,  489,  489,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         911,  919,  931,  920,
         916,  892,  959,  878,
         940,  936,  940,  972,
         931,  899,  922,  917,
         901,  925,  912,  905,
         905,  917,  916,  912,
         906,  929,  928,  917,
         893,  891,  899,  916,
    ],
    [
         925,  929,  945,  934,
         913,  937,  955,  982,
         913,  920,  959,  954,
         907,  945,  940,  963,
         912,  909,  941,  966,
         869,  901,  911,  934,
         907,  894,  873,  925,
         919,  902,  880,  870,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
          -4,    4,    0,    4,
           5,   -9,   19,   27,
          10,   22,   18,   18,
           3,   16,    2,    4,
         -20,   15,  -44,  -53,
         -19,  -17,  -26,  -84,
          24,   21,  -19,  -38,
          45,   51,   10,   16,
    ],
    [
         -26,  -17,  -13,   14,
          -4,   15,   18,   18,
           2,   13,   25,   17,
          -6,   11,   19,   15,
         -20,   -1,   24,   39,
         -18,    6,   18,   28,
         -28,   -6,    8,   18,
         -65,  -36,  -26,  -40,
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
