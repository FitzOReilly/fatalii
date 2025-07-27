use rand::{Rng, SeedableRng};

use crate::bitboard::Bitboard;
use crate::piece;
use crate::position::{CastlingRights, Position};
use crate::side::Side;
use crate::square::Square;
use std::ops::{BitXor, BitXorAssign};

const NUM_ZOBRIST_KEYS: usize = 781;

// Indices
// 0-383: White pieces
// 384-767: Black pieces
// 768: Black to move
// 769 - 772: Castling rights (KQkq)
// 773 - 780: En passant file
const IDX_FIRST_BLACK_PIECE: usize = 384;
const IDX_SIDE_TO_MOVE: usize = 768;
const IDX_WHITE_KINGSIDE: usize = 769;
const IDX_WHITE_QUEENSIDE: usize = 770;
const IDX_BLACK_KINGSIDE: usize = 771;
const IDX_BLACK_QUEENSIDE: usize = 772;
const IDX_FIRST_EN_PASSANT_FILE: usize = 773;

#[static_init::dynamic]
static ZOBRIST_KEYS: [Zobrist; NUM_ZOBRIST_KEYS] = {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let mut keys: [Zobrist; NUM_ZOBRIST_KEYS] = [Default::default(); NUM_ZOBRIST_KEYS];
    for key in &mut keys {
        *key = Zobrist(rng.random());
    }
    keys
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Zobrist(u64);

impl Zobrist {
    pub fn new(pos: &Position) -> Self {
        let mut zobrist = Self(0);

        for side in &[Side::White, Side::Black] {
            for piece_type in &[
                piece::Type::Pawn,
                piece::Type::Knight,
                piece::Type::Bishop,
                piece::Type::Rook,
                piece::Type::Queen,
                piece::Type::King,
            ] {
                let mut squares = pos.piece_occupancy(*side, *piece_type);
                while squares != Bitboard::EMPTY {
                    let square = squares.square_scan_forward_reset();
                    zobrist.toggle_piece(Some(piece::Piece::new(*side, *piece_type)), square)
                }
            }
        }

        zobrist.toggle_side_to_move(pos.side_to_move());
        zobrist.toggle_castling_rights(pos.castling_rights());
        zobrist.toggle_en_passant_square(pos.en_passant_square());

        zobrist
    }

    pub fn toggle_piece(&mut self, piece: Option<piece::Piece>, square: Square) {
        if let Some(p) = piece {
            let side_idx = p.piece_side() as usize * IDX_FIRST_BLACK_PIECE;
            let piece_type_idx = p.piece_type() as usize * Square::NUM_SQUARES;
            let square_idx = square.idx();
            *self ^= ZOBRIST_KEYS[side_idx + piece_type_idx + square_idx];
        }
    }

    pub fn toggle_side_to_move(&mut self, side: Side) {
        if side == Side::Black {
            *self ^= ZOBRIST_KEYS[IDX_SIDE_TO_MOVE];
        }
    }

    pub fn toggle_castling_rights(&mut self, cr: CastlingRights) {
        if cr.contains(CastlingRights::WHITE_KINGSIDE) {
            *self ^= ZOBRIST_KEYS[IDX_WHITE_KINGSIDE];
        }
        if cr.contains(CastlingRights::WHITE_QUEENSIDE) {
            *self ^= ZOBRIST_KEYS[IDX_WHITE_QUEENSIDE];
        }
        if cr.contains(CastlingRights::BLACK_KINGSIDE) {
            *self ^= ZOBRIST_KEYS[IDX_BLACK_KINGSIDE];
        }
        if cr.contains(CastlingRights::BLACK_QUEENSIDE) {
            *self ^= ZOBRIST_KEYS[IDX_BLACK_QUEENSIDE];
        }
    }

    pub fn toggle_en_passant_square(&mut self, en_passant: Bitboard) {
        if en_passant != Bitboard::EMPTY {
            let file = en_passant.to_square().file();
            *self ^= ZOBRIST_KEYS[IDX_FIRST_EN_PASSANT_FILE + file.idx()];
        }
    }
}

impl BitXor for Zobrist {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Zobrist(self.0 ^ rhs.0)
    }
}

