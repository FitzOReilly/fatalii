use eval::Score;

// Minimum depth for principal variation search. Disable null-window searches below this depth.
pub const MIN_PVS_DEPTH: usize = 3;

// Minimum depth for null move pruning.
pub const MIN_NULL_MOVE_PRUNE_DEPTH: usize = 3;

// Minimum depth for late move reductions.
pub const MIN_LATE_MOVE_REDUCTION_DEPTH: usize = 3;

// Enable futility pruning if the evaluation plus this value is less than alpha.
const FUTILITY_MARGIN_BASE: Score = 12;
const FUTILITY_MARGIN_PER_DEPTH: Score = 235;
const FUTILITY_PRUNING_MAX_DEPTH: usize = 5;

// Enable reverse futility pruning if the evaluation plus this value is greater than or equal to beta.
const REVERSE_FUTILITY_MARGIN_BASE: Score = 115;
const REVERSE_FUTILITY_MARGIN_PER_DEPTH: Score = 51;
const REVERSE_FUTILITY_PRUNING_MAX_DEPTH: usize = 6;

// Late move pruning
const LATE_MOVE_PRUNING_BASE: usize = 4;
const LATE_MOVE_PRUNING_FACTOR: usize = 1;
const LATE_MOVE_PRUNING_MAX_DEPTH: usize = 5;

// Late move reductions
const LATE_MOVE_REDUCTIONS_CENTI_BASE: usize = 25;
const LATE_MOVE_REDUCTIONS_CENTI_DIVISOR: usize = 500;

// Prune a move if the static evaluation plus the move's potential improvement
// plus this value is less than alpha.
pub const DELTA_PRUNING_MARGIN_MOVE: Score = 200;

// Prune all moves if the static evaluation plus this value is less than alpha.
pub const DELTA_PRUNING_MARGIN_ALL_MOVES: Score = 1800;

// Static exchange evaluation pruning
const SEE_PRUNING_MARGIN_QUIET: Score = -100;
const SEE_PRUNING_MARGIN_TACTICAL: Score = -50;
const SEE_PRUNING_MAX_DEPTH: usize = 4;

// Aspiration windows initial width and grow rate on fail-low/hign
const ASPIRATION_WINDOW_INITIAL_WIDTH: i32 = 101;
const ASPIRATION_WINDOW_GROW_RATE: i32 = 15;

pub struct SearchParams {
    pub futility_margin_base: Score,
    pub futility_margin_per_depth: Score,
    pub futility_pruning_max_depth: usize,
    pub reverse_futility_margin_base: Score,
    pub reverse_futility_margin_per_depth: Score,
    pub reverse_futility_pruning_max_depth: usize,
    pub late_move_pruning_base: usize,
    pub late_move_pruning_factor: usize,
    pub late_move_pruning_max_depth: usize,
    pub late_move_reductions_centi_base: usize,
    pub late_move_reductions_centi_divisor: usize,
    pub see_pruning_margin_quiet: Score,
    pub see_pruning_margin_tactical: Score,
    pub see_pruning_max_depth: usize,
    pub aspiration_window_initial_width: i32,
    pub aspiration_window_grow_rate: i32,
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            futility_margin_base: FUTILITY_MARGIN_BASE,
            futility_margin_per_depth: FUTILITY_MARGIN_PER_DEPTH,
            futility_pruning_max_depth: FUTILITY_PRUNING_MAX_DEPTH,
            reverse_futility_margin_base: REVERSE_FUTILITY_MARGIN_BASE,
            reverse_futility_margin_per_depth: REVERSE_FUTILITY_MARGIN_PER_DEPTH,
            reverse_futility_pruning_max_depth: REVERSE_FUTILITY_PRUNING_MAX_DEPTH,
            late_move_pruning_base: LATE_MOVE_PRUNING_BASE,
            late_move_pruning_factor: LATE_MOVE_PRUNING_FACTOR,
            late_move_pruning_max_depth: LATE_MOVE_PRUNING_MAX_DEPTH,
            late_move_reductions_centi_base: LATE_MOVE_REDUCTIONS_CENTI_BASE,
            late_move_reductions_centi_divisor: LATE_MOVE_REDUCTIONS_CENTI_DIVISOR,
            see_pruning_margin_quiet: SEE_PRUNING_MARGIN_QUIET,
            see_pruning_margin_tactical: SEE_PRUNING_MARGIN_TACTICAL,
            see_pruning_max_depth: SEE_PRUNING_MAX_DEPTH,
            aspiration_window_initial_width: ASPIRATION_WINDOW_INITIAL_WIDTH,
            aspiration_window_grow_rate: ASPIRATION_WINDOW_GROW_RATE,
        }
    }
}

#[derive(Debug, Default)]
pub struct SearchParamsOptions {
    pub futility_margin_base: Option<Score>,
    pub futility_margin_per_depth: Option<Score>,
    pub futility_pruning_max_depth: Option<usize>,
    pub reverse_futility_margin_base: Option<Score>,
    pub reverse_futility_margin_per_depth: Option<Score>,
    pub reverse_futility_pruning_max_depth: Option<usize>,
    pub late_move_pruning_base: Option<usize>,
    pub late_move_pruning_factor: Option<usize>,
    pub late_move_pruning_max_depth: Option<usize>,
    pub late_move_reductions_centi_base: Option<usize>,
    pub late_move_reductions_centi_divisor: Option<usize>,
    pub see_pruning_margin_quiet: Option<Score>,
    pub see_pruning_margin_tactical: Option<Score>,
    pub see_pruning_max_depth: Option<usize>,
    pub aspiration_window_initial_width: Option<i32>,
    pub aspiration_window_grow_rate: Option<i32>,
}
