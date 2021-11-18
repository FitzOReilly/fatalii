use crossbeam_channel::{unbounded, Sender};
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use movegen::r#move::Move;
use search::search::{Search, SearchCommand, SearchInfo, MAX_SEARCH_DEPTH};
use search::searcher::Searcher;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fmt, thread};

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
    timer: Option<thread::JoinHandle<()>>,
    timer_sender: Sender<TimerCommand>,
}

impl Engine {
    pub fn new(
        search_algo: impl Search + Send + 'static,
        best_move_callback: Box<dyn Fn(Option<Move>) + Send>,
    ) -> Self {
        let best_move = Arc::new(Mutex::new(None));
        let (timer_sender, timer_receiver) = unbounded();
        let timer_sender_clone = timer_sender.clone();
        let searcher = Searcher::new(
            search_algo,
            Box::new(move |info| match info {
                SearchInfo::DepthFinished(res) => match best_move.lock() {
                    Ok(mut m) => *m = Some(res.best_move()),
                    Err(e) => panic!("{}", e),
                },
                SearchInfo::Stopped => match best_move.lock() {
                    // Make sure that the timer is stopped. This is for cases in which the search
                    // finished earlier than the given time, e.g. checkmate positions.
                    Ok(m) => {
                        timer_sender_clone
                            .send(TimerCommand::Stop)
                            .expect("Error sending TimerCommand");
                        best_move_callback(*m);
                    }
                    Err(e) => panic!("{}", e),
                },
                SearchInfo::Terminated => {}
            }),
        );

        let command_sender = searcher.clone_command_sender();
        let timer = thread::spawn(move || loop {
            let message = timer_receiver.recv().expect("Error receiving TimerCommand");
            match message {
                TimerCommand::Start(dur) => {
                    if timer_receiver.recv_timeout(dur).is_err() {
                        command_sender
                            .send(SearchCommand::Stop)
                            .expect("Error sending SearchCommand");
                    }
                }
                TimerCommand::Stop => {}
                TimerCommand::Terminate => break,
            }
        });

        Self {
            searcher,
            pos_hist: None,
            timer: Some(timer),
            timer_sender,
        }
    }

    pub fn set_position_history(&mut self, pos_hist: Option<PositionHistory>) {
        self.pos_hist = pos_hist;
    }

    pub fn search_depth(&mut self, depth: usize) -> Result<(), EngineError> {
        match &self.pos_hist {
            Some(pos_hist) => {
                self.searcher.search(pos_hist.clone(), depth);
                Ok(())
            }
            None => Err(EngineError::SearchWithoutPosition),
        }
    }

    pub fn search_timeout(&mut self, dur: Duration) -> Result<(), EngineError> {
        let res = self.search_infinite();
        if res.is_ok() {
            self.timer_sender
                .send(TimerCommand::Start(dur))
                .expect("Error sending TimerCommand");
        };
        res
    }

    pub fn search_infinite(&mut self) -> Result<(), EngineError> {
        self.search_depth(MAX_SEARCH_DEPTH)
    }

    pub fn stop(&mut self) {
        self.timer_sender
            .send(TimerCommand::Stop)
            .expect("Error sending TimerCommand");
        self.searcher.stop();
    }

    pub fn position(&self) -> Option<&Position> {
        self.pos_hist
            .as_ref()
            .map(|pos_hist| pos_hist.current_pos())
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.stop();
        self.timer_sender
            .send(TimerCommand::Terminate)
            .expect("Error sending TimerCommand");
        if let Some(thread) = self.timer.take() {
            thread.join().expect("Error joining timer thread");
        }
    }
}

#[derive(Debug)]
pub enum TimerCommand {
    Start(Duration),
    Stop,
    Terminate,
}
