use crate::game_phase::{GamePhase, PieceCounts};
use crate::{Eval, Score, EQ_POSITION};
use movegen::bitboard::Bitboard;
use movegen::file::File;
use movegen::piece::{self, Piece};
use movegen::position::Position;
use movegen::rank::Rank;
use movegen::side::Side;
use movegen::square::Square;

type PieceSquareTable = [(Score, Score); 64];

// (opening, endgame)
const KING_WEIGHT: (Score, Score) = (0, 0);
const QUEEN_WEIGHT: (Score, Score) = (900, 910);
const ROOK_WEIGHT: (Score, Score) = (500, 520);
const BISHOP_WEIGHT: (Score, Score) = (330, 310);
const KNIGHT_WEIGHT: (Score, Score) = (320, 300);
const PAWN_WEIGHT: (Score, Score) = (100, 120);

// The side to move gets a small bonus
const TEMPO_WEIGHT: (Score, Score) = (10, 10);

#[derive(Debug, Clone)]
pub struct Complex {
    current_pos: Position,
    opening_score: Score,
    endgame_score: Score,
    game_phase: GamePhase,
    piece_counts: PieceCounts,
}

impl Eval for Complex {
    fn eval(&mut self, pos: &Position) -> Score {
        self.update(pos);

        if self.is_draw() {
            return EQ_POSITION;
        }

        let tempo_multiplier = 1 - 2 * (pos.side_to_move() as i16);
        let tempo_score_mg = tempo_multiplier * TEMPO_WEIGHT.0;
        let tempo_score_eg = tempo_multiplier * TEMPO_WEIGHT.1;
        let score_mg = self.opening_score + tempo_score_mg;
        let score_eg = self.endgame_score + tempo_score_eg;
        let game_phase = self.game_phase.game_phase_clamped();
        let tapered_score = (game_phase as i64 * score_mg as i64
            + (GamePhase::MAX - game_phase) as i64 * score_eg as i64)
            / GamePhase::MAX as i64;

        tapered_score as Score
    }
}

impl Default for Complex {
    fn default() -> Self {
        Self::new()
    }
}

impl Complex {
    pub fn new() -> Self {
        Complex {
            current_pos: Position::empty(),
            opening_score: 0,
            endgame_score: 0,
            game_phase: Default::default(),
            piece_counts: Default::default(),
        }
    }

    // Check if a position is a draw. In positions where a mate is possible, but
    // cannot be forced (e.g. KNNvK), this still returns true.
    fn is_draw(&self) -> bool {
        for p in [
            Piece::WHITE_PAWN,
            Piece::BLACK_PAWN,
            Piece::WHITE_ROOK,
            Piece::BLACK_ROOK,
            Piece::WHITE_QUEEN,
            Piece::BLACK_QUEEN,
        ] {
            if self.piece_counts.count(p) > 0 {
                return false;
            }
        }

        // Mate can be forced with more than 2 knights against a lone king
        let white_knights_count = self.piece_counts.count(Piece::WHITE_KNIGHT);
        if white_knights_count > 2 {
            return false;
        }
        let black_knights_count = self.piece_counts.count(Piece::BLACK_KNIGHT);
        if black_knights_count > 2 {
            return false;
        }

        // Mate can be forced with bishop + knight against a lone king
        let white_bishops_count = self.piece_counts.count(Piece::WHITE_BISHOP);
        if white_knights_count > 0 && white_bishops_count > 0 {
            return false;
        }
        let black_bishops_count = self.piece_counts.count(Piece::BLACK_BISHOP);
        if black_knights_count > 0 && black_bishops_count > 0 {
            return false;
        }

        // Mate can be forced with 2 bishops against a lone king, if the bishops
        // are on different colors
        if white_bishops_count > 1 {
            let white_bishops = self
                .current_pos
                .piece_occupancy(Side::White, piece::Type::Bishop);
            if white_bishops & Bitboard::LIGHT_SQUARES != Bitboard::EMPTY
                && white_bishops & Bitboard::DARK_SQUARES != Bitboard::EMPTY
            {
                return false;
            }
        }
        if black_bishops_count > 1 {
            let black_bishops = self
                .current_pos
                .piece_occupancy(Side::Black, piece::Type::Bishop);
            if black_bishops & Bitboard::LIGHT_SQUARES != Bitboard::EMPTY
                && black_bishops & Bitboard::DARK_SQUARES != Bitboard::EMPTY
            {
                return false;
            }
        }

        true
    }

