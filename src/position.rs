use crate::bishop::Bishop;
use crate::bitboard::Bitboard;
use crate::king::King;
use crate::knight::Knight;
use crate::pawn::Pawn;
use crate::piece;
use crate::piece::Piece;
use crate::queen::Queen;
use crate::rook::Rook;
use crate::side::Side;
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
    pawns: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queens: Bitboard,
    kings: Bitboard,
    white_pieces: Bitboard,
    black_pieces: Bitboard,
    en_passant_square: Bitboard,
    side_to_move: Side,
    castling_rights: CastlingRights,
    plies_since_pawn_move_or_capture: usize,
    move_count: usize,
}

impl Position {
    const PAWN_EAST_ATTACKS: [fn(Bitboard) -> Bitboard; 2] = [
        Pawn::white_east_attack_targets,
        Pawn::black_east_attack_targets,
    ];
    const PAWN_WEST_ATTACKS: [fn(Bitboard) -> Bitboard; 2] = [
        Pawn::white_west_attack_targets,
        Pawn::black_west_attack_targets,
    ];

    pub fn empty() -> Self {
        Position {
            pawns: Bitboard::EMPTY,
            knights: Bitboard::EMPTY,
            bishops: Bitboard::EMPTY,
            rooks: Bitboard::EMPTY,
            queens: Bitboard::EMPTY,
            kings: Bitboard::EMPTY,
            white_pieces: Bitboard::EMPTY,
            black_pieces: Bitboard::EMPTY,
            en_passant_square: Bitboard::EMPTY,
            side_to_move: Side::White,
            castling_rights: CastlingRights::empty(),
            plies_since_pawn_move_or_capture: 0,
            move_count: 1,
        }
    }

