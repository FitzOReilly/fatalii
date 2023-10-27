use crate::bitboard::Bitboard;
use crate::fen::Fen;
use crate::move_generator::MoveGenerator;
use crate::pawn::Pawn;
use crate::piece::{self, Piece};
use crate::position::{CastlingRights, Position};
use crate::r#move::{Move, MoveType};
use crate::rank::Rank;
use crate::repetition_tracker::RepetitionTracker;
use crate::side::Side;
use crate::square::Square;
use crate::zobrist::Zobrist;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone, Debug)]
pub struct PositionHistory {
    pos: Position,
    pos_hash: Zobrist,
    irreversible_props: Vec<IrreversibleProperties>,
    moves: Vec<Move>,
    rep_tracker: RepetitionTracker<Zobrist>,
}

impl PositionHistory {
    pub fn new(pos: Position) -> Self {
        let pos_hash = Zobrist::new(&pos);
        let rep_tracker = RepetitionTracker::new();
        let mut pos_hist = Self {
            pos_hash,
            pos,
            irreversible_props: Vec::<IrreversibleProperties>::new(),
            moves: Vec::<Move>::new(),
            rep_tracker,
        };
        let en_passant_square = pos_hist.current_pos().en_passant_square();
        pos_hist.clear_en_passant_square_if_irrelevant(en_passant_square);
        pos_hist
            .rep_tracker
            .push(pos_hist.current_pos_hash(), false);
        pos_hist
    }

    pub fn current_pos(&self) -> &Position {
        &self.pos
    }

    pub fn current_pos_hash(&self) -> Zobrist {
        self.pos_hash
    }

    pub fn last_move(&self) -> Option<&Move> {
        self.moves.last()
    }

    pub fn last_moved_piece(&self) -> Option<Piece> {
        match self.last_move() {
            Some(&Move::NULL) => None,
            Some(lm) => {
                if lm.is_promotion() {
                    Some(Piece::new(
                        !self.current_pos().side_to_move(),
                        piece::Type::Pawn,
                    ))
                } else {
                    self.current_pos().piece_at(lm.target())
                }
            }
            None => None,
        }
    }

    pub fn do_move(&mut self, m: Move) {
        debug_assert_eq!(self.irreversible_props.len(), self.moves.len());
        match m {
            Move::NULL => self.do_null_move(),
            _ => match m.move_type() {
                MoveType::QUIET => self.do_quiet_move(m),
                MoveType::DOUBLE_PAWN_PUSH => self.do_double_pawn_push(m),
                MoveType::CASTLE_KINGSIDE => self.do_castle(m),
                MoveType::CASTLE_QUEENSIDE => self.do_castle(m),
                MoveType::CAPTURE => self.do_capture(m),
                MoveType::EN_PASSANT_CAPTURE => self.do_capture(m),
                _ if m.is_capture() => self.do_promotion_capture(m),
                _ => self.do_promotion(m),
            },
        }
        debug_assert_eq!(self.irreversible_props.len(), self.moves.len());
        debug_assert_eq!(Zobrist::new(self.current_pos()), self.current_pos_hash());
    }

    pub fn undo_last_move(&mut self) {
        debug_assert_eq!(self.irreversible_props.len(), self.moves.len());
        if let Some(m) = self.moves.pop() {
            debug_assert!(!self.irreversible_props.is_empty());
            let irr = self.irreversible_props.pop().unwrap();
            self.undo_move(m, &irr);
        }
        debug_assert_eq!(self.irreversible_props.len(), self.moves.len());
    }

    pub fn current_pos_repetitions(&self) -> usize {
        self.rep_tracker.current_pos_repetitions()
    }

    pub fn gives_check(&mut self, m: Move) -> bool {
        self.do_move(m);
        let pos = self.current_pos();
        let res = pos.is_in_check(pos.side_to_move());
        self.undo_last_move();
        res
    }

    fn do_null_move(&mut self) {
        self.moves.push(Move::NULL);
        self.irreversible_props.push(IrreversibleProperties::new(
            self.pos.en_passant_square(),
            self.pos.castling_rights(),
            self.pos.plies_since_pawn_move_or_capture(),
            None,
        ));

        let side_to_move = self.pos.side_to_move();
        let plies = self.pos.plies_since_pawn_move_or_capture();
        let move_count = self.pos.move_count();

        let is_reversible = self.pos.en_passant_square() == Bitboard::EMPTY;
        self.pos_hash
            .toggle_en_passant_square(self.pos.en_passant_square());
        self.pos.set_en_passant_square(Bitboard::EMPTY);
        self.pos_hash.toggle_side_to_move(Side::Black);
        self.pos.set_side_to_move(!side_to_move);
        self.pos.set_plies_since_pawn_move_or_capture(plies + 1);
        self.pos.set_move_count(move_count + side_to_move as usize);
        self.rep_tracker.push(self.pos_hash, is_reversible);
    }

