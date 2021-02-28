use crate::bitboard::Bitboard;
use crate::pawn::Pawn;
use crate::piece;
use crate::position::{CastlingRights, Position};
use crate::r#move::{Move, MoveType};
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct IrreversibleProperties {
    en_passant_square: Bitboard,
    castling_rights: CastlingRights,
    plies_since_pawn_move_or_capture: usize,
    captured_piece: Option<piece::Piece>,
}

impl IrreversibleProperties {
    fn new(
        en_passant_square: Bitboard,
        castling_rights: CastlingRights,
        plies_since_pawn_move_or_capture: usize,
        captured_piece: Option<piece::Piece>,
    ) -> IrreversibleProperties {
        IrreversibleProperties {
            en_passant_square,
            castling_rights,
            plies_since_pawn_move_or_capture,
            captured_piece,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositionHistory {
    pos: Position,
    irreversible_properties: Vec<IrreversibleProperties>,
    moves: Vec<Move>,
}

impl PositionHistory {
    pub fn new(pos: Position) -> PositionHistory {
        PositionHistory {
            pos: pos,
            irreversible_properties: Vec::<IrreversibleProperties>::new(),
            moves: Vec::<Move>::new(),
        }
    }

    pub fn current_pos(&self) -> &Position {
        &self.pos
    }

    pub fn do_move(&mut self, m: Move) {
        debug_assert_eq!(self.irreversible_properties.len(), self.moves.len());
        self.moves.push(m);
        let origin = m.origin();
        let target = m.target();
        let moving_piece = self.pos.piece_at(origin).unwrap();
        let side_to_move = self.pos.side_to_move();

        let capture_square = if m.is_en_passant() {
            Pawn::push_origin(target, side_to_move)
        } else {
            target
        };
        let captured_piece = self.pos.piece_at(capture_square);

        self.irreversible_properties
            .push(IrreversibleProperties::new(
                self.pos.en_passant_square(),
                self.pos.castling_rights(),
                self.pos.plies_since_pawn_move_or_capture(),
                captured_piece,
            ));

        let target_piece = if m.is_promotion() {
            piece::Piece::new(side_to_move, m.move_type().promo_piece_unchecked())
        } else {
            moving_piece
        };

        self.pos.set_piece_at(target, Some(target_piece));
        self.pos.set_piece_at(origin, None);

        let en_passant_square = match m.move_type() {
            MoveType::DOUBLE_PAWN_PUSH => {
                let en_passant_square = Pawn::push_origin(target, side_to_move);
                Bitboard::from_square(en_passant_square)
            }
            _ => Bitboard::EMPTY,
        };
        self.pos.set_en_passant_square(en_passant_square);

        if m.is_capture() {
            if m.is_en_passant() {
                let captured_idx = Pawn::push_origin(target, side_to_move);
                self.pos.set_piece_at(captured_idx, None);
            }
            self.pos.remove_castling_rights(target);
        }

        if m.is_capture() || moving_piece.piece_type() == piece::Type::Pawn {
            self.pos.set_plies_since_pawn_move_or_capture(0);
        } else {
            self.pos.set_plies_since_pawn_move_or_capture(
                self.pos.plies_since_pawn_move_or_capture() + 1,
            );
        }

        self.pos.remove_castling_rights(origin);

        let move_count = self.pos.move_count();
        self.pos.set_move_count(move_count + side_to_move as usize);
        self.pos.set_side_to_move(!side_to_move);
        debug_assert_eq!(self.irreversible_properties.len(), self.moves.len());
    }

    pub fn undo_last_move(&mut self) {
        debug_assert_eq!(self.irreversible_properties.len(), self.moves.len());
        match self.moves.pop() {
            Some(m) => self.undo_move(m),
            None => {}
        }
        debug_assert_eq!(self.irreversible_properties.len(), self.moves.len());
    }

    fn undo_move(&mut self, m: Move) {
        let origin = m.origin();
        let target = m.target();
        let moving_piece = self.pos.piece_at(target).unwrap();

        let origin_piece = if m.is_promotion() {
            piece::Piece::new(moving_piece.piece_side(), piece::Type::Pawn)
        } else {
            moving_piece
        };

        self.pos.set_piece_at(origin, Some(origin_piece));
        self.pos.set_piece_at(target, None);

        self.pos.set_side_to_move(!self.pos.side_to_move());
        self.pos
            .set_move_count(self.pos.move_count() - self.pos.side_to_move() as usize);

        debug_assert!(!self.irreversible_properties.is_empty());
        let irr = self.irreversible_properties.pop().unwrap();
        self.pos.set_en_passant_square(irr.en_passant_square);
        self.pos.set_castling_rights(irr.castling_rights);
        self.pos
            .set_plies_since_pawn_move_or_capture(irr.plies_since_pawn_move_or_capture);

        if m.is_capture() {
            let capture_square = if m.is_en_passant() {
                Pawn::push_origin(target, self.pos.side_to_move())
            } else {
                target
            };
            self.pos.set_piece_at(capture_square, irr.captured_piece);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::side::Side;

    #[test]
    fn do_and_undo_move_initial_position() {
        let pos = Position::initial();

        let mut exp_pos_history = Vec::new();
        let mut move_history = Vec::new();
        let mut pos_history = PositionHistory::new(pos);

        // Position after 1. e4
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(Square::E2, Square::E4, MoveType::DOUBLE_PAWN_PUSH);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Bitboard::E4,
            pos_history.current_pos().side_occupancy(Side::White) & (Bitboard::E2 | Bitboard::E4)
        );
        assert_eq!(Bitboard::E3, pos_history.current_pos().en_passant_square());
        assert_eq!(Side::Black, pos_history.current_pos().side_to_move());
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            pos_history.current_pos().castling_rights()
        );
        assert_eq!(
            0,
            pos_history.current_pos().plies_since_pawn_move_or_capture()
        );
        assert_eq!(1, pos_history.current_pos().move_count());

        // Position after 1. e4 c5
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(Square::C7, Square::C5, MoveType::DOUBLE_PAWN_PUSH);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Bitboard::C5,
            pos_history.current_pos().side_occupancy(Side::Black) & (Bitboard::C7 | Bitboard::C5)
        );
        assert_eq!(Bitboard::C6, pos_history.current_pos().en_passant_square());
        assert_eq!(Side::White, pos_history.current_pos().side_to_move());
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            pos_history.current_pos().castling_rights()
        );
        assert_eq!(
            0,
            pos_history.current_pos().plies_since_pawn_move_or_capture()
        );
        assert_eq!(2, pos_history.current_pos().move_count());

        // Position after 1. e4 c5 2. Nf3
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(Square::G1, Square::F3, MoveType::QUIET);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Bitboard::F3,
            pos_history.current_pos().side_occupancy(Side::White) & (Bitboard::G1 | Bitboard::F3)
        );
        assert_eq!(
            Bitboard::EMPTY,
            pos_history.current_pos().en_passant_square()
        );
        assert_eq!(Side::Black, pos_history.current_pos().side_to_move());
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            pos_history.current_pos().castling_rights()
        );
        assert_eq!(
            1,
            pos_history.current_pos().plies_since_pawn_move_or_capture()
        );
        assert_eq!(2, pos_history.current_pos().move_count());

