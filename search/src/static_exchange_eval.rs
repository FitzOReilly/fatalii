use eval::Score;
use movegen::{
    bishop::Bishop, bitboard::Bitboard, king::King, knight::Knight, pawn::Pawn, piece,
    position::Position, r#move::Move, rook::Rook, side::Side, square::Square,
};

pub fn static_exchange_eval(pos: &Position, m: Move, threshold: Score) -> bool {
    let see_square = m.target();
    let mut balance = -threshold;
    balance += gained_material_value(pos, m);

    // We fail to beat the threshold and the opponent hasn't even recaptured yet
    if balance < 0 {
        return false;
    }

    let current_value = moved_material_value(pos, m);

    // Even if we lose the moved piece, we still beat the threshold
    balance -= current_value;
    if balance >= 0 {
        return true;
    }

    // The occupancy after the move. If the move is castle, we don't need to
    // update the rook position because there won't be any recaptures possible
    // anyway.
    let mut occupied = pos.occupancy() & !Bitboard::from_square(m.origin());
    if m.is_en_passant() {
        occupied &= !pos.en_passant_square();
    }
    let diagonal_sliders = pos.piece_type_occupancy(piece::Type::Bishop)
        | pos.piece_type_occupancy(piece::Type::Queen);
    let line_sliders =
        pos.piece_type_occupancy(piece::Type::Rook) | pos.piece_type_occupancy(piece::Type::Queen);
    let mut attackers = initial_attackers(pos, see_square, occupied);
    let mut side_to_move = !pos.side_to_move();

    loop {
        if attackers & pos.side_occupancy(side_to_move) == Bitboard::EMPTY {
            break;
        }

        let mut attacker_type = piece::Type::Pawn;
        let mut attacker_origin = Square::A1;
        for piece_type in [
            piece::Type::Pawn,
            piece::Type::Knight,
            piece::Type::Bishop,
            piece::Type::Rook,
            piece::Type::Queen,
            piece::Type::King,
        ] {
            let piece_type_attackers = attackers & pos.piece_occupancy(side_to_move, piece_type);
            if piece_type_attackers != Bitboard::EMPTY {
                attacker_type = piece_type;
                attacker_origin = piece_type_attackers.square_scan_forward();
                break;
            }
        }

        occupied &= !Bitboard::from_square(attacker_origin);
        // Reveal diagonal sliders
        if let piece::Type::Pawn | piece::Type::Bishop | piece::Type::Queen = attacker_type {
            attackers |= Bishop::targets(see_square, occupied) & diagonal_sliders;
        }
        // Reveal line sliders
        if let piece::Type::Rook | piece::Type::Queen = attacker_type {
            attackers |= Rook::targets(see_square, occupied) & line_sliders;
        }
        attackers &= occupied;
        side_to_move = !side_to_move;
        balance = -balance - 1 - piece_type_value(attacker_type);
        if balance >= 0 {
            break;
        }
    }
    // The side to move has a negative balance and cannot recapture, so it loses
    side_to_move != pos.side_to_move()
}

pub fn gained_material_value(pos: &Position, m: Move) -> Score {
    if m.is_en_passant() {
        return piece_type_value(piece::Type::Pawn);
    }
    let promo_value = match m.promotion_piece() {
        Some(piece_type) => piece_type_value(piece_type) - piece_type_value(piece::Type::Pawn),
        None => 0,
    };
    if m.is_capture() {
        let captured_type = pos
            .piece_at(m.target())
            .expect("Expected target square to be occupied")
            .piece_type();
        return piece_type_value(captured_type) + promo_value;
    }
    promo_value
}

fn moved_material_value(pos: &Position, m: Move) -> i16 {
    let piece_type = match m.promotion_piece() {
        Some(promo_piece) => promo_piece,
        None => pos
            .piece_at(m.origin())
            .expect("Expected origin square to be occupied")
            .piece_type(),
    };
    piece_type_value(piece_type)
}

