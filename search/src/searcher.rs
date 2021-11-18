use crate::search::{Search, SearchCommand, SearchInfo};
use crossbeam_channel::{unbounded, Receiver, Sender};
use movegen::position_history::PositionHistory;

use std::thread;

pub struct Searcher {
    command_sender: Sender<SearchCommand>,
    info_sender: Sender<SearchInfo>,
    worker: Worker,
    search_info_handler: SearchInfoHandler,
}

impl Searcher {
    pub fn search(&self, pos_hist: PositionHistory, depth: usize) {
        self.stop();
        self.command_sender
            .send(SearchCommand::Search(pos_hist, depth))
            .expect("Error sending SearchCommand");
    }

    pub fn stop(&self) {
        self.command_sender
            .send(SearchCommand::Stop)
            .expect("Error sending SearchCommand");
    }

    pub fn clone_command_sender(&self) -> Sender<SearchCommand> {
        self.command_sender.clone()
    }

    pub fn new(
        search_algo: impl Search + Send + 'static,
        info_callback: Box<dyn Fn(SearchInfo) + Send>,
    ) -> Self {
        let (command_sender, command_receiver) = unbounded();
        let (info_sender, info_receiver) = unbounded();

        let worker = Worker::new(search_algo, command_receiver, info_sender.clone());
        let search_info_handler = SearchInfoHandler::new(info_receiver, info_callback);

        Searcher {
            command_sender,
            info_sender,
            worker,
            search_info_handler,
        }
    }
}

impl Drop for Searcher {
    fn drop(&mut self) {
        self.command_sender
            .send(SearchCommand::Stop)
            .expect("Error sending SearchCommand");
        self.command_sender
            .send(SearchCommand::Terminate)
            .expect("Error sending SearchCommand");
        if let Some(thread) = self.worker.thread.take() {
            thread.join().expect("Error joining search thread");
        }

        self.info_sender
            .send(SearchInfo::Terminated)
            .expect("Error sending SearchInfo");
        if let Some(thread) = self.search_info_handler.thread.take() {
            thread.join().expect("Error joining SearchInfoHandler");
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        mut search_algo: impl Search + Send + 'static,
        mut command_receiver: Receiver<SearchCommand>,
        mut info_sender: Sender<SearchInfo>,
    ) -> Self {
        let thread = thread::spawn(move || loop {
            let message = command_receiver
                .recv()
                .expect("Error receiving SearchCommand");

            match message {
                SearchCommand::Search(pos_hist, depth) => {
                    Self::search(
                        &mut search_algo,
                        pos_hist,
                        depth,
                        &mut command_receiver,
                        &mut info_sender,
                    );
                }
                SearchCommand::Stop => {}
                SearchCommand::Terminate => break,
            }
        });
        Worker {
            thread: Some(thread),
        }
    }

    fn search(
        search: &mut impl Search,
        mut pos_hist: PositionHistory,
        depth: usize,
        command_receiver: &mut Receiver<SearchCommand>,
        info_sender: &mut Sender<SearchInfo>,
    ) {
        search.search(&mut pos_hist, depth, command_receiver, info_sender);
    }
}

struct SearchInfoHandler {
    thread: Option<thread::JoinHandle<()>>,
}

impl SearchInfoHandler {
    fn new(
        info_receiver: Receiver<SearchInfo>,
        info_callback: Box<dyn Fn(SearchInfo) + Send>,
    ) -> Self {
        let thread = thread::spawn(move || loop {
            match info_receiver.recv() {
                Ok(SearchInfo::Terminated) => break,
                Ok(res) => info_callback(res),
                Err(_) => break,
            }
        });
        SearchInfoHandler {
            thread: Some(thread),
        }
    }
}
