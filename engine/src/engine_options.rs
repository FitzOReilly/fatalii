use movegen::file::File;
use std::time::Duration;

pub const DEFAULT_HASH_MB: usize = 16;
pub const DEFAULT_HASH_BYTES: usize = DEFAULT_HASH_MB * 2_usize.pow(20);

pub const DEFAULT_MOVE_OVERHEAD_MILLIS: usize = 10;

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
            hash_size: DEFAULT_HASH_BYTES,
            move_overhead: Duration::from_millis(DEFAULT_MOVE_OVERHEAD_MILLIS as u64),
            variant: Variant::Standard,
        }
    }
}
