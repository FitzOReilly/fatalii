use crossbeam_channel::{unbounded, Receiver};
use eval::eval::{Eval, CHECKMATE_BLACK, CHECKMATE_WHITE, EQUAL_POSITION, NEGATIVE_INF};
use movegen::fen::Fen;
use movegen::move_generator::MoveGenerator;
use movegen::piece;
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveList, MoveType};
use movegen::side::Side;
use movegen::square::Square;
use search::alpha_beta::AlphaBeta;
use search::negamax::Negamax;
use search::search::{Search, SearchInfo, SearchResult};
use search::searcher::Searcher;
use std::cmp;
use std::time::Duration;

const TABLE_IDX_BITS: usize = 20;
const TIMEOUT_PER_TEST: Duration = Duration::from_millis(30000);

struct SearchTester {
    searcher: Searcher,
    result_receiver: Receiver<SearchInfo>,
}

impl SearchTester {
    fn new(search_algo: impl Search + Send + 'static) -> SearchTester {
        let (sender, receiver) = unbounded();
        let info_callback = Box::new(move |search_info| {
            sender.send(search_info).unwrap();
        });

        SearchTester {
            searcher: Searcher::new(search_algo, info_callback),
            result_receiver: receiver,
        }
    }

    fn search(&mut self, pos_hist: PositionHistory, depth: usize) -> SearchResult {
        self.searcher.search(pos_hist, depth);
        loop {
            let received = self.result_receiver.recv_timeout(TIMEOUT_PER_TEST);
            println!("{:?}", received);
            let search_result = match received {
                Ok(SearchInfo::DepthFinished(res)) => res,
                unexp => panic!("Expected Ok(SearchInfo::DepthFinished(_)), got {:?}", unexp),
            };
            assert!(
                search_result.depth() <= depth,
                "Expected max depth: {}, actual depth: {}",
                depth,
                search_result.depth()
            );
            if search_result.depth() == depth {
                self.searcher.stop();
                loop {
                    if let Ok(SearchInfo::Stopped) = self.result_receiver.recv() {
                        break;
                    }
                }
                return search_result;
            }
        }
    }
}

fn search_results_equal(min_depth: usize, max_depth: usize, mut searchers: Vec<SearchTester>) {
    assert!(searchers.len() >= 2);

    let ref_searcher = &mut searchers[0];
    let pos_history = PositionHistory::new(Position::initial());
    let mut exp_search_result = Vec::new();
    for depth in min_depth..=max_depth {
        exp_search_result.push(ref_searcher.search(pos_history.clone(), depth));
    }

    for (idx, searcher) in searchers[1..].iter_mut().enumerate() {
        for depth in min_depth..=max_depth {
            let exp_sr = &exp_search_result[depth - 1];
            let act_sr = searcher.search(pos_history.clone(), depth);
            println!(
                "Iteration: {}, Depth: {}, Score (exp / act): ({} / {}), Best move (exp / act): ({} / {})",
                idx,
                depth,
                exp_sr.score(),
                act_sr.score(),
                exp_sr.best_move(),
                act_sr.best_move()
            );
            assert_eq!(exp_sr.score(), act_sr.score());
            assert_eq!(exp_sr.best_move(), act_sr.best_move());
        }
    }
}

fn checkmate_white(search_algo: impl Search + Send + 'static) {
    let mut pos_history = PositionHistory::new(Position::initial());
    pos_history.do_move(Move::new(Square::F2, Square::F3, MoveType::QUIET));
    pos_history.do_move(Move::new(Square::E7, Square::E6, MoveType::QUIET));
    pos_history.do_move(Move::new(
        Square::G2,
        Square::G4,
        MoveType::DOUBLE_PAWN_PUSH,
    ));

    let depth = 2;
    let expected = SearchResult::new(
        depth,
        CHECKMATE_WHITE,
        0,
        Move::new(Square::D8, Square::H4, MoveType::QUIET),
        MoveList::new(),
    );

    let mut tester = SearchTester::new(search_algo);
    let actual = tester.search(pos_history, depth);
    assert_eq!(expected.depth(), actual.depth());
    assert_eq!(expected.score(), actual.score());
    assert_eq!(expected.best_move(), actual.best_move());
}

