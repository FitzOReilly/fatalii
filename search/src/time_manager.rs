use crate::SearchOptions;
use movegen::side::Side;
use std::{cmp, time::Duration};

const DEFAULT_MOVES_TO_GO: usize = 40;

pub struct TimeManager;

impl TimeManager {
    pub fn calc_movetime_hard_limit(
        side_to_move: Side,
        options: &SearchOptions,
    ) -> Option<Duration> {
        if let Some(dur) = options.movetime {
            return Some(dur);
        }

        const MIN_TIME: Duration = Duration::from_millis(0);

        let moves_to_go = options.moves_to_go.unwrap_or(DEFAULT_MOVES_TO_GO);
        let (time_millis, inc_millis) = match side_to_move {
            Side::White if options.white_time.is_some() => (
                options
                    .white_time
                    .unwrap_or(Duration::from_millis(0))
                    .as_millis(),
                options
                    .white_inc
                    .unwrap_or(Duration::from_millis(0))
                    .as_millis(),
            ),
            Side::Black if options.black_time.is_some() => (
                options
                    .black_time
                    .unwrap_or(Duration::from_millis(0))
                    .as_millis(),
                options
                    .black_inc
                    .unwrap_or(Duration::from_millis(0))
                    .as_millis(),
            ),
            _ => return None,
        };
        let quot = (moves_to_go as f64).sqrt() as u64;
        let max_time = time_millis as u64 / quot + inc_millis as u64;
        let hard_time_limit = match time_millis.checked_sub(options.move_overhead.as_millis()) {
            Some(t) => Duration::from_millis(cmp::min(t as u64, max_time)),
            None => MIN_TIME,
        };
        Some(hard_time_limit)
    }

    pub fn calc_movetime_soft_limit(
        side_to_move: Side,
        options: &SearchOptions,
    ) -> Option<Duration> {
        let moves_to_go = options.moves_to_go.unwrap_or(DEFAULT_MOVES_TO_GO);
        let (time_millis, inc_millis) = match side_to_move {
            Side::White if options.white_time.is_some() => (
                options
                    .white_time
                    .unwrap_or(Duration::from_millis(0))
                    .as_millis(),
                options
                    .white_inc
                    .unwrap_or(Duration::from_millis(0))
                    .as_millis(),
            ),
            Side::Black if options.black_time.is_some() => (
                options
                    .black_time
                    .unwrap_or(Duration::from_millis(0))
                    .as_millis(),
                options
                    .black_inc
                    .unwrap_or(Duration::from_millis(0))
                    .as_millis(),
            ),
            _ => return None,
        };
        // We don't add the full increment to the soft limit. Otherwise we would
        // be running into the hard limit almost every move if we have very
        // little time left.
        const INC_DIVISOR: u64 = 2;
        let soft_limit = time_millis as u64 / moves_to_go as u64 + inc_millis as u64 / INC_DIVISOR;
        Some(Duration::from_millis(soft_limit))
    }
}
