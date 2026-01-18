use crate::eval::HasMatingMaterial;
use crate::game_phase::{GamePhase, PieceCounts};
#[cfg(feature = "trace")]
use crate::hand_crafted_eval_coeffs::{Coeff, HandCraftedEvalCoeffs};
use crate::params;
use crate::score_pair::ScorePair;
use crate::{Eval, Score, EQ_POSITION};
use movegen::bishop::Bishop;
use movegen::bitboard::Bitboard;
use movegen::knight::Knight;
use movegen::pawn::Pawn;
use movegen::piece::{self, Piece};
use movegen::position::Position;
use movegen::queen::Queen;
use movegen::rook::Rook;
use movegen::side::Side;

#[derive(Debug, Clone)]
pub struct HandCraftedEval {
    current_pos: Position,
    game_phase: GamePhase,
    piece_counts: PieceCounts,
    pst_scores: ScorePair,
    pawn_structure_scores: ScorePair,
    king_tropism: ScorePair,
    #[cfg(feature = "trace")]
    coeffs: HandCraftedEvalCoeffs,
}

impl Eval for HandCraftedEval {
    fn eval(&mut self, pos: &Position) -> Score {
        self.update(pos);

        let scores = self.pst_scores
            + self.tempo_scores(pos)
            + self.pawn_structure_scores
            + self.mobility_scores(pos)
            + self.bishop_pair_scores(pos)
            + self.king_tropism;

        let game_phase = self.game_phase.game_phase_clamped();
        #[cfg(feature = "trace")]
        (self.coeffs.game_phase = game_phase);

        let tapered_score = ((game_phase as i64 * scores.0 as i64
            + (GamePhase::MAX - game_phase) as i64 * scores.1 as i64)
            / GamePhase::MAX as i64) as Score;

        match (
            self.has_mating_material(Side::White),
            self.has_mating_material(Side::Black),
        ) {
            (true, true) => tapered_score,
            (true, false) => tapered_score.max(EQ_POSITION),
            (false, true) => tapered_score.min(EQ_POSITION),
            (false, false) => EQ_POSITION,
        }
    }
}

impl Default for HandCraftedEval {
    fn default() -> Self {
        Self::new()
    }
}

impl HasMatingMaterial for HandCraftedEval {
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

impl HandCraftedEval {
    pub fn new() -> Self {
        Self {
            current_pos: Position::empty(),
            game_phase: Default::default(),
            piece_counts: Default::default(),
            pst_scores: Default::default(),
            pawn_structure_scores: Default::default(),
            king_tropism: Default::default(),
            #[cfg(feature = "trace")]
            coeffs: Default::default(),
        }
    }

