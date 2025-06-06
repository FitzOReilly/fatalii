use std::cmp;

use crate::move_generator::MoveGenerator;
use crate::position_history::PositionHistory;
use crate::r#move::MoveList;
use crate::transposition_table::{TranspositionTable, TtEntry};
use crate::zobrist::Zobrist;

const AGE: u8 = 0;

#[derive(Clone, Copy, Debug, Default)]
struct TableEntry {
    is_valid: bool,
    depth: usize,
    num_nodes: usize,
}

impl TtEntry for TableEntry {
    fn is_valid(&self) -> bool {
        self.is_valid
    }

    fn depth(&self) -> usize {
        self.depth
    }

    fn age(&self) -> u8 {
        AGE
    }

    fn prio(&self, other: &Self, _age: u8) -> cmp::Ordering {
        self.depth.cmp(&other.depth).reverse()
    }
}

pub struct PerformanceTester {
    pos_history: PositionHistory,
    transpos_table: TranspositionTable<Zobrist, TableEntry>,
}

impl PerformanceTester {
    pub fn new(pos_history: PositionHistory, bytes: usize) -> PerformanceTester {
        Self {
            pos_history,
            transpos_table: TranspositionTable::new(bytes),
        }
    }

    pub fn count_nodes(&mut self, depth: usize) -> usize {
        let mut move_list_stack = vec![MoveList::new(); depth];
        self.count_nodes_recursive(&mut move_list_stack, depth)
    }

    fn count_nodes_recursive(
        &mut self,
        move_list_stack: &mut Vec<MoveList>,
        depth: usize,
    ) -> usize {
        let hash = self.pos_history.current_pos_hash();
        match self.transpos_table.get(&hash) {
            Some(entry) if entry.depth == depth => entry.num_nodes,
            _ => {
                let mut num_nodes = 0;

                match depth {
                    0 => {
                        debug_assert!(move_list_stack.is_empty());
                        num_nodes = 1;
                    }
                    _ => {
                        debug_assert!(!move_list_stack.is_empty());
                        let mut move_list = move_list_stack.pop().unwrap();
                        MoveGenerator::generate_moves(
                            &mut move_list,
                            self.pos_history.current_pos(),
                        );
                        match depth {
                            1 => {
                                debug_assert!(move_list_stack.is_empty());
                                num_nodes = move_list.len();
                            }
                            _ => {
                                for m in move_list.iter() {
                                    self.pos_history.do_move(*m);
                                    num_nodes +=
                                        self.count_nodes_recursive(move_list_stack, depth - 1);
                                    self.pos_history.undo_last_move();
                                }
                                self.transpos_table.insert(
                                    hash,
                                    TableEntry {
                                        is_valid: true,
                                        depth,
                                        num_nodes,
                                    },
                                );
                            }
                        }
                        move_list_stack.push(move_list);
                    }
                };
                num_nodes
            }
        }
    }
}
