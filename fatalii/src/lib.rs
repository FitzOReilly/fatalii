use engine::Engine;
use search::alpha_beta::AlphaBeta;
use std::error::Error;
use std::io;
use uci::parser::{Parser, ParserMessage};
use uci::uci_in::{go, is_ready, position, quit, stop, uci as cmd_uci, ucinewgame};
use uci::uci_out::best_move;

const TABLE_IDX_BITS: usize = 20;

pub fn run() -> Result<(), Box<dyn Error>> {
    let search_algo = AlphaBeta::new(TABLE_IDX_BITS);
    let best_move_callback =
        Box::new(move |m| best_move::write(&mut io::stdout(), m).expect("Error writing best move"));
    let mut engine = Engine::new(search_algo, best_move_callback);

    let writer = Box::new(io::stdout());
    let mut parser = Parser::new(writer);

    parser.register_command(String::from("go"), Box::new(go::run_command));
    parser.register_command(String::from("isready"), Box::new(is_ready::run_command));
    parser.register_command(String::from("position"), Box::new(position::run_command));
    parser.register_command(String::from("quit"), Box::new(quit::run_command));
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
