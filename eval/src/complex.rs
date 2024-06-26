use crate::eval::HasMatingMaterial;
use crate::game_phase::{GamePhase, PieceCounts};
use crate::mobility::Mobility;
use crate::params;
use crate::pawn_structure::PawnStructure;
use crate::score_pair::ScorePair;
use crate::{Eval, Score, EQ_POSITION};
use movegen::bitboard::Bitboard;
use movegen::piece::{self, Piece};
use movegen::position::Position;
use movegen::side::Side;

#[derive(Debug, Clone)]
pub struct Complex {
    current_pos: Position,
    game_phase: GamePhase,
    piece_counts: PieceCounts,
    pst_scores: ScorePair,
    pawn_structure: PawnStructure,
    mobility: Mobility,
    king_tropism: ScorePair,
}

impl Eval for Complex {
    fn eval(&mut self, pos: &Position) -> Score {
        self.update(pos);

        let white_mating_material = self.has_mating_material(Side::White);
        let black_mating_material = self.has_mating_material(Side::Black);
        if !white_mating_material && !black_mating_material {
            return EQ_POSITION;
        }

        let tempo_multiplier = 1 - 2 * (pos.side_to_move() as i16);
        let tempo_scores = tempo_multiplier * params::TEMPO;
        self.pawn_structure.update(pos);
        let pawn_scores = self.pawn_structure.scores();
        let mobility_scores = self.mobility.scores(pos);
        let bishop_pair_scores = Self::bishop_pair_factor(pos) * params::BISHOP_PAIR;
        let scores = self.pst_scores
            + tempo_scores
            + pawn_scores
            + mobility_scores
            + bishop_pair_scores
            + self.king_tropism;
        let game_phase = self.game_phase.game_phase_clamped();
        let tapered_score = ((game_phase as i64 * scores.0 as i64
            + (GamePhase::MAX - game_phase) as i64 * scores.1 as i64)
            / GamePhase::MAX as i64) as Score;

        if !white_mating_material {
            std::cmp::min(EQ_POSITION, tapered_score)
        } else if !black_mating_material {
            std::cmp::max(EQ_POSITION, tapered_score)
        } else {
            tapered_score
        }
    }
}

impl Default for Complex {
    fn default() -> Self {
        Self::new()
    }
}

impl HasMatingMaterial for Complex {
    // Check if one side has enough material to checkmate the opponent. In
    // positions where a mate is possible, but cannot be forced (e.g. KNNvK),
    // this still returns false.
    fn has_mating_material(&self, s: Side) -> bool {
        for p in [
            Piece::new(s, piece::Type::Pawn),
            Piece::new(s, piece::Type::Rook),
            Piece::new(s, piece::Type::Queen),
        ] {
            if self.piece_counts.count(p) > 0 {
                return true;
            }
        }

        // Mate can be forced with more than 2 knights against a lone king
        let knight_count = self.piece_counts.count(Piece::new(s, piece::Type::Knight));
        if knight_count > 2 {
            return true;
        }

        // Mate can be forced with bishop + knight against a lone king
        let bishop_count = self.piece_counts.count(Piece::new(s, piece::Type::Bishop));
        if knight_count > 0 && bishop_count > 0 {
            return true;
        }

        // Mate can be forced with 2 bishops against a lone king, if the bishops
        // are on different colors
        if self.current_pos.has_bishop_pair(s) {
            return true;
        }

        false
    }
}

impl Complex {
    pub fn new() -> Self {
        Self {
            current_pos: Position::empty(),
            game_phase: Default::default(),
            piece_counts: Default::default(),
            pst_scores: ScorePair(0, 0),
            pawn_structure: PawnStructure::new(),
            mobility: Mobility,
            king_tropism: ScorePair(0, 0),
        }
    }

