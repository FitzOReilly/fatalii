use std::error::Error;
use std::io::Write;

#[derive(Debug)]
pub struct ReadyOk;

impl ReadyOk {
    pub fn write(writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        Ok(writeln!(writer, "readyok")?)
    }
}
