use std::cmp;

use movegen::piece::{self, Piece};

const KING_PHASE: usize = 0;
const QUEEN_PHASE: usize = 4;
const ROOK_PHASE: usize = 2;
const BISHOP_PHASE: usize = 1;
const KNIGHT_PHASE: usize = 1;
const PAWN_PHASE: usize = 0;

#[derive(Debug, Clone, Default)]
pub struct GamePhase(usize);

impl GamePhase {
    pub const MAX: usize = 2
        * (KING_PHASE
            + QUEEN_PHASE
            + 2 * ROOK_PHASE
            + 2 * BISHOP_PHASE
            + 2 * KNIGHT_PHASE
            + 8 * PAWN_PHASE);

    pub fn game_phase_clamped(&self) -> usize {
        cmp::min(Self::MAX, self.0)
    }

    pub fn add_piece(&mut self, pt: piece::Type) {
        self.0 += match pt {
            piece::Type::Pawn => PAWN_PHASE,
            piece::Type::Knight => KNIGHT_PHASE,
            piece::Type::Bishop => BISHOP_PHASE,
            piece::Type::Rook => ROOK_PHASE,
            piece::Type::Queen => QUEEN_PHASE,
            piece::Type::King => KING_PHASE,
        };
    }

    pub fn remove_piece(&mut self, pt: piece::Type) {
        self.0 -= match pt {
            piece::Type::Pawn => PAWN_PHASE,
            piece::Type::Knight => KNIGHT_PHASE,
            piece::Type::Bishop => BISHOP_PHASE,
            piece::Type::Rook => ROOK_PHASE,
            piece::Type::Queen => QUEEN_PHASE,
            piece::Type::King => KING_PHASE,
        };
    }
}

#[derive(Debug, Clone, Default)]
pub struct PieceCounts([usize; Piece::NUM_PIECES]);

impl PieceCounts {
    pub fn count(&self, p: Piece) -> usize {
        self.0[p.idx()]
    }

    pub fn add(&mut self, p: Piece) {
        debug_assert!(self.0[p.idx()] <= 9);
        self.0[p.idx()] += 1;
    }

    pub fn remove(&mut self, p: Piece) {
        debug_assert_ne!(0, self.0[p.idx()]);
        self.0[p.idx()] -= 1;
    }
}
