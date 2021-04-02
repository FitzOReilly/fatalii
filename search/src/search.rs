use eval::eval::Score;
use movegen::position_history::PositionHistory;
use movegen::r#move::MoveList;

pub trait Search {
    fn search(pos_history: &mut PositionHistory, depth: usize) -> (Score, MoveList);
}
