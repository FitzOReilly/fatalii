use crate::{
    search::{Search, SearchCommand, SearchInfo},
    search_params::SearchParamsEachAlgo,
    SearchOptions,
};
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use movegen::position_history::PositionHistory;

use std::thread;

pub struct Searcher {
    command_sender: Sender<SearchCommand>,
    info_sender: Sender<SearchInfo>,
    worker: Worker,
    search_info_handler: SearchInfoHandler,
}

impl Searcher {
    pub fn set_hash_size(&self, bytes: usize) {
        let (sender, receiver) = bounded(1);
        self.command_sender
            .send(SearchCommand::SetHashSize(bytes, sender))
            .expect("Error sending SearchCommand");
        receiver
            .recv()
            .expect_err("Expected sender to disconnect after SetHashSize");
    }

    pub fn clear_hash_table(&self) {
        let (sender, receiver) = bounded(1);
        self.command_sender
            .send(SearchCommand::ClearHashTable(sender))
            .expect("Error sending SearchCommand");
        receiver
            .recv()
            .expect_err("Expected sender to disconnect after ClearHashTable");
    }

    pub fn set_search_params(&self, search_params: SearchParamsEachAlgo) {
        let (sender, receiver) = bounded(1);
        self.command_sender
            .send(SearchCommand::SetSearchParams(search_params, sender))
            .expect("Error sending SearchCommand");
        receiver
            .recv()
            .expect_err("Expected sender to disconnect after SetSearchParams");
    }

    pub fn search(&self, pos_hist: PositionHistory, search_options: SearchOptions) {
        self.stop();
        self.command_sender
            .send(SearchCommand::Search(Box::new((pos_hist, search_options))))
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
        info_callback: Box<dyn FnMut(SearchInfo) + Send>,
    ) -> Self {
        let (command_sender, command_receiver) = unbounded();
        let (info_sender, info_receiver) = unbounded();

        let worker = Worker::new(search_algo, command_receiver, info_sender.clone());
        let search_info_handler = SearchInfoHandler::new(info_receiver, info_callback);

        Self {
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
                SearchCommand::SetHashSize(bytes, _sender) => {
                    Self::set_hash_size(&mut search_algo, bytes);
                }
                SearchCommand::ClearHashTable(_sender) => {
                    Self::clear_hash_table(&mut search_algo);
                }
                SearchCommand::SetSearchParams(search_params, _sender) => {
                    Self::set_search_params(&mut search_algo, search_params);
                }
                SearchCommand::Search(inner) => {
                    let (pos_hist, search_options) = *inner;
                    Self::search(
                        &mut search_algo,
                        pos_hist,
                        search_options,
                        &mut command_receiver,
                        &mut info_sender,
                    );
                }
                SearchCommand::Stop => {}
                SearchCommand::Terminate => break,
            }
        });
        Self {
            thread: Some(thread),
        }
    }

    fn set_hash_size(search: &mut impl Search, bytes: usize) {
        search.set_hash_size(bytes);
    }

    fn clear_hash_table(search: &mut impl Search) {
        search.clear_hash_table();
    }

    fn set_search_params(search: &mut impl Search, search_params: SearchParamsEachAlgo) {
        search.set_params(search_params);
    }

    fn search(
        search: &mut impl Search,
        pos_hist: PositionHistory,
        search_options: SearchOptions,
        command_receiver: &mut Receiver<SearchCommand>,
        info_sender: &mut Sender<SearchInfo>,
    ) {
        search.search(pos_hist, search_options, command_receiver, info_sender);
    }
}

struct SearchInfoHandler {
    thread: Option<thread::JoinHandle<()>>,
}

impl SearchInfoHandler {
    fn new(
        info_receiver: Receiver<SearchInfo>,
        mut info_callback: Box<dyn FnMut(SearchInfo) + Send>,
    ) -> Self {
        let thread = thread::spawn(move || loop {
            match info_receiver.recv() {
                Ok(SearchInfo::Terminated) => break,
                Ok(res) => info_callback(res),
                Err(_) => break,
            }
        });
        Self {
            thread: Some(thread),
        }
    }
}
