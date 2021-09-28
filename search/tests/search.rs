use eval::eval::{Eval, CHECKMATE_BLACK, CHECKMATE_WHITE, EQUAL_POSITION};
use movegen::piece;
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use movegen::r#move::{Move, MoveType};
use movegen::side::Side;
use movegen::square::Square;
use search::alpha_beta::AlphaBeta;
use search::negamax::Negamax;
use search::search::{Search, SearchInfo, SearchResult};
use search::searcher::Searcher;
use std::sync::mpsc;
use std::time::Duration;

const TABLE_IDX_BITS: usize = 16;
const TIMEOUT_PER_TEST: Duration = Duration::from_millis(10000);

struct SearchTester {
    searcher: Searcher,
    result_receiver: mpsc::Receiver<SearchInfo>,
}

impl SearchTester {
    fn new(search_algo: impl Search + Send + 'static) -> SearchTester {
        let (sender, receiver) = mpsc::channel();
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
            let search_result = match self.result_receiver.recv_timeout(TIMEOUT_PER_TEST) {
                Ok(SearchInfo::DepthFinished(res)) => res,
                unexp => panic!("Expected SearchInfo::DepthFinished(_), got {:?}", unexp),
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
            assert_eq!(
                *exp_sr, act_sr,
                "Iteration: {}, Depth: {}, Score (exp / act): ({} / {}), Best move (exp / act): ({} / {})",
                idx,
                depth,
                exp_sr.score(),
                act_sr.score(),
                exp_sr.best_move(),
                act_sr.best_move()
            );
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
        Move::new(Square::D8, Square::H4, MoveType::QUIET),
    );

    let mut tester = SearchTester::new(search_algo);
    assert_eq!(expected, tester.search(pos_history, depth));
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
        Move::new(Square::A1, Square::A8, MoveType::QUIET),
    );

    let mut tester = SearchTester::new(search_algo);
    assert_eq!(expected, tester.search(pos_history, depth));
}

fn stalemate(search_algo: impl Search + Send + 'static) {
    let mut pos = Position::empty();
    pos.set_piece_at(Square::E6, Some(piece::Piece::WHITE_KING));
    pos.set_piece_at(Square::E7, Some(piece::Piece::WHITE_PAWN));
    pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
    pos.set_side_to_move(Side::Black);
    let pos_history = PositionHistory::new(pos);

    let depth = 1;
    let expected = SearchResult::new(depth, EQUAL_POSITION, Move::NULL);

    let mut tester = SearchTester::new(search_algo);
    assert_eq!(expected, tester.search(pos_history, depth));
}

fn search_quiescence(search_algo: impl Search + Send + 'static) {
    let mut tester = SearchTester::new(search_algo);

    // Quiescence search should be invoked immediately because the search depth is 0.
    // Expected: If there are no captures, the quiescence search equals the static evaluation
    let depth = 0;
    let mut pos_history = PositionHistory::new(Position::initial());

    pos_history.do_move(Move::new(
        Square::E2,
        Square::E4,
        MoveType::DOUBLE_PAWN_PUSH,
    ));
    assert_eq!(
        Eval::eval(pos_history.current_pos()),
        tester.search(pos_history.clone(), depth).score()
    );

    pos_history.do_move(Move::new(
        Square::C7,
        Square::C5,
        MoveType::DOUBLE_PAWN_PUSH,
    ));
    assert_eq!(
        Eval::eval(pos_history.current_pos()),
        tester.search(pos_history.clone(), depth).score()
    );

    pos_history.do_move(Move::new(Square::G1, Square::F3, MoveType::QUIET));
    assert_eq!(
        Eval::eval(pos_history.current_pos()),
        tester.search(pos_history.clone(), depth).score()
    );

    pos_history.do_move(Move::new(Square::D7, Square::D6, MoveType::QUIET));
    assert_eq!(
        Eval::eval(pos_history.current_pos()),
        tester.search(pos_history.clone(), depth).score()
    );

    pos_history.do_move(Move::new(
        Square::D2,
        Square::D4,
        MoveType::DOUBLE_PAWN_PUSH,
    ));

    pos_history.do_move(Move::new(Square::C5, Square::D4, MoveType::CAPTURE));
    let score_current = Eval::eval(pos_history.current_pos());

    // Position after 1. e4 c5 2. Nf3 d6 3. d4 cxd4.
    // The only possible captures are 4. Nxd4 and 4. Qxd4.
    // Expected: The quiescence search equals the maximum of the static evaluation of the
    // current position and all the captures
    pos_history.do_move(Move::new(Square::F3, Square::D4, MoveType::CAPTURE));
    let score_nxd4 = Eval::eval(pos_history.current_pos());
    pos_history.undo_last_move();
    pos_history.do_move(Move::new(Square::D1, Square::D4, MoveType::CAPTURE));
    let score_qxd4 = Eval::eval(pos_history.current_pos());
    pos_history.undo_last_move();
    let score = *[score_current, score_nxd4, score_qxd4]
        .iter()
        .max()
        .unwrap();
    assert_eq!(score, tester.search(pos_history.clone(), depth).score());

    pos_history.do_move(Move::new(Square::F3, Square::D4, MoveType::CAPTURE));
    assert_eq!(
        Eval::eval(pos_history.current_pos()),
        tester.search(pos_history.clone(), depth).score()
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
