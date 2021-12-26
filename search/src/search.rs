use crossbeam_channel::{Receiver, Sender};
use eval::eval::Score;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use std::fmt;
use std::ops::Neg;

pub const MAX_SEARCH_DEPTH: usize = u8::MAX as usize;

#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    depth: u8,
    score: Score,
    best_move: Move,
    pv: MoveList,
}

impl SearchResult {
    pub fn new(depth: usize, score: Score, best_move: Move, pv: MoveList) -> Self {
        debug_assert!(depth <= MAX_SEARCH_DEPTH);
        Self {
            depth: depth as u8,
            score,
            best_move,
            pv,
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

    pub fn principal_variation(&self) -> &MoveList {
        &self.pv
    }
}

impl Neg for SearchResult {
    type Output = Self;

    // Changes the sign of the score and leaves the best move unchanged
    fn neg(self) -> Self::Output {
        Self::new(
            self.depth(),
            -self.score(),
            self.best_move(),
            self.principal_variation().clone(),
        )
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
    Search(PositionHistory, usize),
    Stop,
    Terminate,
}

#[derive(Debug)]
pub enum SearchInfo {
    DepthFinished(SearchResult),
    Stopped,
    Terminated,
}

impl fmt::Display for SearchInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SearchInfo::DepthFinished(search_res) => write!(f, "Depth finished: {}", search_res),
            SearchInfo::Stopped => write!(f, "Search stopped"),
            SearchInfo::Terminated => write!(f, "Search terminated"),
        }
    }
}

pub trait Search {
    fn search(
        &mut self,
        pos_history: &mut PositionHistory,
        depth: usize,
        command_receiver: &mut Receiver<SearchCommand>,
        info_sender: &mut Sender<SearchInfo>,
    );
}
