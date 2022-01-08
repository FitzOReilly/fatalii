use movegen::r#move::{Move, MoveList};

#[derive(Debug)]
pub struct PvTable {
    table: Vec<Move>,
    indices: Vec<usize>,
    max_depth: usize,
}

impl PvTable {
    pub fn new() -> Self {
        PvTable {
            table: Vec::new(),
            indices: Vec::new(),
            max_depth: 0,
        }
    }

    fn pv(&self, depth: usize) -> &[Move] {
        let begin = self.index(depth);
        let end = begin + depth;
        &self.table[begin..end]
    }

    pub fn pv_into_movelist(&self, depth: usize) -> MoveList {
        let mut res = MoveList::with_capacity(depth);
        for m in self.pv(depth) {
            res.push(*m);
        }
        res
    }

    pub fn update_move_and_copy(&mut self, depth: usize, m: Move) {
        if depth > self.max_depth {
            debug_assert!(depth == self.max_depth + 1);
            self.indices.push(self.table.len());
            self.max_depth += 1;
            for _ in 0..depth {
                self.table.push(Move::NULL);
            }
        }
        let begin = self.index(depth);
        let end = begin + depth;
        self.table[begin] = m;
        debug_assert!(begin + 1 >= depth);
        for i in begin + 1..end {
            self.table[i] = self.table[i - depth];
        }
    }

    fn index(&self, depth: usize) -> usize {
        debug_assert!(depth > 0);
        debug_assert!(depth <= self.max_depth);
        self.indices[depth - 1]
    }
}