    pub fn initial() -> Self {
        Position {
            pawns: Bitboard::RANK_2 | Bitboard::RANK_7,
            knights: Bitboard::B1 | Bitboard::G1 | Bitboard::B8 | Bitboard::G8,
            bishops: Bitboard::C1 | Bitboard::F1 | Bitboard::C8 | Bitboard::F8,
            rooks: Bitboard::A1 | Bitboard::H1 | Bitboard::A8 | Bitboard::H8,
            queens: Bitboard::D1 | Bitboard::D8,
            kings: Bitboard::E1 | Bitboard::E8,
            white_pieces: Bitboard::RANK_1 | Bitboard::RANK_2,
            black_pieces: Bitboard::RANK_7 | Bitboard::RANK_8,
            en_passant_square: Bitboard::EMPTY,
            side_to_move: Side::White,
            castling_rights: CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            plies_since_pawn_move_or_capture: 0,
            move_count: 1,
        }
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

    pub fn piece_at(&self, square_index: usize) -> Option<Piece> {
        let square = Bitboard(0x1 << square_index);
        if square & self.white_pieces != Bitboard::EMPTY {
            if square & self.pawns != Bitboard::EMPTY {
                Some(Piece::WhitePawn)
            } else if square & self.knights != Bitboard::EMPTY {
                Some(Piece::WhiteKnight)
            } else if square & self.bishops != Bitboard::EMPTY {
                Some(Piece::WhiteBishop)
            } else if square & self.rooks != Bitboard::EMPTY {
                Some(Piece::WhiteRook)
            } else if square & self.queens != Bitboard::EMPTY {
                Some(Piece::WhiteQueen)
            } else {
                debug_assert_ne!(square & self.kings, Bitboard::EMPTY);
                Some(Piece::WhiteKing)
            }
        } else if square & self.black_pieces != Bitboard::EMPTY {
            if square & self.pawns != Bitboard::EMPTY {
                Some(Piece::BlackPawn)
            } else if square & self.knights != Bitboard::EMPTY {
                Some(Piece::BlackKnight)
            } else if square & self.bishops != Bitboard::EMPTY {
                Some(Piece::BlackBishop)
            } else if square & self.rooks != Bitboard::EMPTY {
                Some(Piece::BlackRook)
            } else if square & self.queens != Bitboard::EMPTY {
                Some(Piece::BlackQueen)
            } else {
                debug_assert_ne!(square & self.kings, Bitboard::EMPTY);
                Some(Piece::BlackKing)
            }
        } else {
            None
        }
    }

    pub fn set_piece_at(&mut self, square_index: usize, piece: Option<Piece>) {
        let square = Bitboard(0x1 << square_index);

        self.pawns &= !square;
        self.knights &= !square;
        self.bishops &= !square;
        self.rooks &= !square;
        self.queens &= !square;
        self.kings &= !square;
        self.white_pieces &= !square;
        self.black_pieces &= !square;

        match piece {
            Some(Piece::WhitePawn) => {
                self.white_pieces |= square;
                self.pawns |= square;
            }
            Some(Piece::WhiteKnight) => {
                self.white_pieces |= square;
                self.knights |= square;
            }
            Some(Piece::WhiteBishop) => {
                self.white_pieces |= square;
                self.bishops |= square;
            }
            Some(Piece::WhiteRook) => {
                self.white_pieces |= square;
                self.rooks |= square;
            }
            Some(Piece::WhiteQueen) => {
                self.white_pieces |= square;
                self.queens |= square;
            }
            Some(Piece::WhiteKing) => {
                self.white_pieces |= square;
                self.kings |= square;
            }
            Some(Piece::BlackPawn) => {
                self.black_pieces |= square;
                self.pawns |= square;
            }
            Some(Piece::BlackKnight) => {
                self.black_pieces |= square;
                self.knights |= square;
            }
            Some(Piece::BlackBishop) => {
                self.black_pieces |= square;
                self.bishops |= square;
            }
            Some(Piece::BlackRook) => {
                self.black_pieces |= square;
                self.rooks |= square;
            }
            Some(Piece::BlackQueen) => {
                self.black_pieces |= square;
                self.queens |= square;
            }
            Some(Piece::BlackKing) => {
                self.black_pieces |= square;
                self.kings |= square;
            }
            None => {}
        }
    }

    pub fn occupancy(&self) -> Bitboard {
        self.white_pieces | self.black_pieces
    }

    pub fn side_occupancy(&self, side: Side) -> Bitboard {
        match side {
            Side::White => self.white_pieces,
            Side::Black => self.black_pieces,
        }
    }

    pub fn piece_type_occupancy(&self, piece_type: piece::Type) -> Bitboard {
        match piece_type {
            piece::Type::Pawn => self.pawns,
            piece::Type::Knight => self.knights,
            piece::Type::Bishop => self.bishops,
            piece::Type::Rook => self.rooks,
            piece::Type::Queen => self.queens,
            piece::Type::King => self.kings,
        }
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

    fn pawn_attacks(&self, side: Side) -> Bitboard {
        let pawns = self.piece_occupancy(side, piece::Type::Pawn);
        let side_idx = side as usize;
        Self::PAWN_EAST_ATTACKS[side_idx](pawns) | Self::PAWN_WEST_ATTACKS[side_idx](pawns)
    }

    fn piece_attacks(
        &self,
        piece_type: piece::Type,
        piece_targets: &dyn Fn(usize) -> Bitboard,
        side: Side,
    ) -> Bitboard {
        let mut pieces = self.piece_occupancy(side, piece_type);
        let mut targets = Bitboard::EMPTY;
        while pieces != Bitboard::EMPTY {
            let origin = pieces.bit_scan_forward_reset();
            targets |= piece_targets(origin);
        }
        targets
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const EMPTY_SQUARE: u8 = b'-';
        const SPACE: u8 = b' ';
        let mut squares_in_rank = [SPACE; 2 * Bitboard::NUM_FILES - 1];
        for rank in (0..Bitboard::NUM_RANKS).rev() {
            for file in 0..Bitboard::NUM_FILES {
                let square = Bitboard::to_square(rank, file);
                squares_in_rank[2 * file] = match self.piece_at(square) {
                    None => EMPTY_SQUARE,
                    Some(piece) => piece.to_ascii(),
                };
            }
            let rank_str = str::from_utf8(&squares_in_rank).unwrap();
            writeln!(f, "{}", rank_str).unwrap();
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

        assert_eq!(Some(Piece::WhiteRook), pos.piece_at(Bitboard::IDX_A1));
        assert_eq!(Some(Piece::WhiteKnight), pos.piece_at(Bitboard::IDX_B1));
        assert_eq!(Some(Piece::WhiteBishop), pos.piece_at(Bitboard::IDX_C1));
        assert_eq!(Some(Piece::WhiteQueen), pos.piece_at(Bitboard::IDX_D1));
        assert_eq!(Some(Piece::WhiteKing), pos.piece_at(Bitboard::IDX_E1));
        assert_eq!(Some(Piece::WhitePawn), pos.piece_at(Bitboard::IDX_A2));

        assert_eq!(None, pos.piece_at(Bitboard::IDX_A3));
        assert_eq!(None, pos.piece_at(Bitboard::IDX_H6));

        assert_eq!(Some(Piece::BlackRook), pos.piece_at(Bitboard::IDX_A8));
        assert_eq!(Some(Piece::BlackKnight), pos.piece_at(Bitboard::IDX_B8));
        assert_eq!(Some(Piece::BlackBishop), pos.piece_at(Bitboard::IDX_C8));
        assert_eq!(Some(Piece::BlackQueen), pos.piece_at(Bitboard::IDX_D8));
        assert_eq!(Some(Piece::BlackKing), pos.piece_at(Bitboard::IDX_E8));
        assert_eq!(Some(Piece::BlackPawn), pos.piece_at(Bitboard::IDX_A7));

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
        pos.set_piece_at(Bitboard::IDX_E4, Some(Piece::WhitePawn));
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
        pos.set_piece_at(Bitboard::IDX_E1, Some(piece::Piece::WhiteKing));
        pos.set_piece_at(Bitboard::IDX_E2, Some(piece::Piece::WhitePawn));
        pos.set_piece_at(Bitboard::IDX_E8, Some(piece::Piece::BlackKing));
        pos.set_piece_at(Bitboard::IDX_D7, Some(piece::Piece::BlackPawn));
        pos.set_side_to_move(Side::White);
        pos.set_castling_rights(CastlingRights::empty());
        assert!(!pos.is_in_check(Side::White));
        assert!(!pos.is_in_check(Side::Black));

        // Pawn blocks the check
        pos.set_piece_at(Bitboard::IDX_E4, Some(piece::Piece::BlackRook));
        pos.set_piece_at(Bitboard::IDX_B5, Some(piece::Piece::WhiteBishop));
        assert!(!pos.is_in_check(Side::White));
        assert!(!pos.is_in_check(Side::Black));

        pos.set_piece_at(Bitboard::IDX_H1, Some(piece::Piece::BlackRook));
        pos.set_piece_at(Bitboard::IDX_H5, Some(piece::Piece::WhiteBishop));
        assert!(pos.is_in_check(Side::White));
        assert!(pos.is_in_check(Side::Black));
    }
}
