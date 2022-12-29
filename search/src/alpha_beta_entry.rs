use crate::search::MAX_SEARCH_DEPTH;
use eval::Score;
use movegen::r#move::Move;
use movegen::transposition_table::Prio;
use movegen::zobrist::Zobrist;
use std::ops::Neg;
use std::{cmp, mem};

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
    age: u8,
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
            self.age(),
        )
    }
}

impl Prio for AlphaBetaEntry {
    fn prio(&self, other: &Self, age: u8) -> cmp::Ordering {
        let halfmoves_since_self = ((age as u16 + 256 - self.age() as u16) % 256) as u8;
        let halfmoves_since_other = ((age as u16 + 256 - other.age() as u16) % 256) as u8;
        match halfmoves_since_self.cmp(&halfmoves_since_other) {
            cmp::Ordering::Less => cmp::Ordering::Less,
            cmp::Ordering::Equal => self.depth().cmp(&other.depth()).reverse(),
            cmp::Ordering::Greater => cmp::Ordering::Greater,
        }
    }

    fn age(&self) -> u8 {
        self.age
    }
}

impl AlphaBetaEntry {
    pub const ENTRY_SIZE: usize = mem::size_of::<Option<(Zobrist, AlphaBetaEntry)>>();

    pub fn new(
        depth: usize,
        score: Score,
        score_type: ScoreType,
        best_move: Move,
        age: u8,
    ) -> Self {
        debug_assert!(depth <= MAX_SEARCH_DEPTH);
        Self {
            depth: depth as u8,
            score,
            score_type,
            best_move,
            age,
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

    pub fn age(&self) -> u8 {
        self.age
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
                        self.age(),
                    ))
                } else if self.score() < alpha {
                    Some(Self::new(
                        self.depth(),
                        alpha,
                        ScoreType::UpperBound,
                        Move::NULL,
                        self.age(),
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
                self.age(),
            )),
            ScoreType::UpperBound if self.score() < alpha => Some(Self::new(
                self.depth(),
                alpha,
                ScoreType::UpperBound,
                Move::NULL,
                self.age(),
            )),
            _ => None,
        }
    }
}
