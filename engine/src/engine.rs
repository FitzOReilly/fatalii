use crate::best_move_handler::{BestMoveCommand, BestMoveHandler, StopReason};
use crate::search_options::SearchOptions;
use crate::timer::{Timer, TimerCommand};
use crossbeam_channel::{unbounded, Sender};
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use movegen::side::Side;
use search::search::{Search, SearchInfo, SearchResult, MAX_SEARCH_DEPTH};
use search::searcher::Searcher;
use std::cmp;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
    best_move_handler: BestMoveHandler,
    best_move_sender: Sender<BestMoveCommand>,
    timer: Timer,
    timer_sender: Sender<TimerCommand>,
    move_overhead: Duration,
}

impl Engine {
    pub fn new(
        search_algo: impl Search + Send + 'static,
        search_info_callback: Box<dyn FnMut(Option<SearchResult>) + Send>,
        best_move_callback: Box<dyn FnMut(Option<SearchResult>) + Send>,
    ) -> Self {
        let search_result = Arc::new(Mutex::new(None));
        let (best_move_sender, best_move_receiver) = unbounded();
        let best_move_handler = BestMoveHandler::new(
            Arc::clone(&search_result),
            best_move_receiver,
            search_info_callback,
            best_move_callback,
        );
        let best_move_sender_clone = best_move_sender.clone();

        let (timer_sender, timer_command_receiver) = unbounded();

        let search_info_callback = Box::new(move |info| match info {
            SearchInfo::DepthFinished(res) => {
                let _ = best_move_sender_clone.send(BestMoveCommand::DepthFinished(res.clone()));
                match search_result.lock() {
                    Ok(mut data) => *data = Some(res),
                    Err(e) => panic!("{}", e),
                }
            }
            SearchInfo::Stopped => {
                let _ = best_move_sender_clone.send(BestMoveCommand::Stop(StopReason::Finished));
            }
            SearchInfo::Terminated => {}
        });

        let searcher = Searcher::new(search_algo, search_info_callback);

        let search_command_sender = searcher.clone_command_sender();
        let timer = Timer::new(timer_command_receiver, search_command_sender);

        Self {
            searcher,
            pos_hist: None,
            best_move_handler,
            best_move_sender,
            timer,
            timer_sender,
            move_overhead: Duration::from_millis(0),
        }
    }

    pub fn set_hash_size(&self, bytes: usize) {
        self.searcher.set_hash_size(bytes);
    }

    pub fn set_move_overhead(&mut self, move_overhead: Duration) {
        self.move_overhead = move_overhead;
    }

    pub fn set_position_history(&mut self, pos_hist: Option<PositionHistory>) {
        self.pos_hist = pos_hist;
    }

    pub fn search(&mut self, options: SearchOptions) -> Result<(), EngineError> {
        let opt_dur = options.movetime;
        let opt_white_time = options.white_time;
        let opt_black_time = options.black_time;
        let opt_depth = options.depth;
        self.clear_best_move();
        self.set_search_options(options);
        let depth = opt_depth.unwrap_or(MAX_SEARCH_DEPTH);
        self.search_depth(depth)?;
        if let Some(dur) = opt_dur {
            self.start_timer(dur);
        } else if let Some(pos) = self.position() {
            match pos.side_to_move() {
                Side::White => {
                    if let Some(time) = opt_white_time {
                        self.start_timer(self.calc_movetime(time))
                    }
                }
                Side::Black => {
                    if let Some(time) = opt_black_time {
                        self.start_timer(self.calc_movetime(time))
                    }
                }
            }
        }
        Ok(())
    }

    pub fn stop(&self) {
        self.stop_best_move_handler();
        self.stop_timer();
        self.searcher.stop();
    }

    pub fn position(&self) -> Option<&Position> {
        self.pos_hist
            .as_ref()
            .map(|pos_hist| pos_hist.current_pos())
    }

    fn search_depth(&mut self, depth: usize) -> Result<(), EngineError> {
        match &self.pos_hist {
            Some(pos_hist) => {
                self.set_side_to_move(Some(pos_hist.current_pos().side_to_move()));
                self.searcher.search(pos_hist.clone(), depth);
                Ok(())
            }
            None => Err(EngineError::SearchWithoutPosition),
        }
    }

    fn clear_best_move(&self) {
        self.clear_best_move_handler();
        self.stop_timer();
        self.searcher.stop();
    }

    fn clear_best_move_handler(&self) {
        self.best_move_sender
            .send(BestMoveCommand::Clear)
            .expect("Error sending BestMoveCommand");
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

    fn start_timer(&self, dur: Duration) {
        self.timer_sender
            .send(TimerCommand::Start(dur))
            .expect("Error sending TimerCommand");
    }

    fn stop_timer(&self) {
        self.timer_sender
            .send(TimerCommand::Stop)
            .expect("Error sending TimerCommand");
    }

    fn calc_movetime(&self, time: Duration) -> Duration {
        const MIN_TIME: Duration = Duration::from_millis(0);
        const MAX_TIME: Duration = Duration::from_secs(60);
        const MAX_FRACTION: f64 = 8.0;
        match time.checked_sub(self.move_overhead) {
            Some(t) => cmp::min(MAX_TIME, cmp::max(MIN_TIME, t.div_f64(MAX_FRACTION))),
            None => MIN_TIME,
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.timer_sender
            .send(TimerCommand::Terminate)
            .expect("Error sending TimerCommand");
        if let Some(thread) = self.timer.thread.take() {
            thread.join().expect("Error joining timer thread");
        }

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
