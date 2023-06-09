use movegen::square::Square;

#[derive(Debug)]
pub struct HistoryTable {
    table: [u32; Square::NUM_SQUARES * Square::NUM_SQUARES],
}

impl HistoryTable {
    pub fn new() -> Self {
        HistoryTable {
            table: [0; Square::NUM_SQUARES * Square::NUM_SQUARES],
        }
    }

    pub fn prioritize(&mut self, from: Square, to: Square, depth: usize) {
        self.table[Self::idx(from, to)] += (depth * depth) as u32;
    }

    pub fn priority(&self, from: Square, to: Square) -> u32 {
        self.table[Self::idx(from, to)]
    }

    fn idx(from: Square, to: Square) -> usize {
        from.idx() * Square::NUM_SQUARES + to.idx()
    }
}
