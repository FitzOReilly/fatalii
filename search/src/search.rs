use crossbeam_channel::{Receiver, Sender};
use eval::eval::Score;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use std::fmt;
use std::ops::Neg;

pub const MAX_SEARCH_DEPTH: usize = u8::MAX as usize;
pub const REPETITIONS_TO_DRAW: usize = 3;

#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    depth: u8,
    score: Score,
    nodes: u64,
    time_us: u64,
    best_move: Move,
    pv: MoveList,
}

impl SearchResult {
    pub fn new(
        depth: usize,
        score: Score,
        nodes: u64,
        time_us: u64,
        best_move: Move,
        pv: MoveList,
    ) -> Self {
        debug_assert!(depth <= MAX_SEARCH_DEPTH);
        Self {
            depth: depth as u8,
            score,
            nodes,
            time_us,
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

    pub fn nodes(&self) -> u64 {
        self.nodes
    }

    pub fn time_ms(&self) -> u64 {
        self.time_us / 1000
    }

    pub fn time_us(&self) -> u64 {
        self.time_us
    }

    pub fn nodes_per_second(&self) -> i32 {
        match self.time_us() {
            0 => -1,
            _ => (1_000_000 * self.nodes() / self.time_us()) as i32,
        }
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
            self.nodes(),
            self.time_us(),
            self.best_move(),
            self.principal_variation().clone(),
        )
    }
}

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "depth: {}, score: {}, nodes: {}, time in us: {}, best move: {}",
            self.depth(),
            self.score(),
            self.nodes(),
            self.time_us(),
            self.best_move()
        )
    }
}

#[derive(Debug)]
pub enum SearchCommand {
    SetHashSize(usize),
    Search(Box<(PositionHistory, usize)>),
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
    fn set_hash_size(&mut self, bytes: usize);

    fn search(
        &mut self,
        pos_history: PositionHistory,
        max_depth: usize,
        command_receiver: &Receiver<SearchCommand>,
        info_sender: &Sender<SearchInfo>,
    );
}