fn initial_attackers(pos: &Position, see_square: Square, occupied: Bitboard) -> Bitboard {
    let mut attackers = Bitboard::EMPTY;
    let white_pawn_attackers = Pawn::attack_origins(
        Bitboard::from_square(see_square) & !Bitboard::RANK_1,
        Side::White,
    ) & pos.piece_occupancy(Side::White, piece::Type::Pawn);
    attackers |= white_pawn_attackers;
    let black_pawn_attackers = Pawn::attack_origins(
        Bitboard::from_square(see_square) & !Bitboard::RANK_8,
        Side::Black,
    ) & pos.piece_occupancy(Side::Black, piece::Type::Pawn);
    attackers |= black_pawn_attackers;
    let knight_attackers =
        Knight::targets(see_square) & pos.piece_type_occupancy(piece::Type::Knight);
    attackers |= knight_attackers;
    let king_attackers = King::targets(see_square) & pos.piece_type_occupancy(piece::Type::King);
    attackers |= king_attackers;
    let diagonal_slider_attackers = Bishop::targets(see_square, occupied)
        & (pos.piece_type_occupancy(piece::Type::Bishop)
            | pos.piece_type_occupancy(piece::Type::Queen));
    attackers |= diagonal_slider_attackers;
    let line_slider_attackers = Rook::targets(see_square, occupied)
        & (pos.piece_type_occupancy(piece::Type::Rook)
            | pos.piece_type_occupancy(piece::Type::Queen));
    attackers |= line_slider_attackers;
    attackers & occupied
}

pub fn piece_type_value(t: piece::Type) -> Score {
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

    // SEE test suite, taken from Carp
    #[test]
    fn see() {
        for (fen, m, threshold, expected) in [
            (
                "1k1r4/1pp4p/p7/4p3/8/P5P1/1PP4P/2K1R3 w - - 0 1",
                Move::new(Square::E1, Square::E5, MoveType::CAPTURE),
                0,
                true,
            ),
            (
                "1k1r3q/1ppn3p/p4b2/4p3/8/P2N2P1/1PP1R1BP/2K1Q3 w - - 0 1",
                Move::new(Square::D3, Square::E5, MoveType::CAPTURE),
                0,
                false,
            ),
            (
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
                Move::new(Square::G2, Square::H3, MoveType::CAPTURE),
                0,
                true,
            ),
            (
                "k3r3/8/8/4p3/8/2B5/1B6/K7 w - - 0 1",
                Move::new(Square::C3, Square::E5, MoveType::CAPTURE),
                0,
                true,
            ),
            (
                "4kbnr/p1P4p/b1q5/5pP1/4n3/5Q2/PP1PPP1P/RNB1KBNR w KQk f6 0 1",
                Move::new(Square::G5, Square::F6, MoveType::EN_PASSANT_CAPTURE),
                0,
                true,
            ),
            (
                "6k1/1pp4p/p1pb4/6q1/3P1pRr/2P4P/PP1Br1P1/5RKN w - - 0 1",
                Move::new(Square::F1, Square::F4, MoveType::CAPTURE),
                0,
                false,
            ),
            (
                "6RR/4bP2/8/8/5r2/3K4/5p2/4k3 w - - 0 1",
                Move::new(Square::F7, Square::F8, MoveType::PROMOTION_QUEEN),
                0,
                true,
            ),
            (
                "r1bqk1nr/pppp1ppp/2n5/1B2p3/1b2P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1",
                Move::new(Square::E1, Square::G1, MoveType::CASTLE_KINGSIDE),
                0,
                true,
            ),
            (
                "4kbnr/p1P1pppp/b7/4q3/7n/8/PPQPPPPP/RNB1KBNR w KQk - 0 1",
                Move::new(Square::C7, Square::C8, MoveType::PROMOTION_QUEEN),
                0,
                true,
            ),
            (
                "4kbnr/p1P1pppp/b7/4q3/7n/8/PP1PPPPP/RNBQKBNR w KQk - 0 1",
                Move::new(Square::C7, Square::C8, MoveType::PROMOTION_QUEEN),
                0,
                false,
            ),
            (
                "3r3k/3r4/2n1n3/8/3p4/2PR4/1B1Q4/3R3K w - - 0 1",
                Move::new(Square::D3, Square::D4, MoveType::CAPTURE),
                0,
                false,
            ),
            (
                "5rk1/1pp2q1p/p1pb4/8/3P1NP1/2P5/1P1BQ1P1/5RK1 b - - 0 1",
                Move::new(Square::D6, Square::F4, MoveType::CAPTURE),
                0,
                true,
            ),
            (
                "5rk1/1pp2q1p/p1pb4/8/3P1NP1/2P5/1P1BQ1P1/5RK1 b - - 0 1",
                Move::new(Square::D6, Square::F4, MoveType::CAPTURE),
                -100,
                true,
            ),
        ] {
            let pos = Fen::str_to_pos(fen).unwrap();
            let actual = static_exchange_eval(&pos, m, threshold);
            assert_eq!(expected, actual);
        }
    }
}
