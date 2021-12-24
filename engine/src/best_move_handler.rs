use crate::search_options::SearchOptions;
use crossbeam_channel::Receiver;
use movegen::side::Side;
use search::search::SearchResult;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct BestMoveHandler {
    pub thread: Option<thread::JoinHandle<()>>,
}

impl BestMoveHandler {
    pub fn new(
        search_result: Arc<Mutex<Option<SearchResult>>>,
        receiver: Receiver<BestMoveCommand>,
        mut search_info_callback: Box<dyn FnMut(Option<SearchResult>) + Send>,
        mut best_move_callback: Box<dyn FnMut(Option<SearchResult>) + Send>,
    ) -> Self {
        let options = Arc::new(Mutex::new(SearchOptions::new()));
        let mut side_to_move = None;

        let thread = thread::spawn(move || loop {
            let message = receiver.recv().expect("Error receiving BestMoveCommand");
            match message {
                BestMoveCommand::Clear => {
                    side_to_move = None;
                    match search_result.lock() {
                        Ok(mut res) => *res = None,
                        Err(e) => panic!("{}", e),
                    };
                }
                BestMoveCommand::SetOptions(new_options) => match options.lock() {
                    Ok(mut opt) => *opt = new_options,
                    Err(e) => panic!("{}", e),
                },
                BestMoveCommand::SetSideToMove(s) => side_to_move = s,
                BestMoveCommand::DepthFinished => match search_result.lock() {
                    Ok(res) => {
                        search_info_callback(Self::search_result_to_relative(*res, side_to_move))
                    }
                    Err(e) => panic!("{}", e),
                },
                BestMoveCommand::Stop(StopReason::Command) => match search_result.lock() {
                    Ok(mut res) => best_move_callback(Self::search_result_to_relative(
                        res.take(),
                        side_to_move,
                    )),
                    Err(e) => panic!("{}", e),
                },
                BestMoveCommand::Stop(StopReason::Finished) => match options.lock() {
                    Ok(opt) => {
                        if !opt.infinite {
                            match search_result.lock() {
                                Ok(mut res) => best_move_callback(Self::search_result_to_relative(
                                    res.take(),
                                    side_to_move,
                                )),
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

    fn search_result_to_relative(
        search_result: Option<SearchResult>,
        side_to_move: Option<Side>,
    ) -> Option<SearchResult> {
        search_result.map(
            |res| match side_to_move.expect("Expected Some(Side), got None") {
                Side::White => res,
                Side::Black => -res,
            },
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BestMoveCommand {
    Clear,
    SetOptions(SearchOptions),
    SetSideToMove(Option<Side>),
    DepthFinished,
    Stop(StopReason),
    Terminate,
}

#[derive(Clone, Copy, Debug)]
pub enum StopReason {
    Command,
    Finished,
}
