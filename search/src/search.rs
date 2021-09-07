use eval::eval::Score;
use movegen::position_history::PositionHistory;
use movegen::r#move::Move;
use std::fmt;
use std::ops::Neg;

#[derive(Debug, PartialEq)]
pub struct SearchResult {
    depth: u8,
    score: Score,
    best_move: Move,
}

impl SearchResult {
    pub fn new(depth: usize, score: Score, best_move: Move) -> SearchResult {
        debug_assert!(depth < 256);
        SearchResult {
            depth: depth as u8,
            score,
            best_move,
        }
    }

    fn depth(&self) -> usize {
        self.depth as usize
    }

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn best_move(&self) -> Move {
        self.best_move
    }
}

impl Neg for SearchResult {
    type Output = Self;

    // Changes the sign of the score and leaves the best move unchanged
    fn neg(self) -> Self::Output {
        Self::new(self.depth(), -self.score(), self.best_move())
    }
}

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "depth: {}, score: {}, best move: {}",
            self.depth(),
            self.score(),
            self.best_move()
        )
    }
}

pub trait Search {
    fn search(&mut self, pos_history: &mut PositionHistory, depth: usize) -> SearchResult;
}
