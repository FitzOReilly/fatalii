use movegen::{piece::Piece, r#move::Move, square::Square};

#[derive(Debug, Clone)]
pub struct CounterTable {
    table: [Move; Piece::NUM_PIECES * Square::NUM_SQUARES],
}

impl CounterTable {
    pub fn new() -> Self {
        CounterTable {
            table: [Move::NULL; Piece::NUM_PIECES * Square::NUM_SQUARES],
        }
    }

    pub fn update(&mut self, p: Piece, to: Square, m: Move) {
        self.table[Self::idx(p, to)] = m;
    }

    pub fn counter(&self, p: Piece, to: Square) -> Move {
        self.table[Self::idx(p, to)]
    }

    fn idx(p: Piece, s: Square) -> usize {
        p.idx() * Square::NUM_SQUARES + s.idx()
    }
}
