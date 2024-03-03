use eval::Score;
use movegen::{
    bishop::Bishop, bitboard::Bitboard, king::King, knight::Knight, move_generator::MoveGenerator,
    pawn::Pawn, piece, position_history::PositionHistory, r#move::Move, rook::Rook, side::Side,
    square::Square,
};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum CaptureType {
    Winning,
    Equal,
    Losing,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Stage {
    Pawns,
    Knights,
    Sliders,
    King,
}

impl Iterator for Stage {
    type Item = Stage;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Stage::Pawns => {
                *self = Stage::Knights;
                Some(*self)
            }
            Stage::Knights => {
                *self = Stage::Sliders;
                Some(*self)
            }
            Stage::Sliders => {
                *self = Stage::King;
                Some(*self)
            }
            Stage::King => None,
        }
    }
}

fn static_exchange_eval(
    pos_hist: &mut PositionHistory,
    target: Square,
    value_already_exchanged: Score,
) -> Score {
    let mut target_values = Vec::new();
    let mut value_from_start = value_already_exchanged;
    let mut value_to_end = 0;
    let pos = pos_hist.current_pos();
    let mut side_occupancies = [
        pos.side_occupancy(Side::White),
        pos.side_occupancy(Side::Black),
    ];
    let mut side_to_move = pos.side_to_move();
    let target_piece_type = pos
        .piece_at(target)
        .expect("Expected target square to be occupied")
        .piece_type();
    let mut target_piece_value = piece_type_value(target_piece_type);

    let mut stages = [Stage::Pawns, Stage::Pawns];
    let mut pawn_attackers = [
        Pawn::attack_origins(
            Bitboard::from_square(target) & !Bitboard::RANK_1,
            Side::White,
        ) & pos.piece_occupancy(Side::White, piece::Type::Pawn),
        Pawn::attack_origins(
            Bitboard::from_square(target) & !Bitboard::RANK_8,
            Side::Black,
        ) & pos.piece_occupancy(Side::Black, piece::Type::Pawn),
    ];
    let mut knight_attackers = {
        let knight_targets = Knight::targets(target);
        [
            knight_targets & pos.piece_occupancy(Side::White, piece::Type::Knight),
            knight_targets & pos.piece_occupancy(Side::Black, piece::Type::Knight),
        ]
    };
    let mut bishops = [
        pos.piece_occupancy(Side::White, piece::Type::Bishop),
        pos.piece_occupancy(Side::Black, piece::Type::Bishop),
    ];
    let mut rooks = [
        pos.piece_occupancy(Side::White, piece::Type::Rook),
        pos.piece_occupancy(Side::Black, piece::Type::Rook),
    ];
    let mut queens = [
        pos.piece_occupancy(Side::White, piece::Type::Queen),
        pos.piece_occupancy(Side::Black, piece::Type::Queen),
    ];
    let mut king_attacker = {
        let king_targets = King::targets(target);
        [
            king_targets & pos.piece_occupancy(Side::White, piece::Type::Knight),
            king_targets & pos.piece_occupancy(Side::Black, piece::Type::Knight),
        ]
    };

    loop {
        match stages[side_to_move as usize] {
            Stage::Pawns => {
                if pawn_attackers[side_to_move as usize] != Bitboard::EMPTY {
                    let next_attacker_origin =
                        pawn_attackers[side_to_move as usize].square_scan_forward_reset();
                    let attacker_value = piece_type_value(piece::Type::Pawn);
                    update_target(
                        &mut target_values,
                        &mut target_piece_value,
                        &mut value_from_start,
                        &mut side_occupancies,
                        &mut side_to_move,
                        next_attacker_origin,
                        attacker_value,
                    );
                    if value_from_start > attacker_value {
                        break;
                    }
                } else {
                    stages[side_to_move as usize].next();
                }
            }
            Stage::Knights => {
                if knight_attackers[side_to_move as usize] != Bitboard::EMPTY {
                    let next_attacker_origin =
                        knight_attackers[side_to_move as usize].square_scan_forward_reset();
                    let attacker_value = piece_type_value(piece::Type::Knight);
                    update_target(
                        &mut target_values,
                        &mut target_piece_value,
                        &mut value_from_start,
                        &mut side_occupancies,
                        &mut side_to_move,
                        next_attacker_origin,
                        attacker_value,
                    );
                    if value_from_start > attacker_value {
                        break;
                    }
                } else {
                    stages[side_to_move as usize].next();
                }
            }
            Stage::Sliders => {
                // Bishops
                let potential_diagonal_attackers =
                    Bishop::targets(target, side_occupancies[side_to_move as usize]);
                let bishop_attackers =
                    potential_diagonal_attackers & bishops[side_to_move as usize];
                if bishop_attackers != Bitboard::EMPTY {
                    let next_attacker_origin = bishop_attackers.square_scan_forward();
                    // TODO probably not necessary
                    bishops[side_to_move as usize] &= !Bitboard::from_square(next_attacker_origin);
                    let attacker_value = piece_type_value(piece::Type::Bishop);
                    update_target(
                        &mut target_values,
                        &mut target_piece_value,
                        &mut value_from_start,
                        &mut side_occupancies,
                        &mut side_to_move,
                        next_attacker_origin,
                        attacker_value,
                    );
                    if value_from_start > attacker_value {
                        break;
                    }
                    continue;
                }

                // Rooks
                let potential_line_attackers =
                    Rook::targets(target, side_occupancies[side_to_move as usize]);
                let rook_attackers = potential_line_attackers & rooks[side_to_move as usize];
                if rook_attackers != Bitboard::EMPTY {
                    let next_attacker_origin = rook_attackers.square_scan_forward();
                    // TODO probably not necessary
                    rooks[side_to_move as usize] &= !Bitboard::from_square(next_attacker_origin);
                    let attacker_value = piece_type_value(piece::Type::Rook);
                    update_target(
                        &mut target_values,
                        &mut target_piece_value,
                        &mut value_from_start,
                        &mut side_occupancies,
                        &mut side_to_move,
                        next_attacker_origin,
                        attacker_value,
                    );
                    if value_from_start > attacker_value {
                        break;
                    }
                    continue;
                }

                // Queens (diagonal)
                let queen_attackers = potential_diagonal_attackers & queens[side_to_move as usize];
                if queen_attackers != Bitboard::EMPTY {
                    let next_attacker_origin = queen_attackers.square_scan_forward();
                    // TODO probably not necessary
                    queens[side_to_move as usize] &= !Bitboard::from_square(next_attacker_origin);
                    let attacker_value = piece_type_value(piece::Type::Queen);
                    update_target(
                        &mut target_values,
                        &mut target_piece_value,
                        &mut value_from_start,
                        &mut side_occupancies,
                        &mut side_to_move,
                        next_attacker_origin,
                        attacker_value,
                    );
                    if value_from_start > attacker_value {
                        break;
                    }
                    continue;
                }

                // Queens (lines)
                let queen_attackers = potential_line_attackers & queens[side_to_move as usize];
                if queen_attackers != Bitboard::EMPTY {
                    let next_attacker_origin = queen_attackers.square_scan_forward();
                    // TODO probably not necessary
                    queens[side_to_move as usize] &= !Bitboard::from_square(next_attacker_origin);
                    let attacker_value = piece_type_value(piece::Type::Queen);
                    update_target(
                        &mut target_values,
                        &mut target_piece_value,
                        &mut value_from_start,
                        &mut side_occupancies,
                        &mut side_to_move,
                        next_attacker_origin,
                        attacker_value,
                    );
                    if value_from_start > attacker_value {
                        break;
                    }
                    continue;
                }

                stages[side_to_move as usize].next();
            }
            Stage::King => {
                if king_attacker[side_to_move as usize] != Bitboard::EMPTY
                    && pawn_attackers[!side_to_move as usize] == Bitboard::EMPTY
                    && knight_attackers[!side_to_move as usize] == Bitboard::EMPTY
                    && king_attacker[!side_to_move as usize] == Bitboard::EMPTY
                    && Bishop::targets(target, side_occupancies[!side_to_move as usize])
                        & (bishops[!side_to_move as usize] | queens[!side_to_move as usize])
                        == Bitboard::EMPTY
                    && Rook::targets(target, side_occupancies[!side_to_move as usize])
                        & (rooks[!side_to_move as usize] | queens[!side_to_move as usize])
                        == Bitboard::EMPTY
                {
                    let next_attacker_origin =
                        king_attacker[side_to_move as usize].square_scan_forward_reset();
                    let attacker_value = piece_type_value(piece::Type::King);
                    update_target(
                        &mut target_values,
                        &mut target_piece_value,
                        &mut value_from_start,
                        &mut side_occupancies,
                        &mut side_to_move,
                        next_attacker_origin,
                        attacker_value,
                    );
                }
                break;
            }
        }
    }

    // todo!(
    //     "
    // - Abort early
    // - Promotions
    // - Legality check
    // "
    // );

    while let Some(target_value) = target_values.pop() {
        value_to_end = (target_value - value_to_end).max(0);
    }
    value_to_end
}

fn update_target(
    target_values: &mut Vec<Score>,
    target_piece_value: &mut Score,
    value_from_start: &mut Score,
    side_occupancies: &mut [Bitboard; 2],
    side_to_move: &mut Side,
    next_attacker_origin: Square,
    next_attacker_value: Score,
) {
    let victim_value = *target_piece_value;
    target_values.push(victim_value);
    *value_from_start = victim_value - *value_from_start;
    // Remove attacker from its origin square and store its type for
    // the target square
    side_occupancies[*side_to_move as usize] &= !Bitboard::from_square(next_attacker_origin);
    *target_piece_value = next_attacker_value;
    *side_to_move = !*side_to_move;
}

fn _static_exchange_eval_old(
    pos_hist: &mut PositionHistory,
    target: Square,
    value_already_exchanged: Score,
) -> Score {
    let mut target_values = Vec::new();
    let mut value_from_start = value_already_exchanged;
    let mut value_to_end = 0;
    while let Some(m) = _least_valuable_attacker(pos_hist, target) {
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

fn _least_valuable_attacker(pos_hist: &mut PositionHistory, target: Square) -> Option<Move> {
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
