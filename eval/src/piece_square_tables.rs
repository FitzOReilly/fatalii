use crate::{Eval, Score};
use movegen::bitboard::Bitboard;
use movegen::file::File;
use movegen::piece;
use movegen::position::Position;
use movegen::rank::Rank;
use movegen::side::Side;
use movegen::square::Square;
use std::cmp;

type PieceSquareTable = [(Score, Score); 64];

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

#[derive(Debug, Clone)]
pub struct PieceSquareTables {
    current_pos: Position,
    opening_score: Score,
    endgame_score: Score,
    total_material: Score,
}

impl Eval for PieceSquareTables {
    fn eval(&mut self, pos: &Position) -> Score {
        self.update(pos);
        let clamped_material = cmp::min(INTERPOLATE_MAX_MATERIAL, self.total_material);
        let game_phase = clamped_material - INTERPOLATE_MIN_MATERIAL;
        let total_score = (game_phase as i64 * self.opening_score as i64
            + (INTERPOLATE_MAX_MATERIAL - game_phase) as i64 * self.endgame_score as i64)
            / (INTERPOLATE_MAX_MATERIAL - INTERPOLATE_MIN_MATERIAL) as i64;
        total_score as i16
    }
}

impl Default for PieceSquareTables {
    fn default() -> Self {
        Self::new()
    }
}

impl PieceSquareTables {
    pub fn new() -> Self {
        PieceSquareTables {
            current_pos: Position::empty(),
            opening_score: 0,
            endgame_score: 0,
            total_material: 0,
        }
    }

    fn update(&mut self, pos: &Position) {
        for (piece_type, table, weight) in [
            (piece::Type::Pawn, PST_PAWN, PAWN_WEIGHT.0),
            (piece::Type::Knight, PST_KNIGHT, KNIGHT_WEIGHT.0),
            (piece::Type::Bishop, PST_BISHOP, BISHOP_WEIGHT.0),
            (piece::Type::Rook, PST_ROOK, ROOK_WEIGHT.0),
            (piece::Type::Queen, PST_QUEEN, QUEEN_WEIGHT.0),
            (piece::Type::King, PST_KING, KING_WEIGHT.0),
        ] {
            let old_white = self.current_pos.piece_occupancy(Side::White, piece_type);
            let new_white = pos.piece_occupancy(Side::White, piece_type);
            let mut white_remove = old_white & !new_white;
            let mut white_add = new_white & !old_white;
            while white_remove != Bitboard::EMPTY {
                let square = white_remove.square_scan_forward_reset();
                self.opening_score -= table[square.idx()].0;
                self.endgame_score -= table[square.idx()].1;
                self.total_material -= weight;
            }
            while white_add != Bitboard::EMPTY {
                let square = white_add.square_scan_forward_reset();
                self.opening_score += table[square.idx()].0;
                self.endgame_score += table[square.idx()].1;
                self.total_material += weight;
            }
            let old_black = self.current_pos.piece_occupancy(Side::Black, piece_type);
            let new_black = pos.piece_occupancy(Side::Black, piece_type);
            let mut black_remove = old_black & !new_black;
            let mut black_add = new_black & !old_black;
            while black_remove != Bitboard::EMPTY {
                let square = black_remove.square_scan_forward_reset();
                self.opening_score += table[square.flip_vertical().idx()].0;
                self.endgame_score += table[square.flip_vertical().idx()].1;
                self.total_material -= weight;
            }
            while black_add != Bitboard::EMPTY {
                let square = black_add.square_scan_forward_reset();
                self.opening_score -= table[square.flip_vertical().idx()].0;
                self.endgame_score -= table[square.flip_vertical().idx()].1;
                self.total_material += weight;
            }
        }
        self.current_pos = pos.clone();
    }
}

const fn human_readable_to_file_rank(piece_value: Score, pst: [Score; 64]) -> [Score; 64] {
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
const PST_PAWN: PieceSquareTable = {
    let mg = human_readable_to_file_rank(
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
    );
    let eg = human_readable_to_file_rank(
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
    );
    let mut table = [(0, 0); 64];
    let mut idx = 0;
    while idx < 64 {
        table[idx] = (mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

#[rustfmt::skip]
const PST_KNIGHT: PieceSquareTable = {
    let mg = human_readable_to_file_rank(
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
    );
    let eg = human_readable_to_file_rank(
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
    );
    let mut table = [(0, 0); 64];
    let mut idx = 0;
    while idx < 64 {
        table[idx] = (mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

#[rustfmt::skip]
const PST_BISHOP: PieceSquareTable = {
    let mg = human_readable_to_file_rank(
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
    );
    let eg = human_readable_to_file_rank(
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
    );
    let mut table = [(0, 0); 64];
    let mut idx = 0;
    while idx < 64 {
        table[idx] = (mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

#[rustfmt::skip]
const PST_ROOK: PieceSquareTable = {
    let mg = human_readable_to_file_rank(
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
    );
    let eg = human_readable_to_file_rank(
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
    );
    let mut table = [(0, 0); 64];
    let mut idx = 0;
    while idx < 64 {
        table[idx] = (mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

#[rustfmt::skip]
const PST_QUEEN: PieceSquareTable = {
    let mg = human_readable_to_file_rank(
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
    );
    let eg = human_readable_to_file_rank(
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
    );
    let mut table = [(0, 0); 64];
    let mut idx = 0;
    while idx < 64 {
        table[idx] = (mg[idx], eg[idx]);
        idx += 1;
    }
    table
};

#[rustfmt::skip]
const PST_KING: PieceSquareTable = {
    let mg = human_readable_to_file_rank(
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
    );
    let eg = human_readable_to_file_rank(
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
    );
    let mut table = [(0, 0); 64];
    let mut idx = 0;
    while idx < 64 {
        table[idx] = (mg[idx], eg[idx]);
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
