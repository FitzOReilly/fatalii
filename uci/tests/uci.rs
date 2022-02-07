mod test_buffer;

use crate::test_buffer::TestBuffer;
use engine::Engine;
use movegen::fen::Fen;
use movegen::position::Position;
use regex::Regex;
use search::alpha_beta::AlphaBeta;
use search::search::SearchResult;
use std::io::{stdout, Write};
use std::str;
use std::time::Duration;
use uci::parser::{Parser, ParserMessage};
use uci::uci_in::{go, is_ready, position, quit, set_option, stop, uci as cmd_uci, ucinewgame};
use uci::uci_out::{best_move, info};

const TABLE_IDX_BITS: usize = 16;
const FEN_STR: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

fn search_info_callback(res: Option<SearchResult>) {
    info::write(&mut stdout(), res).unwrap();
}

fn best_move_callback(res: Option<SearchResult>) {
    best_move::write(&mut stdout(), res).unwrap();
}

fn contains(v: Vec<u8>, s: &str) -> bool {
    String::from_utf8(v).unwrap().contains(s)
}

#[test]
fn register_command() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(
        search_algo,
        Box::new(search_info_callback),
        Box::new(best_move_callback),
    );
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
    let mut engine = Engine::new(
        search_algo,
        Box::new(search_info_callback),
        Box::new(best_move_callback),
    );
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
    assert!(out.contains("option name Hash type spin default"));
    assert!(out.contains("min"));
    assert!(out.contains("max"));
    assert!(out.contains("uciok\n"));
}

#[test]
fn run_command_isready() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(
        search_algo,
        Box::new(search_info_callback),
        Box::new(best_move_callback),
    );
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
fn run_command_setoption() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(
        search_algo,
        Box::new(search_info_callback),
        Box::new(best_move_callback),
    );
    let test_writer = TestBuffer::new();
    let mut p = Parser::new(Box::new(test_writer));
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
        print!("{}", inv_cmd);
        assert!(p.run_command(inv_cmd, &mut engine).is_err());
    }

    let valid_commands = [
        "setoption name Hash value 16\n",
        "setoption name hash value 16\n",
    ];
    for val_cmd in valid_commands {
        print!("{}", val_cmd);
        assert!(p.run_command(val_cmd, &mut engine).is_ok());
    }
}

