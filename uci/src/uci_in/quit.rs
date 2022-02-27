use crate::parser::{ParserMessage, UciError};
use engine::Engine;
use std::error::Error;
use std::io::Write;

pub fn run_command(
    _writer: &mut dyn Write,
    args: &str,
    _engine: &mut Engine,
) -> Result<Option<ParserMessage>, Box<dyn Error>> {
    // There must be no arguments after "quit"
    if !args.trim().is_empty() {
        return Err(Box::new(UciError::InvalidArgument(
            args.trim_end().to_string(),
        )));
    }

    Ok(Some(ParserMessage::Quit))
    // TODO Command
    // go wtime 121000 btime 121000 winc 1000 binc 1000 movestogo 40
}
