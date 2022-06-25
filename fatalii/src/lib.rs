use engine::{Engine, EngineOptions};
use eval::piece_square_tables::PieceSquareTables;
use eval::Eval;
use search::alpha_beta::AlphaBeta;
use std::error::Error;
use std::io;
use std::sync::{Arc, Mutex};
use uci::uci_in::{
    debug, go, is_ready, position, quit, set_option, stop, uci as cmd_uci, ucinewgame,
};
use uci::UciOut;
use uci::{Parser, ParserMessage};

pub fn run() -> Result<(), Box<dyn Error>> {
    let engine_options = Arc::new(Mutex::new(EngineOptions::default()));
    let uci_out = UciOut::new(
        Box::new(io::stdout()),
        env!("CARGO_PKG_VERSION"),
        Arc::clone(&engine_options),
    );
    let eval_relative = PieceSquareTables::eval_relative;
    let table_idx_bits = 20;
    let search_algo = AlphaBeta::new(eval_relative, table_idx_bits);
    let mut engine = Engine::new(search_algo, uci_out.clone(), engine_options);

    let mut parser = Parser::new(uci_out);
    parser.register_command(String::from("debug"), Box::new(debug::run_command));
    parser.register_command(String::from("go"), Box::new(go::run_command));
    parser.register_command(String::from("isready"), Box::new(is_ready::run_command));
    parser.register_command(String::from("position"), Box::new(position::run_command));
    parser.register_command(String::from("quit"), Box::new(quit::run_command));
    parser.register_command(String::from("setoption"), Box::new(set_option::run_command));
    parser.register_command(String::from("stop"), Box::new(stop::run_command));
    parser.register_command(String::from("uci"), Box::new(cmd_uci::run_command));
    parser.register_command(
        String::from("ucinewgame"),
        Box::new(ucinewgame::run_command),
    );

    let reader = io::stdin();
    let mut buffer = String::new();
    loop {
        reader.read_line(&mut buffer)?;
        match parser.run_command(&buffer, &mut engine) {
            Ok(Some(ParserMessage::Quit)) => break,
            Err(e) => eprintln!("{}", e),
            _ => {}
        }
        buffer.clear();
    }
    Ok(())
}