    fn do_quiet_move(&mut self, m: Move) {
        self.moves.push(m);
        let origin = m.origin();
        let target = m.target();
        let origin_piece = self.pos.piece_at(origin).unwrap();
        let target_piece = origin_piece;
        let side_to_move = self.pos.side_to_move();

        let mut is_reversible = origin_piece.piece_type() != piece::Type::Pawn
            && self.pos.en_passant_square() == Bitboard::EMPTY;

        const CAPTURED_PIECE: Option<piece::Piece> = None;
        self.irreversible_props.push(IrreversibleProperties::new(
            self.pos.en_passant_square(),
            self.pos.castling_rights(),
            self.pos.plies_since_pawn_move_or_capture(),
            CAPTURED_PIECE,
        ));

        self.remove_piece(origin, origin_piece);
        self.set_piece(target, target_piece);

        self.pos_hash
            .toggle_en_passant_square(self.pos.en_passant_square());
        self.pos.set_en_passant_square(Bitboard::EMPTY);

        if origin_piece.piece_type() == piece::Type::Pawn {
            self.pos.set_plies_since_pawn_move_or_capture(0);
        } else {
            self.pos.set_plies_since_pawn_move_or_capture(
                self.pos.plies_since_pawn_move_or_capture() + 1,
            );
        }

        let old_cr = self.pos.castling_rights();
        self.pos.remove_castling_rights(origin);
        let new_cr = self.pos.castling_rights();
        self.pos_hash.toggle_castling_rights(old_cr ^ new_cr);

        is_reversible &= old_cr == new_cr;

        let move_count = self.pos.move_count();
        self.pos.set_move_count(move_count + side_to_move as usize);
        self.pos.set_side_to_move(!side_to_move);
        self.pos_hash.toggle_side_to_move(Side::Black);
        self.rep_tracker.push(self.pos_hash, is_reversible);
    }

    fn do_double_pawn_push(&mut self, m: Move) {
        self.moves.push(m);
        let origin = m.origin();
        let target = m.target();
        let origin_piece = self.pos.piece_at(origin).unwrap();
        let target_piece = origin_piece;
        let side_to_move = self.pos.side_to_move();

        const CAPTURED_PIECE: Option<piece::Piece> = None;
        self.irreversible_props.push(IrreversibleProperties::new(
            self.pos.en_passant_square(),
            self.pos.castling_rights(),
            self.pos.plies_since_pawn_move_or_capture(),
            CAPTURED_PIECE,
        ));

        self.remove_piece(origin, origin_piece);
        self.set_piece(target, target_piece);

        let en_passant_square = Bitboard::from_square(Pawn::push_origin(target, side_to_move));

        self.pos_hash
            .toggle_en_passant_square(self.pos.en_passant_square());
        self.pos.set_en_passant_square(en_passant_square);
        self.pos_hash.toggle_en_passant_square(en_passant_square);

        const IS_REVERSIBLE: bool = false;
        self.pos.set_plies_since_pawn_move_or_capture(0);
        let move_count = self.pos.move_count();
        self.pos.set_move_count(move_count + side_to_move as usize);
        self.pos.set_side_to_move(!side_to_move);
        self.pos_hash.toggle_side_to_move(Side::Black);
        self.clear_en_passant_square_if_irrelevant(en_passant_square);

        self.rep_tracker.push(self.pos_hash, IS_REVERSIBLE);
    }

    fn clear_en_passant_square_if_irrelevant(&mut self, en_passant_square: Bitboard) {
        if !MoveGenerator::has_en_passant_capture(&self.pos) {
            self.pos.set_en_passant_square(Bitboard::EMPTY);
            self.pos_hash.toggle_en_passant_square(en_passant_square);
        }
    }

