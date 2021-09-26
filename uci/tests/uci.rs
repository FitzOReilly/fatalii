use engine::Engine;
use movegen::fen::Fen;
use movegen::position::Position;
use search::alpha_beta::AlphaBeta;
use std::io::Write;
use std::str;
use uci::parser::Parser;
use uci::uci_in::{is_ready, position, uci as cmd_uci};

const TABLE_IDX_BITS: usize = 16;
const FEN_STR: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

#[test]
fn register_command() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(search_algo);
    let mut test_writer = Vec::new();
    let mut p = Parser::new(&mut test_writer);

    assert!(p.run_command("unknown\n", &mut engine).is_err());
    assert!(p.run_command("cmd\n", &mut engine).is_err());
    assert!(p.run_command("cmd args\n", &mut engine).is_err());
    assert!(p.run_command(" \r \t  cmd  args \n", &mut engine).is_err());

    let cmd_handler = |writer: &mut dyn Write, args: &str, _engine: &mut Engine| {
        Ok(writeln!(writer, "{}", args.trim_end_matches('\n'))?)
    };
    p.register_command(String::from("cmd"), Box::new(cmd_handler));

    assert!(p.run_command("unknown\n", &mut engine).is_err());
    assert!(p.run_command("cmd\n", &mut engine).is_ok());
    assert!(p.run_command("cmd args\n", &mut engine).is_ok());
    assert!(p.run_command(" \r \t  cmd  args \n", &mut engine).is_ok());

    let mut outputs = str::from_utf8(&test_writer).unwrap().split('\n');

    assert_eq!("", outputs.next().unwrap());
    assert_eq!("args", outputs.next().unwrap());
    assert_eq!(" args ", outputs.next().unwrap());
}

#[test]
fn run_command_uci() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(search_algo);
    let mut test_writer = Vec::new();
    let mut p = Parser::new(&mut test_writer);

    p.register_command(String::from("uci"), Box::new(cmd_uci::run_command));

    assert!(p.run_command("uci invalid\n", &mut engine).is_err());
    assert!(p.run_command("uci\n", &mut engine).is_ok());

    let out = str::from_utf8(&test_writer).unwrap();
    assert!(out.contains("id name"));
    assert!(out.contains("id author"));
    assert!(out.contains("uciok\n"));
}

#[test]
fn run_command_isready() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(search_algo);
    let mut test_writer = Vec::new();
    let mut p = Parser::new(&mut test_writer);

    p.register_command(String::from("isready"), Box::new(is_ready::run_command));

    assert!(p.run_command("isready invalid\n", &mut engine).is_err());
    assert!(p.run_command("isready\n", &mut engine).is_ok());

    let out = str::from_utf8(&test_writer).unwrap();
    assert_eq!("readyok\n", out);
}

#[test]
fn run_command_position() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(search_algo);
    let mut test_writer = Vec::new();
    let mut p = Parser::new(&mut test_writer);

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