#[test]
fn run_command_position() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(
        search_algo,
        Box::new(search_info_callback),
        Box::new(best_move_callback),
    );
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
    let mut engine = Engine::new(
        search_algo,
        Box::new(search_info_callback),
        Box::new(best_move_callback),
    );
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

    let mut test_writer_info = test_writer.clone();
    let mut test_writer_best_move = test_writer.clone();
    let test_writer_parser = test_writer.clone();
    let search_info_callback =
        Box::new(move |res| info::write(&mut test_writer_info, res).unwrap());
    let best_move_callback =
        Box::new(move |res| best_move::write(&mut test_writer_best_move, res).unwrap());

    let mut engine = Engine::new(search_algo, search_info_callback, best_move_callback);
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
    assert!(p.run_command("go depth 2 depth 2\n", &mut engine).is_err());

    // Run "go" followed by "stop" after setting a position
    assert!(p.run_command("position startpos\n", &mut engine).is_ok());
    assert!(p.run_command("go\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(400));
    let output = String::from_utf8(test_writer.split_off(0)).unwrap();
    assert!(output.contains("info"));
    assert!(output.contains("depth"));
    assert!(output.contains("score"));
    assert!(!output.contains("bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    let output = String::from_utf8(test_writer.split_off(0)).unwrap();
    assert!(output.contains("info"));
    assert!(output.contains("depth"));
    assert!(output.contains("score"));
    assert!(output.contains("bestmove"));
    assert!(p.run_command("stop\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(20));
    let output = String::from_utf8(test_writer.split_off(0)).unwrap();
    assert!(!output.contains("info"));
    assert!(!output.contains("depth"));
    assert!(!output.contains("score"));
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
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut test_writer = TestBuffer::new();

    let mut test_writer_info = test_writer.clone();
    let mut test_writer_best_move = test_writer.clone();
    let test_writer_parser = test_writer.clone();
    let search_info_callback =
        Box::new(move |res| info::write(&mut test_writer_info, res).unwrap());
    let best_move_callback =
        Box::new(move |res| best_move::write(&mut test_writer_best_move, res).unwrap());

    let mut engine = Engine::new(search_algo, search_info_callback, best_move_callback);
    let mut p = Parser::new(Box::new(test_writer_parser));

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
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut test_writer = TestBuffer::new();

    let mut test_writer_info = test_writer.clone();
    let mut test_writer_best_move = test_writer.clone();
    let test_writer_parser = test_writer.clone();
    let search_info_callback =
        Box::new(move |res| info::write(&mut test_writer_info, res).unwrap());
    let best_move_callback =
        Box::new(move |res| best_move::write(&mut test_writer_best_move, res).unwrap());

    let mut engine = Engine::new(search_algo, search_info_callback, best_move_callback);
    let mut p = Parser::new(Box::new(test_writer_parser));

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
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut test_writer = TestBuffer::new();

    let mut test_writer_info = test_writer.clone();
    let mut test_writer_best_move = test_writer.clone();
    let test_writer_parser = test_writer.clone();
    let search_info_callback =
        Box::new(move |res| info::write(&mut test_writer_info, res).unwrap());
    let best_move_callback =
        Box::new(move |res| best_move::write(&mut test_writer_best_move, res).unwrap());

    let mut engine = Engine::new(search_algo, search_info_callback, best_move_callback);
    let mut p = Parser::new(Box::new(test_writer_parser));

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
fn run_command_isready_during_go() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut test_writer = TestBuffer::new();

    let mut test_writer_info = test_writer.clone();
    let mut test_writer_best_move = test_writer.clone();
    let test_writer_parser = test_writer.clone();
    let search_info_callback =
        Box::new(move |res| info::write(&mut test_writer_info, res).unwrap());
    let best_move_callback =
        Box::new(move |res| best_move::write(&mut test_writer_best_move, res).unwrap());

    let mut engine = Engine::new(search_algo, search_info_callback, best_move_callback);
    let mut p = Parser::new(Box::new(test_writer_parser));

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
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut test_writer = TestBuffer::new();

    let mut test_writer_info = test_writer.clone();
    let mut test_writer_best_move = test_writer.clone();
    let test_writer_parser = test_writer.clone();
    let search_info_callback =
        Box::new(move |res| info::write(&mut test_writer_info, res).unwrap());
    let best_move_callback =
        Box::new(move |res| best_move::write(&mut test_writer_best_move, res).unwrap());

    let mut engine = Engine::new(search_algo, search_info_callback, best_move_callback);
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

#[test]
fn info_score_equal_from_both_sides() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut test_writer = TestBuffer::new();

    let mut test_writer_info = test_writer.clone();
    let mut test_writer_best_move = test_writer.clone();
    let test_writer_parser = test_writer.clone();
    let search_info_callback =
        Box::new(move |res| info::write(&mut test_writer_info, res).unwrap());
    let best_move_callback =
        Box::new(move |res| best_move::write(&mut test_writer_best_move, res).unwrap());

    let mut engine = Engine::new(search_algo, search_info_callback, best_move_callback);
    let mut p = Parser::new(Box::new(test_writer_parser));

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
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut test_writer = TestBuffer::new();

    let mut test_writer_info = test_writer.clone();
    let mut test_writer_best_move = test_writer.clone();
    let test_writer_parser = test_writer.clone();
    let search_info_callback =
        Box::new(move |res| info::write(&mut test_writer_info, res).unwrap());
    let best_move_callback =
        Box::new(move |res| best_move::write(&mut test_writer_best_move, res).unwrap());

    let mut engine = Engine::new(search_algo, search_info_callback, best_move_callback);
    let mut p = Parser::new(Box::new(test_writer_parser));

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
    assert!(p.run_command("go movetime 800\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(810));

    let out_str = String::from_utf8(test_writer.split_off(0)).unwrap();
    println!("{}", out_str);
    assert!(out_str.contains("bestmove g2g7"));
}

#[test]
fn mate_in_one_black_to_move() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut test_writer = TestBuffer::new();

    let mut test_writer_info = test_writer.clone();
    let mut test_writer_best_move = test_writer.clone();
    let test_writer_parser = test_writer.clone();
    let search_info_callback =
        Box::new(move |res| info::write(&mut test_writer_info, res).unwrap());
    let best_move_callback =
        Box::new(move |res| best_move::write(&mut test_writer_best_move, res).unwrap());

    let mut engine = Engine::new(search_algo, search_info_callback, best_move_callback);
    let mut p = Parser::new(Box::new(test_writer_parser));

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
    assert!(p.run_command("go movetime 800\n", &mut engine).is_ok());
    std::thread::sleep(Duration::from_millis(810));

    let out_str = String::from_utf8(test_writer.split_off(0)).unwrap();
    println!("{}", out_str);
    assert!(out_str.contains("bestmove g7g2"));
}
