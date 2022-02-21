use std::error::Error;
use std::io::Write;

pub static mut ENGINE_VERSION: &str = "";

pub fn write(writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    Ok(write!(
        writer,
        "id name Fatalii {}\nid author Patrick Heck\n",
        unsafe { ENGINE_VERSION }
    )?)
}
