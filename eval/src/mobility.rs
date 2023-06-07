use movegen::{
    bishop::Bishop, bitboard::Bitboard, knight::Knight, piece, position::Position, queen::Queen,
    rook::Rook, side::Side,
};

use crate::{params, score_pair::ScorePair, Score};

#[derive(Debug, Clone, Default)]
pub struct MobilityCounts {
    pub knight_mob: [i8; params::KNIGHT_MOB_LEN],
    pub bishop_mob: [i8; params::BISHOP_MOB_LEN],
    pub rook_mob: [i8; params::ROOK_MOB_LEN],
    pub queen_mob: [i8; params::QUEEN_MOB_LEN],
}

#[derive(Debug, Clone, Default)]
pub struct Mobility;

impl Mobility {
    pub fn scores(&self, pos: &Position) -> ScorePair {
        let mob_counts = Self::mobility_counts(pos);
        let mut scores = ScorePair(0, 0);

        scores += mob_counts
            .knight_mob
            .iter()
            .zip(&params::MOBILITY_KNIGHT)
            .map(|(n, s)| *n as Score * s)
            .fold(ScorePair(0, 0), |acc, x| acc + x);
        scores += mob_counts
            .bishop_mob
            .iter()
            .zip(&params::MOBILITY_BISHOP)
            .map(|(n, s)| *n as Score * s)
            .fold(ScorePair(0, 0), |acc, x| acc + x);
        scores += mob_counts
            .rook_mob
            .iter()
            .zip(&params::MOBILITY_ROOK)
            .map(|(n, s)| *n as Score * s)
            .fold(ScorePair(0, 0), |acc, x| acc + x);
        scores += mob_counts
            .queen_mob
            .iter()
            .zip(&params::MOBILITY_QUEEN)
            .map(|(n, s)| *n as Score * s)
            .fold(ScorePair(0, 0), |acc, x| acc + x);

        scores
    }

    pub fn mobility_counts(pos: &Position) -> MobilityCounts {
        let mut mob_counts = MobilityCounts::default();
        Self::mobility_counts_one_side(pos, Side::White, &mut mob_counts);
        Self::mobility_counts_one_side(pos, Side::Black, &mut mob_counts);
        mob_counts
    }

    fn mobility_counts_one_side(pos: &Position, side: Side, mob_counts: &mut MobilityCounts) {
        let side_as_int = 1 - 2 * (side as i8);
        let occupancy = pos.occupancy();
        let own_occupancy = pos.side_occupancy(side);

        let mut own_knights = pos.piece_occupancy(side, piece::Type::Knight);
        while own_knights != Bitboard::EMPTY {
            let origin = own_knights.square_scan_forward_reset();
            let targets = Knight::targets(origin) & !own_occupancy;
            mob_counts.knight_mob[targets.pop_count()] += side_as_int;
        }

        let mut own_bishops = pos.piece_occupancy(side, piece::Type::Bishop);
        while own_bishops != Bitboard::EMPTY {
            let origin = own_bishops.square_scan_forward_reset();
            let targets = Bishop::targets(origin, occupancy) & !own_occupancy;
            mob_counts.bishop_mob[targets.pop_count()] += side_as_int;
        }

        let mut own_rooks = pos.piece_occupancy(side, piece::Type::Rook);
        while own_rooks != Bitboard::EMPTY {
            let origin = own_rooks.square_scan_forward_reset();
            let targets = Rook::targets(origin, occupancy) & !own_occupancy;
            mob_counts.rook_mob[targets.pop_count()] += side_as_int;
        }

        let mut own_queens = pos.piece_occupancy(side, piece::Type::Queen);
        while own_queens != Bitboard::EMPTY {
            let origin = own_queens.square_scan_forward_reset();
            let targets = Queen::targets(origin, occupancy) & !own_occupancy;
            mob_counts.queen_mob[targets.pop_count()] += side_as_int;
        }
    }
}
