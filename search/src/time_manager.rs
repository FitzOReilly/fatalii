use crate::SearchOptions;
use movegen::side::Side;
use std::time::Duration;

const DEFAULT_MOVES_TO_GO: usize = 40;

pub struct TimeManager;

impl TimeManager {
    // Returns a tuple with 2 options: the first is the hard time limit, the
    // second the soft time limit
    pub fn calc_time_limits(
        side_to_move: Side,
        options: &SearchOptions,
    ) -> (Option<Duration>, Option<Duration>) {
        if let Some(dur) = options.movetime {
            return (Some(dur), None);
        }

        const MIN_TIME: Duration = Duration::from_millis(0);
        let moves_to_go = options.moves_to_go.unwrap_or(DEFAULT_MOVES_TO_GO);

        let (opt_time, opt_inc) = match side_to_move {
            Side::White => (options.white_time, options.white_inc),
            Side::Black => (options.black_time, options.black_inc),
        };
        let (Some(time_millis), inc_millis) = (
            opt_time.map(|t| t.as_millis()),
            opt_inc.map_or(0, |t| t.as_millis()),
        ) else {
            return (None, None);
        };

        // Hard time limit
        let quot = (moves_to_go as f64).sqrt() as u64;
        let max_time = time_millis as u64 / quot + inc_millis as u64;
        let hard_time_limit = match time_millis.checked_sub(options.move_overhead.as_millis()) {
            Some(t) => Duration::from_millis((t as u64).min(max_time)),
            None => MIN_TIME,
        };

        // Soft time limit
        // We don't add the full increment to the soft limit. Otherwise we would
        // be running into the hard limit almost every move if we have very
        // little time left.
        const INC_DIVISOR: u64 = 2;
        let soft_time_limit = Duration::from_millis(
            time_millis as u64 / moves_to_go as u64 + inc_millis as u64 / INC_DIVISOR,
        )
        .min(hard_time_limit);

        (Some(hard_time_limit), Some(soft_time_limit))
    }
}
