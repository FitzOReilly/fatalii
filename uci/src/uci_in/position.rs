use crate::parser::{split_first_word, ParserMessage, UciError};
use crate::uci_move::UciMove;
use engine::Engine;
use movegen::fen::Fen;
use movegen::position::Position as Pos;
use movegen::position_history::PositionHistory;
use std::error::Error;
use std::io::Write;

pub fn run_command(
    _writer: &mut dyn Write,
    args: &str,
    engine: &mut Engine,
) -> Result<Option<ParserMessage>, Box<dyn Error>> {
    let (mut pos_hist, moves_str) = match split_first_word(args) {
        Some(("fen", tail)) => parse_fen(tail)?,
        Some(("startpos", tail)) => (PositionHistory::new(Pos::initial()), tail),
        _ => {
            return Err(Box::new(UciError::InvalidArgument(format!(
                "position {}",
                args.trim_end()
            ))))
        }
    };

    match split_first_word(moves_str) {
        Some(("moves", tail)) => {
            let iter = tail.split_whitespace();
            for move_str in iter {
                match UciMove::str_to_move(pos_hist.current_pos(), move_str) {
                    Some(m) => pos_hist.do_move(m),
                    None => {
                        return Err(Box::new(UciError::InvalidArgument(format!(
                            "Invalid move `{}` in command: position {}",
                            move_str,
                            args.trim_end()
                        ))))
                    }
                }
            }
        }
        None => {}
        _ => {
            return Err(Box::new(UciError::InvalidArgument(format!(
                "position {}",
                args.trim_end()
            ))))
        }
    };

    engine.set_position_history(Some(pos_hist));
    Ok(None)
}

fn parse_fen(args: &str) -> Result<(PositionHistory, &str), Box<dyn Error>> {
    let trimmed = args.trim_start();
    match trimmed
        .chars()
        .scan(0, |count, c| {
            *count += c.is_whitespace() as usize;
            Some(*count)
        })
        .position(|c| c == 6)
    {
        Some(fen_end) => match Fen::str_to_pos(&trimmed[..fen_end]) {
            Ok(pos) => Ok((PositionHistory::new(pos), &trimmed[fen_end..])),
            Err(e) => {
                return Err(Box::new(UciError::InvalidArgument(format!(
                    "position fen {}\n{}",
                    args, e
                ))))
            }
        },
        None => {
            return Err(Box::new(UciError::InvalidArgument(format!(
                "position fen {}",
                args
            ))))
        }
    }
}
