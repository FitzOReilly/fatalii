use crate::bitboard::Bitboard;
use crate::file::File;
use crate::piece::Piece;
use crate::position::CastlingRights;
use crate::position::Position;
use crate::rank::Rank;
use crate::side::Side;
use crate::square::Square;
use std::error::Error;
use std::fmt;
use std::str;

pub struct Fen;

#[derive(Debug)]
pub enum FenError {
    InvalidFenString(String),
}

impl Error for FenError {}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            FenError::InvalidFenString(s) => format!("Invalid FEN string `{}`", s),
        };
        write!(f, "FEN error: {}", msg)
    }
}

impl Fen {
    pub fn pos_to_str(pos: &Position) -> String {
        let mut fen = String::new();
        Self::pos_to_str_pieces(&mut fen, pos);
        fen.push(' ');
        Self::pos_to_str_side_to_move(&mut fen, pos);
        fen.push(' ');
        Self::pos_to_str_castling_rights(&mut fen, pos);
        fen.push(' ');
        Self::pos_to_str_en_passant_square(&mut fen, pos);
        fen.push(' ');
        Self::pos_to_str_move_count(&mut fen, pos);
        fen
    }

    fn pos_to_str_pieces(fen: &mut String, pos: &Position) {
        for rank in (0..Rank::NUM_RANKS).rev() {
            let mut num_empty_squares = 0;
            for file in 0..File::NUM_FILES {
                let square = Square::from_file_and_rank(File::from_idx(file), Rank::from_idx(rank));
                match pos.piece_at(square) {
                    Some(piece) => {
                        if num_empty_squares > 0 {
                            fen.push((num_empty_squares + b'0') as char);
                            num_empty_squares = 0;
                        }
                        fen.push(piece.to_ascii() as char);
                    }
                    None => {
                        num_empty_squares += 1;
                    }
                }
            }
            if num_empty_squares > 0 {
                fen.push((num_empty_squares + b'0') as char);
            }
            if rank > 0 {
                fen.push('/');
            }
        }
    }

    fn pos_to_str_side_to_move(fen: &mut String, pos: &Position) {
        fen.push(match pos.side_to_move() {
            Side::White => 'w',
            Side::Black => 'b',
        });
    }

    fn pos_to_str_castling_rights(fen: &mut String, pos: &Position) {
        let castling_rights = pos.castling_rights();
        if castling_rights.is_empty() {
            fen.push('-');
        } else {
            if castling_rights.contains(CastlingRights::WHITE_KINGSIDE) {
                fen.push('K');
            }
            if castling_rights.contains(CastlingRights::WHITE_QUEENSIDE) {
                fen.push('Q');
            }
            if castling_rights.contains(CastlingRights::BLACK_KINGSIDE) {
                fen.push('k');
            }
            if castling_rights.contains(CastlingRights::BLACK_QUEENSIDE) {
                fen.push('q');
            }
        }
    }

    fn pos_to_str_en_passant_square(fen: &mut String, pos: &Position) {
        let en_passant_board = pos.en_passant_square();
        if en_passant_board == Bitboard::EMPTY {
            fen.push('-');
        } else {
            let en_passant_square = en_passant_board.to_square();
            fen.push_str(&format!("{}", en_passant_square));
        }
    }

    fn pos_to_str_move_count(fen: &mut String, pos: &Position) {
        fen.push_str(&format!(
            "{} {}",
            pos.plies_since_pawn_move_or_capture(),
            pos.move_count()
        ));
    }

    pub fn str_to_pos(fen: &str) -> Result<Position, FenError> {
        let mut pos = Position::empty();
        let mut iter_fen = fen.split_whitespace();
        if iter_fen.clone().count() != 6 {
            return Err(FenError::InvalidFenString(fen.to_string()));
        }

        match Self::str_to_pos_pieces(&mut pos, iter_fen.next().unwrap())
            .and(Self::str_to_pos_side_to_move(
                &mut pos,
                iter_fen.next().unwrap(),
            ))
            .and(Self::str_to_pos_castling_rights(
                &mut pos,
                iter_fen.next().unwrap(),
            ))
            .and(Self::str_to_pos_en_passant_square(
                &mut pos,
                iter_fen.next().unwrap(),
            ))
            .and(Self::str_to_pos_plies_since_pawn_move_or_capture(
                &mut pos,
                iter_fen.next().unwrap(),
            ))
            .and(Self::str_to_pos_move_count(
                &mut pos,
                iter_fen.next().unwrap(),
            )) {
            Ok(()) => Ok(pos),
            Err(()) => Err(FenError::InvalidFenString(fen.to_string())),
        }
    }

