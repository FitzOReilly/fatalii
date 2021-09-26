use engine::Engine;
use search::alpha_beta::AlphaBeta;
use std::io::Write;
use std::str;
use uci::parser::Parser;
use uci::uci_in::uci as cmd_uci;

const TABLE_IDX_BITS: usize = 16;

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
    assert!(out.contains(&"id name"));
    assert!(out.contains(&"id author"));
    assert!(out.contains(&"uciok"));
}
