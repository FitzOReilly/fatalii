use eval::eval::Score;
use movegen::position_history::PositionHistory;
use movegen::r#move::Move;
use std::ops::Neg;

#[derive(Debug, PartialEq)]
pub struct SearchResult {
    score: Score,
    best_move: Move,
}

impl SearchResult {
    pub fn new(score: Score, best_move: Move) -> SearchResult {
        SearchResult { score, best_move }
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
        Self::new(-self.score(), self.best_move())
    }
}

pub trait Search {
    fn search(&mut self, pos_history: &mut PositionHistory, depth: usize) -> SearchResult;
}