    fn update(&mut self, pos: &Position) {
        let old_white_king = self
            .current_pos
            .piece_occupancy(Side::White, piece::Type::King);
        let new_white_king = pos.piece_occupancy(Side::White, piece::Type::King);
        let old_black_king = self
            .current_pos
            .piece_occupancy(Side::Black, piece::Type::King);
        let new_black_king = pos.piece_occupancy(Side::Black, piece::Type::King);
        let white_king = new_white_king.to_square();
        let black_king = new_black_king.to_square();
        for (piece_type, table, friendly_distance, enemy_distance) in [
            (
                piece::Type::Pawn,
                &params::PST_PAWN,
                &params::DISTANCE_FRIENDLY_PAWN,
                &params::DISTANCE_ENEMY_PAWN,
            ),
            (
                piece::Type::Knight,
                &params::PST_KNIGHT,
                &params::DISTANCE_FRIENDLY_KNIGHT,
                &params::DISTANCE_ENEMY_KNIGHT,
            ),
            (
                piece::Type::Bishop,
                &params::PST_BISHOP,
                &params::DISTANCE_FRIENDLY_BISHOP,
                &params::DISTANCE_ENEMY_BISHOP,
            ),
            (
                piece::Type::Rook,
                &params::PST_ROOK,
                &params::DISTANCE_FRIENDLY_ROOK,
                &params::DISTANCE_ENEMY_ROOK,
            ),
            (
                piece::Type::Queen,
                &params::PST_QUEEN,
                &params::DISTANCE_FRIENDLY_QUEEN,
                &params::DISTANCE_ENEMY_QUEEN,
            ),
            (
                piece::Type::King,
                &params::PST_KING,
                &params::DISTANCE_FRIENDLY_KING,
                &params::DISTANCE_ENEMY_KING,
            ),
        ] {
            let old_white = self.current_pos.piece_occupancy(Side::White, piece_type);
            let new_white = pos.piece_occupancy(Side::White, piece_type);
            let mut white_remove = old_white & !new_white;
            let mut white_add = new_white & !old_white;
            while white_remove != Bitboard::EMPTY {
                let square = white_remove.square_scan_forward_reset();
                self.pst_scores -= table[square.idx()];
                self.game_phase.remove_piece(piece_type);
                self.piece_counts
                    .remove(Piece::new(Side::White, piece_type));
                self.king_tropism -= friendly_distance[white_king.distance(square)];
                self.king_tropism += enemy_distance[black_king.distance(square)];
            }
            while white_add != Bitboard::EMPTY {
                let square = white_add.square_scan_forward_reset();
                self.pst_scores += table[square.idx()];
                self.game_phase.add_piece(piece_type);
                self.piece_counts.add(Piece::new(Side::White, piece_type));
                self.king_tropism += friendly_distance[white_king.distance(square)];
                self.king_tropism -= enemy_distance[black_king.distance(square)];
            }
            let old_black = self.current_pos.piece_occupancy(Side::Black, piece_type);
            let new_black = pos.piece_occupancy(Side::Black, piece_type);
            let mut black_remove = old_black & !new_black;
            let mut black_add = new_black & !old_black;
            while black_remove != Bitboard::EMPTY {
                let square = black_remove.square_scan_forward_reset();
                let square_flipped = square.flip_vertical();
                self.pst_scores += table[square_flipped.idx()];
                self.game_phase.remove_piece(piece_type);
                self.piece_counts
                    .remove(Piece::new(Side::Black, piece_type));
                self.king_tropism -= enemy_distance[white_king.distance(square)];
                self.king_tropism += friendly_distance[black_king.distance(square)];
            }
            while black_add != Bitboard::EMPTY {
                let square = black_add.square_scan_forward_reset();
                let square_flipped = square.flip_vertical();
                self.pst_scores -= table[square_flipped.idx()];
                self.game_phase.add_piece(piece_type);
                self.piece_counts.add(Piece::new(Side::Black, piece_type));
                self.king_tropism += enemy_distance[white_king.distance(square)];
                self.king_tropism -= friendly_distance[black_king.distance(square)];
            }
        }
        if old_white_king != new_white_king || old_black_king != new_black_king {
            // A king was moved, calculate king tropism for all pieces
            self.king_tropism(pos);
        }
        self.current_pos = pos.clone();
    }

    fn king_tropism(&mut self, pos: &Position) {
        let white_king = pos
            .piece_occupancy(Side::White, piece::Type::King)
            .to_square();
        let black_king = pos
            .piece_occupancy(Side::Black, piece::Type::King)
            .to_square();
        self.king_tropism = ScorePair(0, 0);
        for (piece_type, friendly_distance, enemy_distance) in [
            (
                piece::Type::Pawn,
                &params::DISTANCE_FRIENDLY_PAWN,
                &params::DISTANCE_ENEMY_PAWN,
            ),
            (
                piece::Type::Knight,
                &params::DISTANCE_FRIENDLY_KNIGHT,
                &params::DISTANCE_ENEMY_KNIGHT,
            ),
            (
                piece::Type::Bishop,
                &params::DISTANCE_FRIENDLY_BISHOP,
                &params::DISTANCE_ENEMY_BISHOP,
            ),
            (
                piece::Type::Rook,
                &params::DISTANCE_FRIENDLY_ROOK,
                &params::DISTANCE_ENEMY_ROOK,
            ),
            (
                piece::Type::Queen,
                &params::DISTANCE_FRIENDLY_QUEEN,
                &params::DISTANCE_ENEMY_QUEEN,
            ),
            (
                piece::Type::King,
                &params::DISTANCE_FRIENDLY_KING,
                &params::DISTANCE_ENEMY_KING,
            ),
        ] {
            let mut white_pieces = pos.piece_occupancy(Side::White, piece_type);
            while white_pieces != Bitboard::EMPTY {
                let square = white_pieces.square_scan_forward_reset();
                self.king_tropism += friendly_distance[white_king.distance(square)];
                self.king_tropism -= enemy_distance[black_king.distance(square)];
            }
            let mut black_pieces = pos.piece_occupancy(Side::Black, piece_type);
            while black_pieces != Bitboard::EMPTY {
                let square = black_pieces.square_scan_forward_reset();
                self.king_tropism += enemy_distance[white_king.distance(square)];
                self.king_tropism -= friendly_distance[black_king.distance(square)];
            }
        }
    }

    pub fn bishop_pair_factor(pos: &Position) -> Score {
        pos.has_bishop_pair(Side::White) as Score - pos.has_bishop_pair(Side::Black) as Score
    }
}

#[cfg(test)]
mod tests {
    use movegen::fen::Fen;

    use crate::{Eval, EQ_POSITION};

    use super::Complex;

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
            // The opponent has enough mating material, we don't, so just take the pawn and draw
            "7k/4B3/5p2/5K2/8/8/8/8 w - - 4 102", // KBvKP
            "8/6b1/4k3/8/3P4/4K3/8/8 b - - 0 1",  // KBvKP
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
