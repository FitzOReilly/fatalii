use crate::uci_move::UciMove;
use movegen::r#move::Move;
use std::error::Error;
use std::io::Write;

pub fn write(writer: &mut impl Write, m: Option<Move>) -> Result<(), Box<dyn Error>> {
    Ok(writeln!(
        writer,
        "bestmove {}",
        UciMove::move_to_str(m.unwrap_or(Move::NULL))
    )?)
}
