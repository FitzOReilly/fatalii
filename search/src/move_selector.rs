use crate::alpha_beta::AlphaBetaTable;
use crate::alpha_beta_entry::AlphaBetaEntry;
use crate::search_data::SearchData;
use movegen::piece;
use movegen::position::Position;
use movegen::r#move::{Move, MoveList};
use movegen::transposition_table::ENTRIES_PER_BUCKET;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Stage {
    PrincipalVariation,
    Hash,
    QueenPromoCaptures,
    QueenPromos,
    MvvLva,
    Killers,
    History,
    UnderPromoCaptures,
    UnderPromos,
}

pub struct MoveSelector {
    stage: Stage,
    hash_entries: [Option<AlphaBetaEntry>; ENTRIES_PER_BUCKET],
    hash_entry_idx: usize,
}

impl MoveSelector {
    pub fn new() -> Self {
        MoveSelector {
            stage: Stage::PrincipalVariation,
            hash_entries: [None; ENTRIES_PER_BUCKET],
            hash_entry_idx: 0,
        }
    }

    pub fn select_next_move(
        &mut self,
        search_data: &mut SearchData,
        transpos_table: &mut AlphaBetaTable,
        depth: usize,
        move_list: &mut MoveList,
    ) -> Option<Move> {
        if search_data.search_depth() > 1 && depth == search_data.search_depth() {
            return Self::select_root_move(search_data);
        }

        if self.stage == Stage::PrincipalVariation {
            if let Some(m) = Self::select_pv_move(search_data, depth, move_list) {
                return Some(m);
            }
            self.stage = Stage::Hash;
        }

        if self.stage == Stage::Hash {
            if let Some(m) = self.select_hash_move(search_data, transpos_table, move_list) {
                return Some(m);
            }
            self.stage = Stage::QueenPromoCaptures;
        }

        if self.stage == Stage::QueenPromoCaptures {
            if let Some(m) = Self::select_queen_promo_capture(move_list) {
                return Some(m);
            }
            self.stage = Stage::QueenPromos;
        }

        if self.stage == Stage::QueenPromos {
            if let Some(m) = Self::select_queen_promo(move_list) {
                return Some(m);
            }
            self.stage = Stage::MvvLva;
        }

        if self.stage == Stage::MvvLva {
            if let Some(m) = Self::filter_captures_select_mvv_lva(search_data, move_list) {
                return Some(m);
            }
            self.stage = Stage::Killers;
        }

        if self.stage == Stage::Killers {
            if let Some(m) = Self::select_killer(search_data, depth, move_list) {
                return Some(m);
            }
            self.stage = Stage::History;
        }

        if self.stage == Stage::History {
            if let Some(m) = Self::select_history(search_data, move_list) {
                return Some(m);
            }
            self.stage = Stage::UnderPromoCaptures;
        }

        if self.stage == Stage::UnderPromoCaptures {
            if let Some(m) = Self::select_under_promo_capture(move_list) {
                return Some(m);
            }
            self.stage = Stage::UnderPromos;
        }

        debug_assert_eq!(Stage::UnderPromos, self.stage);
        if let Some(m) = Self::select_under_promo(move_list) {
            return Some(m);
        }

        debug_assert!(move_list.is_empty());
        None
    }

    pub fn select_next_move_quiescence_capture(
        &mut self,
        search_data: &mut SearchData,
        transpos_table: &mut AlphaBetaTable,
        move_list: &mut MoveList,
    ) -> Option<Move> {
        if self.stage == Stage::PrincipalVariation || self.stage == Stage::Hash {
            if let Some(m) = self.select_hash_move(search_data, transpos_table, move_list) {
                return Some(m);
            }
            self.stage = Stage::QueenPromoCaptures;
        }

        if self.stage == Stage::QueenPromoCaptures {
            if let Some(m) = Self::select_queen_promo_capture(move_list) {
                return Some(m);
            }
            self.stage = Stage::MvvLva;
        }

        debug_assert_eq!(Stage::MvvLva, self.stage);
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
        &mut self,
        search_data: &mut SearchData,
        transpos_table: &AlphaBetaTable,
        move_list: &mut MoveList,
    ) -> Option<Move> {
        if self.hash_entry_idx == 0 {
            self.hash_entries =
                transpos_table.get_all(&search_data.pos_history().current_pos_hash());
        }
        for entry in self.hash_entries[self.hash_entry_idx..].iter().flatten() {
            self.hash_entry_idx += 1;
            if let Some(idx) = move_list.iter().position(|&x| x == entry.best_move()) {
                return Some(move_list.swap_remove(idx));
            }
        }
        None
    }

