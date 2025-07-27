use assert_matches::assert_matches;
use rexpect::{error::Error, process::wait::WaitStatus, spawn};
use std::{thread, time::Duration};

#[test]
#[ignore]
fn test_cli() -> Result<(), Error> {
    let _ = spawn("cargo build --release", Some(120000))?;

    uci_commands()?;
    quit_while_timer_running()?;
    go_all_options()?;
    chess_960()?;
    stress()?;

    Ok(())
}

fn uci_commands() -> Result<(), Error> {
    let mut p = spawn("cargo run --release", Some(30000))?;

    p.send_line("uci")?;
    p.exp_string("id name")?;
    p.exp_string("id author")?;
    p.exp_string("option name")?;
    p.exp_string("option name UCI_Chess960 type check default false")?;
    p.exp_string("uciok")?;

    p.send_line("isready")?;
    p.exp_string("readyok")?;

    p.send_line("debug on")?;
    p.exp_string("info string")?;
    p.send_line("debug off")?;
    p.exp_string("info string")?;

    p.send_line("ucinewgame")?;

    p.send_line("position startpos")?;
    p.send_line("go infinite")?;
    thread::sleep(Duration::from_millis(100));
    p.send_line("stop")?;
    thread::sleep(Duration::from_millis(10));
    p.exp_string("bestmove")?;

    p.send_line("ucinewgame")?;
    p.send_line("go infinite")?;
    p.exp_string("Engine error")?;

    assert_matches!(p.process.status(), Some(WaitStatus::StillAlive));
    p.send_line("quit")?;
    thread::sleep(Duration::from_millis(100));
    assert_matches!(p.process.status(), Some(WaitStatus::Exited(_, 0)));

    Ok(())
}

fn quit_while_timer_running() -> Result<(), Error> {
    let mut p = spawn("cargo run --release", Some(30000))?;

    p.send_line("uci")?;
    p.send_line("isready")?;
    p.send_line("ucinewgame")?;
    p.send_line("position startpos")?;
    p.send_line("go wtime 121000 btime 121000")?;
    thread::sleep(Duration::from_millis(100));
    p.send_line("quit")?;
    thread::sleep(Duration::from_millis(1000));
    assert_matches!(p.process.status(), Some(WaitStatus::Exited(_, 0)));

    Ok(())
}

fn go_all_options() -> Result<(), Error> {
    let mut p = spawn("cargo run --release", Some(30000))?;

    p.send_line("uci")?;
    p.send_line("isready")?;
    p.send_line("ucinewgame")?;
    p.send_line("position startpos")?;

    p.send_line("go searchmoves e2e4 d2d4 wtime 500 btime 500")?;
    thread::sleep(Duration::from_millis(500));
    p.exp_string("bestmove")?;

    p.send_line("go ponder wtime 500 btime 500")?;
    thread::sleep(Duration::from_millis(500));
    p.exp_string("bestmove")?;

    p.send_line("go wtime 500 btime 500 winc 100 binc 100 movestogo 40 depth 3 nodes 1000 mate 5")?;
    thread::sleep(Duration::from_millis(500));
    p.exp_string("bestmove")?;

    p.send_line("go movetime 100")?;
    p.exp_string("bestmove")?;

    p.send_line("go infinite")?;
    thread::sleep(Duration::from_millis(100));
    p.send_line("stop")?;
    p.exp_string("bestmove")?;

    p.send_line("quit")?;
    thread::sleep(Duration::from_millis(400));
    assert_matches!(p.process.status(), Some(WaitStatus::Exited(_, 0)));

    Ok(())
}

fn chess_960() -> Result<(), Error> {
    let mut p = spawn("cargo run --release", Some(30000))?;

    p.send_line("setoption name UCI_Chess960 value true")?;
    p.send_line("position fen rnkbnqbr/pppppppp/8/8/8/8/PPPPPPPP/RNKBNQBR w HAha - 0 1")?;
    p.send_line("go depth 7")?;
    p.exp_string("bestmove")?;

    p.send_line("position fen rkbnqbrn/pppppppp/8/8/8/8/PPPPPPPP/RKBNQBRN w GAga - 0 1 moves e2e4 e7e5 h1g3 h8g6 f1c4 f8c5 d1c3 d8c6 d2d3 d7d6 c1e3")?;
    p.send_line("go depth 7")?;
    p.exp_string("bestmove")?;

    assert_matches!(p.process.status(), Some(WaitStatus::StillAlive));
    p.send_line("quit")?;
    thread::sleep(Duration::from_millis(100));
    assert_matches!(p.process.status(), Some(WaitStatus::Exited(_, 0)));

    Ok(())
}

fn stress() -> Result<(), Error> {
    let mut p = spawn("cargo run --release", Some(30000))?;

    p.send_line("uci")?;
    p.send_line("debug on")?;

    for hash_size in [1, 8, 64] {
        p.send_line(format!("setoption name Hash value {hash_size}").as_str())?;
        p.send_line("isready")?;

        for i in 0..10_000 {
            println!("Hash size: {hash_size}, iteration: {i}");
            p.send_line("ucinewgame")?;
            p.send_line("isready")?;
            p.send_line("position startpos")?;
            p.send_line("isready")?;
            p.send_line("go wtime 0")?;
            p.send_line("isready")?;
        }
    }
    Ok(())
}
