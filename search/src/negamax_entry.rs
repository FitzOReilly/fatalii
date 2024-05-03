use crate::search::MAX_SEARCH_DEPTH;
use eval::{
    score::{dec_mate_dist_by, inc_mate_dist_by},
    Score,
};
use movegen::{r#move::Move, transposition_table::TtEntry};
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

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn best_move(&self) -> Move {
        self.best_move
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
}

impl Neg for NegamaxEntry {
    type Output = Self;

    // Changes the sign of the score and leaves the rest unchanged
    fn neg(self) -> Self::Output {
        Self::new(self.depth(), -self.score(), self.best_move(), self.age())
    }
}

impl TtEntry for NegamaxEntry {
    fn depth(&self) -> usize {
        self.depth as usize
    }

    fn age(&self) -> u8 {
        self.age
    }

    fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    fn prio(&self, other: &Self, age: u8) -> cmp::Ordering {
        let halfmoves_since_self = ((age as u16 + 256 - self.age() as u16) % 256) as u8;
        let halfmoves_since_other = ((age as u16 + 256 - other.age() as u16) % 256) as u8;
        match halfmoves_since_self.cmp(&halfmoves_since_other) {
            cmp::Ordering::Less => cmp::Ordering::Less,
            cmp::Ordering::Equal => self.depth().cmp(&other.depth()).reverse(),
            cmp::Ordering::Greater => cmp::Ordering::Greater,
        }
    }
}
