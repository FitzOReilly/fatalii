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
    uci_out: &mut UciOut,
    args: &str,
    engine: &mut Engine,
) -> Result<Option<ParserMessage>, Box<dyn Error>> {
    let options = parse_options(uci_out, args, engine)?;
    run(options, engine)
}

fn parse_options(
    uci_out: &UciOut,
    go_args: &str,
    engine: &Engine,
) -> Result<SearchOptions, Box<dyn Error>> {
    let mut options = SearchOptions::default();
    let mut seen_options = HashSet::new();
    let mut s = go_args;
    while let Some((opt, tail)) = split_first_word(s) {
        if !seen_options.insert(opt) {
            return Err(Box::new(UciError::InvalidArgument(format!(
                "Option `{opt}` must not appear more than once in\ngo {go_args}",
            ))));
        }
        s = match opt {
            "searchmoves" => parse_moves(tail, &mut options.search_moves, engine)?,
            "ponder" => {
                options.ponder = true;
                tail
            }
            name @ "wtime" => parse_duration(uci_out, name, tail, &mut options.white_time)?,
            name @ "btime" => parse_duration(uci_out, name, tail, &mut options.black_time)?,
            name @ "winc" => parse_duration(uci_out, name, tail, &mut options.white_inc)?,
            name @ "binc" => parse_duration(uci_out, name, tail, &mut options.black_inc)?,
            name @ "movestogo" => parse_usize(uci_out, name, tail, &mut options.moves_to_go)?,
            name @ "depth" => parse_usize(uci_out, name, tail, &mut options.depth)?,
            name @ "nodes" => parse_usize(uci_out, name, tail, &mut options.nodes)?,
            name @ "mate" => parse_usize(uci_out, name, tail, &mut options.mate_in)?,
            name @ "movetime" => parse_duration(uci_out, name, tail, &mut options.movetime)?,
            "infinite" => {
                options.infinite = true;
                tail
            }
            _ => {
                return Err(Box::new(UciError::InvalidArgument(format!(
                    "go {}",
                    go_args.trim_end()
                ))))
            }
        }
    }
    Ok(options)
}

fn parse_usize<'a>(
    uci_out: &UciOut,
    opt: &str,
    args: &'a str,
    number: &mut Option<usize>,
) -> Result<&'a str, Box<dyn Error>> {
    debug_assert!(number.is_none());
    match split_first_word(args) {
        Some((first_word, tail)) => {
            let signed_num = first_word.parse::<i64>()?;
            if signed_num < 0 {
                uci_out.warn(&format!(
                    "ignoring negative value `{opt} {signed_num}`, using `{opt} 0` instead",
                ))?;
            }
            *number = Some(signed_num.max(0) as usize);
            Ok(tail)
        }
        _ => Err(Box::new(UciError::InvalidArgument(format!("go {args}")))),
    }
}

fn parse_duration<'a>(
    uci_out: &UciOut,
    opt: &str,
    args: &'a str,
    dur: &mut Option<Duration>,
) -> Result<&'a str, Box<dyn Error>> {
    debug_assert!(dur.is_none());
    match split_first_word(args) {
        Some((first_word, tail)) => {
            let signed_dur = first_word.parse::<i64>()?;
            if signed_dur < 0 {
                uci_out.warn(&format!(
                    "ignoring negative value `{opt} {signed_dur}`, using `{opt} 0` instead",
                ))?;
            }
            *dur = Some(Duration::from_millis(signed_dur.max(0) as u64));
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
