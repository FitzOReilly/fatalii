use crate::eval::{Eval, Score};
use movegen::bitboard::Bitboard;
use movegen::file::File;
use movegen::piece;
use movegen::position::Position;
use movegen::rank::Rank;
use movegen::side::Side;
use movegen::square::Square;
use std::cmp;

type PieceSquareTable = [Score; 64];

// (opening, endgame)
const KING_WEIGHT: (Score, Score) = (0, 0);
const QUEEN_WEIGHT: (Score, Score) = (900, 910);
const ROOK_WEIGHT: (Score, Score) = (500, 520);
const BISHOP_WEIGHT: (Score, Score) = (330, 310);
const KNIGHT_WEIGHT: (Score, Score) = (320, 300);
const PAWN_WEIGHT: (Score, Score) = (100, 120);

// Interpolate between the opening and endgame score if the material is between
// these 2 values. Otherwise use only one of the scores.
const INTERPOLATE_MAX_MATERIAL: Score = 2
    * (KING_WEIGHT.0
        + QUEEN_WEIGHT.0
        + 2 * ROOK_WEIGHT.0
        + 2 * BISHOP_WEIGHT.0
        + 2 * KNIGHT_WEIGHT.0
        + 8 * PAWN_WEIGHT.0);
const INTERPOLATE_MIN_MATERIAL: Score = 2 * KING_WEIGHT.0;

pub struct PieceSquareTables;

impl Eval for PieceSquareTables {
    fn eval(pos: &Position) -> Score {
        let mut opening_score = 0;
        let mut endgame_score = 0;
        let mut total_material = PAWN_WEIGHT.0
            * pos.piece_type_occupancy(piece::Type::Pawn).pop_count() as Score
            + KNIGHT_WEIGHT.0 * pos.piece_type_occupancy(piece::Type::Knight).pop_count() as Score
            + BISHOP_WEIGHT.0 * pos.piece_type_occupancy(piece::Type::Bishop).pop_count() as Score
            + ROOK_WEIGHT.0 * pos.piece_type_occupancy(piece::Type::Rook).pop_count() as Score
            + QUEEN_WEIGHT.0 * pos.piece_type_occupancy(piece::Type::Queen).pop_count() as Score
            + KING_WEIGHT.0 * pos.piece_type_occupancy(piece::Type::King).pop_count() as Score;
        total_material = cmp::min(INTERPOLATE_MAX_MATERIAL, total_material);
        let game_phase = total_material - INTERPOLATE_MIN_MATERIAL;

        for (piece_type, table) in [
            (piece::Type::Pawn, PST_PAWN),
            (piece::Type::Knight, PST_KNIGHT),
            (piece::Type::Bishop, PST_BISHOP),
            (piece::Type::Rook, PST_ROOK),
            (piece::Type::Queen, PST_QUEEN),
            (piece::Type::King, PST_KING),
        ] {
            let mut white_piece = pos.piece_occupancy(Side::White, piece_type);
            while white_piece != Bitboard::EMPTY {
                let square = white_piece.square_scan_forward_reset();
                opening_score += table[0][square.idx()];
                endgame_score += table[1][square.idx()];
            }
            let mut black_piece = pos.piece_occupancy(Side::Black, piece_type);
            while black_piece != Bitboard::EMPTY {
                let square = black_piece.square_scan_forward_reset();
                opening_score -= table[0][square.flip_vertical().idx()];
                endgame_score -= table[1][square.flip_vertical().idx()];
            }
        }

        let total_score = (game_phase as i64 * opening_score as i64
            + (INTERPOLATE_MAX_MATERIAL - game_phase) as i64 * endgame_score as i64)
            / (INTERPOLATE_MAX_MATERIAL - INTERPOLATE_MIN_MATERIAL) as i64;
        total_score as i16
    }

    fn eval_relative(pos: &Position) -> Score {
        match pos.side_to_move() {
            Side::White => Self::eval(pos),
            Side::Black => -Self::eval(pos),
        }
    }
}

