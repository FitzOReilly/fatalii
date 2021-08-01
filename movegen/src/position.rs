use crate::bishop::Bishop;
use crate::bitboard::Bitboard;
use crate::file::File;
use crate::king::King;
use crate::knight::Knight;
use crate::pawn::Pawn;
use crate::piece;
use crate::piece::Piece;
use crate::queen::Queen;
use crate::rank::Rank;
use crate::rook::Rook;
use crate::side::Side;
use crate::square::Square;
use std::fmt;
use std::str;

bitflags! {
    pub struct CastlingRights: u8 {
        const WHITE_KINGSIDE = 0b00000001;
        const WHITE_QUEENSIDE = 0b00000010;
        const BLACK_KINGSIDE = 0b00000100;
        const BLACK_QUEENSIDE = 0b00001000;

        const WHITE_BOTH = Self::WHITE_KINGSIDE.bits | Self::WHITE_QUEENSIDE.bits;
        const BLACK_BOTH = Self::BLACK_KINGSIDE.bits | Self::BLACK_QUEENSIDE.bits;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    piece_side_occupancies: [Bitboard; 2],
    piece_type_occupancies: [Bitboard; 6],
    en_passant_square: Bitboard,
    side_to_move: Side,
    castling_rights: CastlingRights,
    plies_since_pawn_move_or_capture: usize,
    move_count: usize,
}

impl Position {
    pub const fn empty() -> Self {
        Position {
            piece_side_occupancies: [Bitboard::EMPTY; 2],
            piece_type_occupancies: [Bitboard::EMPTY; 6],
            en_passant_square: Bitboard::EMPTY,
            side_to_move: Side::White,
            castling_rights: CastlingRights::empty(),
            plies_since_pawn_move_or_capture: 0,
            move_count: 1,
        }
    }

    pub fn initial() -> Self {
        let mut pos = Position {
            piece_side_occupancies: [Bitboard::EMPTY; 2],
            piece_type_occupancies: [Bitboard::EMPTY; 6],
            en_passant_square: Bitboard::EMPTY,
            side_to_move: Side::White,
            castling_rights: CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            plies_since_pawn_move_or_capture: 0,
            move_count: 1,
        };

        pos.piece_side_occupancies[Side::White as usize] = Bitboard::RANK_1 | Bitboard::RANK_2;
        pos.piece_side_occupancies[Side::Black as usize] = Bitboard::RANK_7 | Bitboard::RANK_8;
        pos.piece_type_occupancies[piece::Type::Pawn as usize] =
            Bitboard::RANK_2 | Bitboard::RANK_7;
        pos.piece_type_occupancies[piece::Type::Knight as usize] =
            Bitboard::B1 | Bitboard::G1 | Bitboard::B8 | Bitboard::G8;
        pos.piece_type_occupancies[piece::Type::Bishop as usize] =
            Bitboard::C1 | Bitboard::F1 | Bitboard::C8 | Bitboard::F8;
        pos.piece_type_occupancies[piece::Type::Rook as usize] =
            Bitboard::A1 | Bitboard::H1 | Bitboard::A8 | Bitboard::H8;
        pos.piece_type_occupancies[piece::Type::Queen as usize] = Bitboard::D1 | Bitboard::D8;
        pos.piece_type_occupancies[piece::Type::King as usize] = Bitboard::E1 | Bitboard::E8;

        pos
    }

    pub fn en_passant_square(&self) -> Bitboard {
        self.en_passant_square
    }

    pub fn side_to_move(&self) -> Side {
        self.side_to_move
    }

    pub fn castling_rights(&self) -> CastlingRights {
        self.castling_rights
    }

    pub fn plies_since_pawn_move_or_capture(&self) -> usize {
        self.plies_since_pawn_move_or_capture
    }

    pub fn move_count(&self) -> usize {
        self.move_count
    }

    pub fn set_en_passant_square(&mut self, square_bit: Bitboard) {
        self.en_passant_square = square_bit;
    }

    pub fn set_side_to_move(&mut self, side: Side) {
        self.side_to_move = side;
    }

    pub fn set_castling_rights(&mut self, castling_rights: CastlingRights) {
        self.castling_rights = castling_rights;
    }

    pub fn set_plies_since_pawn_move_or_capture(&mut self, plies: usize) {
        self.plies_since_pawn_move_or_capture = plies;
    }

    pub fn set_move_count(&mut self, move_count: usize) {
        self.move_count = move_count;
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        let square_bit = Bitboard::from_square(square);
        let side = if square_bit & self.piece_side_occupancies[Side::White as usize]
            != Bitboard::EMPTY
        {
            Some(Side::White)
        } else if square_bit & self.piece_side_occupancies[Side::Black as usize] != Bitboard::EMPTY
        {
            Some(Side::Black)
        } else {
            None
        };

        match side {
            Some(piece_side) => {
                let piece_type = if square_bit
                    & self.piece_type_occupancies[piece::Type::Pawn as usize]
                    != Bitboard::EMPTY
                {
                    piece::Type::Pawn
                } else if square_bit & self.piece_type_occupancies[piece::Type::Knight as usize]
                    != Bitboard::EMPTY
                {
                    piece::Type::Knight
                } else if square_bit & self.piece_type_occupancies[piece::Type::Bishop as usize]
                    != Bitboard::EMPTY
                {
                    piece::Type::Bishop
                } else if square_bit & self.piece_type_occupancies[piece::Type::Rook as usize]
                    != Bitboard::EMPTY
                {
                    piece::Type::Rook
                } else if square_bit & self.piece_type_occupancies[piece::Type::Queen as usize]
                    != Bitboard::EMPTY
                {
                    piece::Type::Queen
                } else {
                    debug_assert_ne!(
                        square_bit & self.piece_type_occupancies[piece::Type::King as usize],
                        Bitboard::EMPTY
                    );
                    piece::Type::King
                };
                Some(Piece::new(piece_side, piece_type))
            }
            None => None,
        }
    }

    pub fn set_piece_at(&mut self, square: Square, piece: Option<Piece>) {
        let square_bit = Bitboard::from_square(square);

        for pso in &mut self.piece_side_occupancies {
            *pso &= !square_bit;
        }
        for pto in &mut self.piece_type_occupancies {
            *pto &= !square_bit;
        }

        if let Some(p) = piece {
            self.piece_side_occupancies[p.piece_side() as usize] |= square_bit;
            self.piece_type_occupancies[p.piece_type() as usize] |= square_bit;
        }
    }

    pub fn occupancy(&self) -> Bitboard {
        self.piece_side_occupancies[Side::White as usize]
            | self.piece_side_occupancies[Side::Black as usize]
    }

    pub fn side_occupancy(&self, side: Side) -> Bitboard {
        self.piece_side_occupancies[side as usize]
    }

    pub fn piece_type_occupancy(&self, piece_type: piece::Type) -> Bitboard {
        self.piece_type_occupancies[piece_type as usize]
    }

    pub fn piece_occupancy(&self, side: Side, piece_type: piece::Type) -> Bitboard {
        self.side_occupancy(side) & self.piece_type_occupancy(piece_type)
    }

    pub fn is_in_check(&self, side: Side) -> bool {
        self.piece_occupancy(side, piece::Type::King) & self.attacked_squares(!side)
            != Bitboard::EMPTY
    }

    pub fn attacked_squares(&self, attacking_side: Side) -> Bitboard {
        let occupancy = self.occupancy();

        self.pawn_attacks(attacking_side)
            | self.piece_attacks(piece::Type::Knight, &Knight::targets, attacking_side)
            | self.piece_attacks(
                piece::Type::Bishop,
                &|origin| Bishop::targets(origin, occupancy),
                attacking_side,
            )
            | self.piece_attacks(
                piece::Type::Rook,
                &|origin| Rook::targets(origin, occupancy),
                attacking_side,
            )
            | self.piece_attacks(
                piece::Type::Queen,
                &|origin| Queen::targets(origin, occupancy),
                attacking_side,
            )
            | self.piece_attacks(piece::Type::King, &King::targets, attacking_side)
    }

    pub fn remove_castling_rights(&mut self, square: Square) {
        let removed_castling_rights = match square {
            Square::A1 => CastlingRights::WHITE_QUEENSIDE,
            Square::H1 => CastlingRights::WHITE_KINGSIDE,
            Square::E1 => CastlingRights::WHITE_BOTH,
            Square::A8 => CastlingRights::BLACK_QUEENSIDE,
            Square::H8 => CastlingRights::BLACK_KINGSIDE,
            Square::E8 => CastlingRights::BLACK_BOTH,
            _ => CastlingRights::empty(),
        };
        self.set_castling_rights(self.castling_rights() & !removed_castling_rights);
    }

    fn pawn_attacks(&self, side: Side) -> Bitboard {
        let pawns = self.piece_occupancy(side, piece::Type::Pawn);
        Pawn::attack_targets(pawns, side)
    }

    fn piece_attacks(
        &self,
        piece_type: piece::Type,
        piece_targets: &impl Fn(Square) -> Bitboard,
        side: Side,
    ) -> Bitboard {
        let mut pieces = self.piece_occupancy(side, piece_type);
        let mut targets = Bitboard::EMPTY;
        while pieces != Bitboard::EMPTY {
            let origin = pieces.square_scan_forward_reset();
            targets |= piece_targets(origin);
        }
        targets
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const EMPTY_SQUARE: u8 = b'-';
        const SPACE: u8 = b' ';
        let mut squares_in_rank = [SPACE; 2 * File::NUM_FILES - 1];
        for rank in (0..Rank::NUM_RANKS).rev() {
            for file in 0..File::NUM_FILES {
                let square = Square::from_file_and_rank(File::from_idx(file), Rank::from_idx(rank));
                squares_in_rank[2 * file] = match self.piece_at(square) {
                    None => EMPTY_SQUARE,
                    Some(piece) => piece.to_ascii(),
                };
            }
            let rank_str = str::from_utf8(&squares_in_rank).unwrap();
            writeln!(f, "{}", rank_str)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_position() {
        let pos = Position::initial();
        assert_eq!(Bitboard::EMPTY, pos.en_passant_square());
        assert_eq!(Side::White, pos.side_to_move());
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            pos.castling_rights()
        );
        assert_eq!(0, pos.plies_since_pawn_move_or_capture());
        assert_eq!(1, pos.move_count());

        assert_eq!(Some(Piece::WHITE_ROOK), pos.piece_at(Square::A1));
        assert_eq!(Some(Piece::WHITE_KNIGHT), pos.piece_at(Square::B1));
        assert_eq!(Some(Piece::WHITE_BISHOP), pos.piece_at(Square::C1));
        assert_eq!(Some(Piece::WHITE_QUEEN), pos.piece_at(Square::D1));
        assert_eq!(Some(Piece::WHITE_KING), pos.piece_at(Square::E1));
        assert_eq!(Some(Piece::WHITE_PAWN), pos.piece_at(Square::A2));

        assert_eq!(None, pos.piece_at(Square::A3));
        assert_eq!(None, pos.piece_at(Square::H6));

        assert_eq!(Some(Piece::BLACK_ROOK), pos.piece_at(Square::A8));
        assert_eq!(Some(Piece::BLACK_KNIGHT), pos.piece_at(Square::B8));
        assert_eq!(Some(Piece::BLACK_BISHOP), pos.piece_at(Square::C8));
        assert_eq!(Some(Piece::BLACK_QUEEN), pos.piece_at(Square::D8));
        assert_eq!(Some(Piece::BLACK_KING), pos.piece_at(Square::E8));
        assert_eq!(Some(Piece::BLACK_PAWN), pos.piece_at(Square::A7));

        assert_eq!(
            Bitboard::RANK_2,
            pos.piece_occupancy(Side::White, piece::Type::Pawn)
        );
        assert_eq!(
            Bitboard::B1 | Bitboard::G1,
            pos.piece_occupancy(Side::White, piece::Type::Knight)
        );
        assert_eq!(
            Bitboard::C1 | Bitboard::F1,
            pos.piece_occupancy(Side::White, piece::Type::Bishop)
        );
        assert_eq!(
            Bitboard::A1 | Bitboard::H1,
            pos.piece_occupancy(Side::White, piece::Type::Rook)
        );
        assert_eq!(
            Bitboard::D1,
            pos.piece_occupancy(Side::White, piece::Type::Queen)
        );
        assert_eq!(
            Bitboard::E1,
            pos.piece_occupancy(Side::White, piece::Type::King)
        );
        assert_eq!(
            Bitboard::RANK_7,
            pos.piece_occupancy(Side::Black, piece::Type::Pawn)
        );
        assert_eq!(
            Bitboard::B8 | Bitboard::G8,
            pos.piece_occupancy(Side::Black, piece::Type::Knight)
        );
        assert_eq!(
            Bitboard::C8 | Bitboard::F8,
            pos.piece_occupancy(Side::Black, piece::Type::Bishop)
        );
        assert_eq!(
            Bitboard::A8 | Bitboard::H8,
            pos.piece_occupancy(Side::Black, piece::Type::Rook)
        );
        assert_eq!(
            Bitboard::D8,
            pos.piece_occupancy(Side::Black, piece::Type::Queen)
        );
        assert_eq!(
            Bitboard::E8,
            pos.piece_occupancy(Side::Black, piece::Type::King)
        );

        assert_eq!(
            Bitboard::RANK_2 | Bitboard::RANK_7,
            pos.piece_type_occupancy(piece::Type::Pawn)
        );
        assert_eq!(
            Bitboard::B1 | Bitboard::G1 | Bitboard::B8 | Bitboard::G8,
            pos.piece_type_occupancy(piece::Type::Knight)
        );
        assert_eq!(
            Bitboard::C1 | Bitboard::F1 | Bitboard::C8 | Bitboard::F8,
            pos.piece_type_occupancy(piece::Type::Bishop)
        );
        assert_eq!(
            Bitboard::A1 | Bitboard::H1 | Bitboard::A8 | Bitboard::H8,
            pos.piece_type_occupancy(piece::Type::Rook)
        );
        assert_eq!(
            Bitboard::D1 | Bitboard::D8,
            pos.piece_type_occupancy(piece::Type::Queen)
        );
        assert_eq!(
            Bitboard::E1 | Bitboard::E8,
            pos.piece_type_occupancy(piece::Type::King)
        );

        assert_eq!(
            Bitboard::RANK_1 | Bitboard::RANK_2,
            pos.side_occupancy(Side::White)
        );
        assert_eq!(
            Bitboard::RANK_7 | Bitboard::RANK_8,
            pos.side_occupancy(Side::Black)
        );

        assert_eq!(
            Bitboard::RANK_1 | Bitboard::RANK_2 | Bitboard::RANK_7 | Bitboard::RANK_8,
            pos.occupancy()
        );
    }

    #[test]
    fn set_piece_at() {
        let mut pos = Position::initial();
        pos.set_piece_at(Square::E4, Some(Piece::WHITE_PAWN));
        let square = Bitboard::E4;
        assert_eq!(
            square,
            square & pos.piece_occupancy(Side::White, piece::Type::Pawn)
        );
        assert_eq!(Bitboard::EMPTY, square & pos.side_occupancy(Side::Black));
        assert_eq!(
            Bitboard::EMPTY,
            square & pos.piece_type_occupancy(piece::Type::Knight)
        );
        assert_eq!(
            Bitboard::EMPTY,
            square & pos.piece_type_occupancy(piece::Type::Bishop)
        );
        assert_eq!(
            Bitboard::EMPTY,
            square & pos.piece_type_occupancy(piece::Type::Rook)
        );
        assert_eq!(
            Bitboard::EMPTY,
            square & pos.piece_type_occupancy(piece::Type::Queen)
        );
        assert_eq!(
            Bitboard::EMPTY,
            square & pos.piece_type_occupancy(piece::Type::King)
        );
        assert_eq!(square, square & pos.occupancy());
        pos.set_piece_at(Square::E4, None);
        assert_eq!(Bitboard::EMPTY, square & pos.occupancy());
    }

    #[test]
    fn fmt() {
        let expected_str = "\
            r n b q k b n r\n\
            p p p p p p p p\n\
            - - - - - - - -\n\
            - - - - - - - -\n\
            - - - - - - - -\n\
            - - - - - - - -\n\
            P P P P P P P P\n\
            R N B Q K B N R\n\
        ";
        assert_eq!(expected_str, format!("{}", Position::initial()));
    }

    #[test]
    fn attacked_squares() {
        let pos = Position::initial();

        let attacked_by_white = Bitboard::B1
            | Bitboard::C1
            | Bitboard::D1
            | Bitboard::E1
            | Bitboard::F1
            | Bitboard::G1
            | Bitboard::RANK_2
            | Bitboard::RANK_3;
        assert_eq!(attacked_by_white, pos.attacked_squares(Side::White));

        let attacked_by_black = Bitboard::B8
            | Bitboard::C8
            | Bitboard::D8
            | Bitboard::E8
            | Bitboard::F8
            | Bitboard::G8
            | Bitboard::RANK_7
            | Bitboard::RANK_6;
        assert_eq!(attacked_by_black, pos.attacked_squares(Side::Black));
    }

    #[test]
    fn is_in_check() {
        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::E2, Some(piece::Piece::WHITE_PAWN));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        pos.set_piece_at(Square::D7, Some(piece::Piece::BLACK_PAWN));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        assert!(!pos.is_in_check(Side::White));
        assert!(!pos.is_in_check(Side::Black));

        // Pawn blocks the check
        pos.set_piece_at(Square::E4, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Square::B5, Some(piece::Piece::WHITE_BISHOP));
        assert!(!pos.is_in_check(Side::White));
        assert!(!pos.is_in_check(Side::Black));

        pos.set_piece_at(Square::H1, Some(piece::Piece::BLACK_ROOK));
        pos.set_piece_at(Square::H5, Some(piece::Piece::WHITE_BISHOP));
        assert!(pos.is_in_check(Side::White));
        assert!(pos.is_in_check(Side::Black));
    }

    #[test]
    fn remove_castling_rights() {
        let mut pos = Position::initial();
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            pos.castling_rights()
        );
        pos.remove_castling_rights(Square::D4);
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            pos.castling_rights()
        );
        pos.remove_castling_rights(Square::E1);
        assert_eq!(CastlingRights::BLACK_BOTH, pos.castling_rights());
        pos.remove_castling_rights(Square::A8);
        assert_eq!(CastlingRights::BLACK_KINGSIDE, pos.castling_rights());
        pos.remove_castling_rights(Square::H8);
        assert_eq!(CastlingRights::empty(), pos.castling_rights());
    }
}
