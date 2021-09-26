use std::error::Error;
use std::io::Write;

pub fn write(writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    Ok(writeln!(writer, "uciok")?)
}
