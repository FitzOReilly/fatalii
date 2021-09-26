use std::error::Error;
use std::io::Write;

pub fn write(writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    Ok(write!(writer, "id name Fatalii\nid author Patrick Heck\n")?)
}
