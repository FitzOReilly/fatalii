use engine::Engine;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
pub enum OptionType {
    Button,
    Check,
    Combo,
    Spin,
    String,
}

pub struct UciOption {
    pub name: &'static str,
    pub r#type: OptionType,
    pub default: usize,
    pub min: usize,
    pub max: usize,
    pub fun: fn(&mut Engine, value: usize),
}

pub const OPTIONS: [UciOption; 2] = [
    UciOption {
        name: "Hash",
        r#type: OptionType::Spin,
        default: 8,
        min: 1,
        max: 65536,
        fun: set_hash_size,
    },
    UciOption {
        name: "Move Overhead",
        r#type: OptionType::Spin,
        default: 10,
        min: 0,
        max: 10000,
        fun: set_move_overhead,
    },
];

fn set_hash_size(engine: &mut Engine, megabytes: usize) {
    let bytes = 2_usize.pow(20) * megabytes;
    engine.set_hash_size(bytes);
}

fn set_move_overhead(engine: &mut Engine, move_overhead: usize) {
    engine.set_move_overhead(Duration::from_millis(move_overhead as u64));
}