fn checkmate_black(search_algo: impl Search + Send + 'static) {
    let mut pos = Position::empty();
    pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
    pos.set_piece_at(Square::A1, Some(piece::Piece::WHITE_ROOK));
    pos.set_piece_at(Square::G7, Some(piece::Piece::BLACK_PAWN));
    pos.set_piece_at(Square::H7, Some(piece::Piece::BLACK_PAWN));
    pos.set_piece_at(Square::H8, Some(piece::Piece::BLACK_KING));
    pos.set_side_to_move(Side::White);
    let pos_history = PositionHistory::new(pos);

    let depth = 2;
    let expected = SearchResult::new(
        depth,
        CHECKMATE_BLACK,
        0,
        Move::new(Square::A1, Square::A8, MoveType::QUIET),
        MoveList::new(),
    );

    let mut tester = SearchTester::new(search_algo);
    let actual = tester.search(pos_history, depth);
    assert_eq!(expected.depth(), actual.depth());
    assert_eq!(expected.score(), actual.score());
    assert_eq!(expected.best_move(), actual.best_move());
}

fn stalemate(search_algo: impl Search + Send + 'static) {
    let mut pos = Position::empty();
    pos.set_piece_at(Square::E6, Some(piece::Piece::WHITE_KING));
    pos.set_piece_at(Square::E7, Some(piece::Piece::WHITE_PAWN));
    pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
    pos.set_side_to_move(Side::Black);
    let pos_history = PositionHistory::new(pos);

    let depth = 1;
    let expected = SearchResult::new(depth, EQUAL_POSITION, 0, Move::NULL, MoveList::new());

    let mut tester = SearchTester::new(search_algo);
    let actual = tester.search(pos_history, depth);
    assert_eq!(expected.depth(), actual.depth());
    assert_eq!(expected.score(), actual.score());
    assert_eq!(expected.nodes(), actual.nodes());
    assert_eq!(expected.best_move(), actual.best_move());
}

fn search_quiescence(search_algo: impl Search + Send + 'static) {
    let mut tester = SearchTester::new(search_algo);

    // Expected: If there are no captures, the quiescence search equals the static evaluation
    let depth = 1;
    let mut pos_history = PositionHistory::new(Position::initial());

    let mut move_list = MoveList::new();
    MoveGenerator::generate_moves(&mut move_list, pos_history.current_pos());

    let mut max_score = NEGATIVE_INF;
    for m in move_list.iter() {
        pos_history.do_move(*m);
        max_score = cmp::max(max_score, Eval::eval(pos_history.current_pos()));
        pos_history.undo_last_move();
    }
    assert_eq!(max_score, tester.search(pos_history.clone(), depth).score());
}

fn pv_valid_after_hash_table_hit_depth_1(search_algo: impl Search + Send + 'static) {
    let pos = Position::initial();
    let depth = 1;
    let mut tester = SearchTester::new(search_algo);

    // Fill hash table in initial position
    let mut pos_history = PositionHistory::new(pos.clone());
    let _ = tester.search(pos_history.clone(), depth);
    // Fill PV table with PV after 1. e4
    pos_history.do_move(Move::new(
        Square::E2,
        Square::E4,
        MoveType::DOUBLE_PAWN_PUSH,
    ));
    let _ = tester.search(pos_history.clone(), depth);
    // Search again in initial position. This time, we hit a hash table entry.
    // The PV should still be correct.
    pos_history.undo_last_move();
    let res = tester.search(pos_history.clone(), depth);

    assert!(
        check_moves_valid(&mut pos_history, &mut res.principal_variation().clone()),
        "\nPosition: {}\n{}Invalid PV: {}\n",
        Fen::pos_to_str(&pos),
        pos,
        res.principal_variation()
    );
}

fn pv_valid_after_hash_table_hit_depth_greater_than_1(search_algo: impl Search + Send + 'static) {
    let fen = "6k1/p3nr2/7Q/2B5/4N3/8/br1K1PPP/6NR w - - 3 26";
    let pos = Fen::str_to_pos(fen).unwrap();
    let depth = 4;
    let mut tester = SearchTester::new(search_algo);

    let mut pos_history = PositionHistory::new(pos.clone());
    let res = tester.search(pos_history.clone(), depth);

    assert!(
        check_moves_valid(&mut pos_history, &mut res.principal_variation().clone()),
        "\nPosition: {}\n{}Invalid PV: {}\n",
        fen,
        pos,
        res.principal_variation()
    );
}

fn check_moves_valid(pos_history: &mut PositionHistory, pv: &mut MoveList) -> bool {
    if pv.is_empty() {
        return true;
    }
    let m = pv.remove(0);
    let mut move_list = MoveList::new();
    MoveGenerator::generate_moves(&mut move_list, pos_history.current_pos());
    if move_list.contains(&m) {
        pos_history.do_move(m);
        check_moves_valid(pos_history, pv)
    } else {
        false
    }
}

