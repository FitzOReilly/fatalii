use eval::eval::Score;
use movegen::position_history::PositionHistory;
use movegen::r#move::Move;
use std::fmt;
use std::ops::Neg;
use std::sync::mpsc;

pub const MAX_SEARCH_DEPTH: usize = u8::MAX as usize;

#[derive(Debug, PartialEq)]
pub struct SearchResult {
    depth: u8,
    score: Score,
    best_move: Move,
}

impl SearchResult {
    pub fn new(depth: usize, score: Score, best_move: Move) -> SearchResult {
        debug_assert!(depth <= MAX_SEARCH_DEPTH);
        SearchResult {
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

#[derive(Debug)]
pub enum SearchCommand {
    Search(PositionHistory),
    Stop,
    Terminate,
}

#[derive(Debug)]
pub enum SearchInfo {
    SearchFinished(SearchResult),
    Stopped,
    Terminated,
}

pub trait Search {
    fn search(
        &mut self,
        pos_history: &mut PositionHistory,
        command_receiver: &mut mpsc::Receiver<SearchCommand>,
        info_sender: &mut mpsc::Sender<SearchInfo>,
    );
}
