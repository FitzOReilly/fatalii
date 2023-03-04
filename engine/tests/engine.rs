mod mock_engine_out;

use crossbeam_channel::unbounded;
use engine::{Engine, EngineOptions};
use eval::material_mobility::MaterialMobility;
use mock_engine_out::MockEngineOut;
use more_asserts::assert_le;
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use search::alpha_beta::AlphaBeta;
use search::SearchOptions;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

const EVALUATOR: MaterialMobility = MaterialMobility::new();
const TABLE_SIZE: usize = 16 * 1024;

#[test]
fn search_timeout() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let (sender, receiver) = unbounded();
    let mut engine = Engine::new(
        search_algo,
        MockEngineOut::new(
            Box::new(|_res| {}),
            Box::new(move |_res| {
                sender.send(true).unwrap();
            }),
        ),
        Arc::new(Mutex::new(EngineOptions::default())),
    );
    engine.set_position_history(Some(PositionHistory::new(Position::initial())));

    let movetime = Duration::from_millis(1000);
    let waittime = Duration::from_millis(2000);
    let tol = 80;

    let start = Instant::now();
    assert!(engine
        .search(SearchOptions {
            movetime: Some(movetime),
            ..Default::default()
        })
        .is_ok());
    assert!(receiver.recv_timeout(waittime).is_ok());
    let stop = Instant::now();
    assert_le!(
        (stop.duration_since(start).as_millis() as i128 - movetime.as_millis() as i128).abs(),
        tol
    );
    println!("Search time (movetime): {:?}", stop.duration_since(start));
}

#[test]
fn search_timeout_aborted() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let (sender, receiver) = unbounded();
    let mut engine = Engine::new(
        search_algo,
        MockEngineOut::new(
            Box::new(move |_res| {}),
            Box::new(move |_res| {
                sender.send(true).unwrap();
            }),
        ),
        Arc::new(Mutex::new(EngineOptions::default())),
    );
    engine.set_position_history(Some(PositionHistory::new(Position::initial())));

    let movetime = Duration::from_millis(1000);
    let waittime = Duration::from_millis(2000);
    let sleeptime = Duration::from_millis(100);
    let tol = 80;

    let start = Instant::now();
    assert!(engine
        .search(SearchOptions {
            movetime: Some(movetime),
            ..Default::default()
        })
        .is_ok());
    thread::sleep(sleeptime);
    engine.stop();
    assert!(receiver.recv_timeout(waittime).is_ok());
    let stop = Instant::now();
    assert_le!(
        (stop.duration_since(start).as_millis() as i128 - sleeptime.as_millis() as i128).abs(),
        tol
    );
    println!("Search time (abort): {:?}", stop.duration_since(start));
}
