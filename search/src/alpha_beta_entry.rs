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
    None = 0,
    Exact = 1,
    LowerBound = 2,
    UpperBound = 3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AlphaBetaEntry {
    depth_age_score_type: u16, // 16 bits: 0-6: depth, 7-13: age, 14-15: score_type
    best_move: Move,           // 16 bits
    score: Score,              // 16 bits
    static_eval: Score,        // 16 bits
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
        Self {
            score: -self.score,
            static_eval: -self.static_eval,
            ..self
        }
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
    fn is_valid(&self) -> bool {
        self.score_type() != ScoreType::None
    }

    fn depth(&self) -> usize {
        (self.depth_age_score_type & 0x7f) as usize
    }

    fn age(&self) -> u8 {
        (self.depth_age_score_type >> 7) as u8 & 0x7f
    }

    fn prio(&self, other: &Self, age: u8) -> Ordering {
        let divisor = (MAX_SEARCH_DEPTH + 1) as u16;
        let halfmoves_since_self = ((age as u16 + divisor - self.age() as u16) % divisor) as u8;
        let halfmoves_since_other = ((age as u16 + divisor - other.age() as u16) % divisor) as u8;
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
        age: u8,
        score_type: ScoreType,
        best_move: Move,
        score: Score,
        static_eval: Score,
    ) -> Self {
        debug_assert!(depth <= MAX_SEARCH_DEPTH);
        Self {
            depth_age_score_type: depth as u16 | (age as u16) << 7 | (score_type as u16) << 14,
            best_move,
            score,
            static_eval,
        }
    }

    pub fn score_type(&self) -> ScoreType {
        unsafe { mem::transmute::<u8, ScoreType>((self.depth_age_score_type >> 14) as u8 & 0x3) }
    }

    pub fn with_score_type(&self, score_type: ScoreType) -> Self {
        Self {
            depth_age_score_type: self.depth_age_score_type & 0x3f | (score_type as u16) << 14,
            ..*self
        }
    }

    pub fn best_move(&self) -> Move {
        self.best_move
    }

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn static_eval(&self) -> Score {
        self.static_eval
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
                    Some(self.with_score_type(ScoreType::LowerBound))
                } else if self.score() < alpha {
                    Some(self.with_score_type(ScoreType::UpperBound))
                } else {
                    Some(*self)
                }
            }
            ScoreType::LowerBound if self.score() >= beta => {
                Some(self.with_score_type(ScoreType::LowerBound))
            }
            ScoreType::UpperBound if self.score() < alpha => {
                Some(self.with_score_type(ScoreType::UpperBound))
            }
            _ => None,
        }
    }
}

impl AlphaBetaResult {
    pub fn new(
        depth: usize,
        age: u8,
        score_type: ScoreType,
        best_move: Move,
        score: Score,
        static_eval: Score,
        should_store: bool,
    ) -> Self {
        debug_assert!(depth <= MAX_SEARCH_DEPTH);
        Self {
            entry: AlphaBetaEntry::new(depth, age, score_type, best_move, score, static_eval),
            should_store,
        }
    }

    pub fn entry(&self) -> AlphaBetaEntry {
        self.entry
    }

    pub fn score_type(&self) -> ScoreType {
        self.entry.score_type()
    }

    pub fn best_move(&self) -> Move {
        self.entry.best_move()
    }

    pub fn score(&self) -> Score {
        self.entry.score()
    }

    pub fn static_eval(&self) -> Score {
        self.entry.static_eval()
    }

    pub fn should_store(&self) -> bool {
        self.should_store
    }

    pub fn bound_soft(&self, alpha: Score, beta: Score) -> Option<Self> {
        self.entry.bound_soft(alpha, beta).map(Self::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depth_age_score_type() {
        for (depth, age, score_type) in [
            (0, 0, ScoreType::Exact),
            (0, 0, ScoreType::LowerBound),
            (0, 0, ScoreType::UpperBound),
            (0, 127, ScoreType::Exact),
            (0, 127, ScoreType::LowerBound),
            (0, 127, ScoreType::UpperBound),
            (127, 0, ScoreType::Exact),
            (127, 0, ScoreType::LowerBound),
            (127, 0, ScoreType::UpperBound),
            (127, 127, ScoreType::Exact),
            (127, 127, ScoreType::LowerBound),
            (127, 127, ScoreType::UpperBound),
        ] {
            let entry = AlphaBetaEntry::new(depth, age, score_type, Move::NULL, 0, 0);
            assert_eq!(depth, entry.depth());
            assert_eq!(age, entry.age());
            assert_eq!(score_type, entry.score_type());
        }
    }

    #[test]
    fn size_of_entry_is_16_bytes() {
        assert_eq!(8, mem::size_of::<AlphaBetaEntry>());
        assert_eq!(16, mem::size_of::<(Zobrist, AlphaBetaEntry)>());
    }
}
