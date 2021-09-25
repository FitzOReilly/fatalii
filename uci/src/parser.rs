use engine::Engine;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::Write;

type UciInputHandler = dyn Fn(&mut dyn Write, &str, &mut Engine) -> Result<(), Box<dyn Error>>;

pub struct Parser {
    commands: HashMap<String, Box<UciInputHandler>>,
    writer: Box<dyn Write>,
}

#[derive(Debug)]
pub enum UciError {
    InvalidArgument(String),
    UnknownCommand(String),
}

impl Error for UciError {}

impl fmt::Display for UciError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            UciError::InvalidArgument(s) => format!("Invalid argument `{}`", s),
            UciError::UnknownCommand(s) => format!("Unknown command `{}`", s),
        };
        write!(f, "Uci error: {}", msg)
    }
}

impl Parser {
    pub fn new(writer: Box<dyn Write>) -> Self {
        Self {
            commands: HashMap::new(),
            writer,
        }
    }

    pub fn run_command(&mut self, s: &str, engine: &mut Engine) -> Result<(), Box<dyn Error + '_>> {
        debug_assert!(s.ends_with('\n') || s.is_empty());
        // From the UCI specification:
        // If the engine or the GUI receives an unknown command or token it should just
        // ignore it and try to parse the rest of the string in this line.
        let mut tail = s;
        while let Some((cmd, args)) = split_first_word(tail) {
            if let Some(handler) = self.commands.get(cmd) {
                return handler(&mut self.writer, args, engine);
            }
            tail = args;
        }
        Err(Box::new(UciError::UnknownCommand(s.to_string())))
    }

    pub fn register_command(&mut self, cmd: String, handler: Box<UciInputHandler>) {
        self.commands.insert(cmd, handler);
    }
}

pub fn split_first_word(s: &str) -> Option<(&str, &str)> {
    s.trim_start().split_once(|c: char| c.is_whitespace())
}
