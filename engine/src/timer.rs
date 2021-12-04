use crossbeam_channel::{Receiver, Sender};
use search::search::SearchCommand;
use std::thread;
use std::time::Duration;

pub struct Timer {
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Timer {
    pub fn new(
        timer_command_receiver: Receiver<TimerCommand>,
        search_command_sender: Sender<SearchCommand>,
    ) -> Self {
        let thread = thread::spawn(move || loop {
            let message = timer_command_receiver
                .recv()
                .expect("Error receiving TimerCommand");
            match message {
                TimerCommand::Start(dur) => {
                    if timer_command_receiver.recv_timeout(dur).is_err() {
                        search_command_sender
                            .send(SearchCommand::Stop)
                            .expect("Error sending SearchCommand");
                    }
                }
                TimerCommand::Stop => {}
                TimerCommand::Terminate => break,
            }
        });

        Self {
            thread: Some(thread),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TimerCommand {
    Start(Duration),
    Stop,
    Terminate,
}
