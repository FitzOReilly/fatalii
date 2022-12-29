use crate::search::MAX_SEARCH_DEPTH;
use eval::Score;
use movegen::{r#move::Move, transposition_table::Prio};
use std::{cmp, ops::Neg};

#[derive(Clone, Copy, Debug)]
pub struct NegamaxEntry {
    depth: u8,
    score: Score,
    best_move: Move,
    age: u8,
}

impl NegamaxEntry {
    pub fn new(depth: usize, score: Score, best_move: Move, age: u8) -> Self {
        debug_assert!(depth <= MAX_SEARCH_DEPTH);
        Self {
            depth: depth as u8,
            score,
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

    pub fn best_move(&self) -> Move {
        self.best_move
    }
}

impl Neg for NegamaxEntry {
    type Output = Self;

    // Changes the sign of the score and leaves the rest unchanged
    fn neg(self) -> Self::Output {
        Self::new(self.depth(), -self.score(), self.best_move(), self.age())
    }
}

impl Prio for NegamaxEntry {
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