impl BitXor<&Self> for Zobrist {
    type Output = Self;

    fn bitxor(self, rhs: &Self) -> Self::Output {
        Zobrist(self.0 ^ rhs.0)
    }
}

impl BitXor for &Zobrist {
    type Output = Zobrist;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Zobrist(self.0 ^ rhs.0)
    }
}

impl BitXor<Zobrist> for &Zobrist {
    type Output = Zobrist;

    fn bitxor(self, rhs: Zobrist) -> Self::Output {
        Zobrist(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Zobrist {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<&Self> for Zobrist {
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.0 ^= rhs.0;
    }
}

impl From<u64> for Zobrist {
    fn from(u: u64) -> Self {
        Zobrist(u)
    }
}

impl From<Zobrist> for u64 {
    fn from(z: Zobrist) -> Self {
        z.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::move_generator::MoveGenerator;
    use crate::position_history::PositionHistory;
    use crate::r#move::{Move, MoveList, MoveType};

    #[test]
    fn hash_values_differ() {
        let mut hash_values = Vec::new();

        let mut pos_history = PositionHistory::new(Position::initial());
        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves(&mut move_list, pos_history.current_pos());
        for m in move_list.iter() {
            pos_history.do_move(*m);
            let hash = Zobrist::new(pos_history.current_pos());
            assert!(!hash_values.contains(&hash));
            hash_values.push(hash);
            pos_history.undo_last_move();
        }
    }

    #[test]
    fn hash_values_with_transposition_equal() {
        let mut hash_values = Vec::new();

        let mut pos_history = PositionHistory::new(Position::initial());
        let g2g3 = Move::new(Square::G2, Square::G3, MoveType::QUIET);
        let g1f3 = Move::new(Square::G1, Square::F3, MoveType::QUIET);
        let d7d5 = Move::new(Square::D7, Square::D5, MoveType::DOUBLE_PAWN_PUSH);

        pos_history.do_move(g2g3);
        pos_history.do_move(d7d5);
        pos_history.do_move(g1f3);
        let hash = Zobrist::new(pos_history.current_pos());
        assert!(!hash_values.contains(&hash));
        hash_values.push(hash);
        pos_history.undo_last_move();
        pos_history.undo_last_move();
        pos_history.undo_last_move();

        // Different move order
        pos_history.do_move(g1f3);
        pos_history.do_move(d7d5);
        pos_history.do_move(g2g3);
        let hash = Zobrist::new(pos_history.current_pos());
        assert!(hash_values.contains(&hash));
    }

    #[test]
    fn incremental_hash_values() {
        let mut pos_history = PositionHistory::new(Position::initial());
        let e2e4 = Move::new(Square::E2, Square::E4, MoveType::DOUBLE_PAWN_PUSH);
        let c7c5 = Move::new(Square::C7, Square::C5, MoveType::DOUBLE_PAWN_PUSH);

        pos_history.do_move(e2e4);
        let hash = Zobrist::new(pos_history.current_pos());
        assert_eq!(hash, pos_history.current_pos_hash());

        pos_history.do_move(c7c5);
        let hash = Zobrist::new(pos_history.current_pos());
        assert_eq!(hash, pos_history.current_pos_hash());

        pos_history.do_move(Move::NULL);
        let hash = Zobrist::new(pos_history.current_pos());
        assert_eq!(hash, pos_history.current_pos_hash());

        pos_history.undo_last_move();
        let hash = Zobrist::new(pos_history.current_pos());
        assert_eq!(hash, pos_history.current_pos_hash());

        pos_history.undo_last_move();
        let hash = Zobrist::new(pos_history.current_pos());
        assert_eq!(hash, pos_history.current_pos_hash());

        pos_history.undo_last_move();
        let hash = Zobrist::new(pos_history.current_pos());
        assert_eq!(hash, pos_history.current_pos_hash());
    }
}
