use movegen::position::Position;
use movegen::position_history::PositionHistory;
use movegen::r#move::Move;
use search::search::{Search, SearchInfo};
use search::searcher::Searcher;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum EngineError {
    SearchWithoutPosition,
}

impl Error for EngineError {}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            EngineError::SearchWithoutPosition => "Cannot search without a position".to_string(),
        };
        write!(f, "Engine error: {}", msg)
    }
}

pub struct Engine {
    searcher: Searcher,
    pos_hist: Option<PositionHistory>,
    best_move: Arc<Mutex<Option<Move>>>,
}

impl Engine {
    pub fn new(search_algo: impl Search + Send + 'static) -> Self {
        let best_move = Arc::new(Mutex::new(None));
        let best_move_clone = Arc::clone(&best_move);
        Self {
            searcher: Searcher::new(
                search_algo,
                Box::new(move |info| {
                    if let SearchInfo::DepthFinished(res) = info {
                        match best_move_clone.lock() {
                            Ok(mut data) => *data = Some(res.best_move()),
                            Err(e) => panic!("{}", e),
                        }
                    }
                }),
            ),
            pos_hist: None,
            best_move,
        }
    }

    pub fn set_position_history(&mut self, pos_hist: Option<PositionHistory>) {
        self.pos_hist = pos_hist;
    }

    pub fn search(&mut self) -> Result<(), EngineError> {
        match &self.pos_hist {
            Some(pos_hist) => {
                self.searcher.search(pos_hist.clone());
                Ok(())
            }
            None => Err(EngineError::SearchWithoutPosition),
        }
    }

    pub fn stop(&mut self) {
        self.searcher.stop();
    }

    pub fn position(&self) -> Option<&Position> {
        self.pos_hist
            .as_ref()
            .map(|pos_hist| pos_hist.current_pos())
    }

    pub fn best_move(&self) -> Option<Move> {
        match self.best_move.lock() {
            Ok(data) => *data,
            Err(e) => panic!("{}", e),
        }
    }
}
