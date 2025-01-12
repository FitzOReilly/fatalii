use crate::parser::{split_first_word, ParserMessage, UciError};
use crate::uci_move::UciMove;
use crate::UciOut;
use engine::{Engine, Variant};
use movegen::fen::Fen;
use movegen::position::Position as Pos;
use movegen::position_history::PositionHistory;
use std::error::Error;

pub fn run_command(
    _uci_out: &mut UciOut,
    args: &str,
    engine: &mut Engine,
) -> Result<Option<ParserMessage>, Box<dyn Error>> {
    let (mut pos_hist, moves_str) = match split_first_word(args) {
        Some(("fen", tail)) => parse_fen(tail, engine)?,
        Some(("startpos", tail)) => (PositionHistory::new(Pos::initial()), tail),
        _ => {
            return Err(Box::new(UciError::InvalidArgument(format!(
                "position {}",
                args.trim_end()
            ))))
        }
    };

    let var = engine.variant();

    match split_first_word(moves_str) {
        Some(("moves", tail)) => {
            let iter = tail.split_whitespace();
            match var {
                Variant::Standard => {
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
                Variant::Chess960(king_rook, queen_rook) => {
                    for move_str in iter {
                        match UciMove::str_to_move_chess_960(
                            pos_hist.current_pos(),
                            move_str,
                            king_rook,
                            queen_rook,
                        ) {
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

fn parse_fen<'a>(
    args: &'a str,
    engine: &'a Engine,
) -> Result<(PositionHistory, &'a str), Box<dyn Error>> {
    let trimmed = args.trim_start();
    match trimmed
        .chars()
        .scan(0, |count, c| {
            *count += c.is_whitespace() as usize;
            Some(*count)
        })
        .position(|c| c == 6)
    {
        Some(fen_end) => {
            let opt_pos = match engine.variant() {
                Variant::Standard => Fen::str_to_pos(&trimmed[..fen_end]),
                Variant::Chess960(_, _) => Fen::str_to_pos_chess_960(&trimmed[..fen_end]),
            };
            match opt_pos {
                Ok(pos) => {
                    if let Variant::Chess960(_, _) = engine.variant() {
                        engine.set_variant(Variant::Chess960(
                            pos.kingside_rook_start_file(),
                            pos.queenside_rook_start_file(),
                        ));
                    }
                    Ok((PositionHistory::new(pos), &trimmed[fen_end..]))
                }
                Err(e) => Err(Box::new(UciError::InvalidArgument(format!(
                    "position fen {args}\n{e}",
                )))),
            }
        }
        None => Err(Box::new(UciError::InvalidArgument(format!(
            "position fen {}",
            args.trim_end()
        )))),
    }
}
