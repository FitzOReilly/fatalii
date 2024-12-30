use eval::Score;

pub trait SearchParams {}

#[derive(Debug, Default)]
pub struct AlphaBetaParams {
    pub futility_margin_base: Option<Score>,
    pub futility_margin_per_depth: Option<Score>,
    pub futility_pruning_max_depth: Option<usize>,
    pub reverse_futility_margin_base: Option<Score>,
    pub reverse_futility_margin_per_depth: Option<Score>,
    pub reverse_futility_pruning_max_depth: Option<usize>,
    pub late_move_pruning_base: Option<usize>,
    pub late_move_pruning_factor: Option<usize>,
    pub late_move_pruning_max_depth: Option<usize>,
    pub see_pruning_margin_quiet: Option<Score>,
    pub see_pruning_margin_tactical: Option<Score>,
    pub see_pruning_max_depth: Option<usize>,
    pub aspiration_window_initial_width: Option<i32>,
    pub aspiration_window_grow_rate: Option<i32>,
}

impl SearchParams for AlphaBetaParams {}

#[derive(Debug)]
pub enum SearchParamsEachAlgo {
    AlphaBeta(AlphaBetaParams),
}