    fn do_castle(&mut self, m: Move) {
        self.moves.push(m);
        let origin = m.origin();
        let target = m.target();
        let origin_piece = self.pos.piece_at(origin).unwrap();
        let target_piece = origin_piece;
        let side_to_move = self.pos.side_to_move();

        const CAPTURED_PIECE: Option<piece::Piece> = None;
        self.irreversible_props.push(IrreversibleProperties::new(
            self.pos.en_passant_square(),
            self.pos.castling_rights(),
            self.pos.plies_since_pawn_move_or_capture(),
            CAPTURED_PIECE,
        ));

        self.remove_piece(origin, origin_piece);
        match (self.pos.side_to_move(), m.move_type()) {
            (Side::White, MoveType::CASTLE_KINGSIDE) => {
                let rook_origin =
                    Square::from_file_and_rank(self.pos.kingside_castling_file(), Rank::R1);
                self.remove_piece(rook_origin, piece::Piece::WHITE_ROOK);
                self.set_piece(Square::F1, piece::Piece::WHITE_ROOK);
            }
            (Side::White, MoveType::CASTLE_QUEENSIDE) => {
                let rook_origin =
                    Square::from_file_and_rank(self.pos.queenside_castling_file(), Rank::R1);
                self.remove_piece(rook_origin, piece::Piece::WHITE_ROOK);
                self.set_piece(Square::D1, piece::Piece::WHITE_ROOK);
            }
            (Side::Black, MoveType::CASTLE_KINGSIDE) => {
                let rook_origin =
                    Square::from_file_and_rank(self.pos.kingside_castling_file(), Rank::R8);
                self.remove_piece(rook_origin, piece::Piece::BLACK_ROOK);
                self.set_piece(Square::F8, piece::Piece::BLACK_ROOK);
            }
            (Side::Black, MoveType::CASTLE_QUEENSIDE) => {
                let rook_origin =
                    Square::from_file_and_rank(self.pos.queenside_castling_file(), Rank::R8);
                self.remove_piece(rook_origin, piece::Piece::BLACK_ROOK);
                self.set_piece(Square::D8, piece::Piece::BLACK_ROOK);
            }
            _ => debug_assert!(m.is_castle()),
        }
        self.set_piece(target, target_piece);

        self.pos_hash
            .toggle_en_passant_square(self.pos.en_passant_square());
        self.pos.set_en_passant_square(Bitboard::EMPTY);

        let old_cr = self.pos.castling_rights();
        self.pos.remove_castling_rights(origin);
        let new_cr = self.pos.castling_rights();
        self.pos_hash.toggle_castling_rights(old_cr ^ new_cr);

        const IS_REVERSIBLE: bool = false;
        self.pos
            .set_plies_since_pawn_move_or_capture(self.pos.plies_since_pawn_move_or_capture() + 1);
        let move_count = self.pos.move_count();
        self.pos.set_move_count(move_count + side_to_move as usize);
        self.pos.set_side_to_move(!side_to_move);
        self.pos_hash.toggle_side_to_move(Side::Black);
        self.rep_tracker.push(self.pos_hash, IS_REVERSIBLE);
    }

    fn do_capture(&mut self, m: Move) {
        self.moves.push(m);
        let origin = m.origin();
        let target = m.target();
        let side_to_move = self.pos.side_to_move();
        let origin_piece = self.pos.piece_at(origin).unwrap();
        let target_piece = origin_piece;

        let capture_square = if m.is_en_passant() {
            Pawn::push_origin(target, side_to_move)
        } else {
            target
        };
        let captured_piece = self.pos.piece_at(capture_square);

        self.irreversible_props.push(IrreversibleProperties::new(
            self.pos.en_passant_square(),
            self.pos.castling_rights(),
            self.pos.plies_since_pawn_move_or_capture(),
            captured_piece,
        ));

        debug_assert!(captured_piece.is_some());
        self.remove_piece(origin, origin_piece);
        self.remove_piece(capture_square, captured_piece.unwrap());
        self.set_piece(target, target_piece);

        const EN_PASSANT_SQUARE: Bitboard = Bitboard::EMPTY;
        self.pos_hash
            .toggle_en_passant_square(self.pos.en_passant_square());
        self.pos.set_en_passant_square(EN_PASSANT_SQUARE);

        let old_cr = self.pos.castling_rights();
        self.pos.remove_castling_rights(origin);
        self.pos.remove_castling_rights(target);
        let new_cr = self.pos.castling_rights();
        self.pos_hash.toggle_castling_rights(old_cr ^ new_cr);

        const IS_REVERSIBLE: bool = false;
        self.pos.set_plies_since_pawn_move_or_capture(0);
        let move_count = self.pos.move_count();
        self.pos.set_move_count(move_count + side_to_move as usize);
        self.pos.set_side_to_move(!side_to_move);
        self.pos_hash.toggle_side_to_move(Side::Black);
        self.rep_tracker.push(self.pos_hash, IS_REVERSIBLE);
    }

    fn do_promotion(&mut self, m: Move) {
        self.moves.push(m);
        let origin = m.origin();
        let target = m.target();
        let side_to_move = self.pos.side_to_move();
        let origin_piece = self.pos.piece_at(origin).unwrap();
        let target_piece = piece::Piece::new(side_to_move, m.move_type().promo_piece_unchecked());

        const CAPTURED_PIECE: Option<piece::Piece> = None;
        self.irreversible_props.push(IrreversibleProperties::new(
            self.pos.en_passant_square(),
            self.pos.castling_rights(),
            self.pos.plies_since_pawn_move_or_capture(),
            CAPTURED_PIECE,
        ));

        self.remove_piece(origin, origin_piece);
        self.set_piece(target, target_piece);

        const EN_PASSANT_SQUARE: Bitboard = Bitboard::EMPTY;
        self.pos_hash
            .toggle_en_passant_square(self.pos.en_passant_square());
        self.pos.set_en_passant_square(EN_PASSANT_SQUARE);

        const IS_REVERSIBLE: bool = false;
        self.pos.set_plies_since_pawn_move_or_capture(0);
        let move_count = self.pos.move_count();
        self.pos.set_move_count(move_count + side_to_move as usize);
        self.pos.set_side_to_move(!side_to_move);
        self.pos_hash.toggle_side_to_move(Side::Black);
        self.rep_tracker.push(self.pos_hash, IS_REVERSIBLE);
    }

