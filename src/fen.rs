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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_position() {
        let mut pos = Position::initial();
        let expected_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(expected_str, Fen::from_position(&pos));

        // Position after 1. e4
        pos.set_piece_at(Bitboard::IDX_E2, None);
        pos.set_piece_at(Bitboard::IDX_E4, Some(Piece::WhitePawn));
        pos.set_en_passant_square(Bitboard::E3);
        pos.set_side_to_move(SideToMove::Black);
        let expected_str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        assert_eq!(expected_str, Fen::from_position(&pos));

        // Position after 1. e4 c5
        pos.set_piece_at(Bitboard::IDX_C7, None);
        pos.set_piece_at(Bitboard::IDX_C5, Some(Piece::BlackPawn));
        pos.set_en_passant_square(Bitboard::C6);
        pos.set_side_to_move(SideToMove::White);
        pos.set_move_count(2);
        let expected_str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
        assert_eq!(expected_str, Fen::from_position(&pos));

        // Position after 1. e4 c5 2. Nf3
        pos.set_piece_at(Bitboard::IDX_G1, None);
        pos.set_piece_at(Bitboard::IDX_F3, Some(Piece::WhiteKnight));
        pos.set_en_passant_square(Bitboard::EMPTY);
        pos.set_side_to_move(SideToMove::Black);
        pos.set_plies_since_pawn_move_or_capture(1);
        let expected_str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        assert_eq!(expected_str, Fen::from_position(&pos));

        // Check castling rights
        pos.set_castling_rights(CastlingRights::WHITE_QUEENSIDE | CastlingRights::BLACK_KINGSIDE);
        let expected_str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b Qk - 1 2";
        assert_eq!(expected_str, Fen::from_position(&pos));
        pos.set_castling_rights(CastlingRights::empty());
        let expected_str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b - - 1 2";
        assert_eq!(expected_str, Fen::from_position(&pos));
    }
}
