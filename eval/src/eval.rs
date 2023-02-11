use crate::Score;
use movegen::{position::Position, side::Side};

pub trait Eval {
    fn eval(&mut self, pos: &Position) -> Score;

    fn eval_relative(&mut self, pos: &Position) -> Score {
        match pos.side_to_move() {
            Side::White => self.eval(pos),
            Side::Black => -self.eval(pos),
        }
    }
}
