mod test_buffer;

use crate::test_buffer::TestBuffer;
use assert_matches::assert_matches;
use engine::{Engine, EngineOptions, Variant};
use eval::material_mobility::MaterialMobility;
use movegen::fen::Fen;
use movegen::position::Position;
use regex::Regex;
use search::alpha_beta::AlphaBeta;
use search::search::Search;
use std::str;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use uci::uci_in::{
    debug, go, is_ready, position, quit, set_option, stop, uci as cmd_uci, ucinewgame,
};
use uci::UciOut;
use uci::{Parser, ParserMessage};

const EVALUATOR: MaterialMobility = MaterialMobility::new();
const TABLE_SIZE: usize = 16 * 1024;
const FEN_STR: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
const FEN_STR_CHESS_960: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w HA - 1 8";

fn contains(v: Vec<u8>, s: &str) -> bool {
    String::from_utf8(v).unwrap().contains(s)
}

#[test]
fn run_command_uci() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    {
        let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
        let mut p = Parser::new(uci_out);
        p.register_command(String::from("uci"), Box::new(cmd_uci::run_command));
        assert!(p.run_command("uci invalid\n", &mut engine).is_err());
        assert!(p.run_command("uci\n", &mut engine).is_ok());
    }
    let out = test_writer.into_string();
    assert!(out.contains("id name"));
    assert!(out.contains("id author"));
    assert!(out.contains("option name Hash type spin default"));
    assert!(out.contains("min"));
    assert!(out.contains("max"));
    assert!(out.contains("uciok\n"));
}

#[test]
fn run_command_isready() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    {
        let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
        let mut p = Parser::new(uci_out);

        assert!(p.run_command("unknown\n", &mut engine).is_err());
        assert!(p.run_command("isready\n", &mut engine).is_err());

        p.register_command(String::from("isready"), Box::new(is_ready::run_command));
        assert!(p.run_command("unknown\n", &mut engine).is_err());
        assert!(p.run_command("isready invalid\n", &mut engine).is_err());
        assert!(p.run_command("isready\n", &mut engine).is_ok());
    }
    assert_eq!("readyok\n", test_writer.into_string());
}

#[test]
fn run_command_debug() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    {
        let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
        let mut p = Parser::new(uci_out);

        p.register_command(String::from("debug"), Box::new(debug::run_command));
        assert!(p.run_command("debug\n", &mut engine).is_err());
        assert!(p.run_command("debug invalid\n", &mut engine).is_err());
        assert!(p.run_command("debug on\n", &mut engine).is_ok());
        assert!(p.run_command("debug off\n", &mut engine).is_ok());
    }
    let out = test_writer.into_string();
    assert!(out.contains("info string"));
}

#[test]
fn run_command_setoption() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("setoption"), Box::new(set_option::run_command));

    let invalid_commands = [
        "setoption\n",
        "setoption invalid\n",
        "setoption name\n",
        "setoption name invalid\n",
        "setoption name Hash\n",
        "setoption name Hash value\n",
        "setoption name Hash value invalid\n",
        "setoption name Hash value 16 invalid\n",
    ];
    for inv_cmd in invalid_commands {
        print!("{inv_cmd}");
        assert!(p.run_command(inv_cmd, &mut engine).is_err());
    }

    let valid_commands = [
        "setoption name Hash value 16\n",
        "setoption name hash value 16\n",
        " setoption name Hash value 16\n",
        "setoption  name Hash value 16\n",
        "setoption name  Hash value 16\n",
        "setoption name Hash  value 16\n",
        "setoption name Hash value  16\n",
        "setoption name Hash value 16 \n",
    ];
    for val_cmd in valid_commands {
        print!("{val_cmd}");
        assert!(p.run_command(val_cmd, &mut engine).is_ok());
    }
}

#[test]
fn run_command_position() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("position"), Box::new(position::run_command));

    assert_eq!(None, engine.position());

    let invalid_commands = [
        "position\n",
        "position invalid\n",
        "position fen\n",
        "position startpos invalid\n",
        "position startpos moves e2e5\n",
        "position fen invalid_fen\n",
        &format!("position fen {FEN_STR} not_moves\n"),
        &format!("position fen {FEN_STR} moves invalid_move\n"),
    ];
    for inv_cmd in invalid_commands {
        assert!(p.run_command(inv_cmd, &mut engine).is_err());
    }
    assert_eq!(None, engine.position());

    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert_eq!(Some(&Position::initial()), engine.position());

    assert!(p
        .run_command(format!("position fen {FEN_STR}\n").as_str(), &mut engine)
        .is_ok());
    assert_eq!(Fen::str_to_pos(FEN_STR).ok().as_ref(), engine.position());

    assert!(p
        .run_command("position startpos moves e2e4 c7c5 g1f3\n", &mut engine)
        .is_ok());
    let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
    assert_eq!(Fen::str_to_pos(fen).ok().as_ref(), engine.position());
}

