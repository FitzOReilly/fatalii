use crate::parser::{split_first_word, UciError};
use engine::Engine;
use std::collections::HashSet;
use std::error::Error;
use std::io::Write;
use std::time::Duration;

struct GoOptions {
    depth: Option<usize>,
    movetime: Option<Duration>,
}

impl GoOptions {
    fn new() -> Self {
        Self {
            depth: None,
            movetime: None,
        }
    }
}

pub fn run_command(
    _writer: &mut dyn Write,
    args: &str,
    engine: &mut Engine,
) -> Result<(), Box<dyn Error>> {
    let options = parse_options(args)?;
    run(options, engine)
}

fn parse_options(args: &str) -> Result<GoOptions, Box<dyn Error>> {
    let mut options = GoOptions::new();
    let mut seen_options = HashSet::new();
    let mut s = args;
    while let Some((cmd, tail)) = split_first_word(s) {
        if !seen_options.insert(cmd) {
            return Err(Box::new(UciError::InvalidArgument(format!(
                "Option `{}` must not appear more than once in\ngo {}",
                cmd, args
            ))));
        }
        s = match cmd {
            "depth" => parse_leading_usize(tail, &mut options.depth)?,
            "movetime" => parse_leading_duration(tail, &mut options.movetime)?,
            _ => return Err(Box::new(UciError::InvalidArgument(format!("go {}", args)))),
        }
    }
    Ok(options)
}

fn parse_leading_usize<'a>(
    args: &'a str,
    number: &mut Option<usize>,
) -> Result<&'a str, Box<dyn Error>> {
    debug_assert!(number.is_none());
    match split_first_word(args) {
        Some((first_word, tail)) => {
            *number = Some(first_word.parse::<usize>()?);
            Ok(tail)
        }
        _ => Err(Box::new(UciError::InvalidArgument(format!("go {}", args)))),
    }
}

fn parse_leading_duration<'a>(
    args: &'a str,
    dur: &mut Option<Duration>,
) -> Result<&'a str, Box<dyn Error>> {
    debug_assert!(dur.is_none());
    match split_first_word(args) {
        Some((first_word, tail)) => {
            *dur = Some(Duration::from_millis(first_word.parse::<u64>()?));
            Ok(tail)
        }
        _ => Err(Box::new(UciError::InvalidArgument(format!("go {}", args)))),
    }
}

fn run(options: GoOptions, engine: &mut Engine) -> Result<(), Box<dyn Error>> {
    engine
        .search(options.depth, options.movetime)
        .map_err(|e| e.into())
}