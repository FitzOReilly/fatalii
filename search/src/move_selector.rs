use crate::alpha_beta::AlphaBetaTable;
use crate::search_data::SearchData;
use movegen::r#move::{Move, MoveList};

pub struct MoveSelector;

impl MoveSelector {
    pub fn select_next_move(
        search_data: &mut SearchData,
        transpos_table: &mut AlphaBetaTable,
        depth: usize,
        move_list: &mut MoveList,
    ) -> Option<Move> {
        if let Some(m) = Self::select_pv_move(search_data, depth, move_list) {
            return Some(m);
        }

        if let Some(m) = Self::select_hash_move(search_data, transpos_table, move_list) {
            return Some(m);
        }

        move_list.pop()
    }

    fn select_pv_move(
        search_data: &mut SearchData,
        depth: usize,
        move_list: &mut MoveList,
    ) -> Option<Move> {
        if search_data.pv_depth() > 0 {
            debug_assert_eq!(search_data.pv_depth(), depth - 1);
            search_data.decrease_pv_depth();
            // Select the PV move from the previous iteration
            let prev_pv = search_data.pv(search_data.search_depth() - 1);
            let pv_move = prev_pv[search_data.search_depth() - depth];
            let idx = move_list
                .iter()
                .position(|&x| x == pv_move)
                .unwrap_or_else(|| {
                    panic!(
                        "\nPV move not found in move list\n\
                    Search depth: {}\nDepth: {}\nMove list: {}\nPV move: {}\nPV table:\n{}",
                        search_data.search_depth(),
                        depth,
                        move_list,
                        pv_move,
                        search_data.pv_table()
                    )
                });
            return Some(move_list.swap_remove(idx));
        }
        None
    }

    fn select_hash_move(
        search_data: &mut SearchData,
        transpos_table: &AlphaBetaTable,
        move_list: &mut MoveList,
    ) -> Option<Move> {
        if let Some(entry) = transpos_table.get(&search_data.pos_history().current_pos_hash()) {
            if let Some(idx) = move_list.iter().position(|&x| x == entry.best_move()) {
                return Some(move_list.swap_remove(idx));
            }
        }
        None
    }
}
