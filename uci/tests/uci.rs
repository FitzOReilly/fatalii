mod test_buffer;

use crate::test_buffer::TestBuffer;
use engine::Engine;
use movegen::fen::Fen;
use movegen::position::Position;
use movegen::r#move::Move;
use search::alpha_beta::AlphaBeta;
use std::io::{stdout, Write};
use std::str;
use std::time::Duration;
use uci::parser::{Parser, ParserMessage};
use uci::uci_in::{go, is_ready, position, quit, stop, uci as cmd_uci, ucinewgame};
use uci::uci_out::best_move;

const TABLE_IDX_BITS: usize = 16;
const FEN_STR: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

fn best_move_callback(m: Option<Move>) {
    best_move::write(&mut stdout(), m).unwrap();
}

fn contains(v: Vec<u8>, s: &str) -> bool {
    String::from_utf8(v).unwrap().contains(s)
}

#[test]
fn register_command() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(search_algo, Box::new(best_move_callback));
    let test_writer = TestBuffer::new();
    {
        let mut p = Parser::new(Box::new(test_writer.clone()));

        assert!(p.run_command("unknown\n", &mut engine).is_err());
        assert!(p.run_command("cmd\n", &mut engine).is_err());
        assert!(p.run_command("cmd args\n", &mut engine).is_err());
        assert!(p.run_command(" \r \t  cmd  args \n", &mut engine).is_err());

        let cmd_handler = |writer: &mut dyn Write, args: &str, _engine: &mut Engine| {
            writeln!(writer, "{}", args.trim_end_matches('\n'))?;
            Ok(None)
        };
        p.register_command(String::from("cmd"), Box::new(cmd_handler));

        assert!(p.run_command("unknown\n", &mut engine).is_err());
        assert!(p.run_command("cmd\n", &mut engine).is_ok());
        assert!(p.run_command("cmd args\n", &mut engine).is_ok());
        assert!(p.run_command(" \r \t  cmd  args \n", &mut engine).is_ok());
    }
    let out_str = test_writer.into_string();
    let mut outputs = out_str.split('\n');
    assert_eq!("", outputs.next().unwrap());
    assert_eq!("args", outputs.next().unwrap());
    assert_eq!(" args ", outputs.next().unwrap());
}

#[test]
fn run_command_uci() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(search_algo, Box::new(best_move_callback));
    let test_writer = TestBuffer::new();
    {
        let mut p = Parser::new(Box::new(test_writer.clone()));
        p.register_command(String::from("uci"), Box::new(cmd_uci::run_command));
        assert!(p.run_command("uci invalid\n", &mut engine).is_err());
        assert!(p.run_command("uci\n", &mut engine).is_ok());
    }
    let out = test_writer.into_string();
    assert!(out.contains("id name"));
    assert!(out.contains("id author"));
    assert!(out.contains("uciok\n"));
}

#[test]
fn run_command_isready() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(search_algo, Box::new(best_move_callback));
    let test_writer = TestBuffer::new();
    {
        let mut p = Parser::new(Box::new(test_writer.clone()));
        p.register_command(String::from("isready"), Box::new(is_ready::run_command));
        assert!(p.run_command("isready invalid\n", &mut engine).is_err());
        assert!(p.run_command("isready\n", &mut engine).is_ok());
    }
    assert_eq!("readyok\n", test_writer.into_string());
}

#[test]
fn run_command_position() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(search_algo, Box::new(best_move_callback));
    let test_writer = Vec::new();
    let mut p = Parser::new(Box::new(test_writer));

    p.register_command(String::from("position"), Box::new(position::run_command));

    assert_eq!(None, engine.position());

    let invalid_commands = [
        "position\n",
        "position invalid\n",
        "position fen\n",
        "position startpos invalid\n",
        "position startpos moves e2e5\n",
        "position fen invalid_fen\n",
        &format!("position fen {} not_moves\n", FEN_STR),
        &format!("position fen {} moves invalid_move\n", FEN_STR),
    ];
    for inv_cmd in invalid_commands {
        assert!(p.run_command(inv_cmd, &mut engine).is_err());
    }
    assert_eq!(None, engine.position());

    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert_eq!(Some(&Position::initial()), engine.position());

    assert!(p
        .run_command(format!("position fen {}\n", FEN_STR).as_str(), &mut engine)
        .is_ok());
    assert_eq!(Fen::str_to_pos(FEN_STR).ok().as_ref(), engine.position());

    assert!(p
        .run_command("position startpos moves e2e4 c7c5 g1f3\n", &mut engine)
        .is_ok());
    let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
    assert_eq!(Fen::str_to_pos(fen).ok().as_ref(), engine.position());
}

