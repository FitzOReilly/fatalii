use engine::Engine;
use search::alpha_beta::AlphaBeta;
use std::io::Write;
use uci::parser::Parser;

const TABLE_IDX_BITS: usize = 16;

#[test]
fn register_command() {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let mut engine = Engine::new(search_algo);
    let mut p = Parser::new(Box::new(std::io::stdout()));

    assert!(p.run_command("unknown\n", &mut engine).is_err());
    assert!(p.run_command("cmd\n", &mut engine).is_err());
    assert!(p.run_command("cmd args\n", &mut engine).is_err());
    assert!(p.run_command(" \r \t  cmd  args \n", &mut engine).is_err());

    let cmd_handler = |writer: &mut dyn Write, args: &str, _engine: &mut Engine| {
        Ok(writeln!(writer, "cmd called with args: {}", args)?)
    };
    p.register_command(String::from("cmd"), Box::new(cmd_handler));

    assert!(p.run_command("unknown\n", &mut engine).is_err());
    assert!(p.run_command("cmd\n", &mut engine).is_ok());
    assert!(p.run_command("cmd args\n", &mut engine).is_ok());
    assert!(p.run_command(" \r \t  cmd  args \n", &mut engine).is_ok());
}
