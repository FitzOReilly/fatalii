use movegen::move_generator::MoveGenerator;
use movegen::piece;
use movegen::position::Position;
use movegen::r#move::{Move, MoveList};
use regex::Regex;
use std::str;

pub struct UciMove;

impl UciMove {
    pub fn move_to_str(m: Move) -> String {
        match m {
            Move::NULL => String::from("0000"),
            _ => {
                let mut s = String::new();
                s.push_str(str::from_utf8(&m.origin().to_ascii()).unwrap());
                s.push_str(str::from_utf8(&m.target().to_ascii()).unwrap());
                match m.promotion_piece() {
                    Some(piece::Type::Knight) => s.push('n'),
                    Some(piece::Type::Bishop) => s.push('b'),
                    Some(piece::Type::Rook) => s.push('r'),
                    Some(piece::Type::Queen) => s.push('q'),
                    None => {}
                    Some(p) => panic!("Invalid promotion piece `{:?}` in move `{}`", p, m),
                }
                s
            }
        }
    }

    pub fn str_to_move(pos: &Position, move_str: &str) -> Option<Move> {
        if move_str == "0000" {
            return Some(Move::NULL);
        }

        let re_uci_move = Regex::new("([a-h][1-8]){2}[nbrq]?").unwrap();
        if !re_uci_move.is_match(move_str) {
            return None;
        }

        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves(&mut move_list, pos);

        move_list
            .iter()
            .find(|&&m| UciMove::move_to_str(m) == move_str)
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use movegen::fen::Fen;
    use movegen::r#move::MoveType;
    use movegen::square::Square;

    #[test]
    fn move_to_str() {
        assert_eq!("0000", UciMove::move_to_str(Move::NULL));

        let m = Move::new(Square::E2, Square::E3, MoveType::QUIET);
        assert_eq!("e2e3", UciMove::move_to_str(m));

        let m = Move::new(Square::E2, Square::E4, MoveType::DOUBLE_PAWN_PUSH);
        assert_eq!("e2e4", UciMove::move_to_str(m));

        let m = Move::new(Square::E1, Square::G1, MoveType::CASTLE_KINGSIDE);
        assert_eq!("e1g1", UciMove::move_to_str(m));

        let m = Move::new(Square::E8, Square::C8, MoveType::CASTLE_QUEENSIDE);
        assert_eq!("e8c8", UciMove::move_to_str(m));

        let m = Move::new(Square::C4, Square::D5, MoveType::CAPTURE);
        assert_eq!("c4d5", UciMove::move_to_str(m));

        let m = Move::new(Square::D5, Square::E6, MoveType::EN_PASSANT_CAPTURE);
        assert_eq!("d5e6", UciMove::move_to_str(m));

        let m = Move::new(Square::A7, Square::A8, MoveType::PROMOTION_KNIGHT);
        assert_eq!("a7a8n", UciMove::move_to_str(m));

        let m = Move::new(Square::A7, Square::A8, MoveType::PROMOTION_BISHOP);
        assert_eq!("a7a8b", UciMove::move_to_str(m));

        let m = Move::new(Square::A7, Square::A8, MoveType::PROMOTION_ROOK);
        assert_eq!("a7a8r", UciMove::move_to_str(m));

        let m = Move::new(Square::A7, Square::A8, MoveType::PROMOTION_QUEEN);
        assert_eq!("a7a8q", UciMove::move_to_str(m));

        let m = Move::new(Square::G2, Square::H1, MoveType::PROMOTION_CAPTURE_KNIGHT);
        assert_eq!("g2h1n", UciMove::move_to_str(m));

        let m = Move::new(Square::G2, Square::H1, MoveType::PROMOTION_CAPTURE_BISHOP);
        assert_eq!("g2h1b", UciMove::move_to_str(m));

        let m = Move::new(Square::G2, Square::H1, MoveType::PROMOTION_CAPTURE_ROOK);
        assert_eq!("g2h1r", UciMove::move_to_str(m));

        let m = Move::new(Square::G2, Square::H1, MoveType::PROMOTION_CAPTURE_QUEEN);
        assert_eq!("g2h1q", UciMove::move_to_str(m));
    }

    #[test]
    fn str_to_move() {
        // Position from https://www.chessprogramming.org/Perft_Results
        let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let pos = Fen::str_to_pos(fen).unwrap();

        assert_eq!(Some(Move::NULL), UciMove::str_to_move(&pos, "0000"));
        assert_eq!(None, UciMove::str_to_move(&pos, "1e1c"));
        assert_eq!(None, UciMove::str_to_move(&pos, "e1c1"));
        assert_eq!(
            Some(Move::new(Square::E1, Square::G1, MoveType::CASTLE_KINGSIDE)),
            UciMove::str_to_move(&pos, "e1g1")
        );
    }

    #[test]
    fn move_to_str_to_move_roundtrip() {
        // Position from https://www.chessprogramming.org/Perft_Results
        let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let pos = Fen::str_to_pos(fen).unwrap();

        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves(&mut move_list, &pos);

        for &m in move_list.iter() {
            assert_eq!(
                m,
                UciMove::str_to_move(&pos, &UciMove::move_to_str(m)).unwrap()
            );
        }
    }
}
