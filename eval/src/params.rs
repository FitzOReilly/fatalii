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
pub const TEMPO: ScorePair = ScorePair(31, 25);

pub const PASSED_PAWN: ScorePair = ScorePair(-7, 34);
pub const ISOLATED_PAWN: ScorePair = ScorePair(-23, -8);

// Piece square tables:
// We only define values for the queenside (left side) and mirror them to the
// kingside (right side) so that we end up with symmetrical PSTs.
#[rustfmt::skip]
const PST_PAWN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
           0,    0,    0,    0,
         122,  143,  166,  191,
          82,  108,  156,  132,
          94,   99,  100,  126,
          69,   84,   98,  117,
          77,   90,  102,  102,
          70,   96,   96,   88,
           0,    0,    0,    0,
    ],
    [
           0,    0,    0,    0,
         287,  287,  258,  192,
         174,  155,  125,  130,
         100,   88,   84,   64,
          81,   72,   63,   60,
          74,   69,   63,   63,
          80,   67,   74,   72,
           0,    0,    0,    0,
    ],
);
#[rustfmt::skip]
const PST_KNIGHT_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         176,  238,  288,  307,
         357,  379,  519,  440,
         382,  415,  446,  485,
         418,  407,  413,  423,
         374,  407,  426,  417,
         385,  407,  413,  422,
         384,  394,  403,  402,
         321,  373,  369,  355,
    ],
    [
         242,  260,  292,  287,
         246,  278,  237,  257,
         262,  258,  303,  269,
         256,  292,  309,  304,
         281,  278,  300,  295,
         254,  258,  275,  282,
         239,  254,  257,  276,
         241,  232,  261,  274,
    ],
);
#[rustfmt::skip]
const PST_BISHOP_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         402,  398,  357,  322,
         448,  461,  450,  466,
         486,  482,  473,  483,
         450,  440,  440,  465,
         420,  444,  451,  454,
         464,  456,  474,  452,
         441,  470,  445,  444,
         433,  428,  420,  421,
    ],
    [
         279,  299,  271,  294,
         249,  286,  289,  287,
         265,  274,  289,  286,
         283,  294,  300,  327,
         274,  278,  292,  307,
         253,  276,  276,  295,
         272,  267,  289,  285,
         254,  288,  267,  287,
    ],
);
#[rustfmt::skip]
const PST_ROOK_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         582,  559,  620,  644,
         590,  558,  620,  621,
         601,  566,  565,  596,
         567,  567,  568,  561,
         548,  539,  532,  549,
         514,  546,  532,  533,
         483,  546,  525,  548,
         543,  539,  551,  569,
    ],
    [
         529,  539,  518,  507,
         519,  529,  518,  513,
         504,  522,  512,  507,
         507,  506,  518,  530,
         502,  513,  527,  516,
         509,  501,  512,  504,
         522,  506,  509,  509,
         482,  508,  501,  501,
    ],
);
#[rustfmt::skip]
const PST_QUEEN_MG_EG: ([Score; 32], [Score; 32]) = (
    [
         994, 1005, 1025, 1059,
        1022,  990, 1032,  965,
        1043, 1033, 1029, 1076,
        1033,  993, 1016, 1007,
         990, 1024, 1018, 1002,
        1008, 1011, 1008, 1011,
         995, 1027, 1026, 1012,
        1000,  972,  993, 1015,
    ],
    [
        1036, 1030, 1042, 1022,
        1000, 1039, 1077, 1099,
         986, 1016, 1071, 1057,
        1001, 1043, 1055, 1078,
        1018, 1003, 1039, 1058,
         949, 1007, 1021, 1031,
        1019,  978,  982, 1034,
         989,  997,  978,  941,
    ],
);
#[rustfmt::skip]
const PST_KING_MG_EG: ([Score; 32], [Score; 32]) = (
    [
          51,   10,   81,   58,
         -10,  -15,   40,   56,
          27,  110,   59,   50,
         -10,   28,  -11,   -3,
         -55,  -17,  -59,  -97,
         -13,  -39,  -65, -115,
           9,   -1,  -42,  -68,
          23,   38,  -14,   -4,
    ],
    [
         -34,  -11,  -18,  -12,
           2,    0,   10,   10,
         -17,   10,   26,    8,
          -3,    9,   33,   22,
         -15,    1,   37,   44,
         -24,    5,   22,   39,
         -25,    0,   18,   23,
         -63,  -42,  -24,  -32,
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
