use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct SearchOptions {
    pub depth: Option<usize>,
    pub movetime: Option<Duration>,
    pub infinite: bool,
}

impl SearchOptions {
    pub fn new() -> Self {
        Self {
            depth: None,
            movetime: None,
            infinite: false,
        }
    }
}

impl Default for SearchOptions {
    fn default() -> Self {
        SearchOptions::new()
    }
}
