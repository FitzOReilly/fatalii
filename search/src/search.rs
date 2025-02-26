use crate::search_params::SearchParamsOptions;
use crate::SearchOptions;
use crossbeam_channel::{Receiver, Sender};
use eval::Score;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use std::fmt;
use std::ops::Neg;

pub const MAX_SEARCH_DEPTH: usize = 127;
pub const REPETITIONS_TO_DRAW: usize = 3;
pub const PLIES_WITHOUT_PAWN_MOVE_OR_CAPTURE_TO_DRAW: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResult {
    depth: u8,
    selective_depth: u8,
    score: Score,
    nodes: u64,
    time_us: u64,
    hash_load_factor_permille: u16,
    best_move: Move,
    pv: MoveList,
}

impl SearchResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        depth: usize,
        selective_depth: usize,
        score: Score,
        nodes: u64,
        time_us: u64,
        hash_load_factor_permille: u16,
        best_move: Move,
        pv: MoveList,
    ) -> Self {
        debug_assert!(depth <= MAX_SEARCH_DEPTH);
        Self {
            depth: depth as u8,
            selective_depth: selective_depth as u8,
            score,
            nodes,
            time_us,
            hash_load_factor_permille,
            best_move,
            pv,
        }
    }

    pub fn depth(&self) -> usize {
        self.depth as usize
    }

    pub fn selective_depth(&self) -> usize {
        self.selective_depth as usize
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

    pub fn hash_load_factor_permille(&self) -> u16 {
        self.hash_load_factor_permille
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
            self.selective_depth(),
            -self.score(),
            self.nodes(),
            self.time_us(),
            self.hash_load_factor_permille(),
            self.best_move(),
            self.principal_variation().clone(),
        )
    }
}

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "depth: {}, selective depth: {}, score: {}, nodes: {}, time in us: {}, hash load factor permille: {}, best move: {}",
            self.depth(),
            self.selective_depth(),
            self.score(),
            self.nodes(),
            self.time_us(),
            self.hash_load_factor_permille(),
            self.best_move()
        )
    }
}

#[derive(Debug)]
pub enum SearchCommand {
    SetHashSize(usize, Sender<()>),
    ClearHashTable(Sender<()>),
    SetSearchParams(SearchParamsOptions, Sender<()>),
    Search(Box<(PositionHistory, SearchOptions)>),
    Stop,
    Terminate,
}

#[derive(Debug)]
pub enum SearchInfo {
    DepthFinished(SearchResult),
    Stopped(Move),
    Terminated,
}

impl fmt::Display for SearchInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SearchInfo::DepthFinished(search_res) => write!(f, "Depth finished: {search_res}"),
            SearchInfo::Stopped(best_move) => write!(f, "Search stopped: {best_move}"),
            SearchInfo::Terminated => write!(f, "Search terminated"),
        }
    }
}

pub trait Search {
    fn set_hash_size(&mut self, bytes: usize);

    fn clear_hash_table(&mut self);

    fn set_params(&mut self, params: SearchParamsOptions);

    fn search(
        &mut self,
        pos_history: PositionHistory,
        search_options: SearchOptions,
        command_receiver: &Receiver<SearchCommand>,
        info_sender: &Sender<SearchInfo>,
    );
}