        // Position after 1. e4 c5 2. Nf3 d6
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(Square::D7, Square::D6, MoveType::QUIET);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Bitboard::D6,
            pos_history.current_pos().side_occupancy(Side::Black) & (Bitboard::D7 | Bitboard::D6)
        );
        assert_eq!(
            Bitboard::EMPTY,
            pos_history.current_pos().en_passant_square()
        );
        assert_eq!(Side::White, pos_history.current_pos().side_to_move());
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            pos_history.current_pos().castling_rights()
        );
        assert_eq!(
            0,
            pos_history.current_pos().plies_since_pawn_move_or_capture()
        );
        assert_eq!(3, pos_history.current_pos().move_count());

        // Position after 1. e4 c5 2. Nf3
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Position after 1. e4 c5
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Position after 1. e4
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Initial position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
    }

    #[test]
    fn do_and_undo_move_castling_rights() {
        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::A1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Square::H1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Square::B7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::G7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::A8, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Square::H8, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Square::B2, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::G2, Some(piece::Piece::BLACK_PAWN));
        pos.set_castling_rights(CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH);

        let mut exp_pos_history = Vec::new();
        let mut move_history = Vec::new();
        let mut pos_history = PositionHistory::new(pos);

        // Position after 1. 0-0
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(Square::E1, Square::G1, MoveType::CASTLE_KINGSIDE);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Bitboard::G1,
            pos_history.current_pos().side_occupancy(Side::White) & (Bitboard::E1 | Bitboard::G1)
        );
        assert_eq!(
            CastlingRights::BLACK_BOTH,
            pos_history.current_pos().castling_rights()
        );

        // Position after 1. 0-0 Ke7
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(Square::E8, Square::E7, MoveType::QUIET);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Bitboard::E7,
            pos_history.current_pos().side_occupancy(Side::Black) & (Bitboard::E8 | Bitboard::E7)
        );
        assert_eq!(
            CastlingRights::empty(),
            pos_history.current_pos().castling_rights()
        );

        // Position after 1. 0-0
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Starting position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Position after 1. Ra2
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(Square::A1, Square::A2, MoveType::QUIET);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            CastlingRights::WHITE_KINGSIDE | CastlingRights::BLACK_BOTH,
            pos_history.current_pos().castling_rights()
        );

        // Position after 1. Ra2 Rxh1
        // White loses kingside castling rights after the rook on h1 gets captured
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(Square::H8, Square::H1, MoveType::CAPTURE);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            CastlingRights::BLACK_QUEENSIDE,
            pos_history.current_pos().castling_rights()
        );

        // Position after 1. Ra2
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(
            &prev_pos,
            pos_history.current_pos(),
            "\nExpected Position:\n{}\nActual Position:\n{}\n",
            prev_pos,
            pos_history.current_pos()
        );

        // Starting position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Position after 1. bxa8=N
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(
            Square::B7,
            Square::A8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Knight),
        );
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_KINGSIDE,
            pos_history.current_pos().castling_rights()
        );

        // Position after 1. bxa8=N bxa1=B
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(
            Square::B2,
            Square::A1,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
        );
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            CastlingRights::WHITE_KINGSIDE | CastlingRights::BLACK_KINGSIDE,
            pos_history.current_pos().castling_rights()
        );

        // Position after 1. bxa8=N bxa1=B 2. gxh8=R+
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(
            Square::G7,
            Square::H8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Rook),
        );
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            CastlingRights::WHITE_KINGSIDE,
            pos_history.current_pos().castling_rights()
        );

        // Position after 1. bxa8=N bxa1=B
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Position after 1. bxa8=N bxa1=B 2. gxh8=B
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(
            Square::G7,
            Square::H8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
        );
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            CastlingRights::WHITE_KINGSIDE,
            pos_history.current_pos().castling_rights()
        );

        // Position after 1. bxa8=N bxa1=B 2. gxh8=B+ gxh1=Q+
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(
            Square::G2,
            Square::H1,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Queen),
        );
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            CastlingRights::empty(),
            pos_history.current_pos().castling_rights()
        );

        // Position after 1. bxa8=N bxa1=B 2. gxh8=B
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Position after 1. bxa8=N bxa1=B
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Position after 1. bxa8=N
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Starting position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
    }

    #[test]
    fn do_and_undo_move_en_passant() {
        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::D5, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::C5, Some(piece::Piece::BLACK_PAWN));
        pos.set_en_passant_square(Bitboard::C6);

        let mut exp_pos_history = Vec::new();
        let mut move_history = Vec::new();
        let mut pos_history = PositionHistory::new(pos);

        // Position after 1. dxc6
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(Square::D5, Square::C6, MoveType::EN_PASSANT_CAPTURE);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Bitboard::C6,
            pos_history.current_pos().side_occupancy(Side::White) & (Bitboard::D5 | Bitboard::C6)
        );
        assert_eq!(
            Bitboard::EMPTY,
            pos_history.current_pos().side_occupancy(Side::Black) & Bitboard::C5
        );

        // Starting position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
    }

    #[test]
    fn do_and_undo_move_promotions() {
        let mut pos = Position::empty();
        pos.set_piece_at(Square::E2, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::A7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::H7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::E7, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::A2, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::H2, Some(piece::Piece::BLACK_PAWN));

        let mut exp_pos_history = Vec::new();
        let mut move_history = Vec::new();
        let mut pos_history = PositionHistory::new(pos);

        // Position after 1. a8=Q
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(
            Square::A7,
            Square::A8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Queen),
        );
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Some(piece::Piece::WHITE_QUEEN),
            pos_history.current_pos().piece_at(Square::A8)
        );

        // Position after 1. a8=Q a1=R
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(
            Square::A2,
            Square::A1,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Rook),
        );
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Some(piece::Piece::BLACK_ROOK),
            pos_history.current_pos().piece_at(Square::A1)
        );

        // Position after 1. a8=Q a1=R 2. h8=B
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(
            Square::H7,
            Square::H8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Bishop),
        );
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Some(piece::Piece::WHITE_BISHOP),
            pos_history.current_pos().piece_at(Square::H8)
        );

        // Position after 1. a8=Q a1=R 2. h8=B h1=N
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(
            Square::H2,
            Square::H1,
            MoveType::new_with_promo_piece(MoveType::PROMOTION, piece::Type::Knight),
        );
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Some(piece::Piece::BLACK_KNIGHT),
            pos_history.current_pos().piece_at(Square::H1)
        );

        // Position after 1. a8=Q a1=R 2. h8=B
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Position after 1. a8=Q a1=R
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Position after 1. a8=Q
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Starting position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
    }

    #[test]
    fn do_and_undo_move_promotion_captures() {
        let mut pos = Position::empty();
        pos.set_piece_at(Square::E2, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::A7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::H7, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::B1, Some(piece::Piece::WHITE_KNIGHT));
        pos.set_piece_at(Square::G1, Some(piece::Piece::WHITE_KNIGHT));
        pos.set_piece_at(Square::E7, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::A2, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::H2, Some(piece::Piece::BLACK_PAWN));
        pos.set_piece_at(Square::B8, Some(piece::Piece::BLACK_KNIGHT));
        pos.set_piece_at(Square::G8, Some(piece::Piece::BLACK_KNIGHT));

        let mut exp_pos_history = Vec::new();
        let mut move_history = Vec::new();
        let mut pos_history = PositionHistory::new(pos);

        // Position after 1. axb8=Q
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(
            Square::A7,
            Square::B8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Queen),
        );
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Some(piece::Piece::WHITE_QUEEN),
            pos_history.current_pos().piece_at(Square::B8)
        );

        // Position after 1. axb8=Q axb1=R
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(
            Square::A2,
            Square::B1,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Rook),
        );
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Some(piece::Piece::BLACK_ROOK),
            pos_history.current_pos().piece_at(Square::B1)
        );

        // Position after 1. axb8=Q axb1=R 2. hxg8=B
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(
            Square::H7,
            Square::G8,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Bishop),
        );
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Some(piece::Piece::WHITE_BISHOP),
            pos_history.current_pos().piece_at(Square::G8)
        );

        // Position after 1. axb8=Q axb1=R 2. hxg8=B hxg1=N+
        exp_pos_history.push(pos_history.current_pos().clone());
        let m = Move::new(
            Square::H2,
            Square::G1,
            MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, piece::Type::Knight),
        );
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Some(piece::Piece::BLACK_KNIGHT),
            pos_history.current_pos().piece_at(Square::G1)
        );

        // Position after 1. axb8=Q axb1=R 2. hxg8=B
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Position after 1. axb8=Q axb1=R
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Position after 1. axb8=Q
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());

        // Starting position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
    }
}
