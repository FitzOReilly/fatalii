use crate::engine_out::EngineOut;
use crossbeam_channel::Receiver;
use movegen::side::Side;
use search::search::SearchResult;
use search::SearchOptions;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct BestMoveHandler {
    pub thread: Option<thread::JoinHandle<()>>,
}

impl BestMoveHandler {
    pub fn new(
        search_result: Arc<Mutex<Option<SearchResult>>>,
        receiver: Receiver<BestMoveCommand>,
        engine_out: impl EngineOut + Send + 'static,
    ) -> Self {
        let options = Arc::new(Mutex::new(SearchOptions::default()));
        let mut side_to_move = None;

        let thread = thread::spawn(move || loop {
            let message = receiver.recv().expect("Error receiving BestMoveCommand");
            match message {
                BestMoveCommand::SetOptions(new_options) => match options.lock() {
                    Ok(mut opt) => *opt = *new_options,
                    Err(e) => panic!("{}", e),
                },
                BestMoveCommand::SetSideToMove(s) => side_to_move = s,
                BestMoveCommand::DepthFinished(res) => engine_out
                    .info_depth_finished(Self::search_result_to_relative(Some(res), side_to_move))
                    .expect("Error writing search info"),
                BestMoveCommand::Stop(StopReason::Command) => match search_result.lock() {
                    Ok(mut res) => engine_out
                        .best_move(Self::search_result_to_relative(res.take(), side_to_move))
                        .expect("Error writing best move"),
                    Err(e) => panic!("{}", e),
                },
                BestMoveCommand::Stop(StopReason::Finished) => match options.lock() {
                    Ok(opt) => {
                        if !opt.infinite {
                            match search_result.lock() {
                                Ok(mut res) => engine_out
                                    .best_move(Self::search_result_to_relative(
                                        res.take(),
                                        side_to_move,
                                    ))
                                    .expect("Error writing best move"),
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

#[derive(Clone, Debug)]
pub enum BestMoveCommand {
    SetOptions(Box<SearchOptions>),
    SetSideToMove(Option<Side>),
    DepthFinished(SearchResult),
    Stop(StopReason),
    Terminate,
}

#[derive(Clone, Copy, Debug)]
pub enum StopReason {
    Command,
    Finished,
}
