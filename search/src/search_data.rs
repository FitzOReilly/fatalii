use std::time::{Duration, Instant};

use crate::node_counter::NodeCounter;
use crate::pv_table::PvTable;
use crate::search::{SearchCommand, SearchInfo};
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};

pub struct SearchData<'a> {
    command_receiver: &'a Receiver<SearchCommand>,
    info_sender: &'a Sender<SearchInfo>,
    pos_history: PositionHistory,
    start_time: Instant,
    hard_time_limit: Option<Duration>,
    search_depth: usize,
    pv_depth: usize,
    pv_table: PvTable,
    node_counter: NodeCounter,
}

impl<'a> SearchData<'a> {
    pub fn new(
        command_receiver: &'a Receiver<SearchCommand>,
        info_sender: &'a Sender<SearchInfo>,
        pos_history: PositionHistory,
        start_time: Instant,
        hard_time_limit: Option<Duration>,
    ) -> Self {
        Self {
            command_receiver,
            info_sender,
            pos_history,
            start_time,
            hard_time_limit,
            search_depth: 0,
            pv_depth: 0,
            pv_table: PvTable::new(),
            node_counter: NodeCounter::new(),
        }
    }

    pub fn try_recv_cmd(&self) -> Result<SearchCommand, TryRecvError> {
        self.command_receiver.try_recv()
    }

    pub fn send_info(&self, search_info: SearchInfo) {
        self.info_sender
            .send(search_info)
            .expect("Error sending SearchInfo");
    }

    pub fn pos_history(&self) -> &PositionHistory {
        &self.pos_history
    }

    pub fn pos_history_mut(&mut self) -> &mut PositionHistory {
        &mut self.pos_history
    }

    pub fn start_time(&self) -> Instant {
        self.start_time
    }

    pub fn hard_time_limit(&self) -> Option<Duration> {
        self.hard_time_limit
    }

    pub fn search_depth(&self) -> usize {
        self.search_depth
    }

    pub fn pv_depth(&self) -> usize {
        self.pv_depth
    }

    pub fn pv_table(&self) -> &PvTable {
        &self.pv_table
    }

    pub fn pv_table_mut(&mut self) -> &mut PvTable {
        &mut self.pv_table
    }

    pub fn pv(&self, depth: usize) -> &[Move] {
        self.pv_table().pv(depth)
    }

    pub fn pv_owned(&self, depth: usize) -> MoveList {
        self.pv_table().pv_into_movelist(depth)
    }

    pub fn node_counter(&self) -> &NodeCounter {
        &self.node_counter
    }

    pub fn increase_search_depth(&mut self) {
        self.pv_depth = self.search_depth;
        self.search_depth += 1;
    }

    pub fn decrease_pv_depth(&mut self) {
        self.pv_depth -= 1;
    }

    pub fn end_pv(&mut self) {
        self.pv_depth = 0;
    }

    pub fn increment_nodes(&mut self, plies_from_end: usize) {
        self.node_counter
            .increment_nodes(self.search_depth(), plies_from_end)
    }

    pub fn increment_cache_hits(&mut self, plies_from_end: usize) {
        self.node_counter
            .increment_cache_hits(self.search_depth(), plies_from_end)
    }

    pub fn increment_eval_calls(&mut self) {
        self.node_counter.increment_eval_calls(self.search_depth());
    }
}