    fn str_to_pos_pieces(pos: &mut Position, fen: &str) -> Result<(), ()> {
        let iter_ranks = fen.split('/');
        let mut rank = Rank::NUM_RANKS;
        for fen_rank in iter_ranks {
            if rank == 0 {
                return Err(());
            }
            rank -= 1;
            let mut file = 0;
            for c in fen_rank.bytes() {
                if file >= File::NUM_FILES {
                    return Err(());
                }
                match c {
                    b'1'..=b'8' => {
                        file += (c - b'0') as usize;
                    }
                    _ => {
                        let piece = match Piece::from_ascii(c) {
                            Ok(p) => p,
                            Err(_) => return Err(()),
                        };
                        let square =
                            Square::from_file_and_rank(File::from_idx(file), Rank::from_idx(rank));
                        pos.set_piece_at(square, Some(piece));
                        file += 1;
                    }
                }
            }
            if file != File::NUM_FILES {
                return Err(());
            }
        }
        if rank != 0 {
            return Err(());
        }
        Ok(())
    }

    fn str_to_pos_side_to_move(pos: &mut Position, fen: &str) -> Result<(), ()> {
        let c = fen.bytes().next().unwrap();
        match c {
            b'w' => pos.set_side_to_move(Side::White),
            b'b' => pos.set_side_to_move(Side::Black),
            _ => return Err(()),
        }
        Ok(())
    }

    fn str_to_pos_castling_rights(pos: &mut Position, fen: &str) -> Result<(), ()> {
        let mut castling_rights = CastlingRights::empty();
        for c in fen.bytes() {
            castling_rights |= match c {
                b'-' => CastlingRights::empty(),
                b'K' => CastlingRights::WHITE_KINGSIDE,
                b'Q' => CastlingRights::WHITE_QUEENSIDE,
                b'k' => CastlingRights::BLACK_KINGSIDE,
                b'q' => CastlingRights::BLACK_QUEENSIDE,
                _ => return Err(()),
            };
            pos.set_castling_rights(castling_rights);
        }
        Ok(())
    }

    fn str_to_pos_en_passant_square(pos: &mut Position, fen: &str) -> Result<(), ()> {
        let mut iter_ep = fen.bytes();
        let c = iter_ep.next().unwrap();
        match c {
            b'-' => {}
            f @ b'a'..=b'h' => {
                let file = File::from_ascii(f).unwrap();
                let r = iter_ep.next().unwrap();
                let rank = Rank::from_ascii(r).unwrap();
                match (pos.side_to_move(), rank) {
                    (Side::White, Rank::R6) | (Side::Black, Rank::R3) => {}
                    _ => return Err(()),
                }
                let square = Square::from_file_and_rank(file, rank);
                pos.set_en_passant_square(Bitboard::from_square(square));
            }
            _ => return Err(()),
        }
        Ok(())
    }

    fn str_to_pos_plies_since_pawn_move_or_capture(
        pos: &mut Position,
        fen: &str,
    ) -> Result<(), ()> {
        let plies = match fen.parse::<usize>() {
            Ok(p) => p,
            Err(_) => return Err(()),
        };
        pos.set_plies_since_pawn_move_or_capture(plies);
        Ok(())
    }

