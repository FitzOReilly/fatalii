use crate::parser::{ParserMessage, UciError};
use crate::UciOut;
use engine::{Engine, EngineOut};
use std::error::Error;

pub fn run_command(
    uci_out: &mut UciOut,
    args: &str,
    _engine: &mut Engine,
) -> Result<Option<ParserMessage>, Box<dyn Error>> {
    match args.trim() {
        "on" => {
            uci_out.set_debug(true);
            uci_out.info_string("debug on")?;
        }
        "off" => {
            uci_out.set_debug(false);
            uci_out.info_string("debug off")?;
        }
        _ => {
            return Err(Box::new(UciError::InvalidArgument(format!(
                "debug {}",
                args.trim_end()
            ))));
        }
    }

    Ok(None)
}