const fn human_readable_to_file_rank(
    piece_value: Score,
    pst: PieceSquareTable,
) -> PieceSquareTable {
    let mut res = [0; 64];
    let mut idx = 0;
    while idx < 64 {
        let rank = 7 - idx / 8;
        let file = idx % 8;
        let new_idx = Square::from_file_and_rank(File::from_idx(file), Rank::from_idx(rank)).idx();
        res[new_idx] = piece_value + pst[idx];
        idx += 1;
    }
    res
}

#[rustfmt::skip]
const PST_PAWN: [PieceSquareTable; 2] = [
    human_readable_to_file_rank(
        PAWN_WEIGHT.0,
        [
              0,   0,   0,   0,   0,   0,   0,   0,
             30,  40,  45,  50,  50,  45,  40,  30,
             10,  20,  25,  30,  30,  25,  20,  10,
              5,  10,  15,  25,  25,  15,  10,   5,
              0,   5,  10,  20,  20,  10,   5,   0,
              5,  -5,  -5,   0,   0,  -5,  -5,   5,
              5,  10,  10, -20, -20,  10,  10,   5,
              0,   0,   0,   0,   0,   0,   0,   0,
        ],
    ),
    human_readable_to_file_rank(
        PAWN_WEIGHT.1,
        [
              0,   0,   0,   0,   0,   0,   0,   0,
             50,  60,  65,  70,  70,  65,  60,  50,
             25,  35,  40,  45,  45,  40,  35,  25,
             10,  15,  20,  25,  25,  20,  15,  10,
              5,  10,  15,  20,  20,  15,  10,   5,
              0,   5,   5,  10,  10,   5,   5,   0,
              0,   0,   0,   0,   0,   0,   0,   0,
              0,   0,   0,   0,   0,   0,   0,   0,
        ],
    ),
];

#[rustfmt::skip]
const PST_KNIGHT: [PieceSquareTable; 2] = [
    human_readable_to_file_rank(
        KNIGHT_WEIGHT.0,
        [
            -40, -25, -20, -20, -20, -20, -25, -40,
            -25, -10,   0,   0,   0,   0, -10, -25,
            -20,   0,  10,  15,  15,  10,   0, -20,
            -20,   5,  15,  20,  20,  15,   5, -20,
            -20,   0,  15,  20,  20,  15,   0, -20,
            -20,   5,  10,  15,  15,  10,   5, -20,
            -25, -10,   0,   5,   5,   0, -10, -25,
            -40, -25, -20, -20, -20, -20, -25, -40,
        ],
    ),
    human_readable_to_file_rank(
        KNIGHT_WEIGHT.1,
        [
            -40, -25, -20, -20, -20, -20, -25, -40,
            -25, -10,   0,   0,   0,   0, -10, -25,
            -20,   0,   5,  10,  10,   5,   0, -20,
            -20,   0,  10,  15,  15,  10,   0, -20,
            -20,   0,  10,  15,  15,  10,   0, -20,
            -20,   0,   5,  10,  10,   5,   0, -20,
            -25, -10,   0,   0,   0,   0, -10, -25,
            -40, -25, -20, -20, -20, -20, -25, -40,
        ],
    ),
];

#[rustfmt::skip]
const PST_BISHOP: [PieceSquareTable; 2] = [
    human_readable_to_file_rank(
        BISHOP_WEIGHT.0,
        [
            -20, -10, -10, -10, -10, -10, -10, -20,
            -10,   0,   0,   0,   0,   0,   0, -10,
            -10,   0,   5,  10,  10,   5,   0, -10,
            -10,   5,   5,  10,  10,   5,   5, -10,
            -10,   0,  10,  10,  10,  10,   0, -10,
            -10,  10,  10,  10,  10,  10,  10, -10,
            -10,   5,   0,   0,   0,   0,   5, -10,
            -20, -10, -10, -10, -10, -10, -10, -20,
        ],
    ),
    human_readable_to_file_rank(
        BISHOP_WEIGHT.1,
        [
            -10,  -5,  -5,  -5,  -5,  -5,  -5, -10,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   5,  10,  10,   5,   0,  -5,
             -5,   0,   5,  10,  10,   5,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   0,   0,   0,   0,   0,  -5,
            -10,  -5,  -5,  -5,  -5,  -5,  -5, -10,
        ],
    ),
];

