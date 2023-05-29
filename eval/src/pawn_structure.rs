use crate::params;
use crate::score_pair::ScorePair;

use movegen::bitboard::Bitboard;
use movegen::pawn::Pawn;
use movegen::piece;
use movegen::position::Position;
use movegen::side::Side;
use movegen::square::Square;

#[derive(Debug, Clone)]
pub struct PawnStructure {
    current_pos: Position,
    scores: ScorePair,
}

impl Default for PawnStructure {
    fn default() -> Self {
        Self::new()
    }
}

impl PawnStructure {
    pub fn new() -> Self {
        Self {
            current_pos: Position::empty(),
            scores: ScorePair(0, 0),
        }
    }

    pub fn scores(&self) -> ScorePair {
        self.scores
    }

    pub fn update(&mut self, pos: &Position) {
        let old_white_pawns = self
            .current_pos
            .piece_occupancy(Side::White, piece::Type::Pawn);
        let new_white_pawns = pos.piece_occupancy(Side::White, piece::Type::Pawn);
        let old_black_pawns = self
            .current_pos
            .piece_occupancy(Side::Black, piece::Type::Pawn);
        let new_black_pawns = pos.piece_occupancy(Side::Black, piece::Type::Pawn);

        if old_white_pawns != new_white_pawns || old_black_pawns != new_black_pawns {
            let white_pawns = pos.piece_occupancy(Side::White, piece::Type::Pawn);
            let black_pawns = pos.piece_occupancy(Side::Black, piece::Type::Pawn);
            let passed_pawn_score =
                Self::passed_pawn_count(white_pawns, black_pawns) as i16 * params::PASSED_PAWN;
            let isolated_pawn_score =
                Self::isolated_pawn_count(white_pawns, black_pawns) as i16 * params::ISOLATED_PAWN;
            self.scores = passed_pawn_score + isolated_pawn_score;
            self.current_pos = pos.clone();
        }
    }

    pub fn passed_pawn_count(white_pawns: Bitboard, black_pawns: Bitboard) -> i8 {
        Self::passed_pawn_count_one_side(white_pawns, black_pawns, Side::White)
            - Self::passed_pawn_count_one_side(black_pawns, white_pawns, Side::Black)
    }

    pub fn isolated_pawn_count(white_pawns: Bitboard, black_pawns: Bitboard) -> i8 {
        Self::isolated_pawn_count_one_side(white_pawns)
            - Self::isolated_pawn_count_one_side(black_pawns)
    }

    fn passed_pawn_count_one_side(
        own_pawns: Bitboard,
        opp_pawns: Bitboard,
        side_to_move: Side,
    ) -> i8 {
        let all_pawns = own_pawns | opp_pawns;
        let opp_pawn_attack_targets = Pawn::attack_targets(opp_pawns, !side_to_move);

        let mut passed_count = 0;
        let mut own_pawns_mut = own_pawns;
        while own_pawns_mut != Bitboard::EMPTY {
            let pawn = own_pawns_mut.square_scan_forward_reset();
            passed_count +=
                Self::is_passed(all_pawns, opp_pawn_attack_targets, pawn, side_to_move) as i8;
        }
        passed_count
    }

    fn is_passed(
        all_pawns: Bitboard,
        opp_pawn_attack_targets: Bitboard,
        pawn: Square,
        side_to_move: Side,
    ) -> bool {
        let pawn_bb = Bitboard::from_square(pawn);
        Pawn::front_span(pawn_bb, side_to_move) & (all_pawns) == Bitboard::EMPTY
            && Pawn::front_fill(pawn_bb, side_to_move) & opp_pawn_attack_targets == Bitboard::EMPTY
    }

    fn isolated_pawn_count_one_side(own_pawns: Bitboard) -> i8 {
        let mut isolated_count = 0;
        let mut own_pawns_mut = own_pawns;
        while own_pawns_mut != Bitboard::EMPTY {
            let pawn = own_pawns_mut.square_scan_forward_reset();
            isolated_count += Self::is_isolated(own_pawns, pawn) as i8;
        }
        isolated_count
    }

    fn is_isolated(own_pawns: Bitboard, pawn: Square) -> bool {
        let pawn_file = Bitboard::from_square(pawn).file_fill();
        let adjacent_files = pawn_file.east_one() | pawn_file.west_one();
        adjacent_files & own_pawns == Bitboard::EMPTY
    }
}
