use movegen::file::File;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct EngineOptions {
    pub hash_size: usize,
    pub move_overhead: Duration,
    pub variant: Variant,
}

#[derive(Clone, Copy, Debug)]
pub enum Variant {
    Standard,
    Chess960(File, File), // Kingside castling file, queenside castling file
}

impl Default for EngineOptions {
    fn default() -> Self {
        EngineOptions {
            hash_size: 8 * 2_usize.pow(20),
            move_overhead: Duration::from_millis(10),
            variant: Variant::Standard,
        }
    }
}