    fn str_to_pos_move_count(pos: &mut Position, fen: &str) -> Result<(), ()> {
        let move_count = match fen.parse::<usize>() {
            Ok(p) => p,
            Err(_) => return Err(()),
        };
        pos.set_move_count(move_count);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_fen() {
        let missing_square_in_rank = "nbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(Fen::str_to_pos(missing_square_in_rank).is_err());
        let too_many_squares_in_rank = "rrnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(Fen::str_to_pos(too_many_squares_in_rank).is_err());
        let missing_rank = "pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(Fen::str_to_pos(missing_rank).is_err());
        let too_many_ranks = "rnbqkbnr/rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(Fen::str_to_pos(too_many_ranks).is_err());
        let invalid_piece = "xnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(Fen::str_to_pos(invalid_piece).is_err());
        let invalid_side_to_move = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1";
        assert!(Fen::str_to_pos(invalid_side_to_move).is_err());
        let invalid_castling_rights = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w x - 0 1";
        assert!(Fen::str_to_pos(invalid_castling_rights).is_err());
        let invalid_en_passant = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq x 0 1";
        assert!(Fen::str_to_pos(invalid_en_passant).is_err());
        let illegal_en_passant = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e1 0 1";
        assert!(Fen::str_to_pos(illegal_en_passant).is_err());
        let invalid_halfmove_clock = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - x 1";
        assert!(Fen::str_to_pos(invalid_halfmove_clock).is_err());
        let invalid_move_count = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 x";
        assert!(Fen::str_to_pos(invalid_move_count).is_err());
        let too_short = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0";
        assert!(Fen::str_to_pos(too_short).is_err());
        let too_long = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 2";
        assert!(Fen::str_to_pos(too_long).is_err());
    }

    #[test]
    fn conversion_between_str_and_pos() {
        let mut pos = Position::initial();
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(fen, Fen::pos_to_str(&pos));
        assert_eq!(
            pos,
            Fen::str_to_pos(&fen).unwrap(),
            "\nExpected Position as FEN: {}\nActual Position as FEN:   {}\n",
            fen,
            Fen::pos_to_str(&Fen::str_to_pos(&fen).unwrap())
        );

        // Position after 1. e4
        pos.set_piece_at(Square::E2, None);
        pos.set_piece_at(Square::E4, Some(Piece::WHITE_PAWN));
        pos.set_en_passant_square(Bitboard::E3);
        pos.set_side_to_move(Side::Black);
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        assert_eq!(fen, Fen::pos_to_str(&pos));
        assert_eq!(
            pos,
            Fen::str_to_pos(&fen).unwrap(),
            "\nExpected Position as FEN: {}\nActual Position as FEN:   {}\n",
            fen,
            Fen::pos_to_str(&Fen::str_to_pos(&fen).unwrap())
        );

        // Position after 1. e4 c5
        pos.set_piece_at(Square::C7, None);
        pos.set_piece_at(Square::C5, Some(Piece::BLACK_PAWN));
        pos.set_en_passant_square(Bitboard::C6);
        pos.set_side_to_move(Side::White);
        pos.set_move_count(2);
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
        assert_eq!(fen, Fen::pos_to_str(&pos));
        assert_eq!(
            pos,
            Fen::str_to_pos(&fen).unwrap(),
            "\nExpected Position as FEN: {}\nActual Position as FEN:   {}\n",
            fen,
            Fen::pos_to_str(&Fen::str_to_pos(&fen).unwrap())
        );

        // Position after 1. e4 c5 2. Nf3
        pos.set_piece_at(Square::G1, None);
        pos.set_piece_at(Square::F3, Some(Piece::WHITE_KNIGHT));
        pos.set_en_passant_square(Bitboard::EMPTY);
        pos.set_side_to_move(Side::Black);
        pos.set_plies_since_pawn_move_or_capture(1);
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        assert_eq!(fen, Fen::pos_to_str(&pos));
        assert_eq!(
            pos,
            Fen::str_to_pos(&fen).unwrap(),
            "\nExpected Position as FEN: {}\nActual Position as FEN:   {}\n",
            fen,
            Fen::pos_to_str(&Fen::str_to_pos(&fen).unwrap())
        );

        // Check castling rights
        pos.set_castling_rights(CastlingRights::WHITE_QUEENSIDE | CastlingRights::BLACK_KINGSIDE);
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b Qk - 1 2";
        assert_eq!(fen, Fen::pos_to_str(&pos));
        pos.set_castling_rights(CastlingRights::empty());
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b - - 1 2";
        assert_eq!(fen, Fen::pos_to_str(&pos));
        assert_eq!(
            pos,
            Fen::str_to_pos(&fen).unwrap(),
            "\nExpected Position as FEN: {}\nActual Position as FEN:   {}\n",
            fen,
            Fen::pos_to_str(&Fen::str_to_pos(&fen).unwrap())
        );
    }
}
