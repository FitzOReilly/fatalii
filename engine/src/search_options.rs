use std::time::Duration;

#[derive(Clone, Copy, Debug, Default)]
pub struct SearchOptions {
    pub white_time: Option<Duration>,
    pub black_time: Option<Duration>,
    pub white_inc: Option<Duration>,
    pub black_inc: Option<Duration>,
    pub depth: Option<usize>,
    pub movetime: Option<Duration>,
    pub infinite: bool,
}
