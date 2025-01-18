const LEN_DEPTH: usize = 64;
const LEN_MOVE_COUNT: usize = 64;

#[derive(Debug, Clone)]
pub struct LmrTable {
    table: [[u8; 64]; 64],
}

impl LmrTable {
    pub fn new(centi_base: usize, centi_divisor: usize) -> Self {
        let mut table = [[0; LEN_MOVE_COUNT]; LEN_DEPTH];
        let base = centi_base as f64 / 100.0;
        let divisor = centi_divisor as f64 / 100.0;
        for (depth, table_row) in table.iter_mut().enumerate().skip(1) {
            let log_depth = (depth as f64).log2();
            for (move_count, reduction) in table_row.iter_mut().enumerate().skip(1) {
                let log_move_count = (move_count as f64).log2();
                *reduction = (base + log_depth * log_move_count / divisor) as u8;
            }
        }
        Self { table }
    }

    pub fn late_move_depth_reduction(&self, depth: usize, move_count: usize) -> usize {
        self.table[depth.min(LEN_DEPTH - 1)][move_count.min(LEN_MOVE_COUNT - 1)].into()
    }
}
