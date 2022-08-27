use crate::alpha_beta::AlphaBetaTable;
use crate::search_data::SearchData;
use movegen::piece;
use movegen::position::Position;
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

        if let Some(m) = Self::filter_captures_select_mvv_lva(search_data, move_list) {
            return Some(m);
        }

        move_list.pop()
    }

    pub fn select_next_move_quiescence(
        search_data: &mut SearchData,
        transpos_table: &mut AlphaBetaTable,
        move_list: &mut MoveList,
    ) -> Option<Move> {
        if let Some(m) = Self::select_hash_move(search_data, transpos_table, move_list) {
            return Some(m);
        }

        let opt_m = Self::select_mvv_lva(search_data, move_list);
        debug_assert!(opt_m.is_some() || move_list.is_empty());
        opt_m
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

    fn filter_captures_select_mvv_lva(
        search_data: &mut SearchData,
        move_list: &mut MoveList,
    ) -> Option<Move> {
        if let Some((idx, &m)) = move_list
            .iter()
            .enumerate()
            .filter(|&(_, x)| x.is_capture())
            .min_by_key(|&(_, x)| {
                let (attacker, target) =
                    Self::capture_piece_types(search_data.pos_history().current_pos(), *x);
                Self::capture_priority(attacker, target)
            })
        {
            debug_assert_eq!(m, move_list[idx]);
            let next_move = move_list.swap_remove(idx);
            return Some(next_move);
        }

        None
    }

    fn select_mvv_lva(search_data: &mut SearchData, move_list: &mut MoveList) -> Option<Move> {
        if let Some((idx, &m)) = move_list.iter().enumerate().min_by_key(|&(_, x)| {
            let (attacker, target) =
                Self::capture_piece_types(search_data.pos_history().current_pos(), *x);
            Self::capture_priority(attacker, target)
        }) {
            debug_assert_eq!(m, move_list[idx]);
            let next_move = move_list.swap_remove(idx);
            return Some(next_move);
        }

        None
    }

    fn capture_piece_types(pos: &Position, m: Move) -> (piece::Type, piece::Type) {
        debug_assert!(m.is_capture());
        let attacker = pos
            .piece_at(m.origin())
            .expect("No piece on origin square")
            .piece_type();
        let target = match m.is_en_passant() {
            true => piece::Type::Pawn,
            false => pos
                .piece_at(m.target())
                .expect("No piece on target square")
                .piece_type(),
        };
        (attacker, target)
    }

    fn capture_priority(attacker: piece::Type, target: piece::Type) -> i32 {
        const LEN_ATTACKER_PRIO: i32 = 6;
        let attacker_prio = |t: &piece::Type| match t {
            piece::Type::Pawn => 0,
            piece::Type::Knight => 1,
            piece::Type::Bishop => 2,
            piece::Type::Rook => 3,
            piece::Type::Queen => 4,
            piece::Type::King => 5,
        };
        let target_prio = |t: &piece::Type| match t {
            piece::Type::Queen => 0,
            piece::Type::Rook => 1,
            piece::Type::Bishop => 2,
            piece::Type::Knight => 3,
            piece::Type::Pawn => 4,
            piece::Type::King => panic!("King cannot be captured"),
        };
        LEN_ATTACKER_PRIO * target_prio(&target) + attacker_prio(&attacker)
    }
}
