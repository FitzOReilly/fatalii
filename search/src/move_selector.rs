use crate::alpha_beta::AlphaBetaTable;
use crate::counter_table::CounterTable;
use crate::history_table::HistoryTable;
use crate::search_data::SearchData;
use crate::static_exchange_eval::static_exchange_eval;
use eval::Score;
use movegen::piece;
use movegen::position::Position;
use movegen::r#move::{Move, MoveList};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Stage {
    PrincipalVariation,
    Hash,
    QueenPromoCaptures,
    QueenPromos,
    WinningOrEqualCaptures,
    Killers,
    Counters,
    History,
    LosingCaptures,
    UnderPromoCaptures,
    UnderPromos,
}

#[derive(Debug, Clone)]
struct MoveInfo {
    r#move: Move,
}

pub struct MoveSelector {
    stage: Stage,
    moves: Vec<MoveInfo>,
}

impl MoveSelector {
    pub fn new(move_list: MoveList) -> Self {
        MoveSelector {
            stage: Stage::PrincipalVariation,
            moves: move_list.iter().map(|&x| MoveInfo { r#move: x }).collect(),
        }
    }

    pub fn select_next_move(
        &mut self,
        search_data: &mut SearchData,
        transpos_table: &mut AlphaBetaTable,
        counter_table: &CounterTable,
        history_table: &HistoryTable,
    ) -> Option<Move> {
        if search_data.search_depth() > 1 && search_data.ply() == 0 {
            if self.stage == Stage::PrincipalVariation {
                if let Some(m) = self.select_pv_move(search_data) {
                    search_data.root_moves_mut().move_to_front(m);
                    search_data.root_moves_mut().current_idx += 1;
                    return Some(m);
                }
                self.stage = Stage::Hash;
            }
            return self.select_root_move(search_data);
        }

        if self.stage == Stage::PrincipalVariation {
            if let Some(m) = self.select_pv_move(search_data) {
                return Some(m);
            }
            self.stage = Stage::Hash;
        }

        if self.stage == Stage::Hash {
            if let Some(m) = self.select_hash_move(search_data, transpos_table) {
                return Some(m);
            }
            self.stage = Stage::QueenPromoCaptures;
        }

        if self.stage == Stage::QueenPromoCaptures {
            if let Some(m) = self.select_queen_promo_capture() {
                return Some(m);
            }
            self.stage = Stage::QueenPromos;
        }

        if self.stage == Stage::QueenPromos {
            if let Some(m) = self.select_queen_promo() {
                return Some(m);
            }
            self.stage = Stage::WinningOrEqualCaptures;
        }

        if self.stage == Stage::WinningOrEqualCaptures {
            if let Some(m) = self.select_winning_capture(search_data) {
                return Some(m);
            }
            self.stage = Stage::Killers;
        }

        if self.stage == Stage::Killers {
            if let Some(m) = self.select_killer(search_data) {
                return Some(m);
            }
            self.stage = Stage::Counters;
        }

        if self.stage == Stage::Counters {
            if let Some(m) = self.select_counter(search_data, counter_table) {
                return Some(m);
            }
            self.stage = Stage::History;
        }

        if self.stage == Stage::History {
            if let Some(m) = self.select_history(search_data, history_table) {
                return Some(m);
            }
            self.stage = Stage::LosingCaptures;
        }

        if self.stage == Stage::LosingCaptures {
            if let Some(m) = self.select_losing_capture(search_data) {
                return Some(m);
            }
            self.stage = Stage::UnderPromoCaptures;
        }

        if self.stage == Stage::UnderPromoCaptures {
            if let Some(m) = self.select_under_promo_capture() {
                return Some(m);
            }
            self.stage = Stage::UnderPromos;
        }

        debug_assert_eq!(Stage::UnderPromos, self.stage);
        if let Some(m) = self.select_under_promo() {
            return Some(m);
        }

        debug_assert!(self.moves.is_empty());
        None
    }

    pub fn select_next_move_quiescence_capture(
        &mut self,
        search_data: &mut SearchData,
        transpos_table: &mut AlphaBetaTable,
    ) -> Option<Move> {
        if self.stage == Stage::PrincipalVariation || self.stage == Stage::Hash {
            if let Some(m) = self.select_hash_move(search_data, transpos_table) {
                return Some(m);
            }
            self.stage = Stage::QueenPromoCaptures;
        }

        if self.stage == Stage::QueenPromoCaptures {
            if let Some(m) = self.select_queen_promo_capture() {
                return Some(m);
            }
            self.stage = Stage::QueenPromos;
        }

        if self.stage == Stage::QueenPromos {
            if let Some(m) = self.select_queen_promo() {
                return Some(m);
            }
            self.stage = Stage::WinningOrEqualCaptures;
        }

        debug_assert_eq!(Stage::WinningOrEqualCaptures, self.stage);
        self.select_winning_capture(search_data)
    }

    fn select_pv_move(&mut self, search_data: &mut SearchData) -> Option<Move> {
        if search_data.prev_pv_depth() > 0 {
            // Select the PV move from the previous iteration
            let prev_pv = search_data.pv(search_data.search_depth() - 1);
            let pv_move = prev_pv[search_data.ply()];
            let idx = self
                .moves
                .iter()
                .position(|x| x.r#move == pv_move)
                .unwrap_or_else(|| {
                    let move_list =
                        MoveList::from(self.moves.iter().map(|x| x.r#move).collect::<Vec<_>>());
                    panic!(
                        "\nPV move not found in move list\n\
                        Search depth: {}\nNet search depth: {}\nRemaining depth: {}\nPly: {}\nPrevious PV depth: {}\n\
                        Total extensions: {}\nTotal reductions: {}\n\
                        Move list: {}\nPV move: {}, {:?}\nPV table:\n{}\
                        Position:\n{}",
                        search_data.search_depth(),
                        search_data.net_search_depth(),
                        search_data.remaining_depth(),
                        search_data.ply(),
                        search_data.prev_pv_depth(),
                        search_data.total_extensions(),
                        search_data.total_reductions(),
                        move_list,
                        pv_move,
                        pv_move,
                        search_data.pv_table(),
                        search_data.current_pos(),
                    )
                });
            search_data.decrease_prev_pv_depth();
            return Some(self.moves.swap_remove(idx).r#move);
        }
        None
    }

    fn select_hash_move(
        &mut self,
        search_data: &mut SearchData,
        transpos_table: &mut AlphaBetaTable,
    ) -> Option<Move> {
        if let Some(entry) = transpos_table.get(&search_data.current_pos_hash()) {
            if let Some(idx) = self
                .moves
                .iter()
                .position(|x| x.r#move == entry.best_move())
            {
                return Some(self.moves.swap_remove(idx).r#move);
            }
        }
        None
    }

    fn select_queen_promo_capture(&mut self) -> Option<Move> {
        if let Some(idx) = self.moves.iter().enumerate().position(|(_, x)| {
            let m = x.r#move;
            m.is_promotion() && m.is_capture() && m.promotion_piece() == Some(piece::Type::Queen)
        }) {
            let next_move = self.moves.swap_remove(idx).r#move;
            return Some(next_move);
        }

        None
    }

    fn select_queen_promo(&mut self) -> Option<Move> {
        if let Some(idx) = self.moves.iter().enumerate().position(|(_, x)| {
            x.r#move.is_promotion() && x.r#move.promotion_piece() == Some(piece::Type::Queen)
        }) {
            let next_move = self.moves.swap_remove(idx).r#move;
            return Some(next_move);
        }

        None
    }

    fn select_under_promo_capture(&mut self) -> Option<Move> {
        for piece_type in [piece::Type::Knight, piece::Type::Rook, piece::Type::Bishop] {
            if let Some(idx) = self.moves.iter().enumerate().position(|(_, x)| {
                let m = x.r#move;
                m.is_promotion() && m.is_capture() && m.promotion_piece() == Some(piece_type)
            }) {
                let next_move = self.moves.swap_remove(idx).r#move;
                return Some(next_move);
            }
        }

        None
    }

    fn select_under_promo(&mut self) -> Option<Move> {
        for piece_type in [piece::Type::Knight, piece::Type::Rook, piece::Type::Bishop] {
            if let Some(idx) = self.moves.iter().enumerate().position(|(_, x)| {
                x.r#move.is_promotion() && x.r#move.promotion_piece() == Some(piece_type)
            }) {
                let next_move = self.moves.swap_remove(idx).r#move;
                return Some(next_move);
            }
        }

        None
    }

    fn select_winning_capture(&mut self, search_data: &mut SearchData) -> Option<Move> {
        let mut end = self.moves.len();
        while let Some((idx, m, _)) = self.moves[..end]
            .iter()
            .enumerate()
            .filter(|(_, x)| x.r#move.is_capture())
            .map(|(idx, x)| {
                let cap_score = Self::capture_score(search_data.current_pos(), x.r#move);
                (idx, x.r#move, cap_score)
            })
            .max_by_key(|&(_, _, cap_score)| cap_score)
        {
            if static_exchange_eval(search_data.current_pos(), m, 0) {
                debug_assert_eq!(m, self.moves[idx].r#move);
                let next_move = self.moves.swap_remove(idx).r#move;
                return Some(next_move);
            } else {
                end -= 1;
                self.moves.swap(idx, end);
            }
        }
        None
    }

    fn select_losing_capture(&mut self, search_data: &mut SearchData) -> Option<Move> {
        if let Some((idx, m, _)) = self
            .moves
            .iter()
            .enumerate()
            .filter(|(_, x)| x.r#move.is_capture())
            .map(|(idx, x)| {
                let cap_score = Self::capture_score(search_data.current_pos(), x.r#move);
                (idx, x.r#move, cap_score)
            })
            .max_by_key(|&(_, _, cap_score)| cap_score)
        {
            debug_assert_eq!(m, self.moves[idx].r#move);
            let next_move = self.moves.swap_remove(idx).r#move;
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

    fn select_killer(&mut self, search_data: &mut SearchData) -> Option<Move> {
        if search_data.ply() >= search_data.search_depth() {
            return None;
        }

        for k in search_data.killers().iter().flatten() {
            if let Some(idx) = self.moves.iter().position(|x| x.r#move == *k) {
                let next_move = self.moves.swap_remove(idx).r#move;
                return Some(next_move);
            }
        }

        None
    }

    fn select_counter(
        &mut self,
        search_data: &mut SearchData,
        counter_table: &CounterTable,
    ) -> Option<Move> {
        if let Some(last_move) = search_data.pos_history().last_move() {
            if let Some(last_moved_piece) = search_data.pos_history().last_moved_piece() {
                let counter = counter_table.counter(last_moved_piece, last_move.target());
                if counter != Move::NULL {
                    if let Some(idx) = self.moves.iter().position(|x| x.r#move == counter) {
                        return Some(self.moves.swap_remove(idx).r#move);
                    }
                }
            }
        }

        None
    }

    fn select_history(
        &mut self,
        search_data: &mut SearchData,
        history_table: &HistoryTable,
    ) -> Option<Move> {
        if let Some((idx, m)) = self
            .moves
            .iter()
            .enumerate()
            .filter(|&(_, x)| !x.r#move.is_capture() && !x.r#move.is_promotion())
            .max_by_key(|&(_, x)| {
                let p = search_data
                    .current_pos()
                    .piece_at(x.r#move.origin())
                    .expect("Expected a piece at move origin");
                history_table.value(p, x.r#move.target())
            })
        {
            debug_assert_eq!(m.r#move, self.moves[idx].r#move);
            let next_move = self.moves.swap_remove(idx).r#move;
            return Some(next_move);
        }

        None
    }

    fn select_root_move(&mut self, search_data: &mut SearchData) -> Option<Move> {
        debug_assert_ne!(0, search_data.root_moves().current_idx);
        let candidates = search_data.root_moves_mut();
        let moves = &candidates.move_list;
        let idx = &mut candidates.current_idx;
        if idx < &mut moves.len() {
            let m = moves[*idx].r#move;
            *idx += 1;
            return Some(m);
        }
        None
    }

    fn mvv_lva_score(attacker: piece::Type, target: piece::Type) -> Score {
        debug_assert_ne!(piece::Type::King, target);

        const NUM_VICTIMS: usize = 5;
        const NUM_ATTACKERS: usize = 6;
        const MVV_LVA_SCORES: [[Score; NUM_ATTACKERS]; NUM_VICTIMS] = [
            // attackers: P, N, B, R, Q, K
            [15, 14, 13, 12, 11, 10], // victim: P
            [25, 24, 23, 22, 21, 20], // victim: N
            [35, 34, 33, 32, 31, 30], // victim: B
            [45, 44, 43, 42, 41, 40], // victim: R
            [55, 54, 53, 52, 51, 50], // victim: Q
        ];

        let attacker_idx = Self::piece_type_idx(attacker);
        let victim_idx = Self::piece_type_idx(target);

        MVV_LVA_SCORES[victim_idx][attacker_idx]
    }

    fn capture_score(pos: &Position, m: Move) -> Score {
        let (attacker, target) = Self::capture_piece_types(pos, m);
        Self::mvv_lva_score(attacker, target)
    }

    fn piece_type_idx(pt: piece::Type) -> usize {
        match pt {
            piece::Type::Pawn => 0,
            piece::Type::Knight => 1,
            piece::Type::Bishop => 2,
            piece::Type::Rook => 3,
            piece::Type::Queen => 4,
            piece::Type::King => 5,
        }
    }
}
