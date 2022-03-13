use crate::parser::{split_first_word, ParserMessage, UciError};
use crate::uci_option::OPTIONS;
use crate::UciOut;
use engine::Engine;
use std::error::Error;

pub fn run_command(
    _uci_out: &mut UciOut,
    args: &str,
    engine: &mut Engine,
) -> Result<Option<ParserMessage>, Box<dyn Error>> {
    match split_first_word(args.trim_end()) {
        Some(("name", args_after_name)) => {
            let mut name_parts = Vec::new();
            let mut value = None;
            let mut remaining = args_after_name;
            while let Some((word, tail)) = split_first_word(remaining) {
                let word_lower = word.to_lowercase();
                if word_lower == "value" {
                    value = Some(tail.to_string());
                    break;
                }
                name_parts.push(word_lower);
                remaining = tail;
            }

            if name_parts.is_empty() {
                return make_err_invalid_argument(args);
            }

            let name = name_parts.join(" ");

            let opt = match OPTIONS.iter().find(|&x| x.name.to_lowercase() == name) {
                Some(o) => o,
                None => return make_err_invalid_argument(args),
            };

            let val = match value {
                Some(v) => match v.parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => return make_err_invalid_argument(args),
                },
                None => return make_err_invalid_argument(args),
            };

            if val < opt.min || val > opt.max {
                return make_err_invalid_argument(args);
            }

            (opt.fun)(engine, val);
        }
        _ => return make_err_invalid_argument(args),
    };

    Ok(None)
}

fn make_err_invalid_argument(args: &str) -> Result<Option<ParserMessage>, Box<dyn Error>> {
    Err(Box::new(UciError::InvalidArgument(format!(
        "setoption {}",
        args.trim_end()
    ))))
}
