use std::time::{Duration, Instant};

use crate::history_table::HistoryTable;
use crate::node_counter::NodeCounter;
use crate::pv_table::PvTable;
use crate::search::{SearchCommand, SearchInfo};
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use movegen::piece::Piece;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use movegen::square::Square;

pub type Killers = [Option<Move>; NUM_KILLERS];

const NUM_KILLERS: usize = 2;

pub struct SearchData<'a> {
    command_receiver: &'a Receiver<SearchCommand>,
    info_sender: &'a Sender<SearchInfo>,
    pos_history: PositionHistory,
    halfmove_count: usize,
    start_time: Instant,
    hard_time_limit: Option<Duration>,
    max_nodes: Option<usize>,
    search_depth: usize,
    pv_depth: usize,
    pv_table: PvTable,
    node_counter: NodeCounter,
    killers: Vec<Killers>,
    history_table: HistoryTable,
}

impl<'a> SearchData<'a> {
    pub fn new(
        command_receiver: &'a Receiver<SearchCommand>,
        info_sender: &'a Sender<SearchInfo>,
        pos_history: PositionHistory,
        start_time: Instant,
        hard_time_limit: Option<Duration>,
        max_nodes: Option<usize>,
    ) -> Self {
        let halfmove_count = pos_history.current_pos().halfmove_count();
        Self {
            command_receiver,
            info_sender,
            pos_history,
            halfmove_count,
            start_time,
            hard_time_limit,
            max_nodes,
            search_depth: 0,
            pv_depth: 0,
            pv_table: PvTable::new(),
            node_counter: NodeCounter::new(),
            killers: Vec::new(),
            history_table: HistoryTable::new(),
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

    pub fn halfmove_count(&self) -> usize {
        self.halfmove_count
    }

    pub fn age(&self) -> u8 {
        (self.halfmove_count() % 256) as u8
    }

    pub fn start_time(&self) -> Instant {
        self.start_time
    }

    pub fn hard_time_limit(&self) -> Option<Duration> {
        self.hard_time_limit
    }

    pub fn max_nodes(&self) -> Option<usize> {
        self.max_nodes
    }

    pub fn searched_nodes(&self) -> usize {
        self.node_counter.sum_nodes() as usize
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

    pub fn killers(&self, depth: usize) -> &Killers {
        let len = self.killers.len();
        debug_assert!(len >= depth);
        &self.killers[len - depth]
    }

    pub fn contains_killer(&self, depth: usize, m: Move) -> bool {
        let len = self.killers.len();
        debug_assert!(len >= depth);
        self.killers[len - depth].contains(&Some(m))
    }

    pub fn insert_killer(&mut self, depth: usize, m: Move) {
        let len = self.killers.len();
        debug_assert!(len >= depth);
        for idx in (0..NUM_KILLERS - 2).rev() {
            self.killers[len - depth][idx + 1] = self.killers[len - depth][idx];
        }
        self.killers[len - depth][0] = Some(m);
    }

    pub fn prioritize_history(&mut self, p: Piece, to: Square, depth: usize) {
        self.history_table.prioritize(p, to, depth)
    }

    pub fn history_priority(&self, p: Piece, to: Square) -> u32 {
        self.history_table.priority(p, to)
    }

    pub fn increase_search_depth(&mut self) {
        self.pv_depth = self.search_depth;
        self.search_depth += 1;
        self.killers.push([None; NUM_KILLERS]);
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