#[test]
fn run_command_position_chess_960() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(String::from("setoption"), Box::new(set_option::run_command));

    assert!(p
        .run_command("setoption name UCI_Chess960 value true\n", &mut engine)
        .is_ok());

    assert_matches!(engine.variant(), Variant::Chess960(_, _));
    assert_eq!(None, engine.position());

    let invalid_commands = [
        &format!("position fen {FEN_STR}\n"),
        &format!("position fen {FEN_STR} moves e1g1\n"),
    ];
    for inv_cmd in invalid_commands {
        assert!(p.run_command(inv_cmd, &mut engine).is_err());
    }
    assert_eq!(None, engine.position());

    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert_eq!(Some(&Position::initial()), engine.position());

    assert!(p
        .run_command(
            format!("position fen {FEN_STR_CHESS_960}\n").as_str(),
            &mut engine
        )
        .is_ok());
    assert_eq!(Fen::str_to_pos(FEN_STR).ok().as_ref(), engine.position());

    assert!(p
        .run_command(
            &format!("position fen {FEN_STR_CHESS_960} moves e1h1\n"),
            &mut engine
        )
        .is_ok());
    let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQ1RK1 b - - 2 8";
    assert_eq!(
        Fen::str_to_pos_chess_960(fen).ok().as_ref(),
        engine.position()
    );
}

