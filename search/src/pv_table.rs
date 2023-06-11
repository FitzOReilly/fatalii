use movegen::r#move::{Move, MoveList};
use std::fmt;

#[derive(Debug, Clone)]
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

    pub fn pv(&self, depth: usize) -> &[Move] {
        let begin = self.index(depth);
        let end = begin + depth;
        &self.table[begin..end]
    }

    pub fn pv_into_movelist(&self, depth: usize) -> MoveList {
        let mut res = MoveList::with_capacity(depth);
        for (i, m) in self.pv(depth).iter().enumerate() {
            match m {
                &Move::NULL => {
                    res.truncate(i);
                    break;
                }
                _ => res.push(*m),
            }
        }
        res
    }

    pub fn update_move_and_copy(&mut self, depth: usize, m: Move) {
        debug_assert!(depth > 0);
        self.reserve(depth);
        let begin = self.index(depth);
        let end = begin + depth;
        self.table[begin] = m;
        debug_assert!(begin + 1 >= depth);
        for i in begin + 1..end {
            self.table[i] = self.table[i - depth];
        }
    }

    pub fn update_move_and_truncate(&mut self, depth: usize, m: Move) {
        debug_assert!(depth > 0);
        self.reserve(depth);
        let begin = self.index(depth);
        self.table[begin] = m;
        if depth > 1 {
            self.table[begin + 1] = Move::NULL;
        }
    }

    fn index(&self, depth: usize) -> usize {
        debug_assert!(depth > 0);
        debug_assert!(depth <= self.max_depth);
        self.indices[depth - 1]
    }

    fn reserve(&mut self, depth: usize) {
        if depth > self.max_depth {
            debug_assert!(depth == self.max_depth + 1);
            self.indices.push(self.table.len());
            self.max_depth += 1;
            for _ in 0..depth {
                self.table.push(Move::NULL);
            }
        }
    }
}

impl fmt::Display for PvTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for d in 1..=self.max_depth {
            writeln!(f, "Depth {}: {}", d, self.pv_into_movelist(d))?;
        }
        Ok(())
    }
}