    fn do_promotion_capture(&mut self, m: Move) {
        self.moves.push(m);
        let origin = m.origin();
        let target = m.target();
        let side_to_move = self.pos.side_to_move();
        let origin_piece = self.pos.piece_at(origin).unwrap();
        let target_piece = piece::Piece::new(side_to_move, m.move_type().promo_piece_unchecked());

        let capture_square = target;
        let captured_piece = self.pos.piece_at(capture_square);

        self.irreversible_props.push(IrreversibleProperties::new(
            self.pos.en_passant_square(),
            self.pos.castling_rights(),
            self.pos.plies_since_pawn_move_or_capture(),
            captured_piece,
        ));

        debug_assert!(captured_piece.is_some());
        self.remove_piece(origin, origin_piece);
        self.remove_piece(capture_square, captured_piece.unwrap());
        self.set_piece(target, target_piece);

        const EN_PASSANT_SQUARE: Bitboard = Bitboard::EMPTY;
        self.pos_hash
            .toggle_en_passant_square(self.pos.en_passant_square());
        self.pos.set_en_passant_square(EN_PASSANT_SQUARE);
        self.pos_hash.toggle_en_passant_square(EN_PASSANT_SQUARE);

        let old_cr = self.pos.castling_rights();
        self.pos.remove_castling_rights(target);
        let new_cr = self.pos.castling_rights();
        self.pos_hash.toggle_castling_rights(old_cr ^ new_cr);

        const IS_REVERSIBLE: bool = false;
        self.pos.set_plies_since_pawn_move_or_capture(0);
        let move_count = self.pos.move_count();
        self.pos.set_move_count(move_count + side_to_move as usize);
        self.pos.set_side_to_move(!side_to_move);
        self.pos_hash.toggle_side_to_move(Side::Black);
        self.rep_tracker.push(self.pos_hash, IS_REVERSIBLE);
    }

    fn undo_move(&mut self, m: Move, irr: &IrreversibleProperties) {
        match m {
            Move::NULL => self.undo_null_move(irr),
            _ if m.is_castle() => self.undo_castle(m, irr),
            _ => self.undo_other_move(m, irr),
        }
        debug_assert_eq!(Zobrist::new(self.current_pos()), self.current_pos_hash());
    }

    fn undo_null_move(&mut self, irr: &IrreversibleProperties) {
        self.rep_tracker.pop();
        self.pos.set_en_passant_square(irr.en_passant_square);
        self.pos_hash
            .toggle_en_passant_square(irr.en_passant_square);
        self.pos
            .set_plies_since_pawn_move_or_capture(irr.plies_since_pawn_move_or_capture);
        self.pos_hash.toggle_side_to_move(Side::Black);
        self.pos.set_side_to_move(!self.pos.side_to_move());
        self.pos
            .set_move_count(self.pos.move_count() - self.pos.side_to_move() as usize);
    }

