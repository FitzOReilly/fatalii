use crate::search::MAX_SEARCH_DEPTH;
use eval::eval::Score;
use movegen::r#move::Move;
use std::ops::Neg;

#[derive(Clone, Copy, Debug)]
pub struct NegamaxEntry {
    depth: u8,
    score: Score,
    best_move: Move,
}

impl NegamaxEntry {
    pub fn new(depth: usize, score: Score, best_move: Move) -> Self {
        debug_assert!(depth <= MAX_SEARCH_DEPTH);
        Self {
            depth: depth as u8,
            score,
            best_move,
        }
    }

    pub fn depth(&self) -> usize {
        self.depth as usize
    }

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn best_move(&self) -> Move {
        self.best_move
    }
}

impl Neg for NegamaxEntry {
    type Output = Self;

    // Changes the sign of the score and leaves the rest unchanged
    fn neg(self) -> Self::Output {
        Self::new(self.depth(), -self.score(), self.best_move())
    }
}
