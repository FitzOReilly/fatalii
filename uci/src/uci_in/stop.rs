use crate::parser::{ParserMessage, UciError};
use engine::Engine;
use std::error::Error;
use std::io::Write;

pub fn run_command(
    _writer: &mut dyn Write,
    args: &str,
    engine: &mut Engine,
) -> Result<Option<ParserMessage>, Box<dyn Error>> {
    // There must be no arguments after "stop"
    if !args.trim().is_empty() {
        return Err(Box::new(UciError::InvalidArgument(
            args.trim_end().to_string(),
        )));
    }

    engine.stop();
    Ok(None)
}
