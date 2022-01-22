use assert_matches::assert_matches;
use rexpect::{errors::Result, process::wait::WaitStatus, spawn};
use std::{thread, time::Duration};

#[test]
fn test_cli() -> Result<()> {
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