    fn undo_castle(&mut self, m: Move, irr: &IrreversibleProperties) {
        self.rep_tracker.pop();
        let origin = m.origin();
        let target = m.target();
        let target_piece = self.pos.piece_at(target).unwrap();
        let origin_piece = target_piece;

        self.pos.set_side_to_move(!self.pos.side_to_move());
        self.pos_hash.toggle_side_to_move(Side::Black);
        self.pos
            .set_move_count(self.pos.move_count() - self.pos.side_to_move() as usize);

        self.remove_piece(target, target_piece);
        match (self.pos.side_to_move(), m.move_type()) {
            (Side::White, MoveType::CASTLE_KINGSIDE) => {
                let rook_origin =
                    Square::from_file_and_rank(self.pos.kingside_castling_file(), Rank::R1);
                self.remove_piece(Square::F1, piece::Piece::WHITE_ROOK);
                self.set_piece(rook_origin, piece::Piece::WHITE_ROOK);
            }
            (Side::White, MoveType::CASTLE_QUEENSIDE) => {
                let rook_origin =
                    Square::from_file_and_rank(self.pos.queenside_castling_file(), Rank::R1);
                self.remove_piece(Square::D1, piece::Piece::WHITE_ROOK);
                self.set_piece(rook_origin, piece::Piece::WHITE_ROOK);
            }
            (Side::Black, MoveType::CASTLE_KINGSIDE) => {
                let rook_origin =
                    Square::from_file_and_rank(self.pos.kingside_castling_file(), Rank::R8);
                self.remove_piece(Square::F8, piece::Piece::BLACK_ROOK);
                self.set_piece(rook_origin, piece::Piece::BLACK_ROOK);
            }
            (Side::Black, MoveType::CASTLE_QUEENSIDE) => {
                let rook_origin =
                    Square::from_file_and_rank(self.pos.queenside_castling_file(), Rank::R8);
                self.remove_piece(Square::D8, piece::Piece::BLACK_ROOK);
                self.set_piece(rook_origin, piece::Piece::BLACK_ROOK);
            }
            _ => debug_assert!(m.is_castle()),
        }
        self.set_piece(origin, origin_piece);

        self.pos.set_en_passant_square(irr.en_passant_square);
        self.pos_hash
            .toggle_en_passant_square(irr.en_passant_square);
        self.pos_hash
            .toggle_castling_rights(self.pos.castling_rights() ^ irr.castling_rights);
        self.pos.set_castling_rights(irr.castling_rights);
        self.pos
            .set_plies_since_pawn_move_or_capture(irr.plies_since_pawn_move_or_capture);
    }

    fn undo_other_move(&mut self, m: Move, irr: &IrreversibleProperties) {
        self.rep_tracker.pop();
        let origin = m.origin();
        let target = m.target();
        let target_piece = self.pos.piece_at(target).unwrap_or_else(|| {
            panic!(
                "Expected a piece on {}\nPosition: {}\n{}Move: {}",
                target,
                Fen::pos_to_str_chess_960(&self.pos),
                self.pos,
                m,
            )
        });

        let origin_piece = if m.is_promotion() {
            piece::Piece::new(target_piece.piece_side(), piece::Type::Pawn)
        } else {
            target_piece
        };

        self.pos.set_side_to_move(!self.pos.side_to_move());
        self.pos_hash.toggle_side_to_move(Side::Black);
        self.pos
            .set_move_count(self.pos.move_count() - self.pos.side_to_move() as usize);

        self.remove_piece(target, target_piece);
        self.set_piece(origin, origin_piece);

        self.pos_hash
            .toggle_en_passant_square(self.pos.en_passant_square());
        self.pos.set_en_passant_square(irr.en_passant_square);
        self.pos_hash
            .toggle_en_passant_square(irr.en_passant_square);
        self.pos_hash
            .toggle_castling_rights(self.pos.castling_rights() ^ irr.castling_rights);
        self.pos.set_castling_rights(irr.castling_rights);
        self.pos
            .set_plies_since_pawn_move_or_capture(irr.plies_since_pawn_move_or_capture);

        if m.is_capture() {
            let capture_square = if m.is_en_passant() {
                Pawn::push_origin(target, self.pos.side_to_move())
            } else {
                target
            };
            debug_assert!(irr.captured_piece.is_some());
            self.set_piece(capture_square, irr.captured_piece.unwrap());
        }
    }

    fn set_piece(&mut self, square: Square, piece: piece::Piece) {
        debug_assert_eq!(None, self.pos.piece_at(square));
        self.pos.set_piece_at(square, Some(piece));
        self.pos_hash.toggle_piece(Some(piece), square);
    }

