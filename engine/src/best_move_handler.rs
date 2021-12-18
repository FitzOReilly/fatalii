use crate::search_options::SearchOptions;
use crossbeam_channel::Receiver;
use movegen::r#move::Move;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct BestMoveHandler {
    pub thread: Option<thread::JoinHandle<()>>,
}

impl BestMoveHandler {
    pub fn new(
        best_move: Arc<Mutex<Option<Move>>>,
        receiver: Receiver<BestMoveCommand>,
        mut best_move_callback: Box<dyn FnMut(Option<Move>) + Send>,
    ) -> Self {
        let options = Arc::new(Mutex::new(SearchOptions::new()));

        let thread = thread::spawn(move || loop {
            let message = receiver.recv().expect("Error receiving BestMoveCommand");
            match message {
                BestMoveCommand::Clear => match best_move.lock() {
                    Ok(mut m) => *m = None,
                    Err(e) => panic!("{}", e),
                },
                BestMoveCommand::SetOptions(new_options) => match options.lock() {
                    Ok(mut opt) => *opt = new_options,
                    Err(e) => panic!("{}", e),
                },
                BestMoveCommand::Stop(StopReason::Command) => match best_move.lock() {
                    Ok(mut m) => best_move_callback(m.take()),
                    Err(e) => panic!("{}", e),
                },
                BestMoveCommand::Stop(StopReason::Finished) => match options.lock() {
                    Ok(opt) => {
                        if !opt.infinite {
                            match best_move.lock() {
                                Ok(mut m) => best_move_callback(m.take()),
                                Err(e) => panic!("{}", e),
                            }
                        }
                    }
                    Err(e) => panic!("{}", e),
                },
                BestMoveCommand::Terminate => break,
            }
        });

        Self {
            thread: Some(thread),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BestMoveCommand {
    Clear,
    SetOptions(SearchOptions),
    Stop(StopReason),
    Terminate,
}

#[derive(Clone, Copy, Debug)]
pub enum StopReason {
    Command,
    Finished,
}
