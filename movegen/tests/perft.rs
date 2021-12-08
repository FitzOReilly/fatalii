use movegen::fen::Fen;
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use perft::PerformanceTester;

const TABLE_IDX_BITS: usize = 16;

#[test]
fn perft_initial_position_low_depth() {
    let pos_history = PositionHistory::new(Position::initial());
    let mut perft = PerformanceTester::new(pos_history, TABLE_IDX_BITS);
    assert_eq!(1, perft.count_nodes(0));
    assert_eq!(20, perft.count_nodes(1));
    assert_eq!(400, perft.count_nodes(2));
    assert_eq!(8_902, perft.count_nodes(3));
}

#[test]
#[ignore]
fn perft_initial_position_high_depth() {
    let pos_history = PositionHistory::new(Position::initial());
    let mut perft = PerformanceTester::new(pos_history, TABLE_IDX_BITS);
    assert_eq!(197_281, perft.count_nodes(4));
    assert_eq!(4_865_609, perft.count_nodes(5));
    assert_eq!(119_060_324, perft.count_nodes(6));
}

#[test]
fn perft_tricky_position_low_depth() {
    // Position from https://www.chessprogramming.org/Perft_Results
    let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    let pos_history = PositionHistory::new(Fen::str_to_pos(&fen).unwrap());
    let mut perft = PerformanceTester::new(pos_history, TABLE_IDX_BITS);
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
    let pos_history = PositionHistory::new(Fen::str_to_pos(&fen).unwrap());
    let mut perft = PerformanceTester::new(pos_history, TABLE_IDX_BITS);
    assert_eq!(2_103_487, perft.count_nodes(4));
    assert_eq!(89_941_194, perft.count_nodes(5));
}
