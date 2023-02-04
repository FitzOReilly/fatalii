use crate::parser::{split_first_word, ParserMessage, UciError};
use crate::uci_move::UciMove;
use crate::UciOut;
use engine::{Engine, EngineError};
use movegen::r#move::MoveList;
use search::SearchOptions;
use std::collections::HashSet;
use std::error::Error;
use std::time::Duration;

pub fn run_command(
    _uci_out: &mut UciOut,
    args: &str,
    engine: &mut Engine,
) -> Result<Option<ParserMessage>, Box<dyn Error>> {
    let options = parse_options(args, engine)?;
    run(options, engine)
}

fn parse_options(args: &str, engine: &Engine) -> Result<SearchOptions, Box<dyn Error>> {
    let mut options = SearchOptions::default();
    let mut seen_options = HashSet::new();
    let mut s = args;
    while let Some((cmd, tail)) = split_first_word(s) {
        if !seen_options.insert(cmd) {
            return Err(Box::new(UciError::InvalidArgument(format!(
                "Option `{cmd}` must not appear more than once in\ngo {args}",
            ))));
        }
        s = match cmd {
            "searchmoves" => parse_moves(tail, &mut options.search_moves, engine)?,
            "ponder" => {
                options.ponder = true;
                tail
            }
            "wtime" => parse_duration(tail, &mut options.white_time)?,
            "btime" => parse_duration(tail, &mut options.black_time)?,
            "winc" => parse_duration(tail, &mut options.white_inc)?,
            "binc" => parse_duration(tail, &mut options.black_inc)?,
            "movestogo" => parse_usize(tail, &mut options.moves_to_go)?,
            "depth" => parse_usize(tail, &mut options.depth)?,
            "nodes" => parse_usize(tail, &mut options.nodes)?,
            "mate" => parse_usize(tail, &mut options.mate_in)?,
            "movetime" => parse_duration(tail, &mut options.movetime)?,
            "infinite" => {
                options.infinite = true;
                tail
            }
            _ => {
                return Err(Box::new(UciError::InvalidArgument(format!(
                    "go {}",
                    args.trim_end()
                ))))
            }
        }
    }
    Ok(options)
}

fn parse_usize<'a>(args: &'a str, number: &mut Option<usize>) -> Result<&'a str, Box<dyn Error>> {
    debug_assert!(number.is_none());
    match split_first_word(args) {
        Some((first_word, tail)) => {
            *number = Some(first_word.parse::<usize>()?);
            Ok(tail)
        }
        _ => Err(Box::new(UciError::InvalidArgument(format!("go {args}")))),
    }
}

fn parse_duration<'a>(
    args: &'a str,
    dur: &mut Option<Duration>,
) -> Result<&'a str, Box<dyn Error>> {
    debug_assert!(dur.is_none());
    match split_first_word(args) {
        Some((first_word, tail)) => {
            *dur = Some(Duration::from_millis(first_word.parse::<u64>()?));
            Ok(tail)
        }
        _ => Err(Box::new(UciError::InvalidArgument(format!("go {args}")))),
    }
}

fn parse_moves<'a>(
    args: &'a str,
    search_moves: &mut Option<MoveList>,
    engine: &Engine,
) -> Result<&'a str, Box<dyn Error>> {
    debug_assert!(search_moves.is_none());
    let pos = match engine.position() {
        Some(p) => p,
        None => return Err(Box::new(EngineError::SearchWithoutPosition)),
    };
    let mut move_list = MoveList::new();
    let mut s = args;
    while let Some((move_str, tail)) = split_first_word(s) {
        match UciMove::str_to_move(pos, move_str) {
            Some(m) => move_list.push(m),
            None => break,
        }
        s = tail;
    }
    *search_moves = Some(move_list);
    Ok(s)
}

fn run(
    options: SearchOptions,
    engine: &mut Engine,
) -> Result<Option<ParserMessage>, Box<dyn Error>> {
    match engine.search(options) {
        Ok(_) => Ok(None),
        Err(e) => Err(e.into()),
    }
}
