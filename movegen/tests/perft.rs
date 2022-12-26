use movegen::fen::Fen;
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use perft::PerformanceTester;

const BYTES: usize = 32 * 64 * 1024;

#[test]
fn perft_initial_position_low_depth() {
    let pos_history = PositionHistory::new(Position::initial());
    let mut perft = PerformanceTester::new(pos_history, BYTES);
    assert_eq!(1, perft.count_nodes(0));
    assert_eq!(20, perft.count_nodes(1));
    assert_eq!(400, perft.count_nodes(2));
    assert_eq!(8_902, perft.count_nodes(3));
}

#[test]
#[ignore]
fn perft_initial_position_high_depth() {
    let pos_history = PositionHistory::new(Position::initial());
    let mut perft = PerformanceTester::new(pos_history, BYTES);
    assert_eq!(197_281, perft.count_nodes(4));
    assert_eq!(4_865_609, perft.count_nodes(5));
    assert_eq!(119_060_324, perft.count_nodes(6));
}

#[test]
fn perft_middlegame_position_low_depth() {
    // Position from https://www.chessprogramming.org/Perft_Results
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let pos_history = PositionHistory::new(Fen::str_to_pos(&fen).unwrap());
    let mut perft = PerformanceTester::new(pos_history, BYTES);
    assert_eq!(1, perft.count_nodes(0));
    assert_eq!(48, perft.count_nodes(1));
    assert_eq!(2_039, perft.count_nodes(2));
    assert_eq!(97_862, perft.count_nodes(3));
}

#[test]
#[ignore]
fn perft_middlegame_position_high_depth() {
    // Position from https://www.chessprogramming.org/Perft_Results
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let pos_history = PositionHistory::new(Fen::str_to_pos(&fen).unwrap());
    let mut perft = PerformanceTester::new(pos_history, BYTES);
    assert_eq!(4_085_603, perft.count_nodes(4));
    assert_eq!(193_690_690, perft.count_nodes(5));
    // assert_eq!(8_031_647_685, perft.count_nodes(6));
}

#[test]
fn perft_tricky_position_low_depth() {
    // Position from https://www.chessprogramming.org/Perft_Results
    let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    let pos_history = PositionHistory::new(Fen::str_to_pos(&fen).unwrap());
    let mut perft = PerformanceTester::new(pos_history, BYTES);
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
    let mut perft = PerformanceTester::new(pos_history, BYTES);
    assert_eq!(2_103_487, perft.count_nodes(4));
    assert_eq!(89_941_194, perft.count_nodes(5));
}
