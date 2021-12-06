use crate::parser::{ParserMessage, UciError};
use crate::uci_out::ready_ok::ReadyOk;
use engine::Engine;
use std::error::Error;
use std::io::Write;

pub fn run_command(
    writer: &mut dyn Write,
    args: &str,
    _engine: &mut Engine,
) -> Result<Option<ParserMessage>, Box<dyn Error>> {
    // There must be no arguments after "isready"
    if !args.trim().is_empty() {
        return Err(Box::new(UciError::InvalidArgument(
            args.trim_end().to_string(),
        )));
    }

    ReadyOk::write(writer)?;
    Ok(None)
}
