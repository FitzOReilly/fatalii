use crate::parser::UciError;
use crate::uci_out::{id, option, uci_ok};
use engine::Engine;
use std::error::Error;
use std::io::Write;

pub fn run_command(
    mut writer: &mut dyn Write,
    args: &str,
    _engine: &mut Engine,
) -> Result<(), Box<dyn Error>> {
    // There must be no arguments after "uci"
    if !args.trim().is_empty() {
        return Err(Box::new(UciError::InvalidArgument(args.to_string())));
    }

    id::write(&mut writer)?;
    option::write(&mut writer)?;
    uci_ok::write(&mut writer)
}