    #[cfg(feature = "trace")]
    pub fn coeffs(&self) -> &HandCraftedEvalCoeffs {
        &self.coeffs
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
                let p = Piece::new(Side::White, piece_type);
                self.piece_counts.remove(p);
                self.king_tropism -= friendly_distance[white_king.distance(square)];
                self.king_tropism += enemy_distance[black_king.distance(square)];
                #[cfg(feature = "trace")]
                self.coeffs.add_piece(p, square, white_king, black_king, -1);
            }
            while white_add != Bitboard::EMPTY {
                let square = white_add.square_scan_forward_reset();
                self.pst_scores += table[square.idx()];
                self.game_phase.add_piece(piece_type);
                let p = Piece::new(Side::White, piece_type);
                self.piece_counts.add(p);
                self.king_tropism += friendly_distance[white_king.distance(square)];
                self.king_tropism -= enemy_distance[black_king.distance(square)];
                #[cfg(feature = "trace")]
                self.coeffs.add_piece(p, square, white_king, black_king, 1);
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
                let p = Piece::new(Side::Black, piece_type);
                self.piece_counts.remove(p);
                self.king_tropism -= enemy_distance[white_king.distance(square)];
                self.king_tropism += friendly_distance[black_king.distance(square)];
                #[cfg(feature = "trace")]
                self.coeffs.add_piece(p, square, white_king, black_king, -1);
            }
            while black_add != Bitboard::EMPTY {
                let square = black_add.square_scan_forward_reset();
                let square_flipped = square.flip_vertical();
                self.pst_scores -= table[square_flipped.idx()];
                self.game_phase.add_piece(piece_type);
                let p = Piece::new(Side::Black, piece_type);
                self.piece_counts.add(p);
                self.king_tropism += enemy_distance[white_king.distance(square)];
                self.king_tropism -= friendly_distance[black_king.distance(square)];
                #[cfg(feature = "trace")]
                self.coeffs.add_piece(p, square, white_king, black_king, 1);
            }
        }
        if old_white_king != new_white_king || old_black_king != new_black_king {
            // A king was moved, calculate king tropism for all pieces
            self.update_king_tropism(pos);
        }
        self.update_pawn_structure_scores(pos);
        self.current_pos = pos.clone();
    }

    fn update_king_tropism(&mut self, pos: &Position) {
        let white_king = pos
            .piece_occupancy(Side::White, piece::Type::King)
            .to_square();
        let black_king = pos
            .piece_occupancy(Side::Black, piece::Type::King)
            .to_square();
        self.king_tropism = ScorePair(0, 0);
        #[cfg(feature = "trace")]
        self.coeffs.clear_distance();
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
            // No need to calculate this for kings: The distance to itself is
            // always 0 and the scores for the distance between each other
            // cancel each other out.
        ] {
            let mut white_pieces = pos.piece_occupancy(Side::White, piece_type);
            while white_pieces != Bitboard::EMPTY {
                let square = white_pieces.square_scan_forward_reset();
                self.king_tropism += friendly_distance[white_king.distance(square)];
                self.king_tropism -= enemy_distance[black_king.distance(square)];
                #[cfg(feature = "trace")]
                {
                    let p = Piece::new(Side::White, piece_type);
                    self.coeffs
                        .add_distance(p, square, white_king, black_king, 1);
                }
            }
            let mut black_pieces = pos.piece_occupancy(Side::Black, piece_type);
            while black_pieces != Bitboard::EMPTY {
                let square = black_pieces.square_scan_forward_reset();
                self.king_tropism += enemy_distance[white_king.distance(square)];
                self.king_tropism -= friendly_distance[black_king.distance(square)];
                #[cfg(feature = "trace")]
                {
                    let p = Piece::new(Side::Black, piece_type);
                    self.coeffs
                        .add_distance(p, square, white_king, black_king, 1);
                }
            }
        }
    }

    fn tempo_scores(&mut self, pos: &Position) -> ScorePair {
        let tempo_multiplier = 1 - 2 * (pos.side_to_move() as i16);
        #[cfg(feature = "trace")]
        (self.coeffs.tempo = tempo_multiplier.into());
        tempo_multiplier * params::TEMPO
    }

    fn bishop_pair_scores(&mut self, pos: &Position) -> ScorePair {
        let bishop_pair_factor =
            pos.has_bishop_pair(Side::White) as Score - pos.has_bishop_pair(Side::Black) as Score;
        #[cfg(feature = "trace")]
        (self.coeffs.bishop_pair = bishop_pair_factor.into());
        bishop_pair_factor * params::BISHOP_PAIR
    }

    fn update_pawn_structure_scores(&mut self, pos: &Position) {
        let old_white_pawns = self
            .current_pos
            .piece_occupancy(Side::White, piece::Type::Pawn);
        let white_pawns = pos.piece_occupancy(Side::White, piece::Type::Pawn);
        let old_black_pawns = self
            .current_pos
            .piece_occupancy(Side::Black, piece::Type::Pawn);
        let black_pawns = pos.piece_occupancy(Side::Black, piece::Type::Pawn);

        if old_white_pawns != white_pawns || old_black_pawns != black_pawns {
            let passed_pawn_score = self.passed_pawn_scores(white_pawns, black_pawns);
            let isolated_pawn_score =
                Self::isolated_pawn_count(white_pawns, black_pawns) as i16 * params::ISOLATED_PAWN;
            let backward_pawn_score =
                Self::backward_pawn_count(white_pawns, black_pawns) as i16 * params::BACKWARD_PAWN;
            let doubled_pawn_score =
                Self::doubled_pawn_count(white_pawns, black_pawns) as i16 * params::DOUBLED_PAWN;
            self.pawn_structure_scores =
                passed_pawn_score + isolated_pawn_score + backward_pawn_score + doubled_pawn_score;

            #[cfg(feature = "trace")]
            {
                *self.coeffs.isolated_pawn = Self::isolated_pawn_count(white_pawns, black_pawns);
                *self.coeffs.backward_pawn = Self::backward_pawn_count(white_pawns, black_pawns);
                *self.coeffs.doubled_pawn = Self::doubled_pawn_count(white_pawns, black_pawns);
            }
        }
    }

    fn isolated_pawn_count(white_pawns: Bitboard, black_pawns: Bitboard) -> i8 {
        Self::isolated_pawn_count_one_side(white_pawns)
            - Self::isolated_pawn_count_one_side(black_pawns)
    }

    fn backward_pawn_count(white_pawns: Bitboard, black_pawns: Bitboard) -> i8 {
        Self::backward_pawn_count_one_side(white_pawns, black_pawns, Side::White)
            - Self::backward_pawn_count_one_side(black_pawns, white_pawns, Side::Black)
    }

    fn doubled_pawn_count(white_pawns: Bitboard, black_pawns: Bitboard) -> i8 {
        let mut doubled_pawn_count = 0;
        for file in [
            Bitboard::FILE_A,
            Bitboard::FILE_B,
            Bitboard::FILE_C,
            Bitboard::FILE_D,
            Bitboard::FILE_E,
            Bitboard::FILE_F,
            Bitboard::FILE_G,
            Bitboard::FILE_H,
        ] {
            let white_pawns_on_file = (white_pawns & file).pop_count() as i8;
            doubled_pawn_count += std::cmp::max(0, white_pawns_on_file - 1);
            let black_pawns_on_file = (black_pawns & file).pop_count() as i8;
            doubled_pawn_count -= std::cmp::max(0, black_pawns_on_file - 1);
        }
        doubled_pawn_count
    }

    fn passed_pawn_scores(&mut self, white_pawns: Bitboard, black_pawns: Bitboard) -> ScorePair {
        #[cfg(feature = "trace")]
        self.coeffs.passed_pawn.fill(Coeff(0));

        let mut scores = ScorePair(0, 0);
        // Ignore the rank just before promotion (7th for white, 2nd for black).
        // Pawns on these ranks are always passed, so they are already
        // considered by the pawn PSTs.
        let mut white_passed =
            Self::passed_pawns_one_side(white_pawns, black_pawns, Side::White) & !Bitboard::RANK_7;
        while white_passed != Bitboard::EMPTY {
            let square = white_passed.square_scan_forward_reset();
            scores += params::PASSED_PAWN[square.idx()];
            #[cfg(feature = "trace")]
            (*self.coeffs.passed_pawn[square.fold_to_queenside().idx()] += 1);
        }
        let mut black_passed =
            Self::passed_pawns_one_side(black_pawns, white_pawns, Side::Black) & !Bitboard::RANK_2;
        while black_passed != Bitboard::EMPTY {
            let square = black_passed.square_scan_forward_reset();
            scores -= params::PASSED_PAWN[square.flip_vertical().idx()];
            #[cfg(feature = "trace")]
            (*self.coeffs.passed_pawn[square.flip_vertical().fold_to_queenside().idx()] -= 1);
        }
        scores
    }

    fn passed_pawns_one_side(
        own_pawns: Bitboard,
        opp_pawns: Bitboard,
        side_to_move: Side,
    ) -> Bitboard {
        let obstructed = Pawn::rear_span(
            own_pawns | opp_pawns | opp_pawns.east_one() | opp_pawns.west_one(),
            side_to_move,
        );
        own_pawns & !obstructed
    }

    fn isolated_pawn_count_one_side(own_pawns: Bitboard) -> i8 {
        let occupied_files = own_pawns.file_fill();
        let connected_files = occupied_files.east_one() | occupied_files.west_one();
        let isolated_pawns = own_pawns & !connected_files;
        isolated_pawns.pop_count() as i8
    }

    fn backward_pawn_count_one_side(
        own_pawns: Bitboard,
        opp_pawns: Bitboard,
        side_to_move: Side,
    ) -> i8 {
        let own_pawn_stops = Pawn::push_targets(own_pawns, Bitboard::EMPTY, side_to_move).0;
        let own_front_attack_span = Pawn::front_attack_span(own_pawns, side_to_move);
        let opp_attack_targets = Pawn::attack_targets(opp_pawns, !side_to_move);
        let backward_pawn_targets = own_pawn_stops & !own_front_attack_span & opp_attack_targets;
        let backward_pawns = Pawn::single_push_origins(backward_pawn_targets, side_to_move);
        backward_pawns.pop_count() as i8
    }

    fn mobility_scores(&mut self, pos: &Position) -> ScorePair {
        #[cfg(feature = "trace")]
        {
            self.coeffs.knight_mobility.fill(Coeff(0));
            self.coeffs.bishop_mobility.fill(Coeff(0));
            self.coeffs.rook_mobility.fill(Coeff(0));
            self.coeffs.queen_mobility.fill(Coeff(0));
        }

        let mut scores = ScorePair(0, 0);

        let occupancy = pos.occupancy();
        let white_occupancy = pos.side_occupancy(Side::White);
        let black_occupancy = pos.side_occupancy(Side::Black);

        let all_knights = pos.piece_type_occupancy(piece::Type::Knight);
        for (own_occupancy, score_factor) in [(white_occupancy, 1), (black_occupancy, -1)] {
            let mut own_knights = all_knights & own_occupancy;
            while own_knights != Bitboard::EMPTY {
                let origin = own_knights.square_scan_forward_reset();
                let targets = Knight::targets(origin) & !own_occupancy;
                scores += score_factor * params::MOBILITY_KNIGHT[targets.pop_count()];
                #[cfg(feature = "trace")]
                (*self.coeffs.knight_mobility[targets.pop_count()] += score_factor as i8);
            }
        }
        let all_bishops = pos.piece_type_occupancy(piece::Type::Bishop);
        for (own_occupancy, score_factor) in [(white_occupancy, 1), (black_occupancy, -1)] {
            let mut own_bishops = all_bishops & own_occupancy;
            while own_bishops != Bitboard::EMPTY {
                let origin = own_bishops.square_scan_forward_reset();
                let targets = Bishop::targets(origin, occupancy) & !own_occupancy;
                scores += score_factor * params::MOBILITY_BISHOP[targets.pop_count()];
                #[cfg(feature = "trace")]
                (*self.coeffs.bishop_mobility[targets.pop_count()] += score_factor as i8);
            }
        }
        let all_rooks = pos.piece_type_occupancy(piece::Type::Rook);
        for (own_occupancy, score_factor) in [(white_occupancy, 1), (black_occupancy, -1)] {
            let mut own_rooks = all_rooks & own_occupancy;
            while own_rooks != Bitboard::EMPTY {
                let origin = own_rooks.square_scan_forward_reset();
                let targets = Rook::targets(origin, occupancy) & !own_occupancy;
                scores += score_factor * params::MOBILITY_ROOK[targets.pop_count()];
                #[cfg(feature = "trace")]
                (*self.coeffs.rook_mobility[targets.pop_count()] += score_factor as i8);
            }
        }
        let all_queens = pos.piece_type_occupancy(piece::Type::Queen);
        for (own_occupancy, score_factor) in [(white_occupancy, 1), (black_occupancy, -1)] {
            let mut own_queens = all_queens & own_occupancy;
            while own_queens != Bitboard::EMPTY {
                let origin = own_queens.square_scan_forward_reset();
                let targets = Queen::targets(origin, occupancy) & !own_occupancy;
                scores += score_factor * params::MOBILITY_QUEEN[targets.pop_count()];
                #[cfg(feature = "trace")]
                (*self.coeffs.queen_mobility[targets.pop_count()] += score_factor as i8);
            }
        }

        scores
    }
}