#[test]
fn run_command_ucinewgame() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(search_algo, Box::new(best_move_callback));
    let test_writer = Vec::new();
    let mut p = Parser::new(Box::new(test_writer));

    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(
        String::from("ucinewgame"),
        Box::new(ucinewgame::run_command),
    );

    assert_eq!(None, engine.position());

    assert!(p.run_command("ucinewgame invalid\n", &mut engine).is_err());

    assert!(p.run_command("ucinewgame\n", &mut engine).is_ok());
    assert_eq!(None, engine.position());

    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert_eq!(Some(&Position::initial()), engine.position());

    assert!(p.run_command("ucinewgame\n", &mut engine).is_ok());
    assert_eq!(None, engine.position());
}

#[test]
fn run_command_go() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut test_writer = TestBuffer::new();

    let mut test_writer_engine = test_writer.clone();
    let test_writer_parser = test_writer.clone();
    let best_move_callback =
        Box::new(move |m| best_move::write(&mut test_writer_engine, m).unwrap());

    let mut engine = Engine::new(search_algo, best_move_callback);
    let mut p = Parser::new(Box::new(test_writer_parser));

    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(String::from("go"), Box::new(go::run_command));
    p.register_command(String::from("stop"), Box::new(stop::run_command));

    // Run "go" without setting a position
    assert!(p.run_command("go invalid\n", &mut engine).is_err());
    assert!(p.run_command("go\n", &mut engine).is_err());

    // Run "go" with invalid options
    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p.run_command("go invalid\n", &mut engine).is_err());
    assert!(p.run_command("go depth\n", &mut engine).is_err());
    assert!(p.run_command("go depth invalid\n", &mut engine).is_err());
    assert!(p.run_command("go movetime\n", &mut engine).is_err());
    assert!(p.run_command("go movetime invalid\n", &mut engine).is_err());
    // Options must not appear more than once
    assert!(p.run_command("go depth 3 depth 3\n", &mut engine).is_err());

    // Run "go" followed by "stop" after setting a position
    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p.run_command("go\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(200));
    assert!(!contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(10));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(10));
    assert!(!contains(test_writer.split_off(0), "bestmove"));

    // Option "depth"
    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p.run_command("go depth 3\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(200));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(10));
    assert!(!contains(test_writer.split_off(0), "bestmove"));

    // Option "movetime"
    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p.run_command("go movetime 100\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(200));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(10));
    assert!(!contains(test_writer.split_off(0), "bestmove"));

    // Option "infinite"
    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p.run_command("go infinite\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(200));
    assert!(!contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(10));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(10));
    assert!(!contains(test_writer.split_off(0), "bestmove"));

    // Combine multiple options
    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p
        .run_command("go depth 3 movetime 100\n", &mut engine)
        .is_ok());
    std::thread::sleep(Duration::from_millis(200));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(10));
    assert!(!contains(test_writer.split_off(0), "bestmove"));

    assert!(p.run_command("go depth 3 infinite\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(200));
    assert!(!contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(10));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(10));
    assert!(!contains(test_writer.split_off(0), "bestmove"));

    assert!(p
        .run_command("go movetime 100 infinite\n", &mut engine)
        .is_ok());
    std::thread::sleep(Duration::from_millis(200));
    assert!(!contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(10));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(10));
    assert!(!contains(test_writer.split_off(0), "bestmove"));
}

#[test]
fn run_command_quit() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut test_writer = TestBuffer::new();

    let mut test_writer_engine = test_writer.clone();
    let test_writer_parser = test_writer.clone();
    let best_move_callback =
        Box::new(move |m| best_move::write(&mut test_writer_engine, m).unwrap());

    let mut engine = Engine::new(search_algo, best_move_callback);
    let mut p = Parser::new(Box::new(test_writer_parser));

    p.register_command(String::from("quit"), Box::new(quit::run_command));

    assert!(p.run_command("quit invalid\n", &mut engine).is_err());
    let res = p.run_command("quit\n", &mut engine);
    assert!(res.is_ok());
    assert_eq!(Some(ParserMessage::Quit), res.unwrap());
    assert!(String::from_utf8(test_writer.split_off(0))
        .unwrap()
        .is_empty());
    let res = p.run_command("ignored quit\n", &mut engine);
    assert!(res.is_ok());
    assert_eq!(Some(ParserMessage::Quit), res.unwrap());
    assert!(String::from_utf8(test_writer.split_off(0))
        .unwrap()
        .is_empty());
}
