use crate::uci_move::UciMove;
use movegen::r#move::Move;
use search::search::SearchResult;
use std::error::Error;
use std::io::Write;

pub fn write(writer: &mut impl Write, res: Option<SearchResult>) -> Result<(), Box<dyn Error>> {
    match res {
        Some(sr) => {
            let pv_str = sr
                .principal_variation()
                .iter()
                .take_while(|m| **m != Move::NULL)
                .map(|m| UciMove::move_to_str(*m))
                .collect::<Vec<String>>()
                .join(" ");
            Ok(writeln!(
                writer,
                "info depth {} score cp {} pv {}",
                sr.depth(),
                sr.score(),
                pv_str
            )?)
        }
        None => Ok(()),
    }
}
