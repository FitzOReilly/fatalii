#[macro_use]
extern crate more_asserts;

use crossbeam_channel::unbounded;
use engine::Engine;
use movegen::piece::Piece;
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use movegen::square::Square;
use search::alpha_beta::AlphaBeta;
use std::thread;
use std::time::Duration;
use std::time::Instant;

const TABLE_IDX_BITS: usize = 16;

#[test]
fn search_timeout() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let (sender, receiver) = unbounded();
    let mut engine = Engine::new(
        search_algo,
        Box::new(move |_m| {
            sender.send(true).unwrap();
        }),
    );
    engine.set_position_history(Some(PositionHistory::new(Position::initial())));

    let movetime = Duration::from_millis(1000);
    let waittime = Duration::from_millis(2000);
    let tol = 50;

    let start = Instant::now();
    assert!(engine.search_timeout(movetime).is_ok());
    assert!(receiver.recv_timeout(waittime).is_ok());
    let stop = Instant::now();
    assert_le!(
        (stop.duration_since(start).as_millis() as i128 - movetime.as_millis() as i128).abs(),
        tol
    );
    println!(
        "Search time (movetime): {} µs",
        stop.duration_since(start).as_micros()
    );
}

#[test]
fn search_timeout_aborted() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let (sender, receiver) = unbounded();
    let mut engine = Engine::new(
        search_algo,
        Box::new(move |_m| {
            sender.send(true).unwrap();
        }),
    );
    engine.set_position_history(Some(PositionHistory::new(Position::initial())));

    let movetime = Duration::from_millis(1000);
    let waittime = Duration::from_millis(2000);
    let sleeptime = Duration::from_millis(100);
    let tol = 50;

    let start = Instant::now();
    assert!(engine.search_timeout(movetime).is_ok());
    thread::sleep(sleeptime);
    engine.stop();
    assert!(receiver.recv_timeout(waittime).is_ok());
    let stop = Instant::now();
    assert_le!(
        (stop.duration_since(start).as_millis() as i128 - sleeptime.as_millis() as i128).abs(),
        tol
    );
    println!(
        "Search time (abort): {} µs",
        stop.duration_since(start).as_micros()
    );
}

#[test]
fn search_timeout_finished_early() {
    // Test for positions in which the search finishes earlier than the given timeout
    // (e.g. checkmate)
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let (sender, receiver) = unbounded();
    let mut engine = Engine::new(
        search_algo,
        Box::new(move |m| {
            sender.send(true).unwrap();
            println!("Best move: {}", m.unwrap());
        }),
    );
    let mut pos = Position::empty();
    pos.set_piece_at(Square::H1, Some(Piece::WHITE_KING));
    pos.set_piece_at(Square::H3, Some(Piece::BLACK_KING));
    pos.set_piece_at(Square::A1, Some(Piece::BLACK_ROOK));
    engine.set_position_history(Some(PositionHistory::new(pos)));

    let movetime = Duration::from_millis(1000);
    let waittime = Duration::from_millis(100);

    let start = Instant::now();
    assert!(engine.search_timeout(movetime).is_ok());
    assert!(receiver.recv_timeout(waittime).is_ok());
    let stop = Instant::now();
    println!(
        "Search time (finished early): {} µs",
        stop.duration_since(start).as_micros()
    );

    // Test if the timer was properly stopped, even though it didn't receive an explicit stop
    // command. It shouldn't interrupt the next search.
    engine.set_position_history(Some(PositionHistory::new(Position::initial())));

    let movetime = Duration::from_millis(2000);
    let waittime = Duration::from_millis(4000);
    let tol = 50;

    let start = Instant::now();
    assert!(engine.search_timeout(movetime).is_ok());
    assert!(receiver.recv_timeout(waittime).is_ok());
    let stop = Instant::now();
    assert_le!(
        (stop.duration_since(start).as_millis() as i128 - movetime.as_millis() as i128).abs(),
        tol
    );
}