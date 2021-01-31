use movegen::move_generator::MoveGenerator;

pub struct PerformanceTester {
    movegen: MoveGenerator,
}

impl PerformanceTester {
    pub fn new(movegen: MoveGenerator) -> PerformanceTester {
        PerformanceTester { movegen }
    }

    pub fn count_nodes(&mut self, depth: usize) -> usize {
        let mut num_nodes = 0;

        match depth {
            0 => {
                num_nodes = 1;
            }
            1 => {
                self.movegen.generate_moves();
                num_nodes += self.movegen.move_list().len();
            }
            _ => {
                self.movegen.generate_moves();
                let moves = self.movegen.move_list();
                for m in moves.iter() {
                    self.movegen.do_move(*m);
                    num_nodes += self.count_nodes(depth - 1);
                    self.movegen.undo_move(*m);
                }
            }
        };

        num_nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use movegen::position::Position;

    #[test]
    fn perft_initial_position_low_depth() {
        let movegen = MoveGenerator::new(Position::initial());
        let mut perft = PerformanceTester::new(movegen);
        assert_eq!(1, perft.count_nodes(0));
        assert_eq!(20, perft.count_nodes(1));
        assert_eq!(400, perft.count_nodes(2));
        assert_eq!(8_902, perft.count_nodes(3));
    }

    #[test]
    #[ignore]
    fn perft_initial_position_high_depth() {
        let movegen = MoveGenerator::new(Position::initial());
        let mut perft = PerformanceTester::new(movegen);
        assert_eq!(197_281, perft.count_nodes(4));
        assert_eq!(4_865_609, perft.count_nodes(5));
        assert_eq!(119_060_324, perft.count_nodes(6));
    }
}
