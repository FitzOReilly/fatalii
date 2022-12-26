use engine::{Engine, Variant};
use movegen::file::File;
use std::time::Duration;

pub const DEFAULT_HASH_MB: usize = 16;
pub const DEFAULT_HASH_BYTES: usize = DEFAULT_HASH_MB * 2_usize.pow(20);

#[allow(dead_code)]
pub enum OptionType {
    Button,
    Check(CheckProps),
    Combo,
    Spin(SpinProps),
    String,
}

pub struct CheckProps {
    pub default: bool,
    pub fun: fn(&mut Engine, value: bool) -> String,
}

pub struct SpinProps {
    pub default: usize,
    pub min: usize,
    pub max: usize,
    pub fun: fn(&mut Engine, value: usize) -> String,
}

pub struct UciOption {
    pub name: &'static str,
    pub r#type: OptionType,
}

pub const OPTIONS: [UciOption; 3] = [
    UciOption {
        name: "Hash",
        r#type: OptionType::Spin(SpinProps {
            default: DEFAULT_HASH_MB,
            min: 1,
            max: 65536,
            fun: set_hash_size,
        }),
    },
    UciOption {
        name: "Move Overhead",
        r#type: OptionType::Spin(SpinProps {
            default: 10,
            min: 0,
            max: 10000,
            fun: set_move_overhead,
        }),
    },
    UciOption {
        name: "UCI_Chess960",
        r#type: OptionType::Check(CheckProps {
            default: false,
            fun: set_chess_960,
        }),
    },
];

fn set_hash_size(engine: &mut Engine, megabytes: usize) -> String {
    let bytes = 2_usize.pow(20) * megabytes;
    engine.set_hash_size(bytes);
    format!("Hash set to {} MB", megabytes)
}

fn set_move_overhead(engine: &mut Engine, move_overhead: usize) -> String {
    engine.set_move_overhead(Duration::from_millis(move_overhead as u64));
    format!("Move Overhead set to {} ms", move_overhead)
}

fn set_chess_960(engine: &mut Engine, enable: bool) -> String {
    engine.set_variant(Variant::Chess960(File::H, File::A));
    match enable {
        true => String::from("Chess 960 enabled"),
        false => String::from("Chess 960 disabled"),
    }
}