#[cfg(test)]
mod tests {
    use movegen::{
        fen::Fen, move_generator::MoveGenerator, position::Position,
        position_history::PositionHistory, r#move::MoveList,
    };
    use rand::seq::{IndexedMutRandom, SliceRandom};

    use crate::{Eval, EQ_POSITION};

    use super::HandCraftedEval;

    #[test]
    fn draw_by_insufficient_material() {
        let mut evaluator = HandCraftedEval::new();

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

    #[test]
    fn incremental_eval() {
        // Test, if the evaluation for a position is independent from the
        // previously evaluated position
        let mut evaluator = HandCraftedEval::new();
        let position_count = 1000;
        let mut positions_with_evals = Vec::new();
        #[cfg(feature = "trace")]
        let mut positions_with_eval_coeffs = Vec::new();
        let mut pos_hist = PositionHistory::new(Position::initial());
        let mut move_list = MoveList::new();
        let mut rng = rand::rng();
        for _ in 0..position_count {
            let pos = pos_hist.current_pos().clone();
            let static_eval = evaluator.eval(&pos);
            positions_with_evals.push((pos.clone(), static_eval));
            #[cfg(feature = "trace")]
            positions_with_eval_coeffs.push((pos, evaluator.coeffs().clone()));
            MoveGenerator::generate_moves(&mut move_list, pos_hist.current_pos());
            match move_list.choose_mut(&mut rng) {
                Some(m) => pos_hist.do_move(*m),
                None => pos_hist = PositionHistory::new(Position::initial()),
            }
        }
        positions_with_evals.shuffle(&mut rng);
        for (pos, static_eval) in positions_with_evals {
            assert_eq!(evaluator.eval(&pos), static_eval);
        }
        #[cfg(feature = "trace")]
        {
            positions_with_eval_coeffs.shuffle(&mut rng);
            for (pos, coeffs) in positions_with_eval_coeffs {
                evaluator.eval(&pos);
                assert_eq!(evaluator.coeffs(), &coeffs);
            }
        }
    }
}
