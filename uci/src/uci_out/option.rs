use std::error::Error;
use std::io::Write;

pub fn write(_writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    Ok(())
}
