use movegen::fen::Fen;
use movegen::performance_tester::PerformanceTester;
use movegen::position::Position;
use movegen::position_history::PositionHistory;

const TABLE_SIZE: usize = 16 * 1024 * 1024;

fn kiwipete_position() -> Position {
    // Position from https://www.chessprogramming.org/Perft_Results
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    Fen::str_to_pos(fen).unwrap()
}

fn tricky_position() -> Position {
    // Position from https://www.chessprogramming.org/Perft_Results
    let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    Fen::str_to_pos(fen).unwrap()
}

fn verify_node_count(pos: Position, depth: usize, expected_node_count: usize) {
    let pos_history = PositionHistory::new(pos);
    let mut perft = PerformanceTester::new(pos_history, TABLE_SIZE);
    assert_eq!(expected_node_count, perft.count_nodes(depth));
}

#[test]
fn perft_initial_position_depth_0() {
    verify_node_count(Position::initial(), 0, 1);
}

#[test]
fn perft_initial_position_depth_1() {
    verify_node_count(Position::initial(), 1, 20);
}

#[test]
fn perft_initial_position_depth_2() {
    verify_node_count(Position::initial(), 2, 400);
}

#[test]
fn perft_initial_position_depth_3() {
    verify_node_count(Position::initial(), 3, 8_902);
}

#[test]
fn perft_initial_position_depth_4() {
    verify_node_count(Position::initial(), 4, 197_281);
}

#[test]
#[ignore]
fn perft_initial_position_depth_5() {
    verify_node_count(Position::initial(), 5, 4_865_609);
}

#[test]
#[ignore]
fn perft_initial_position_depth_6() {
    verify_node_count(Position::initial(), 6, 119_060_324);
}

#[test]
fn perft_kiwipete_position_depth_0() {
    verify_node_count(kiwipete_position(), 0, 1);
}

#[test]
fn perft_kiwipete_position_depth_1() {
    verify_node_count(kiwipete_position(), 1, 48);
}

#[test]
fn perft_kiwipete_position_depth_2() {
    verify_node_count(kiwipete_position(), 2, 2_039);
}

#[test]
fn perft_kiwipete_position_depth_3() {
    verify_node_count(kiwipete_position(), 3, 97_862);
}

#[test]
#[ignore]
fn perft_kiwipete_position_depth_4() {
    verify_node_count(kiwipete_position(), 4, 4_085_603);
}

#[test]
#[ignore]
fn perft_kiwipete_position_depth_5() {
    verify_node_count(kiwipete_position(), 5, 193_690_690);
}

#[test]
#[ignore]
fn perft_kiwipete_position_depth_6() {
    verify_node_count(kiwipete_position(), 6, 8_031_647_685);
}

#[test]
fn perft_tricky_position_depth_0() {
    verify_node_count(tricky_position(), 0, 1);
}

#[test]
fn perft_tricky_position_depth_1() {
    verify_node_count(tricky_position(), 1, 44);
}

#[test]
fn perft_tricky_position_depth_2() {
    verify_node_count(tricky_position(), 2, 1_486);
}

#[test]
fn perft_tricky_position_depth_3() {
    verify_node_count(tricky_position(), 3, 62_379);
}

#[test]
#[ignore]
fn perft_tricky_position_depth_4() {
    verify_node_count(tricky_position(), 4, 2_103_487);
}

#[test]
#[ignore]
fn perft_tricky_position_depth_5() {
    verify_node_count(tricky_position(), 5, 89_941_194);
}
