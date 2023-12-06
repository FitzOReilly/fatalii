use eval::Score;
use movegen::{
    move_generator::MoveGenerator, piece, position_history::PositionHistory, r#move::Move,
    square::Square,
};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum CaptureType {
    Winning,
    Equal,
    Losing,
}

fn static_exchange_eval(
    pos_hist: &mut PositionHistory,
    target: Square,
    value_already_exchanged: Score,
) -> Score {
    let mut target_values = Vec::new();
    let mut value_from_start = value_already_exchanged;
    let mut value_to_end = 0;
    while let Some(m) = least_valuable_attacker(pos_hist, target) {
        debug_assert!(m.is_capture() && !m.is_en_passant());
        let victim_value = piece_type_value(
            pos_hist
                .current_pos()
                .piece_at(target)
                .expect("Expected target square to be occupied")
                .piece_type(),
        );
        let attacker_value = piece_type_value(
            pos_hist
                .current_pos()
                .piece_at(m.origin())
                .expect("Expected origin square to be occupied")
                .piece_type(),
        );
        value_from_start = victim_value - value_from_start;
        if value_from_start > attacker_value {
            value_to_end = victim_value;
            break;
        }
        target_values.push(victim_value);
        pos_hist.do_move(m);
    }
    while let Some(target_value) = target_values.pop() {
        value_to_end = (target_value - value_to_end).max(0);
        pos_hist.undo_last_move();
    }
    value_to_end
}

pub fn see_capture(pos_hist: &mut PositionHistory, m: Move) -> CaptureType {
    debug_assert!(m.is_capture());
    let victim_value = piece_type_value(if m.is_en_passant() {
        piece::Type::Pawn
    } else {
        pos_hist
            .current_pos()
            .piece_at(m.target())
            .expect("Expected target square to be occupied")
            .piece_type()
    });
    let attacker_value = piece_type_value(
        pos_hist
            .current_pos()
            .piece_at(m.origin())
            .expect("Expected origin square to be occupied")
            .piece_type(),
    );
    if victim_value > attacker_value {
        return see_value_to_capture_type(victim_value - attacker_value);
    }
    pos_hist.do_move(m);
    let value = victim_value - static_exchange_eval(pos_hist, m.target(), victim_value);
    pos_hist.undo_last_move();
    see_value_to_capture_type(value)
}

fn see_value_to_capture_type(see: Score) -> CaptureType {
    match see {
        _ if see > 0 => CaptureType::Winning,
        _ if see == 0 => CaptureType::Equal,
        _ if see < 0 => CaptureType::Losing,
        _ => unreachable!(),
    }
}

fn least_valuable_attacker(pos_hist: &mut PositionHistory, target: Square) -> Option<Move> {
    MoveGenerator::least_valuable_attacker(pos_hist.current_pos(), target)
}

pub fn piece_type_value(t: piece::Type) -> i16 {
    match t {
        piece::Type::Pawn => 100,
        piece::Type::Knight => 300,
        piece::Type::Bishop => 300,
        piece::Type::Rook => 500,
        piece::Type::Queen => 900,
        piece::Type::King => 20000,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use movegen::{fen::Fen, r#move::MoveType};

    #[test]
    fn see() {
        for (fen, m, expected) in [
            (
                "1k1r4/1pp4p/p7/4p3/8/P5P1/1PP4P/2K1R3 w - - 0 1",
                Move::new(Square::E1, Square::E5, MoveType::CAPTURE),
                CaptureType::Winning,
            ),
            (
                "1k1r3q/1ppn3p/p4b2/4p3/8/P2N2P1/1PP1R1BP/2K1Q3 w - - 0 1",
                Move::new(Square::D3, Square::E5, MoveType::CAPTURE),
                CaptureType::Losing,
            ),
        ] {
            let pos = Fen::str_to_pos(fen).unwrap();
            let mut pos_hist = PositionHistory::new(pos);
            let actual = see_capture(&mut pos_hist, m);
            assert_eq!(expected, actual);
        }
    }
}
