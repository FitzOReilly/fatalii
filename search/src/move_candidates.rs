use movegen::r#move::{Move, MoveList};

#[derive(Debug, Default, Clone)]
pub struct MoveCandidates {
    pub move_list: MoveList,
    pub current_idx: usize,
}

impl MoveCandidates {
    pub fn move_to_front(&mut self, best_move: Move) {
        let idx = self
            .move_list
            .iter()
            .position(|&x| x == best_move)
            .expect("Expected to find move {x} in candidates: {candidates:?}");
        let slice = &mut self.move_list[0..=idx];
        slice.rotate_right(1);
    }
}