fn count_searched_nodes(search_algo: impl Search + Send + 'static, pos: Position, depth: usize) {
    let mut tester = SearchTester::new(search_algo);
    let pos_history = PositionHistory::new(pos.clone());
    let res = tester.search(pos_history, depth);
    print!(
        "Counting search nodes...\nPosition:\n{}Depth: {}\nSearched nodes (lower is better): {}\n",
        pos,
        depth,
        res.nodes()
    );
}

#[test]
#[ignore]
fn negamax_search_results_independent_of_transposition_table_size() {
    // Expected: The search result should be the same for different table sizes. The
    // transposition table should only improve the performance of the search, but not the
    // evaluation or the best move.

    let max_table_idx_bits = TABLE_IDX_BITS;
    let min_depth = 1;
    let max_depth = 2;
    let mut searchers = Vec::new();
    for table_idx_bits in 1..=max_table_idx_bits {
        searchers.push(SearchTester::new(Negamax::new(table_idx_bits)));
    }
    search_results_equal(min_depth, max_depth, searchers);
}

#[test]
fn negamax_checkmate_white() {
    let negamax = Negamax::new(TABLE_IDX_BITS);
    checkmate_white(negamax);
}

#[test]
fn negamax_checkmate_black() {
    let negamax = Negamax::new(TABLE_IDX_BITS);
    checkmate_black(negamax);
}

#[test]
fn negamax_stalemate() {
    let negamax = Negamax::new(TABLE_IDX_BITS);
    stalemate(negamax);
}

#[test]
fn negamax_search_quiescence() {
    let negamax = Negamax::new(TABLE_IDX_BITS);
    search_quiescence(negamax);
}

#[test]
fn negamax_pv_valid_after_hash_table_hit_depth_1() {
    let negamax = Negamax::new(TABLE_IDX_BITS);
    pv_valid_after_hash_table_hit_depth_1(negamax);
}

#[test]
#[ignore]
fn negamax_pv_valid_after_hash_table_hit_depth_greater_than_1() {
    let negamax = Negamax::new(TABLE_IDX_BITS);
    pv_valid_after_hash_table_hit_depth_greater_than_1(negamax);
}

#[test]
#[ignore]
fn alpha_beta_search_results_independent_of_transposition_table_size() {
    // Expected: The search result should be the same for different table sizes. The
    // transposition table should only improve the performance of the search, but not the
    // evaluation or the best move.

    let max_table_idx_bits = TABLE_IDX_BITS;
    let min_depth = 1;
    let max_depth = 5;
    let mut searchers = Vec::new();
    for table_idx_bits in 1..=max_table_idx_bits {
        searchers.push(SearchTester::new(AlphaBeta::new(table_idx_bits)));
    }
    search_results_equal(min_depth, max_depth, searchers);
}

#[test]
fn alpha_beta_checkmate_white() {
    let alpha_beta = AlphaBeta::new(TABLE_IDX_BITS);
    checkmate_white(alpha_beta);
}

#[test]
fn alpha_beta_checkmate_black() {
    let alpha_beta = AlphaBeta::new(TABLE_IDX_BITS);
    checkmate_black(alpha_beta);
}

#[test]
fn alpha_beta_stalemate() {
    let alpha_beta = AlphaBeta::new(TABLE_IDX_BITS);
    stalemate(alpha_beta);
}

#[test]
fn alpha_beta_search_quiescence() {
    let alpha_beta = AlphaBeta::new(TABLE_IDX_BITS);
    search_quiescence(alpha_beta);
}

#[test]
fn alpha_beta_pv_valid_after_hash_table_hit_depth_1() {
    let alpha_beta = AlphaBeta::new(TABLE_IDX_BITS);
    pv_valid_after_hash_table_hit_depth_1(alpha_beta);
}

#[test]
#[ignore]
fn alpha_beta_pv_valid_after_hash_table_hit_depth_greater_than_1() {
    let alpha_beta = AlphaBeta::new(TABLE_IDX_BITS);
    pv_valid_after_hash_table_hit_depth_greater_than_1(alpha_beta);
}

#[test]
#[ignore = "benchmark"]
fn alpha_beta_count_searched_nodes_middlegame_position() {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let pos = Fen::str_to_pos(fen).unwrap();
    let depth = 6;
    let alpha_beta = AlphaBeta::new(TABLE_IDX_BITS);
    count_searched_nodes(alpha_beta, pos, depth);
}
