use movegen::r#move::MoveList;
use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub struct SearchOptions {
    pub search_moves: Option<MoveList>,
    pub ponder: bool,
    pub white_time: Option<Duration>,
    pub black_time: Option<Duration>,
    pub white_inc: Option<Duration>,
    pub black_inc: Option<Duration>,
    pub moves_to_go: Option<usize>,
    pub depth: Option<usize>,
    pub nodes: Option<usize>,
    pub mate_in: Option<usize>,
    pub movetime: Option<Duration>,
    pub infinite: bool,
    pub move_overhead: Duration,
}
