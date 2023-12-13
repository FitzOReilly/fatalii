use std::time::{Duration, Instant};

use crate::counter_table::CounterTable;
use crate::history_table::HistoryTable;
use crate::move_candidates::MoveCandidates;
use crate::node_counter::NodeCounter;
use crate::pv_table::PvTable;
use crate::search::{SearchCommand, SearchInfo};
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use movegen::piece::Piece;
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use movegen::square::Square;
use movegen::zobrist::Zobrist;

pub type Killers = [Option<Move>; NUM_KILLERS];

const NUM_KILLERS: usize = 2;

#[derive(Debug, Clone)]
pub struct SearchData<'a> {
    command_receiver: &'a Receiver<SearchCommand>,
    info_sender: &'a Sender<SearchInfo>,
    pos_history: PositionHistory,
    halfmove_count: usize,
    start_time: Instant,
    hard_time_limit: Option<Duration>,
    max_nodes: Option<usize>,
    search_depth: usize,
    ply: usize,
    pv_depth: usize,
    pv_table: PvTable,
    prev_pv_table: PvTable,
    node_counter: NodeCounter,
    killers: Vec<Killers>,
    counter_table: CounterTable,
    history_table: HistoryTable,
    root_moves: MoveCandidates,
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
            ply: 0,
            pv_depth: 0,
            pv_table: PvTable::new(),
            prev_pv_table: PvTable::new(),
            node_counter: NodeCounter::new(),
            killers: Vec::new(),
            counter_table: CounterTable::new(),
            history_table: HistoryTable::new(),
            root_moves: MoveCandidates::default(),
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

    pub fn current_pos(&self) -> &Position {
        self.pos_history.current_pos()
    }

    pub fn current_pos_hash(&self) -> Zobrist {
        self.pos_history.current_pos_hash()
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

    pub fn ply(&self) -> usize {
        self.ply
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

    pub fn killers(&self) -> &Killers {
        debug_assert!(self.ply < self.killers.len());
        &self.killers[self.ply]
    }

    pub fn insert_killer(&mut self, m: Move) {
        let ply = self.ply;
        debug_assert!(ply < self.killers.len());
        // If m is already in the list of killers, move it to the front
        let max_idx = match self.killers[ply].iter().position(|&k| k == Some(m)) {
            Some(p) => p,
            None => NUM_KILLERS - 1,
        };
        for idx in (0..max_idx).rev() {
            self.killers[ply][idx + 1] = self.killers[ply][idx];
        }
        self.killers[ply][0] = Some(m);
    }

    pub fn reset_killers_next_ply(&mut self) {
        if self.ply() + 1 < self.search_depth() {
            self.killers[self.ply + 1].fill(None);
        }
    }

    pub fn update_counter(&mut self, p: Piece, to: Square, m: Move) {
        self.counter_table.update(p, to, m);
    }

    pub fn counter(&self, p: Piece, to: Square) -> Move {
        self.counter_table.counter(p, to)
    }

    pub fn prioritize_history(&mut self, p: Piece, to: Square, depth: usize) {
        self.history_table.prioritize(p, to, depth);
    }

    pub fn history_priority(&self, p: Piece, to: Square) -> u32 {
        self.history_table.priority(p, to)
    }

    pub fn reset_current_search_depth(&mut self) {
        // This method will be called if we fail low/high, i.e. we didn't find the best move inside
        // the aspiration window. At depth 1, we search with an infinite window, so this method
        // should only be called at search depths > 1.
        debug_assert!(self.search_depth() > 1);
        self.pv_table = self.prev_pv_table.clone();
        self.pv_depth = self.search_depth() - 1;
        self.root_moves_mut().reset_counts();
    }

    pub fn increase_search_depth(&mut self) {
        self.prev_pv_table = self.pv_table.clone();
        self.pv_depth = self.search_depth();
        self.search_depth += 1;
        self.killers.push([None; NUM_KILLERS]);
        self.root_moves_mut().order_by_subtree_size();
        self.root_moves_mut().reset_counts();
    }

    pub fn decrease_pv_depth(&mut self) {
        self.pv_depth -= 1;
    }

    pub fn end_pv(&mut self) {
        self.pv_depth = 0;
    }

    pub fn do_move(&mut self, m: Move) {
        self.node_counter
            .increment_nodes(self.search_depth(), self.ply);
        self.ply += 1;
        self.pos_history_mut().do_move(m);
    }

    pub fn undo_last_move(&mut self) {
        self.ply -= 1;
        self.pos_history_mut().undo_last_move();
    }

    pub fn increment_cache_hits(&mut self) {
        self.node_counter
            .increment_cache_hits(self.search_depth(), self.ply);
    }

    pub fn increment_eval_calls(&mut self) {
        self.node_counter.increment_eval_calls(self.search_depth());
    }

    pub fn set_root_moves(&mut self, root_moves: &MoveList) {
        debug_assert!(self.root_moves.move_list.is_empty());
        self.root_moves = MoveCandidates::from(root_moves);
    }

    pub fn root_moves(&self) -> &MoveCandidates {
        &self.root_moves
    }

    pub fn root_moves_mut(&mut self) -> &mut MoveCandidates {
        &mut self.root_moves
    }

    pub fn set_subtree_size(&mut self, m: Move, node_count: u64) {
        self.root_moves_mut().set_subtree_size(m, node_count);
    }

    pub fn move_to_front(&mut self, best_move: Move) {
        self.root_moves_mut().move_to_front(best_move);
        self.root_moves_mut().alpha_raised_count += 1;
    }

    pub fn should_stop_search_immediately(&self) -> bool {
        if self.search_depth() > 1 {
            if let Ok(SearchCommand::Stop) = self.try_recv_cmd() {
                return true;
            }
            if let Some(limit) = self.hard_time_limit() {
                if self.start_time().elapsed() > limit {
                    return true;
                }
            }
            if let Some(max_nodes) = self.max_nodes() {
                if self.searched_nodes() >= max_nodes {
                    return true;
                }
            }
        }
        false
    }
}
