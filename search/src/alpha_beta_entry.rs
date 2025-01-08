use crate::search::MAX_SEARCH_DEPTH;
use eval::score::{dec_mate_dist_by, inc_mate_dist_by};
use eval::Score;
use movegen::r#move::Move;
use movegen::transposition_table::TtEntry;
use movegen::zobrist::Zobrist;
use std::cmp::Ordering;
use std::mem;
use std::ops::Neg;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlphaBetaResult {
    entry: AlphaBetaEntry,
    should_store: bool,
}

impl From<AlphaBetaEntry> for AlphaBetaResult {
    fn from(entry: AlphaBetaEntry) -> Self {
        Self {
            entry,
            should_store: true,
        }
    }
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

impl Neg for AlphaBetaResult {
    type Output = Self;

    // Changes the sign of the score and leaves the rest unchanged
    fn neg(self) -> Self::Output {
        Self {
            entry: -self.entry,
            ..self
        }
    }
}

impl TtEntry for AlphaBetaEntry {
    fn depth(&self) -> usize {
        self.depth as usize
    }

    fn age(&self) -> u8 {
        self.age
    }

    fn prio(&self, other: &Self, age: u8) -> Ordering {
        let halfmoves_since_self = ((age as u16 + 256 - self.age() as u16) % 256) as u8;
        let halfmoves_since_other = ((age as u16 + 256 - other.age() as u16) % 256) as u8;
        // Prioritize newer entries
        let age_cmp = halfmoves_since_self.cmp(&halfmoves_since_other);
        if let Ordering::Less | Ordering::Greater = age_cmp {
            return age_cmp;
        }
        // Prioritize PV nodes (if they are from the current search)
        if halfmoves_since_self == 0 {
            if self.score_type() == ScoreType::Exact && other.score_type() != ScoreType::Exact {
                return Ordering::Less;
            }
            if other.score_type() == ScoreType::Exact && self.score_type() != ScoreType::Exact {
                return Ordering::Greater;
            }
        }
        // Prioritize entries with higher search depth
        let depth_cmp = self.depth().cmp(&other.depth());
        if let Ordering::Less | Ordering::Greater = depth_cmp {
            return depth_cmp.reverse();
        }
        // Prioritize: PV nodes > Cut nodes > All nodes
        self.score_type().cmp(&other.score_type())
    }
}

impl AlphaBetaEntry {
    pub const ENTRY_SIZE: usize = mem::size_of::<Option<(Zobrist, Self)>>();

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

    pub fn with_increased_mate_distance(&self, plies: usize) -> Self {
        Self {
            score: inc_mate_dist_by(self.score, plies),
            ..*self
        }
    }

    pub fn with_decreased_mate_distance(&self, plies: usize) -> Self {
        Self {
            score: dec_mate_dist_by(self.score, plies),
            ..*self
        }
    }

    pub fn bound_soft(&self, alpha: Score, beta: Score) -> Option<Self> {
        match self.score_type() {
            ScoreType::Exact => {
                if self.score() >= beta {
                    Some(Self::new(
                        self.depth(),
                        self.score(),
                        ScoreType::LowerBound,
                        self.best_move(),
                        self.age(),
                    ))
                } else if self.score() < alpha {
                    Some(Self::new(
                        self.depth(),
                        self.score(),
                        ScoreType::UpperBound,
                        self.best_move(),
                        self.age(),
                    ))
                } else {
                    Some(*self)
                }
            }
            ScoreType::LowerBound if self.score() >= beta => Some(Self::new(
                self.depth(),
                self.score(),
                ScoreType::LowerBound,
                self.best_move(),
                self.age(),
            )),
            ScoreType::UpperBound if self.score() < alpha => Some(Self::new(
                self.depth(),
                self.score(),
                ScoreType::UpperBound,
                self.best_move(),
                self.age(),
            )),
            _ => None,
        }
    }
}

impl AlphaBetaResult {
    pub fn new(
        depth: usize,
        score: Score,
        score_type: ScoreType,
        best_move: Move,
        age: u8,
        should_store: bool,
    ) -> Self {
        debug_assert!(depth <= MAX_SEARCH_DEPTH);
        Self {
            entry: AlphaBetaEntry::new(depth, score, score_type, best_move, age),
            should_store,
        }
    }

    pub fn entry(&self) -> AlphaBetaEntry {
        self.entry
    }

    pub fn score(&self) -> Score {
        self.entry.score()
    }

    pub fn score_type(&self) -> ScoreType {
        self.entry.score_type()
    }

    pub fn best_move(&self) -> Move {
        self.entry.best_move()
    }

    pub fn should_store(&self) -> bool {
        self.should_store
    }

    pub fn bound_soft(&self, alpha: Score, beta: Score) -> Option<Self> {
        self.entry.bound_soft(alpha, beta).map(Self::from)
    }
}
