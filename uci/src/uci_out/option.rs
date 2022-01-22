use crate::uci_option::{OptionType, UciOption, OPTIONS};
use std::error::Error;
use std::io::Write;

pub fn write(writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    for opt in OPTIONS {
        write_option(writer, &opt)?;
    }
    Ok(())
}

pub fn write_option(writer: &mut impl Write, opt: &UciOption) -> Result<(), Box<dyn Error>> {
    match opt.r#type {
        OptionType::Spin => {
            writeln!(
                writer,
                "option name {} type spin default {} min {} max {}",
                opt.name, opt.default, opt.min, opt.max
            )?;
        }
        OptionType::Button | OptionType::Check | OptionType::Combo | OptionType::String => {
            unimplemented!();
        }
    }
    Ok(())
}
