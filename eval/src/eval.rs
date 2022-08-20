use movegen::{position::Position, side::Side};

pub type Score = i16;

// Avoid overflow of -NEGATIVE_INF
pub const NEGATIVE_INF: Score = Score::MIN + 1;
pub const POSITIVE_INF: Score = Score::MAX;
pub const EQUAL_POSITION: Score = 0;
// We must be able to distinguish these values from +/-inf
pub const CHECKMATE_WHITE: Score = NEGATIVE_INF + 1;
pub const CHECKMATE_BLACK: Score = POSITIVE_INF - 1;

pub trait Eval {
    fn eval(&mut self, pos: &Position) -> Score;

    fn eval_relative(&mut self, pos: &Position) -> Score {
        match pos.side_to_move() {
            Side::White => self.eval(pos),
            Side::Black => -self.eval(pos),
        }
    }
}