    fn update(&mut self, pos: &Position) {
        for (piece_type, table) in [
            (piece::Type::Pawn, &PST_PAWN),
            (piece::Type::Knight, &PST_KNIGHT),
            (piece::Type::Bishop, &PST_BISHOP),
            (piece::Type::Rook, &PST_ROOK),
            (piece::Type::Queen, &PST_QUEEN),
            (piece::Type::King, &PST_KING),
        ] {
            let old_white = self.current_pos.piece_occupancy(Side::White, piece_type);
            let new_white = pos.piece_occupancy(Side::White, piece_type);
            let mut white_remove = old_white & !new_white;
            let mut white_add = new_white & !old_white;
            while white_remove != Bitboard::EMPTY {
                let square = white_remove.square_scan_forward_reset();
                self.opening_score -= table[square.idx()].0;
                self.endgame_score -= table[square.idx()].1;
                self.game_phase.remove_piece(piece_type);
                self.piece_counts
                    .remove(Piece::new(Side::White, piece_type));
            }
            while white_add != Bitboard::EMPTY {
                let square = white_add.square_scan_forward_reset();
                self.opening_score += table[square.idx()].0;
                self.endgame_score += table[square.idx()].1;
                self.game_phase.add_piece(piece_type);
                self.piece_counts.add(Piece::new(Side::White, piece_type));
            }
            let old_black = self.current_pos.piece_occupancy(Side::Black, piece_type);
            let new_black = pos.piece_occupancy(Side::Black, piece_type);
            let mut black_remove = old_black & !new_black;
            let mut black_add = new_black & !old_black;
            while black_remove != Bitboard::EMPTY {
                let square = black_remove.square_scan_forward_reset();
                self.opening_score += table[square.flip_vertical().idx()].0;
                self.endgame_score += table[square.flip_vertical().idx()].1;
                self.game_phase.remove_piece(piece_type);
                self.piece_counts
                    .remove(Piece::new(Side::Black, piece_type));
            }
            while black_add != Bitboard::EMPTY {
                let square = black_add.square_scan_forward_reset();
                self.opening_score -= table[square.flip_vertical().idx()].0;
                self.endgame_score -= table[square.flip_vertical().idx()].1;
                self.game_phase.add_piece(piece_type);
                self.piece_counts.add(Piece::new(Side::Black, piece_type));
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
              0,  -5,   5,  20,  20,   5,  -5,   0,
              5,   0,   0,   0,   0,   0,   0,   5,
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
            -10,  10,   5,  10,  10,   5,  10, -10,
            -10,   5,  15,  10,  10,  15,   5, -10,
            -10,  10,  10,  10,  10,  10,  10, -10,
            -10,  15,  10,  10,  10,  10,  15, -10,
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
             -5,  -5,   0,   5,   5,   0,  -5,  -5,
            -10,  -5,   5,  10,  10,   5,  -5, -10,
        ],
    );
    let eg = human_readable_to_file_rank(
        ROOK_WEIGHT.1,
        [
            -10,  -5,  -5,  -5,  -5,  -5,  -5, -10,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
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
            -30, -30, -30, -40, -40, -30, -30, -30,
            -20, -20, -25, -25, -25, -25, -20, -20,
             10,  10, -10, -10, -10, -10,  10,  10,
             20,  30,  -5,  -5,  -5,  -5,  30,  20,
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
    use movegen::{fen::Fen, square::Square};

    use crate::{Eval, EQ_POSITION};

    use super::Complex;

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

    #[test]
    fn draw_by_insufficient_material() {
        let mut evaluator = Complex::new();

        for draw in [
            "7k/8/8/8/3K4/8/8/8 w - - 0 1",    // KvK
            "7k/8/8/8/3KN3/8/8/8 w - - 0 1",   // KNvK
            "7k/8/8/8/3KB3/8/8/8 w - - 0 1",   // KBvK
            "7k/8/8/5B2/3KB3/8/8/8 w - - 0 1", // KBBvK, bishops on same color
            "6bk/8/8/8/3KB3/8/8/8 w - - 0 1",  // KBvKB, bishops on same color
            // In these positions, mate is possible, but cannot be forced
            "7k/8/8/3N4/3KN3/8/8/8 w - - 0 1",   // KNNvK
            "k7/b1KB4/8/8/8/8/8/8 w - - 0 1",    // KBvKB, bishops on different colors
            "1n2k3/8/8/8/8/8/8/2B1K3 w - - 0 1", // KBvKN
        ] {
            let pos = Fen::str_to_pos(draw).unwrap();
            assert_eq!(
                EQ_POSITION,
                evaluator.eval(&pos),
                "\nPosition: {draw}\n{pos}"
            );
        }

        for non_draw in [
            "7k/8/8/8/3KQ3/8/8/8 w - - 0 1",      // KQvK
            "7k/8/8/8/3KR3/8/8/8 w - - 0 1",      // KRvK
            "7k/8/8/8/3KP3/8/8/8 w - - 0 1",      // KPvK
            "7k/8/8/8/3KBB2/8/8/8 w - - 0 1",     // KBBvK, bishops on different colors
            "7k/8/8/8/3KBN2/8/8/8 w - - 0 1",     // KBNvK
            "4k3/8/8/8/8/8/8/1NN1K1N1 w - - 0 1", // KNNNvK
        ] {
            let pos = Fen::str_to_pos(non_draw).unwrap();
            assert_ne!(
                EQ_POSITION,
                evaluator.eval(&pos),
                "\nPosition: {non_draw}\n{pos}"
            );
        }
    }
}