#[rustfmt::skip]
const PST_ROOK: [PieceSquareTable; 2] = [
    human_readable_to_file_rank(
        ROOK_WEIGHT.0,
        [
              0,   0,   0,   0,   0,   0,   0,   0,
              5,  10,  10,  10,  10,  10,  10,   5,
             -5,   0,   0,   5,   5,   0,   0,  -5,
             -5,   0,   0,   5,   5,   0,   0,  -5,
             -5,   0,   0,   5,   5,   0,   0,  -5,
             -5,   0,   0,   5,   5,   0,   0,  -5,
             -5,   0,   0,   5,   5,   0,   0,  -5,
              0,   0,   0,   5,   5,   0,   0,   0,
        ],
    ),
    human_readable_to_file_rank(
        ROOK_WEIGHT.1,
        [
             -5,  -5,  -5,  -5,  -5,  -5,  -5,  -5,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,  -5,  -5,  -5,  -5,  -5,  -5,  -5,
        ],
    ),
];

#[rustfmt::skip]
const PST_QUEEN: [PieceSquareTable; 2] = [
    human_readable_to_file_rank(
        QUEEN_WEIGHT.0,
        [
            -20, -10, -10,  -5,  -5, -10, -10, -20,
            -10,   0,   0,   0,   0,   0,   0, -10,
            -10,   0,   5,   5,   5,   5,   0, -10,
             -5,   0,   5,   5,   5,   5,   0,  -5,
              0,   0,   5,   5,   5,   5,   0,  -5,
            -10,   5,   5,   5,   5,   5,   0, -10,
            -10,   0,   5,   0,   0,   0,   0, -10,
            -20, -10, -10,  -5,  -5, -10, -10, -20,
        ],
    ),
    human_readable_to_file_rank(
        QUEEN_WEIGHT.1,
        [
             -5,  -5,  -5,  -5,  -5,  -5,  -5,  -5,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,  -5,  -5,  -5,  -5,  -5,  -5,  -5,
        ],
    ),
];

#[rustfmt::skip]
const PST_KING: [PieceSquareTable; 2] = [
    human_readable_to_file_rank(
        KING_WEIGHT.0,
        [
            -30, -40, -40, -50, -50, -40, -40, -30,
            -30, -40, -40, -50, -50, -40, -40, -30,
            -30, -40, -40, -50, -50, -40, -40, -30,
            -30, -40, -40, -50, -50, -40, -40, -30,
            -20, -30, -30, -40, -40, -30, -30, -20,
            -10, -20, -20, -20, -20, -20, -20, -10,
             20,  20,   0,   0,   0,   0,  20,  20,
             20,  30,  10, -10,   0,   0,  30,  20,
        ],
    ),
    human_readable_to_file_rank(
        KING_WEIGHT.1,
        [
            -50, -35, -25, -20, -20, -25, -35, -50,
            -35, -15,  -5,   0,   0,  -5, -15, -35,
            -25,  -5,  10,  15,  15,  10,  -5, -25,
            -20,   0,  15,  25,  25,  15,   0, -20,
            -20,   0,  15,  25,  25,  15,   0, -20,
            -25,  -5,  10,  15,  15,  10,  -5, -25,
            -35, -15,  -5,   0,   0,  -5, -15, -35,
            -50, -35, -25, -20, -20, -25, -35, -50,
        ],
    ),
];

#[cfg(test)]
mod tests {
    use movegen::square::Square;

    #[test]
    fn human_readable_to_file_rank() {
        #[rustfmt::skip]
        let arr = [
             0,  1,  2,  3,  4,  5,  6,  7,
             8,  9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39,
            40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 54, 55,
            56, 57, 58, 59, 60, 61, 62, 63,
        ];

        let res = super::human_readable_to_file_rank(100, arr);
        assert_eq!(156, res[Square::A1.idx()]);
        assert_eq!(148, res[Square::A2.idx()]);
        assert_eq!(100, res[Square::A8.idx()]);
        assert_eq!(157, res[Square::B1.idx()]);
        assert_eq!(163, res[Square::H1.idx()]);
        assert_eq!(115, res[Square::H7.idx()]);
        assert_eq!(107, res[Square::H8.idx()]);
    }
}