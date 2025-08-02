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
            let passed_pawn_score = Self::passed_pawn_scores(white_pawns, black_pawns);
            let isolated_pawn_score =
                Self::isolated_pawn_count(white_pawns, black_pawns) as i16 * params::ISOLATED_PAWN;
            let backward_pawn_score =
                Self::backward_pawn_count(white_pawns, black_pawns) as i16 * params::BACKWARD_PAWN;
            let doubled_pawn_score =
                Self::doubled_pawn_count(white_pawns, black_pawns) as i16 * params::DOUBLED_PAWN;
            self.scores =
                passed_pawn_score + isolated_pawn_score + backward_pawn_score + doubled_pawn_score;
            self.current_pos = pos.clone();
        }
    }

    pub fn isolated_pawn_count(white_pawns: Bitboard, black_pawns: Bitboard) -> i8 {
        Self::isolated_pawn_count_one_side(white_pawns)
            - Self::isolated_pawn_count_one_side(black_pawns)
    }

    pub fn backward_pawn_count(white_pawns: Bitboard, black_pawns: Bitboard) -> i8 {
        Self::backward_pawn_count_one_side(white_pawns, black_pawns, Side::White)
            - Self::backward_pawn_count_one_side(black_pawns, white_pawns, Side::Black)
    }

    pub fn doubled_pawn_count(white_pawns: Bitboard, black_pawns: Bitboard) -> i8 {
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

    pub fn passed_pawn_scores(white_pawns: Bitboard, black_pawns: Bitboard) -> ScorePair {
        let mut scores = ScorePair(0, 0);
        // Ignore the rank just before promotion (7th for white, 2nd for black).
        // Pawns on these ranks are always passed, so they are already
        // considered by the pawn PSTs.
        let mut white_passed =
            Self::passed_pawns_one_side(white_pawns, black_pawns, Side::White) & !Bitboard::RANK_7;
        while white_passed != Bitboard::EMPTY {
            let square = white_passed.square_scan_forward_reset();
            scores += params::PASSED_PAWN[square.idx()];
        }
        let mut black_passed =
            Self::passed_pawns_one_side(black_pawns, white_pawns, Side::Black) & !Bitboard::RANK_2;
        while black_passed != Bitboard::EMPTY {
            let square = black_passed.square_scan_forward_reset();
            scores -= params::PASSED_PAWN[square.flip_vertical().idx()];
        }
        scores
    }

    pub fn passed_pawns_one_side(
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
        let mut isolated_count = 0;
        let mut own_pawns_mut = own_pawns;
        while own_pawns_mut != Bitboard::EMPTY {
            let pawn = own_pawns_mut.square_scan_forward_reset();
            isolated_count += Self::is_isolated(own_pawns, pawn) as i8;
        }
        isolated_count
    }

    fn is_isolated(own_pawns: Bitboard, pawn: Square) -> bool {
        let pawn_file = Bitboard::from(pawn).file_fill();
        let adjacent_files = pawn_file.east_one() | pawn_file.west_one();
        adjacent_files & own_pawns == Bitboard::EMPTY
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
}
