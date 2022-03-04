use crate::search::MAX_SEARCH_DEPTH;
use eval::Score;
use movegen::r#move::Move;
use movegen::zobrist::Zobrist;
use std::mem;
use std::ops::Neg;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ScoreType {
    Exact = 0,
    LowerBound = 1,
    UpperBound = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlphaBetaEntry {
    depth: u8,
    score: Score,
    score_type: ScoreType,
    best_move: Move,
}

impl Neg for AlphaBetaEntry {
    type Output = Self;

    // Changes the sign of the score and leaves the rest unchanged
    fn neg(self) -> Self::Output {
        Self::new(
            self.depth(),
            -self.score(),
            self.score_type(),
            self.best_move(),
        )
    }
}

impl AlphaBetaEntry {
    pub const ENTRY_SIZE: usize = mem::size_of::<Option<(Zobrist, AlphaBetaEntry)>>();

    pub fn new(depth: usize, score: Score, score_type: ScoreType, best_move: Move) -> Self {
        debug_assert!(depth <= MAX_SEARCH_DEPTH);
        Self {
            depth: depth as u8,
            score,
            score_type,
            best_move,
        }
    }

    pub fn depth(&self) -> usize {
        self.depth as usize
    }

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn score_type(&self) -> ScoreType {
        self.score_type
    }

    pub fn best_move(&self) -> Move {
        self.best_move
    }

    pub fn bound_hard(&self, alpha: Score, beta: Score) -> Option<Self> {
        match self.score_type() {
            ScoreType::Exact => {
                if self.score() >= beta {
                    Some(Self::new(
                        self.depth(),
                        beta,
                        ScoreType::LowerBound,
                        Move::NULL,
                    ))
                } else if self.score() < alpha {
                    Some(Self::new(
                        self.depth(),
                        alpha,
                        ScoreType::UpperBound,
                        Move::NULL,
                    ))
                } else {
                    Some(*self)
                }
            }
            ScoreType::LowerBound if self.score() >= beta => Some(Self::new(
                self.depth(),
                beta,
                ScoreType::LowerBound,
                Move::NULL,
            )),
            ScoreType::UpperBound if self.score() < alpha => Some(Self::new(
                self.depth(),
                alpha,
                ScoreType::UpperBound,
                Move::NULL,
            )),
            _ => None,
        }
    }
}
