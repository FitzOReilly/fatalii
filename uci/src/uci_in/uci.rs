use crate::parser::{ParserMessage, UciError};
use crate::UciOut;
use engine::Engine;
use std::error::Error;

pub fn run_command(
    uci_out: &mut UciOut,
    args: &str,
    _engine: &mut Engine,
) -> Result<Option<ParserMessage>, Box<dyn Error>> {
    // There must be no arguments after "uci"
    if !args.trim().is_empty() {
        return Err(Box::new(UciError::InvalidArgument(
            args.trim_end().to_string(),
        )));
    }

    uci_out.id()?;
    uci_out.all_options()?;
    uci_out.uci_ok()?;
    Ok(None)
}
