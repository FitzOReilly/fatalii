use movegen::{
    piece::Piece,
    position::Position,
    r#move::{Move, MoveList},
    square::Square,
};

const MAX_BONUS: i32 = (i16::MAX / 2) as i32;
const HISTORY_DIVISOR: i32 = 16384;

#[derive(Debug, Clone)]
pub struct HistoryTable {
    table: [i16; Piece::NUM_PIECES * Square::NUM_SQUARES],
}

impl HistoryTable {
    pub fn new() -> Self {
        HistoryTable {
            table: [0; Piece::NUM_PIECES * Square::NUM_SQUARES],
        }
    }

    pub fn update(&mut self, m: Move, depth: usize, moves_tried: &MoveList, pos: &Position) {
        let bonus = Self::bonus(depth);
        // Add a bonus to the fail-high move
        let piece = pos
            .piece_at(m.origin())
            .expect("Expected a piece at move origin");
        self.update_history(piece, m.target(), bonus);
        // Subtract a penalty to all other tried moves
        for mt in moves_tried.iter() {
            let piece = pos
                .piece_at(mt.origin())
                .expect("Expected a piece at move origin");
            self.update_history(piece, mt.target(), -bonus);
        }
    }

    fn bonus(depth: usize) -> i32 {
        (16 * (depth * depth) as i32 + 128 * (depth as i32 - 1).max(0)).min(MAX_BONUS)
    }

    fn update_history(&mut self, p: Piece, s: Square, delta: i32) {
        let idx = Self::idx(p, s);
        let current = &mut self.table[idx];
        *current += (delta - *current as i32 * delta.abs() / HISTORY_DIVISOR) as i16;
    }

    pub fn value(&self, p: Piece, to: Square) -> i16 {
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
