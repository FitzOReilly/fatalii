use std::time::{Duration, Instant};

use crate::move_candidates::MoveCandidates;
use crate::node_counter::NodeCounter;
use crate::pv_table::PvTable;
use crate::search::{SearchCommand, SearchInfo, MAX_SEARCH_DEPTH};
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use eval::{Eval, Score};
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList};
use movegen::zobrist::Zobrist;

pub type Killers = [Option<Move>; NUM_KILLERS];

const NUM_KILLERS: usize = 2;

// The maximum number of search depth extensions
const MAX_EXTENSIONS: usize = 2;

// The number of fractions (i.e. the number of extending moves) needed to extend
// the search by 1 ply
const FRACTIONS_PER_EXTENSION: usize = 2;

#[derive(Debug, Clone, Default)]
pub struct StackElement {
    extensions: usize,
    reductions: usize,
    is_in_check: Option<bool>,
    eval_relative: Option<Score>,
}

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
    selective_depth: usize,
    ply: usize,
    prev_pv_depth: usize,
    pv_table: PvTable,
    prev_pv_table: PvTable,
    node_counter: NodeCounter,
    killers: Vec<Killers>,
    root_moves: MoveCandidates,
    search_stack: Vec<StackElement>,
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
            selective_depth: 0,
            ply: 0,
            prev_pv_depth: 0,
            pv_table: PvTable::new(),
            prev_pv_table: PvTable::new(),
            node_counter: NodeCounter::new(),
            killers: Vec::new(),
            root_moves: MoveCandidates::default(),
            search_stack: vec![Default::default()],
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
        (self.halfmove_count() % (MAX_SEARCH_DEPTH + 1)) as u8
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

    pub fn selective_depth(&self) -> usize {
        self.selective_depth
    }

    pub fn set_current_extension(&mut self, ext: usize) {
        self.search_stack[self.ply].extensions = ext;
    }

    pub fn total_extensions(&self) -> usize {
        (self
            .search_stack
            .iter()
            .map(|elem| elem.extensions)
            .sum::<usize>()
            / FRACTIONS_PER_EXTENSION)
            .min(MAX_EXTENSIONS)
    }

    pub fn current_reduction(&mut self) -> usize {
        self.search_stack[self.ply].reductions
    }

    pub fn set_current_reduction(&mut self, red: usize) {
        self.search_stack[self.ply].reductions = red;
    }

    pub fn total_reductions(&self) -> usize {
        self.search_stack.iter().map(|elem| elem.reductions).sum()
    }

    pub fn net_search_depth(&self) -> usize {
        debug_assert!(self.search_depth() + self.total_extensions() > self.total_reductions());
        self.search_depth() + self.total_extensions() - self.total_reductions()
    }

    pub fn remaining_depth(&self) -> usize {
        if self.ply() < self.net_search_depth() {
            self.net_search_depth() - self.ply()
        } else {
            0
        }
    }

    pub fn ply(&self) -> usize {
        self.ply
    }

    pub fn prev_pv_depth(&self) -> usize {
        self.prev_pv_depth
    }

    pub fn pv_table(&self) -> &PvTable {
        &self.pv_table
    }

    pub fn pv(&self, depth: usize) -> &[Move] {
        self.pv_table().pv(depth)
    }

    pub fn pv_owned(&self, depth: usize) -> MoveList {
        self.pv_table().pv_into_movelist(depth)
    }

    pub fn update_pv_move_and_copy(&mut self, m: Move) {
        if self.search_depth() > self.ply() {
            let depth = self.search_depth() - self.ply();
            self.pv_table.update_move_and_copy(depth, m);
        }
    }

    pub fn update_pv_move_and_truncate(&mut self, m: Move) {
        if self.search_depth() > self.ply() {
            let depth = self.search_depth() - self.ply();
            self.pv_table.update_move_and_truncate(depth, m);
        }
    }

    pub fn node_counter(&self) -> &NodeCounter {
        &self.node_counter
    }

    pub fn killers(&mut self) -> &Killers {
        self.resize_killers();
        &self.killers[self.ply]
    }

    pub fn insert_killer(&mut self, m: Move) {
        self.resize_killers();
        let killers = &mut self.killers[self.ply];
        // If m is already in the list of killers, move it to the front
        let max_idx = match killers.iter().position(|&k| k == Some(m)) {
            Some(p) => p,
            None => NUM_KILLERS - 1,
        };
        killers[0..=max_idx].rotate_right(1);
        killers[0] = Some(m);
    }

    pub fn reset_killers_next_ply(&mut self) {
        if self.killers.len() > self.ply + 1 {
            self.killers[self.ply + 1].fill(None);
        }
    }

    fn resize_killers(&mut self) {
        if self.killers.len() <= self.ply {
            self.killers.resize_with(self.ply + 1, Default::default);
        }
    }

    pub fn reset_current_search_depth(&mut self) {
        // This method will be called if we fail low/high, i.e. we didn't find the best move inside
        // the aspiration window. At depth 1, we search with an infinite window, so this method
        // should only be called at search depths > 1.
        debug_assert!(self.search_depth() > 1);
        self.pv_table = self.prev_pv_table.clone();
        self.prev_pv_depth = self.search_depth() - 1;
        self.selective_depth = 0;
        self.root_moves_mut().reset_counts();
    }

    pub fn increase_search_depth(&mut self) {
        self.prev_pv_table = self.pv_table.clone();
        self.prev_pv_depth = self.search_depth();
        self.search_depth += 1;
        self.selective_depth = 0;
        self.root_moves_mut().order_by_subtree_size();
        self.root_moves_mut().reset_counts();
    }

    pub fn decrease_prev_pv_depth(&mut self) {
        self.prev_pv_depth -= 1;
    }

    pub fn end_prev_pv(&mut self) {
        self.prev_pv_depth = 0;
    }

    pub fn do_move(&mut self, m: Move) {
        self.node_counter
            .increment_nodes(self.search_depth(), self.ply);
        self.pos_history_mut().do_move(m);
        self.ply += 1;
        self.selective_depth = self.selective_depth.max(self.ply);
        self.search_stack.push(Default::default());
    }

    pub fn undo_last_move(&mut self) {
        self.search_stack.pop();
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

    pub fn is_in_check(&mut self) -> bool {
        let pos = self.current_pos();
        if self.search_stack[self.ply].is_in_check.is_none() {
            self.search_stack[self.ply].is_in_check = Some(pos.is_in_check(pos.side_to_move()));
        }
        self.search_stack[self.ply].is_in_check.unwrap()
    }

    pub fn gives_check(&self, m: Move) -> bool {
        self.current_pos().gives_check(m)
    }

    pub fn static_eval(&mut self, evaluator: &mut Box<dyn Eval + Send>) -> Score {
        if self.search_stack[self.ply].eval_relative.is_none() {
            self.increment_eval_calls();
            let eval = evaluator.eval_relative(self.current_pos());
            self.search_stack[self.ply].eval_relative = Some(eval);
        }
        self.search_stack[self.ply].eval_relative.unwrap()
    }

    pub fn set_static_eval(&mut self, static_eval: Score) {
        self.search_stack[self.ply].eval_relative = Some(static_eval);
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

    pub fn calc_extension(&mut self, m: Move) -> usize {
        match m != Move::NULL && self.ply() <= self.search_depth() && self.is_in_check() {
            true => 1,
            false => 0,
        }
    }
}
