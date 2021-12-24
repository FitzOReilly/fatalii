use crate::uci_move::UciMove;
use search::search::SearchResult;
use std::error::Error;
use std::io::Write;

pub fn write(writer: &mut impl Write, res: Option<SearchResult>) -> Result<(), Box<dyn Error>> {
    match res {
        Some(sr) => Ok(writeln!(
            writer,
            "info depth {} score cp {} pv {}",
            sr.depth(),
            sr.score(),
            UciMove::move_to_str(sr.best_move())
        )?),
        None => Ok(()),
    }
}
