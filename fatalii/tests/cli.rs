use assert_matches::assert_matches;
use rexpect::{errors::Result, process::wait::WaitStatus, spawn};
use std::{thread, time::Duration};

#[test]
fn test_cli() -> Result<()> {
    uci_commands()?;
    quit_while_timer_running()?;
    go_all_options()?;
    Ok(())
}

fn uci_commands() -> Result<()> {
    let mut p = spawn("cargo run", Some(60000))?;

    p.send_line("uci")?;
    p.exp_string("id name")?;
    p.exp_string("id author")?;
    p.exp_string("option name")?;
    p.exp_string("uciok")?;

    p.send_line("isready")?;
    p.exp_string("readyok")?;

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

fn quit_while_timer_running() -> Result<()> {
    let mut p = spawn("cargo run", Some(60000))?;

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

fn go_all_options() -> Result<()> {
    let mut p = spawn("cargo run", Some(60000))?;

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
