use movegen::{piece::Piece, square::Square};

#[derive(Debug, Clone)]
pub struct HistoryTable {
    table: [u32; Piece::NUM_PIECES * Square::NUM_SQUARES],
}

impl HistoryTable {
    pub fn new() -> Self {
        HistoryTable {
            table: [0; Piece::NUM_PIECES * Square::NUM_SQUARES],
        }
    }

    pub fn prioritize(&mut self, p: Piece, to: Square, depth: usize) {
        self.table[Self::idx(p, to)] += (depth * depth) as u32;
    }

    pub fn priority(&self, p: Piece, to: Square) -> u32 {
        self.table[Self::idx(p, to)]
    }

    pub fn clear(&mut self) {
        for entry in self.table.iter_mut() {
            *entry = 0;
        }
    }

    // Reduce the weight of old entries
    pub fn decay(&mut self) {
        for entry in self.table.iter_mut() {
            *entry /= 2;
        }
    }

    fn idx(p: Piece, s: Square) -> usize {
        p.idx() * Square::NUM_SQUARES + s.idx()
    }
}
