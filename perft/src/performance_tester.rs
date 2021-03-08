use movegen::move_generator::MoveGenerator;
use movegen::position_history::PositionHistory;
use movegen::r#move::MoveList;

pub struct PerformanceTester {
    pos_history: PositionHistory,
}

impl PerformanceTester {
    pub fn new(pos_history: PositionHistory) -> PerformanceTester {
        PerformanceTester { pos_history }
    }

    pub fn count_nodes(&mut self, depth: usize) -> usize {
        let mut move_list_stack = vec![MoveList::new(); depth];
        self.recursive_count_nodes(&mut move_list_stack, depth)
    }

    fn recursive_count_nodes(
        &mut self,
        move_list_stack: &mut Vec<MoveList>,
        depth: usize,
    ) -> usize {
        let mut num_nodes = 0;

        match depth {
            0 => {
                debug_assert!(move_list_stack.is_empty());
                num_nodes = 1;
            }
            _ => {
                debug_assert!(!move_list_stack.is_empty());
                let mut move_list = move_list_stack.pop().unwrap();
                MoveGenerator::generate_moves(&mut move_list, self.pos_history.current_pos());
                match depth {
                    1 => {
                        debug_assert!(move_list_stack.is_empty());
                        num_nodes = move_list.len();
                    }
                    _ => {
                        for m in move_list.iter() {
                            self.pos_history.do_move(*m);
                            num_nodes += self.recursive_count_nodes(move_list_stack, depth - 1);
                            self.pos_history.undo_last_move();
                        }
                    }
                }
                move_list_stack.push(move_list);
            }
        };

        num_nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use movegen::fen::Fen;
    use movegen::position::Position;

    #[test]
    fn perft_initial_position_low_depth() {
        let pos_history = PositionHistory::new(Position::initial());
        let mut perft = PerformanceTester::new(pos_history);
        assert_eq!(1, perft.count_nodes(0));
        assert_eq!(20, perft.count_nodes(1));
        assert_eq!(400, perft.count_nodes(2));
        assert_eq!(8_902, perft.count_nodes(3));
    }

    #[test]
    #[ignore]
    fn perft_initial_position_high_depth() {
        let pos_history = PositionHistory::new(Position::initial());
        let mut perft = PerformanceTester::new(pos_history);
        assert_eq!(197_281, perft.count_nodes(4));
        assert_eq!(4_865_609, perft.count_nodes(5));
        assert_eq!(119_060_324, perft.count_nodes(6));
    }

    #[test]
    fn perft_tricky_position_low_depth() {
        // Position from https://www.chessprogramming.org/Perft_Results
        let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let pos_history = PositionHistory::new(Fen::to_position(&fen));
        let mut perft = PerformanceTester::new(pos_history);
        assert_eq!(1, perft.count_nodes(0));
        assert_eq!(44, perft.count_nodes(1));
        assert_eq!(1_486, perft.count_nodes(2));
        assert_eq!(62_379, perft.count_nodes(3));
    }

    #[test]
    #[ignore]
    fn perft_tricky_position_high_depth() {
        // Position from https://www.chessprogramming.org/Perft_Results
        let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let pos_history = PositionHistory::new(Fen::to_position(&fen));
        let mut perft = PerformanceTester::new(pos_history);
        assert_eq!(2_103_487, perft.count_nodes(4));
        assert_eq!(89_941_194, perft.count_nodes(5));
    }
}