    fn select_queen_promo_capture(move_list: &mut MoveList) -> Option<Move> {
        if let Some(idx) = move_list.iter().enumerate().position(|(_, x)| {
            x.is_promotion() && x.is_capture() && x.promotion_piece() == Some(piece::Type::Queen)
        }) {
            let next_move = move_list.swap_remove(idx);
            return Some(next_move);
        }

        None
    }

    fn select_queen_promo(move_list: &mut MoveList) -> Option<Move> {
        if let Some(idx) = move_list
            .iter()
            .enumerate()
            .position(|(_, x)| x.is_promotion() && x.promotion_piece() == Some(piece::Type::Queen))
        {
            let next_move = move_list.swap_remove(idx);
            return Some(next_move);
        }

        None
    }

    fn select_under_promo_capture(move_list: &mut MoveList) -> Option<Move> {
        if let Some(idx) = move_list.iter().enumerate().position(|(_, x)| {
            x.is_promotion() && x.is_capture() && x.promotion_piece() == Some(piece::Type::Knight)
        }) {
            let next_move = move_list.swap_remove(idx);
            return Some(next_move);
        }
        if let Some(idx) = move_list.iter().enumerate().position(|(_, x)| {
            x.is_promotion() && x.is_capture() && x.promotion_piece() == Some(piece::Type::Rook)
        }) {
            let next_move = move_list.swap_remove(idx);
            return Some(next_move);
        }
        if let Some(idx) = move_list.iter().enumerate().position(|(_, x)| {
            x.is_promotion() && x.is_capture() && x.promotion_piece() == Some(piece::Type::Bishop)
        }) {
            let next_move = move_list.swap_remove(idx);
            return Some(next_move);
        }

        None
    }

    fn select_under_promo(move_list: &mut MoveList) -> Option<Move> {
        if let Some(idx) = move_list
            .iter()
            .enumerate()
            .position(|(_, x)| x.is_promotion() && x.promotion_piece() == Some(piece::Type::Knight))
        {
            let next_move = move_list.swap_remove(idx);
            return Some(next_move);
        }
        if let Some(idx) = move_list
            .iter()
            .enumerate()
            .position(|(_, x)| x.is_promotion() && x.promotion_piece() == Some(piece::Type::Rook))
        {
            let next_move = move_list.swap_remove(idx);
            return Some(next_move);
        }
        if let Some(idx) = move_list
            .iter()
            .enumerate()
            .position(|(_, x)| x.is_promotion() && x.promotion_piece() == Some(piece::Type::Bishop))
        {
            let next_move = move_list.swap_remove(idx);
            return Some(next_move);
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
            .filter(|&(_, x)| x.is_capture() && !x.is_promotion())
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

    fn select_killer(
        search_data: &mut SearchData,
        depth: usize,
        move_list: &mut MoveList,
    ) -> Option<Move> {
        if depth == 0 {
            return None;
        }

        for k in search_data.killers(depth).iter().flatten() {
            if let Some(idx) = move_list.iter().position(|x| x == k) {
                let next_move = move_list.swap_remove(idx);
                return Some(next_move);
            }
        }

        None
    }

    fn select_history(search_data: &mut SearchData, move_list: &mut MoveList) -> Option<Move> {
        if let Some((idx, &m)) = move_list
            .iter()
            .enumerate()
            .filter(|&(_, x)| !x.is_promotion())
            .max_by_key(|&(_, x)| {
                let p = search_data
                    .pos_history()
                    .current_pos()
                    .piece_at(x.origin())
                    .expect("Expected a piece at move origin");
                search_data.history_priority(p, x.target())
            })
        {
            debug_assert_eq!(m, move_list[idx]);
            let next_move = move_list.swap_remove(idx);
            return Some(next_move);
        }

        None
    }

    fn select_root_move(search_data: &mut SearchData) -> Option<Move> {
        if search_data.root_moves().current_idx == 0 {
            search_data.decrease_pv_depth();
        }
        let candidates = search_data.root_moves_mut();
        let moves = &candidates.move_list;
        let idx = &mut candidates.current_idx;
        if idx < &mut moves.len() {
            let m = moves[*idx];
            *idx += 1;
            return Some(m);
        }

        None
    }
}
