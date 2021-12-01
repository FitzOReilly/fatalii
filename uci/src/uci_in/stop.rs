use crate::parser::UciError;
use engine::Engine;
use std::error::Error;
use std::io::Write;

pub fn run_command(
    _writer: &mut dyn Write,
    args: &str,
    engine: &mut Engine,
) -> Result<(), Box<dyn Error>> {
    // There must be no arguments after "stop"
    if !args.trim().is_empty() {
        return Err(Box::new(UciError::InvalidArgument(args.to_string())));
    }

    engine.stop();
    Ok(())
}
