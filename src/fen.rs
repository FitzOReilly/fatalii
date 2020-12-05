use crate::bitboard::Bitboard;
use crate::piece::Piece;
use crate::position::CastlingRights;
use crate::position::Position;
use crate::position::SideToMove;

pub struct Fen;

impl Fen {
    fn from_position(pos: &Position) -> String {
        let mut fen = String::new();
        Self::from_pieces(&mut fen, pos);
        fen.push(' ');
        Self::from_side_to_move(&mut fen, pos);
        fen.push(' ');
        Self::from_castling_rights(&mut fen, pos);
        fen.push(' ');
        Self::from_en_passant_square(&mut fen, pos);
        fen.push(' ');
        Self::from_move_counts(&mut fen, pos);
        fen
    }

    fn from_pieces(fen: &mut String, pos: &Position) {
        for rank in (0..Bitboard::NUM_RANKS).rev() {
            let mut num_empty_squares = 0;
            for file in 0..Bitboard::NUM_FILES {
                let square = Bitboard::to_square(rank, file);
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

    fn from_side_to_move(fen: &mut String, pos: &Position) {
        fen.push(match pos.side_to_move() {
            SideToMove::White => 'w',
            SideToMove::Black => 'b',
        });
    }

    fn from_castling_rights(fen: &mut String, pos: &Position) {
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

    fn from_en_passant_square(fen: &mut String, pos: &Position) {
        let en_passant_square = pos.en_passant_square();
        if en_passant_square == Bitboard::EMPTY {
            fen.push('-');
        } else {
            let square = en_passant_square.bit_idx();
            let file = Bitboard::to_file(square);
            let rank = Bitboard::to_rank(square);
            fen.push((file as u8 + b'a') as char);
            fen.push((rank as u8 + b'1') as char);
        }
    }

    fn from_move_counts(fen: &mut String, pos: &Position) {
        fen.push_str(&format!(
            "{} {}",
            pos.plies_since_pawn_move_or_capture(),
            pos.move_count()
        ));
    }

    fn to_position(fen: &str) -> Position {
        let mut pos = Position::empty();
        let mut iter_fen = fen.split_whitespace();
        Self::to_pieces(&mut pos, iter_fen.next().unwrap());
        Self::to_side_to_move(&mut pos, iter_fen.next().unwrap());
        Self::to_castling_rights(&mut pos, iter_fen.next().unwrap());
        Self::to_en_passant_square(&mut pos, iter_fen.next().unwrap());
        Self::to_plies_since_pawn_move_or_capture(&mut pos, iter_fen.next().unwrap());
        Self::to_move_count(&mut pos, iter_fen.next().unwrap());
        pos
    }

    fn to_pieces(pos: &mut Position, fen: &str) {
        let iter_ranks = fen.split('/');
        let mut rank = Bitboard::NUM_RANKS;
        for fen_rank in iter_ranks {
            rank -= 1;
            let mut file = 0;
            for c in fen_rank.bytes() {
                match c {
                    b'1'..=b'8' => {
                        file += (c - b'0') as usize;
                    }
                    _ => {
                        let piece = Piece::from_ascii(c).unwrap();
                        let square = Bitboard::to_square(rank, file);
                        pos.set_piece_at(square, Some(piece));
                        file += 1;
                    }
                }
            }
        }
    }

    fn to_side_to_move(pos: &mut Position, fen: &str) {
        let c = fen.bytes().next().unwrap();
        match c {
            b'w' => pos.set_side_to_move(SideToMove::White),
            b'b' => pos.set_side_to_move(SideToMove::Black),
            _ => panic!("Invalid side to move `{}`", fen),
        }
    }

    fn to_castling_rights(pos: &mut Position, fen: &str) {
        let mut castling_rights = CastlingRights::empty();
        for c in fen.bytes() {
            castling_rights |= match c {
                b'-' => CastlingRights::empty(),
                b'K' => CastlingRights::WHITE_KINGSIDE,
                b'Q' => CastlingRights::WHITE_QUEENSIDE,
                b'k' => CastlingRights::BLACK_KINGSIDE,
                b'q' => CastlingRights::BLACK_QUEENSIDE,
                _ => {
                    panic!("Invalid castling rights `{}`", fen);
                }
            };
            pos.set_castling_rights(castling_rights);
        }
    }

    fn to_en_passant_square(pos: &mut Position, fen: &str) {
        let mut iter_ep = fen.bytes();
        let c = iter_ep.next().unwrap();
        match c {
            b'-' => {}
            f @ b'a'..=b'h' => {
                let file = (f - b'a') as usize;
                let r = iter_ep.next().unwrap();
                let rank = (r - b'1') as usize;
                pos.set_en_passant_square(Bitboard(0x1 << Bitboard::to_square(rank, file)));
            }
            _ => panic!("Invalid en passant square `{}`", fen),
        }
    }

    fn to_plies_since_pawn_move_or_capture(pos: &mut Position, fen: &str) {
        let plies = fen.parse::<usize>().unwrap();
        pos.set_plies_since_pawn_move_or_capture(plies);
    }

    fn to_move_count(pos: &mut Position, fen: &str) {
        let move_count = fen.parse::<usize>().unwrap();
        pos.set_move_count(move_count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_and_to_position() {
        let mut pos = Position::initial();
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(fen, Fen::from_position(&pos));
        assert_eq!(
            pos,
            Fen::to_position(&fen),
            "\nExpected Position as FEN: {}\nActual Position as FEN:   {}\n",
            fen,
            Fen::from_position(&Fen::to_position(&fen))
        );

        // Position after 1. e4
        pos.set_piece_at(Bitboard::IDX_E2, None);
        pos.set_piece_at(Bitboard::IDX_E4, Some(Piece::WhitePawn));
        pos.set_en_passant_square(Bitboard::E3);
        pos.set_side_to_move(SideToMove::Black);
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        assert_eq!(fen, Fen::from_position(&pos));
        assert_eq!(
            pos,
            Fen::to_position(&fen),
            "\nExpected Position as FEN: {}\nActual Position as FEN:   {}\n",
            fen,
            Fen::from_position(&Fen::to_position(&fen))
        );

        // Position after 1. e4 c5
        pos.set_piece_at(Bitboard::IDX_C7, None);
        pos.set_piece_at(Bitboard::IDX_C5, Some(Piece::BlackPawn));
        pos.set_en_passant_square(Bitboard::C6);
        pos.set_side_to_move(SideToMove::White);
        pos.set_move_count(2);
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
        assert_eq!(fen, Fen::from_position(&pos));
        assert_eq!(
            pos,
            Fen::to_position(&fen),
            "\nExpected Position as FEN: {}\nActual Position as FEN:   {}\n",
            fen,
            Fen::from_position(&Fen::to_position(&fen))
        );

        // Position after 1. e4 c5 2. Nf3
        pos.set_piece_at(Bitboard::IDX_G1, None);
        pos.set_piece_at(Bitboard::IDX_F3, Some(Piece::WhiteKnight));
        pos.set_en_passant_square(Bitboard::EMPTY);
        pos.set_side_to_move(SideToMove::Black);
        pos.set_plies_since_pawn_move_or_capture(1);
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        assert_eq!(fen, Fen::from_position(&pos));
        assert_eq!(
            pos,
            Fen::to_position(&fen),
            "\nExpected Position as FEN: {}\nActual Position as FEN:   {}\n",
            fen,
            Fen::from_position(&Fen::to_position(&fen))
        );

        // Check castling rights
        pos.set_castling_rights(CastlingRights::WHITE_QUEENSIDE | CastlingRights::BLACK_KINGSIDE);
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b Qk - 1 2";
        assert_eq!(fen, Fen::from_position(&pos));
        pos.set_castling_rights(CastlingRights::empty());
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b - - 1 2";
        assert_eq!(fen, Fen::from_position(&pos));
        assert_eq!(
            pos,
            Fen::to_position(&fen),
            "\nExpected Position as FEN: {}\nActual Position as FEN:   {}\n",
            fen,
            Fen::from_position(&Fen::to_position(&fen))
        );
    }
}