#[test]
fn run_command_ucinewgame() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

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
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let mut test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

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
    assert!(p.run_command("go depth 2 depth 2\n", &mut engine).is_err());

    // Run "go" followed by "stop" after setting a position
    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p.run_command("go\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));
    let output = String::from_utf8(test_writer.split_off(0)).unwrap();
    assert!(output.contains("info"));
    assert!(output.contains("depth"));
    assert!(output.contains("score"));
    assert!(output.contains("nodes"));
    assert!(output.contains("nps"));
    assert!(output.contains("time"));
    assert!(output.contains("hashfull"));
    assert!(output.contains("pv"));
    assert!(!output.contains("bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    let output = String::from_utf8(test_writer.split_off(0)).unwrap();
    assert!(output.contains("bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    let output = String::from_utf8(test_writer.split_off(0)).unwrap();
    assert!(!output.contains("info"));
    assert!(!output.contains("bestmove"));

    // Option "depth"
    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p.run_command("go depth 2\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));
    let output = String::from_utf8(test_writer.split_off(0)).unwrap();
    assert!(output.contains("depth 2"));
    assert!(output.contains("bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(!contains(test_writer.split_off(0), "bestmove"));

    // Option "nodes" should not stop the search before depth 1 is finished
    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p.run_command("go nodes 1\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));
    let output = String::from_utf8(test_writer.split_off(0)).unwrap();
    assert!(output.contains("info"));
    assert!(output.contains("bestmove"));
    assert!(!output.contains("0000"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(!contains(test_writer.split_off(0), "bestmove"));

    // Option "movetime"
    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p.run_command("go movetime 100\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(!contains(test_writer.split_off(0), "bestmove"));

    // Option "infinite"
    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p.run_command("go infinite\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));
    assert!(!contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(!contains(test_writer.split_off(0), "bestmove"));

    // Combine multiple options
    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p
        .run_command("go depth 2 movetime 100\n", &mut engine)
        .is_ok());
    std::thread::sleep(Duration::from_millis(400));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(!contains(test_writer.split_off(0), "bestmove"));

    assert!(p.run_command("go depth 2 infinite\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));
    assert!(!contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(!contains(test_writer.split_off(0), "bestmove"));

    assert!(p
        .run_command("go movetime 100 infinite\n", &mut engine)
        .is_ok());
    std::thread::sleep(Duration::from_millis(400));
    assert!(!contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(!contains(test_writer.split_off(0), "bestmove"));
}

#[test]
fn run_command_go_time() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let mut test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(String::from("go"), Box::new(go::run_command));

    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(p.run_command("go wtime\n", &mut engine).is_err());
    assert!(p.run_command("go btime\n", &mut engine).is_err());
    assert!(p.run_command("go winc\n", &mut engine).is_err());
    assert!(p.run_command("go binc\n", &mut engine).is_err());
    assert!(p
        .run_command("go wtime 500 btime 800 winc 100 binc 100\n", &mut engine)
        .is_ok());
    std::thread::sleep(Duration::from_millis(500));
    assert!(contains(test_writer.split_off(0), "bestmove"));

    assert!(p
        .run_command("position startpos moves e2e4\n", &mut engine)
        .is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(p
        .run_command("go wtime 500 btime 800 winc 100 binc 100\n", &mut engine)
        .is_ok());
    std::thread::sleep(Duration::from_millis(800));
    assert!(contains(test_writer.split_off(0), "bestmove"));
}

#[test]
fn run_command_go_time_limit_exceeded() {
    // Even if the time limit is exceeded, the engine should still send
    // a bestmove response to a go command
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let mut test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("setoption"), Box::new(set_option::run_command));
    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(String::from("go"), Box::new(go::run_command));

    assert!(p
        .run_command("setoption name Move Overhead value 100\n", &mut engine)
        .is_ok());

    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(p.run_command("go wtime 0 btime 0\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(50));
    assert!(contains(test_writer.split_off(0), "bestmove"));

    assert!(p
        .run_command("position startpos moves e2e4\n", &mut engine)
        .is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(p.run_command("go wtime 0 btime 0\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(50));
    assert!(contains(test_writer.split_off(0), "bestmove"));
}

#[test]
fn run_command_go_twice() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let mut test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(String::from("go"), Box::new(go::run_command));
    p.register_command(String::from("stop"), Box::new(stop::run_command));

    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p.run_command("go infinite\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));
    assert!(!contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("go infinite\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));
    assert!(!contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(contains(test_writer.split_off(0), "bestmove"));
}

#[test]
fn run_command_go_with_negative_value() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let mut test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(String::from("go"), Box::new(go::run_command));
    p.register_command(String::from("stop"), Box::new(stop::run_command));

    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p
        .run_command("go wtime 100 btime -1\n", &mut engine)
        .is_ok());
    std::thread::sleep(Duration::from_millis(400));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("go wtime -1\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));
    assert!(contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("go depth -1\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));
    assert!(contains(test_writer.split_off(0), "bestmove"));
}

#[test]
fn run_command_isready_during_go() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let mut test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("isready"), Box::new(is_ready::run_command));
    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(String::from("go"), Box::new(go::run_command));
    p.register_command(String::from("stop"), Box::new(stop::run_command));

    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p.run_command("go infinite\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));
    assert!(!contains(test_writer.split_off(0), "bestmove"));
    assert!(p.run_command("isready\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    let out_str = String::from_utf8(test_writer.split_off(0)).unwrap();
    assert!(out_str.contains("readyok\n"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    let out_str = String::from_utf8(test_writer.split_off(0)).unwrap();
    assert!(out_str.contains("bestmove"));
}

#[test]
fn run_command_quit() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let mut test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

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

#[test]
fn info_score_equal_from_both_sides() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let mut test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(String::from("go"), Box::new(go::run_command));

    let re_info_score = Regex::new(r"score cp \d+").unwrap();

    // Set up King's Gambit
    assert!(p
        .run_command(
            "position fen rnbqkbnr/pppp1ppp/8/4p3/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 2\n",
            &mut engine
        )
        .is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(p.run_command("go depth 1\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));

    let out_str = String::from_utf8(test_writer.split_off(0)).unwrap();
    let out_lines = out_str.split('\n');
    let mut rev_out_lines = out_lines.rev();
    let _empty = rev_out_lines.next();
    let best_move_first = rev_out_lines.next().unwrap();
    let info_first = rev_out_lines.next().unwrap();
    let score_first = re_info_score
        .captures(info_first)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str();

    // Set up mirrored position
    assert!(p
        .run_command(
            "position fen rnbqkbnr/pppp2pp/8/4pp2/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2\n",
            &mut engine
        )
        .is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(p.run_command("go depth 1\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));

    let out_str = String::from_utf8(test_writer.split_off(0)).unwrap();
    let out_lines = out_str.split('\n');
    let mut rev_out_lines = out_lines.rev();
    let _empty = rev_out_lines.next();
    let best_move_second = rev_out_lines.next().unwrap();
    let info_second = rev_out_lines.next().unwrap();
    let score_second = re_info_score
        .captures(info_second)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str();

    assert_ne!(best_move_first, best_move_second);
    assert_eq!(score_first, score_second);
}

#[test]
fn mate_in_one_white_to_move() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let mut test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(String::from("go"), Box::new(go::run_command));

    // Set up mate in one
    assert!(p
        .run_command(
            "position fen 8/7k/7P/8/8/8/6Q1/6K1 w - - 0 1\n",
            &mut engine
        )
        .is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(p.run_command("go movetime 100\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(200));

    let out_str = String::from_utf8(test_writer.split_off(0)).unwrap();
    println!("{out_str}");
    assert!(out_str.contains("bestmove g2g7"));
}

#[test]
fn mate_in_one_black_to_move() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let mut test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(String::from("go"), Box::new(go::run_command));

    // Set up mate in one
    assert!(p
        .run_command(
            "position fen 6k1/6q1/8/8/8/7p/7K/8 b - - 0 1\n",
            &mut engine
        )
        .is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(p.run_command("go movetime 100\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(200));

    let out_str = String::from_utf8(test_writer.split_off(0)).unwrap();
    println!("{out_str}");
    assert!(out_str.contains("bestmove g7g2"));
}

#[test]
fn alpha_beta_threefold_repetition() {
    let alpha_beta = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    threefold_repetition(alpha_beta);
}

fn threefold_repetition(search_algo: impl Search + Send + 'static) {
    let test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(String::from("go"), Box::new(go::run_command));

    // Set up threefold repetition on next move
    assert!(p
        .run_command(
            "position startpos moves e2e3 b8c6 f1b5 g8f6 d1f3 c6b4 b1a3 e7e5 c2c3 e5e4 f3f5 b4c6 \
            g1e2 d7d5 f5e5 c8e6 e2d4 f8d6 e5g5 f6d7 g5g7 d6e5 g7h6 c6d4 e3d4 d8f6 d2d3 f6h6 c1h6 \
            c7c6 d4e5 c6b5 a3b5 e8e7 h6g5 f7f6 e5f6 d7f6 e1d2 a8g8 h2h4 e4d3 a1e1 h7h6 b5d4 h6g5 \
            e1e6 e7f7 h4h5 f6e4 d2e3 d3d2 h5h6 g8e8 e6e8 h8e8 h1g1 d2d1q g1d1 e4c3 e3d2 c3d1 d2d1 \
            f7g6 d4b5 g6h6 d1d2 e8f8 d2e2 f8f4 a2a3 a7a6 b5c7 f4d4 e2e3 d4d1 e3e2 d1b1 b2b4 b1b2 \
            e2e1 b2b1 e1e2 b1b2 e2e1 d5d4 c7e6 b2b1 e1d2 b1b2 d2e1 b2b1 e1d2 b1f1 d2e2 f1g1 e6d4 \
            g1g2 d4e6 g2g1 e6c5 b7b6 c5a6 g1a1 a6c7 a1a2 e2f3 a2a3 f3g4 a3c3 c7d5 c3b3 f2f4 g5f4 \
            g4f4 h6g6 f4e5 b3b1 e5d4 b1d1 d4c4 b6b5 c4c5 g6g5 d5b6 d1e1 c5b5 e1e5 b5c6 e5e6 c6c5 \
            e6e5 c5d4 g5f4 b6d5 f4f5 d5e3 f5e6 e3g4 e5b5 d4c3 b5f5 g4e3 f5f4 e3c4 e6d5 c4e3 d5e4 \
            e3d1 e4d5 d1e3 d5e4 e3d1 f4f1 c3d2 f1f3 d1c3 e4d4 c3b5 d4c4 b5d6 c4b4 d2e2 f3a3 d6f5 \
            b4c4 f5e3 c4c5 e2d2 a3b3 d2e2 c5d4 e3f5 d4e5 f5e3 e5d4 e3f5 d4e5 f5e3 e5e4 e3d1 e4d4 \
            d1f2 b3e3 e2d2 e3a3 d2e2 a3e3 e2d2 e3b3 d2e2 b3g3 e2d2 g3e3 f2g4 e3g3 g4f2 g3e3 f2g4 \
            e3e6 g4f2 e6f6 d2e2 f6a6 e2f3 a6a3 f3f4 d4d5 f2g4 a3a4 f4f5 d5c4 f5e5 c4b3 g4e3 b3a2 \
            e3f5 a2a1 e5e6 a1b1 e6e7 b1a1 e7f7 a1b1 f5e3 b1a1 f7g7 a1b1 g7f7 b1a1 f7g7 a1b1\n",
            &mut engine
        )
        .is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(p
        .run_command(
            "go wtime 8955 btime 109427 winc 1000 binc 1000 movestogo 4\n",
            &mut engine
        )
        .is_ok());
    std::thread::sleep(Duration::from_millis(400));
}

#[test]
fn search_stopped_after_depth_1_if_move_is_forced() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let mut test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(String::from("go"), Box::new(go::run_command));
    p.register_command(String::from("stop"), Box::new(stop::run_command));
    p.register_command(
        String::from("ucinewgame"),
        Box::new(ucinewgame::run_command),
    );

    let fen_white_to_move = "7R/8/8/8/8/8/2k5/K5r1 w - - 0 1";
    let fen_black_to_move = "r1bQkb1r/ppp2ppp/2p2n2/8/4P3/8/PPP2PPP/RNB1KB1R b KQkq - 0 6";
    for (fen, go_option, contains_depth_2) in [
        (fen_white_to_move, "depth 4", true),
        (fen_white_to_move, "nodes 5000", true),
        (fen_white_to_move, "movetime 100", true),
        (fen_white_to_move, "infinite", true),
        (fen_white_to_move, "wtime 10000", false),
        (fen_white_to_move, "btime 10000", true),
        (fen_black_to_move, "depth 4", true),
        (fen_black_to_move, "nodes 5000", true),
        (fen_black_to_move, "movetime 100", true),
        (fen_black_to_move, "infinite", true),
        (fen_black_to_move, "wtime 10000", true),
        (fen_black_to_move, "btime 10000", false),
    ] {
        assert!(p.run_command("ucinewgame\n", &mut engine).is_ok());
        assert!(p
            .run_command(&format!("position fen {fen}\n"), &mut engine)
            .is_ok());
        std::thread::sleep(Duration::from_millis(20));
        assert!(p
            .run_command(&format!("go {go_option}\n"), &mut engine)
            .is_ok());
        std::thread::sleep(Duration::from_millis(200));

        assert!(p.run_command("stop\n", &mut engine).is_ok());
        std::thread::sleep(Duration::from_millis(20));

        let out_str = String::from_utf8(test_writer.split_off(0)).unwrap();
        println!("{out_str}");
        assert_eq!(contains_depth_2, out_str.contains("depth 2"));
    }
}

#[test]
#[ignore]
fn stress() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    {
        let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
        let mut p = Parser::new(uci_out);

        p.register_command(String::from("debug"), Box::new(debug::run_command));
        p.register_command(String::from("isready"), Box::new(is_ready::run_command));
        p.register_command(String::from("go"), Box::new(go::run_command));
        p.register_command(String::from("position"), Box::new(position::run_command));
        p.register_command(String::from("setoption"), Box::new(set_option::run_command));
        p.register_command(String::from("uci"), Box::new(cmd_uci::run_command));
        p.register_command(
            String::from("ucinewgame"),
            Box::new(ucinewgame::run_command),
        );

        assert!(p.run_command("uci\n", &mut engine).is_ok());
        assert!(p.run_command("debug on\n", &mut engine).is_ok());

        for hash_size in [1, 8, 64] {
            assert!(p
                .run_command(
                    format!("setoption name Hash value {hash_size}\n").as_str(),
                    &mut engine
                )
                .is_ok());
            assert!(p.run_command("isready\n", &mut engine).is_ok());

            for i in 0..10_000 {
                println!("Hash size: {hash_size}, iteration: {i}");
                assert!(p.run_command("ucinewgame\n", &mut engine).is_ok());
                assert!(p.run_command("isready\n", &mut engine).is_ok());
                assert!(p.run_command("position startpos\n", &mut engine).is_ok());
                assert!(p.run_command("isready\n", &mut engine).is_ok());
                assert!(p.run_command("go wtime 0\n", &mut engine).is_ok());
                assert!(p.run_command("isready\n", &mut engine).is_ok());
            }
        }
    }
}

#[test]
fn play_move_after_unclaimed_threefold_repetition() {
    let search_algo = AlphaBeta::new(Box::new(EVALUATOR), TABLE_SIZE);
    let mut test_writer = TestBuffer::new();
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(test_writer.clone()),
        "0.1.2",
        Arc::clone(&engine_options),
    );
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);
    let mut p = Parser::new(uci_out);

    p.register_command(String::from("position"), Box::new(position::run_command));
    p.register_command(String::from("go"), Box::new(go::run_command));

    // The given position and move history result in a threefold repetition.
    // The engine should do a normal search because the draw was not claimed.
    // It should not return a null move.
    assert!(p
        .run_command(
            "position startpos moves g1f3 g8f6 f3g1 f6g8 g1f3 g8f6 f3g1 f6g8\n",
            &mut engine
        )
        .is_ok());
    std::thread::sleep(Duration::from_millis(20));
    assert!(p.run_command("go depth 1\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(200));

    let out_str = String::from_utf8(test_writer.split_off(0)).unwrap();
    println!("{out_str}");
    assert!(!out_str.contains("bestmove 0000"));
}