    fn remove_piece(&mut self, square: Square, piece: piece::Piece) {
        debug_assert_eq!(Some(piece), self.pos.piece_at(square));
        self.pos.set_piece_at(square, None);
        self.pos_hash.toggle_piece(Some(piece), square);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{fen::Fen, side::Side};

    #[test]
    fn do_and_undo_move_initial_position() {
        let pos = Position::initial();

        let mut exp_pos_history = Vec::new();
        let mut exp_pos_hash_history = Vec::new();
        let mut move_history = Vec::new();
        let mut pos_history = PositionHistory::new(pos);

        // Position after 1. e4
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
        let m = Move::new(Square::E2, Square::E4, MoveType::DOUBLE_PAWN_PUSH);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Bitboard::E4,
            pos_history.current_pos().side_occupancy(Side::White) & (Bitboard::E2 | Bitboard::E4)
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
            0,
            pos_history.current_pos().plies_since_pawn_move_or_capture()
        );
        assert_eq!(1, pos_history.current_pos().move_count());

        // Position after 1. e4 c5
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
        let m = Move::new(Square::C7, Square::C5, MoveType::DOUBLE_PAWN_PUSH);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Bitboard::C5,
            pos_history.current_pos().side_occupancy(Side::Black) & (Bitboard::C7 | Bitboard::C5)
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
        assert_eq!(2, pos_history.current_pos().move_count());

        // Position after 1. e4 c5 2. Nf3
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Position after 1. e4 c5
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Position after 1. e4
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Initial position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());
    }

    #[test]
    fn do_and_undo_move_castle() {
        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::A1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Square::H1, Some(piece::Piece::WHITE_ROOK));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::A8, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Square::H8, Some(piece::Piece::BLACK_ROOK));
        pos.set_castling_rights(CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH);

        let mut exp_pos_history = Vec::new();
        let mut exp_pos_hash_history = Vec::new();
        let mut move_history = Vec::new();
        let mut pos_history = PositionHistory::new(pos);

        // Position after 1. 0-0
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
        let m = Move::new(Square::E1, Square::G1, MoveType::CASTLE_KINGSIDE);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Some(piece::Piece::WHITE_KING),
            pos_history.current_pos().piece_at(Square::G1)
        );
        assert_eq!(
            Some(piece::Piece::WHITE_ROOK),
            pos_history.current_pos().piece_at(Square::F1)
        );
        assert_eq!(
            CastlingRights::BLACK_BOTH,
            pos_history.current_pos().castling_rights()
        );

        // Position after 1. 0-0 0-0-0
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
        let m = Move::new(Square::E8, Square::C8, MoveType::CASTLE_QUEENSIDE);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Some(piece::Piece::BLACK_KING),
            pos_history.current_pos().piece_at(Square::C8)
        );
        assert_eq!(
            Some(piece::Piece::BLACK_ROOK),
            pos_history.current_pos().piece_at(Square::D8)
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
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Starting position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Position after 1. 0-0-0
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
        let m = Move::new(Square::E1, Square::C1, MoveType::CASTLE_QUEENSIDE);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Some(piece::Piece::WHITE_KING),
            pos_history.current_pos().piece_at(Square::C1)
        );
        assert_eq!(
            Some(piece::Piece::WHITE_ROOK),
            pos_history.current_pos().piece_at(Square::D1)
        );
        assert_eq!(
            CastlingRights::BLACK_BOTH,
            pos_history.current_pos().castling_rights()
        );

        // Position after 1. 0-0-0 0-0
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
        let m = Move::new(Square::E8, Square::G8, MoveType::CASTLE_KINGSIDE);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Some(piece::Piece::BLACK_KING),
            pos_history.current_pos().piece_at(Square::G8)
        );
        assert_eq!(
            Some(piece::Piece::BLACK_ROOK),
            pos_history.current_pos().piece_at(Square::F8)
        );
        assert_eq!(
            CastlingRights::empty(),
            pos_history.current_pos().castling_rights()
        );

        // Position after 1. 0-0-0
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Starting position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());
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
        let mut exp_pos_hash_history = Vec::new();
        let mut move_history = Vec::new();
        let mut pos_history = PositionHistory::new(pos);

        // Position after 1. 0-0
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Starting position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Position after 1. Ra2
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Starting position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Position after 1. bxa8=N
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Position after 1. bxa8=N bxa1=B 2. gxh8=B
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Position after 1. bxa8=N bxa1=B
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Position after 1. bxa8=N
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Starting position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());
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
        let mut exp_pos_hash_history = Vec::new();
        let mut move_history = Vec::new();
        let mut pos_history = PositionHistory::new(pos);

        // Position after 1. dxc6
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());
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
        let mut exp_pos_hash_history = Vec::new();
        let mut move_history = Vec::new();
        let mut pos_history = PositionHistory::new(pos);

        // Position after 1. a8=Q
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Position after 1. a8=Q a1=R
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Position after 1. a8=Q
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Starting position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());
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
        let mut exp_pos_hash_history = Vec::new();
        let mut move_history = Vec::new();
        let mut pos_history = PositionHistory::new(pos);

        // Position after 1. axb8=Q
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        exp_pos_hash_history.push(pos_history.current_pos_hash());
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
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Position after 1. axb8=Q axb1=R
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Position after 1. axb8=Q
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Starting position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());
    }

    #[test]
    fn do_and_undo_null_move() {
        let pos = Position::initial();

        let mut exp_pos_history = Vec::new();
        let mut exp_pos_hash_history = Vec::new();
        let mut move_history = Vec::new();
        let mut pos_history = PositionHistory::new(pos);

        // Position after 1. null
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
        let m = Move::NULL;
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(Side::Black, pos_history.current_pos().side_to_move());
        assert_eq!(
            1,
            pos_history.current_pos().plies_since_pawn_move_or_capture()
        );
        assert_eq!(1, pos_history.current_pos().move_count());

        // Position after 1. null null
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
        let m = Move::NULL;
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(Side::White, pos_history.current_pos().side_to_move());
        assert_eq!(
            2,
            pos_history.current_pos().plies_since_pawn_move_or_capture()
        );
        assert_eq!(2, pos_history.current_pos().move_count());

        // Position after 1. null
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Initial position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Position after 1. e4
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
        let m = Move::new(Square::E2, Square::E4, MoveType::DOUBLE_PAWN_PUSH);
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Bitboard::EMPTY,
            pos_history.current_pos().en_passant_square()
        );
        assert_eq!(Side::Black, pos_history.current_pos().side_to_move());

        // Position after 1. e4 null
        exp_pos_history.push(pos_history.current_pos().clone());
        exp_pos_hash_history.push(pos_history.current_pos_hash());
        let m = Move::NULL;
        move_history.push(m);
        pos_history.do_move(m);
        assert_eq!(
            Bitboard::EMPTY,
            pos_history.current_pos().en_passant_square()
        );
        assert_eq!(Side::White, pos_history.current_pos().side_to_move());

        // Position after 1. e4
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());

        // Initial position
        pos_history.undo_last_move();
        let prev_pos = exp_pos_history
            .pop()
            .expect("Expected Some(Position), got None");
        assert_eq!(&prev_pos, pos_history.current_pos());
        let prev_pos_hash = exp_pos_hash_history.pop().unwrap();
        assert_eq!(prev_pos_hash, pos_history.current_pos_hash());
    }

    #[test]
    fn current_pos_repetitions() {
        let mut pos_history = PositionHistory::new(Position::initial());
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::G1, Square::F3, MoveType::QUIET));
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::G8, Square::F6, MoveType::QUIET));
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::F3, Square::G1, MoveType::QUIET));
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::F6, Square::G8, MoveType::QUIET));
        assert_eq!(2, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::NULL);
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::NULL);
        assert_eq!(3, pos_history.current_pos_repetitions());

        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(2, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
    }

    #[test]
    fn repetitions_castling_rights() {
        // Two position are not considered equal if the castling rights have changed
        let fen = "r3k3/8/8/8/8/8/8/4K2R w Kq - 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();

        let mut pos_history = PositionHistory::new(pos.clone());
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::NULL);
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::NULL);
        assert_eq!(2, pos_history.current_pos_repetitions());

        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());

        let mut pos_history = PositionHistory::new(pos);
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::H1, Square::G1, MoveType::QUIET));
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::A8, Square::B8, MoveType::QUIET));
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::G1, Square::H1, MoveType::QUIET));
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::B8, Square::A8, MoveType::QUIET));
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::H1, Square::G1, MoveType::QUIET));
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::A8, Square::B8, MoveType::QUIET));
        assert_eq!(2, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::G1, Square::H1, MoveType::QUIET));
        assert_eq!(2, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::B8, Square::A8, MoveType::QUIET));
        assert_eq!(2, pos_history.current_pos_repetitions());

        pos_history.undo_last_move();
        assert_eq!(2, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(2, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
    }

    #[test]
    fn repetitions_en_passant() {
        // Two position are not considered equal if the en passant square has changed
        let fen = "4k3/8/8/1Pp5/8/8/8/4K3 w - c6 0 1";
        let pos = Fen::str_to_pos(fen).unwrap();

        let mut pos_history = PositionHistory::new(pos.clone());
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::NULL);
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::NULL);
        assert_eq!(1, pos_history.current_pos_repetitions());

        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());

        let mut pos_history = PositionHistory::new(pos);
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::E1, Square::D1, MoveType::QUIET));
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::E8, Square::D8, MoveType::QUIET));
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::D1, Square::E1, MoveType::QUIET));
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::D8, Square::E8, MoveType::QUIET));
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.do_move(Move::new(Square::E1, Square::D1, MoveType::QUIET));
        assert_eq!(2, pos_history.current_pos_repetitions());

        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
        pos_history.undo_last_move();
        assert_eq!(1, pos_history.current_pos_repetitions());
    }

    #[test]
    fn castles_chess_960() {
        let fen = "4k3/8/8/8/8/8/8/1R3KR1 w GB - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut pos_hist = PositionHistory::new(pos.clone());

        let white_kingside_castle = Move::new(Square::F1, Square::G1, MoveType::CASTLE_KINGSIDE);
        let white_queenside_castle = Move::new(Square::F1, Square::C1, MoveType::CASTLE_QUEENSIDE);

        pos_hist.do_move(white_kingside_castle);
        assert_eq!(
            Some(piece::Piece::WHITE_KING),
            pos_hist.current_pos().piece_at(Square::G1)
        );
        assert_eq!(
            Some(piece::Piece::WHITE_ROOK),
            pos_hist.current_pos().piece_at(Square::F1)
        );

        pos_hist.undo_last_move();
        pos_hist.do_move(white_queenside_castle);
        assert_eq!(
            Some(piece::Piece::WHITE_KING),
            pos_hist.current_pos().piece_at(Square::C1)
        );
        assert_eq!(
            Some(piece::Piece::WHITE_ROOK),
            pos_hist.current_pos().piece_at(Square::D1)
        );
        assert_eq!(None, pos_hist.current_pos().piece_at(Square::B1));

        let fen = "1r3kr1/8/8/8/8/8/8/4K3 b gb - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut pos_hist = PositionHistory::new(pos.clone());

        let black_kingside_castle = Move::new(Square::F8, Square::G8, MoveType::CASTLE_KINGSIDE);
        let black_queenside_castle = Move::new(Square::F8, Square::C8, MoveType::CASTLE_QUEENSIDE);

        pos_hist.do_move(black_kingside_castle);
        assert_eq!(
            Some(piece::Piece::BLACK_KING),
            pos_hist.current_pos().piece_at(Square::G8)
        );
        assert_eq!(
            Some(piece::Piece::BLACK_ROOK),
            pos_hist.current_pos().piece_at(Square::F8)
        );

        pos_hist.undo_last_move();
        pos_hist.do_move(black_queenside_castle);
        assert_eq!(
            Some(piece::Piece::BLACK_KING),
            pos_hist.current_pos().piece_at(Square::C8)
        );
        assert_eq!(
            Some(piece::Piece::BLACK_ROOK),
            pos_hist.current_pos().piece_at(Square::D8)
        );
        assert_eq!(None, pos_hist.current_pos().piece_at(Square::B8));

        let fen = "r1k4r/8/8/8/8/8/8/R1K4R w HAha - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut pos_hist = PositionHistory::new(pos.clone());

        println!("Position\n{}", pos_hist.current_pos());
        pos_hist.do_move(Move::new(
            Square::C1,
            Square::C1,
            MoveType::CASTLE_QUEENSIDE,
        ));
        println!("Position\n{}", pos_hist.current_pos());
        pos_hist.undo_last_move();
        println!("Position\n{}", pos_hist.current_pos());

        let fen = "rk5r/8/8/8/8/8/8/RK5R w HAha - 0 1";
        let pos = Fen::str_to_pos_chess_960(fen).unwrap();
        let mut pos_hist = PositionHistory::new(pos.clone());

        println!("Position\n{}", pos_hist.current_pos());
        pos_hist.do_move(Move::new(
            Square::B1,
            Square::C1,
            MoveType::CASTLE_QUEENSIDE,
        ));
        println!("Position\n{}", pos_hist.current_pos());
        pos_hist.do_move(Move::new(
            Square::B8,
            Square::C8,
            MoveType::CASTLE_QUEENSIDE,
        ));
        println!("Position\n{}", pos_hist.current_pos());
        pos_hist.undo_last_move();
        println!("Position\n{}", pos_hist.current_pos());
        pos_hist.undo_last_move();
        println!("Position\n{}", pos_hist.current_pos());
    }

    #[test]
    fn repetitions_with_irrelevant_en_passant_square() {
        // Expected: Detect repetitions in positions after a double pawn push
        // when there is no legal en passant capture.

        let g7g5 = Move::new(Square::G7, Square::G5, MoveType::DOUBLE_PAWN_PUSH);
        let c3d3 = Move::new(Square::C3, Square::D3, MoveType::QUIET);
        let b7a6 = Move::new(Square::B7, Square::A6, MoveType::QUIET);
        let d3c3 = Move::new(Square::D3, Square::C3, MoveType::QUIET);
        let a6b7 = Move::new(Square::A6, Square::B7, MoveType::QUIET);

        let fen_before_pawn_push = "8/1b1rr1p1/3kpp1p/2pn4/2P1B1P1/2R2N1P/2P2PK1/3R4 b - - 20 35";
        let pos = Fen::str_to_pos(fen_before_pawn_push).unwrap();
        let mut pos_hist = PositionHistory::new(pos.clone());
        pos_hist.do_move(g7g5);
        pos_hist.do_move(c3d3);
        pos_hist.do_move(b7a6);
        pos_hist.do_move(d3c3);
        pos_hist.do_move(a6b7);
        assert_eq!(2, pos_hist.current_pos_repetitions());

        let fen_after_pawn_push = "8/1b1rr3/3kpp1p/2pn2p1/2P1B1P1/2R2N1P/2P2PK1/3R4 w - g6 0 36";
        let pos = Fen::str_to_pos(fen_after_pawn_push).unwrap();
        let mut pos_hist = PositionHistory::new(pos.clone());
        pos_hist.do_move(c3d3);
        pos_hist.do_move(b7a6);
        pos_hist.do_move(d3c3);
        pos_hist.do_move(a6b7);
        assert_eq!(2, pos_hist.current_pos_repetitions());
    }
}
