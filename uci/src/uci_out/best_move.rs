use crate::uci_move::UciMove;
use crate::uci_out::info;
use search::search::SearchResult;
use std::error::Error;
use std::io::Write;

pub fn write(writer: &mut impl Write, res: Option<SearchResult>) -> Result<(), Box<dyn Error>> {
    info::write(writer, res.clone())?;
    match res {
        Some(sr) => Ok(writeln!(
            writer,
            "bestmove {}",
            UciMove::move_to_str(sr.best_move())
        )?),
        None => Ok(()),
    }
}
