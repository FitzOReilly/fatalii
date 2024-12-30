use engine::{Engine, Variant, DEFAULT_HASH_MB, DEFAULT_MOVE_OVERHEAD_MILLIS};
use eval::Score;
use movegen::file::File;
use search::search_params::{AlphaBetaParams, SearchParamsEachAlgo};
use std::time::Duration;

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
    pub default: i64,
    pub min: i64,
    pub max: i64,
    pub fun: fn(&mut Engine, value: i64) -> String,
}

pub struct UciOption {
    pub name: &'static str,
    pub r#type: OptionType,
}

pub const OPTIONS: [UciOption; 3] = [
    UciOption {
        name: "Hash",
        r#type: OptionType::Spin(SpinProps {
            default: DEFAULT_HASH_MB as i64,
            min: 1,
            max: 65536,
            fun: set_hash_size,
        }),
    },
    UciOption {
        name: "Move Overhead",
        r#type: OptionType::Spin(SpinProps {
            default: DEFAULT_MOVE_OVERHEAD_MILLIS as i64,
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

fn set_hash_size(engine: &mut Engine, megabytes: i64) -> String {
    let bytes = 2_usize.pow(20) * megabytes as usize;
    engine.set_hash_size(bytes);
    format!("Hash set to {megabytes} MB")
}

fn set_move_overhead(engine: &mut Engine, move_overhead: i64) -> String {
    engine.set_move_overhead(Duration::from_millis(move_overhead as u64));
    format!("Move Overhead set to {move_overhead} ms")
}

fn set_chess_960(engine: &mut Engine, enable: bool) -> String {
    engine.set_variant(Variant::Chess960(File::H, File::A));
    match enable {
        true => String::from("Chess 960 enabled"),
        false => String::from("Chess 960 disabled"),
    }
}

#[allow(dead_code)]
fn set_futility_margin_base(engine: &mut Engine, margin_base: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        futility_margin_base: Some(margin_base as Score),
        ..Default::default()
    }));
    format!("futility-margin-base set to {margin_base}")
}

#[allow(dead_code)]
fn set_futility_margin_per_depth(engine: &mut Engine, margin_per_depth: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        futility_margin_per_depth: Some(margin_per_depth as Score),
        ..Default::default()
    }));
    format!("futility-margin-per-depth set to {margin_per_depth}")
}

#[allow(dead_code)]
fn set_futility_pruning_max_depth(engine: &mut Engine, depth: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        futility_pruning_max_depth: Some(depth as usize),
        ..Default::default()
    }));
    format!("futility-pruning-max-depth set to {depth}")
}

#[allow(dead_code)]
fn set_reverse_futility_margin_base(engine: &mut Engine, margin_base: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        reverse_futility_margin_base: Some(margin_base as Score),
        ..Default::default()
    }));
    format!("reverse-futility-margin-base set to {margin_base}")
}

#[allow(dead_code)]
fn set_reverse_futility_margin_per_depth(engine: &mut Engine, margin_per_depth: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        reverse_futility_margin_per_depth: Some(margin_per_depth as Score),
        ..Default::default()
    }));
    format!("reverse-futility-margin-per-depth set to {margin_per_depth}")
}

#[allow(dead_code)]
fn set_reverse_futility_pruning_max_depth(engine: &mut Engine, depth: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        reverse_futility_pruning_max_depth: Some(depth as usize),
        ..Default::default()
    }));
    format!("reverse-futility-pruning-max-depth set to {depth}")
}

#[allow(dead_code)]
fn set_late_move_pruning_base(engine: &mut Engine, base: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        late_move_pruning_base: Some(base as usize),
        ..Default::default()
    }));
    format!("late-move-pruning-base set to {base}")
}

#[allow(dead_code)]
fn set_late_move_pruning_factor(engine: &mut Engine, factor: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        late_move_pruning_factor: Some(factor as usize),
        ..Default::default()
    }));
    format!("late-move-pruning-factor set to {factor}")
}

#[allow(dead_code)]
fn set_late_move_pruning_max_depth(engine: &mut Engine, depth: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        late_move_pruning_max_depth: Some(depth as usize),
        ..Default::default()
    }));
    format!("late-move-pruning-max-depth set to {depth}")
}

#[allow(dead_code)]
fn set_see_pruning_margin_quiet(engine: &mut Engine, margin_quiet: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        see_pruning_margin_quiet: Some(margin_quiet as Score),
        ..Default::default()
    }));
    format!("see-pruning-margin-quiet set to {margin_quiet}")
}

#[allow(dead_code)]
fn set_see_pruning_margin_tactical(engine: &mut Engine, margin_tactical: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        see_pruning_margin_tactical: Some(margin_tactical as Score),
        ..Default::default()
    }));
    format!("see-pruning-margin-tactical set to {margin_tactical}")
}

#[allow(dead_code)]
fn set_see_pruning_max_depth(engine: &mut Engine, depth: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        see_pruning_max_depth: Some(depth as usize),
        ..Default::default()
    }));
    format!("see-pruning-max-depth set to {depth}")
}

#[allow(dead_code)]
fn set_aspiration_window_initial_width(engine: &mut Engine, width: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        aspiration_window_initial_width: Some(width as i32),
        ..Default::default()
    }));
    format!("aspiration-window-initial-width set to {width}")
}

#[allow(dead_code)]
fn set_aspiration_window_grow_rate(engine: &mut Engine, grow_rate: i64) -> String {
    engine.set_search_params(SearchParamsEachAlgo::AlphaBeta(AlphaBetaParams {
        aspiration_window_grow_rate: Some(grow_rate as i32),
        ..Default::default()
    }));
    format!("aspiration-window-grow-rate set to {grow_rate}")
}
