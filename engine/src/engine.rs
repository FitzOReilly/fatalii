use crate::best_move_handler::{BestMoveCommand, BestMoveHandler, StopReason};
use crate::engine_out::EngineOut;
use crate::{EngineOptions, Variant};
use crossbeam_channel::{unbounded, Sender};
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use movegen::side::Side;
use search::search::{Search, SearchInfo};
use search::search_params::SearchParamsOptions;
use search::searcher::Searcher;
use search::SearchOptions;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("Engine error: Cannot search without a position")]
    SearchWithoutPosition,
}

pub struct Engine {
    searcher: Searcher,
    pos_hist: Option<PositionHistory>,
    best_move_handler: BestMoveHandler,
    best_move_sender: Sender<BestMoveCommand>,
    engine_options: Arc<Mutex<EngineOptions>>,
}

impl Engine {
    pub fn new(
        search_algo: impl Search + Send + 'static,
        engine_out: impl EngineOut + Send + 'static,
        engine_options: Arc<Mutex<EngineOptions>>,
    ) -> Self {
        let (best_move_sender, best_move_receiver) = unbounded();
        let best_move_handler = BestMoveHandler::new(best_move_receiver, engine_out);
        let best_move_sender_clone = best_move_sender.clone();

        let search_info_callback = Box::new(move |info| match info {
            SearchInfo::DepthFinished(res) => {
                let _ = best_move_sender_clone.send(BestMoveCommand::DepthFinished(res.clone()));
            }
            SearchInfo::Stopped(best_move) => {
                let _ = best_move_sender_clone
                    .send(BestMoveCommand::Stop(StopReason::Finished(best_move)));
            }
            SearchInfo::Terminated => {}
        });

        let searcher = Searcher::new(search_algo, search_info_callback);

        Self {
            searcher,
            pos_hist: None,
            best_move_handler,
            best_move_sender,
            engine_options,
        }
    }

    pub fn hash_size(&self) -> usize {
        match self.engine_options.lock() {
            Ok(opt) => opt.hash_size,
            Err(e) => panic!("{}", e),
        }
    }

    pub fn set_hash_size(&self, bytes: usize) {
        match self.engine_options.lock() {
            Ok(mut opt) => opt.hash_size = bytes,
            Err(e) => panic!("{}", e),
        }
        self.searcher.set_hash_size(bytes);
    }

    pub fn move_overhead(&self) -> Duration {
        match self.engine_options.lock() {
            Ok(opt) => opt.move_overhead,
            Err(e) => panic!("{}", e),
        }
    }

    pub fn set_move_overhead(&mut self, move_overhead: Duration) {
        match self.engine_options.lock() {
            Ok(mut opt) => opt.move_overhead = move_overhead,
            Err(e) => panic!("{}", e),
        };
    }

    pub fn set_search_params(&mut self, search_params: SearchParamsOptions) {
        self.searcher.set_search_params(search_params);
    }

    pub fn variant(&self) -> Variant {
        match self.engine_options.lock() {
            Ok(opt) => opt.variant,
            Err(e) => panic!("{}", e),
        }
    }

    pub fn set_variant(&self, variant: Variant) {
        match self.engine_options.lock() {
            Ok(mut opt) => opt.variant = variant,
            Err(e) => panic!("{}", e),
        };
    }

    pub fn set_position_history(&mut self, pos_hist: Option<PositionHistory>) {
        self.pos_hist = pos_hist;
    }

    pub fn clear_position_history(&mut self) {
        self.pos_hist = None;
        self.searcher.clear_hash_table();
    }

    pub fn search(&mut self, options: SearchOptions) -> Result<(), EngineError> {
        let mut search_options = options.clone();
        search_options.move_overhead = self.move_overhead();
        self.clear_best_move();
        self.set_search_options(options);
        self.search_with_options(search_options)?;
        Ok(())
    }

    pub fn stop(&self) {
        self.stop_best_move_handler();
        self.searcher.stop();
    }

    pub fn position(&self) -> Option<&Position> {
        self.pos_hist
            .as_ref()
            .map(|pos_hist| pos_hist.current_pos())
    }

    fn search_with_options(&mut self, search_options: SearchOptions) -> Result<(), EngineError> {
        match &self.pos_hist {
            Some(pos_hist) => {
                self.set_side_to_move(Some(pos_hist.current_pos().side_to_move()));
                self.searcher.search(pos_hist.clone(), search_options);
                Ok(())
            }
            None => Err(EngineError::SearchWithoutPosition),
        }
    }

    fn clear_best_move(&self) {
        self.searcher.stop();
    }

    fn set_search_options(&self, options: SearchOptions) {
        self.best_move_sender
            .send(BestMoveCommand::SetOptions(Box::new(options)))
            .expect("Error sending BestMoveCommand");
    }

    fn set_side_to_move(&self, side: Option<Side>) {
        self.best_move_sender
            .send(BestMoveCommand::SetSideToMove(side))
            .expect("Error sending BestMoveCommand");
    }

    fn stop_best_move_handler(&self) {
        self.best_move_sender
            .send(BestMoveCommand::Stop(StopReason::Command))
            .expect("Error sending BestMoveCommand");
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.best_move_sender
            .send(BestMoveCommand::Terminate)
            .expect("Error sending BestMoveCommand");
        if let Some(thread) = self.best_move_handler.thread.take() {
            thread
                .join()
                .expect("Error joining best move handler thread");
        }
    }
}
