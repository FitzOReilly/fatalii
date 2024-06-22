use movegen::r#move::{Move, MoveList};

#[derive(Debug, Clone)]
pub struct MoveData {
    pub r#move: Move,
    pub subtree_size: u64,
}

#[derive(Debug, Default, Clone)]
pub struct MoveCandidates {
    pub move_list: Vec<MoveData>,
    pub current_idx: usize,
    pub alpha_raised_count: usize,
}

impl From<&MoveList> for MoveCandidates {
    fn from(move_list: &MoveList) -> Self {
        Self {
            move_list: move_list
                .iter()
                .map(|&x| MoveData {
                    r#move: x,
                    subtree_size: 0,
                })
                .collect(),
            ..Default::default()
        }
    }
}

impl MoveCandidates {
    pub fn move_to_front(&mut self, best_move: Move) {
        let idx = self.index(best_move);
        let slice = &mut self.move_list[0..=idx];
        slice.rotate_right(1);
    }

    pub fn set_subtree_size(&mut self, m: Move, node_count: u64) {
        let idx = self.index(m);
        self.move_list[idx].subtree_size = node_count;
    }

    pub fn order_by_subtree_size(&mut self) {
        self.move_list[self.alpha_raised_count..]
            .sort_unstable_by_key(|md| u64::MAX - md.subtree_size);
    }

    pub fn reset_counts(&mut self) {
        self.current_idx = 0;
        self.alpha_raised_count = 0;
    }

    fn index(&self, m: Move) -> usize {
        self.move_list
            .iter()
            .position(|x| x.r#move == m)
            .unwrap_or_else(|| {
                panic!(
                    "Expected to find move {m} in candidates: {:?}",
                    self.move_list
                        .iter()
                        .map(|x| format!("{}", x.r#move))
                        .collect::<Vec<_>>(),
                )
            })
    }
}
