use crate::bishop::Bishop;
use crate::bitboard::Bitboard;
use crate::castling_squares::CastlingSquares;
use crate::file::File;
use crate::king::King;
use crate::knight::Knight;
use crate::pawn::Pawn;
use crate::piece;
use crate::piece::Piece;
use crate::queen::Queen;
use crate::r#move::Move;
use crate::r#move::MoveType;
use crate::rank::Rank;
use crate::rook::Rook;
use crate::side::Side;
use crate::square::Square;
use bitflags::bitflags;
use std::fmt;
use std::str;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CastlingRights: u8 {
        const WHITE_KINGSIDE = 0b00000001;
        const WHITE_QUEENSIDE = 0b00000010;
        const BLACK_KINGSIDE = 0b00000100;
        const BLACK_QUEENSIDE = 0b00001000;

        const WHITE_BOTH = Self::WHITE_KINGSIDE.bits() | Self::WHITE_QUEENSIDE.bits();
        const BLACK_BOTH = Self::BLACK_KINGSIDE.bits() | Self::BLACK_QUEENSIDE.bits();
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    piece_side_occupancies: [Bitboard; 2],
    piece_type_occupancies: [Bitboard; 6],
    en_passant_square: Bitboard,
    side_to_move: Side,
    castling_rights: CastlingRights,
    castling_files: u16,
    castling_squares: CastlingSquares,
    plies_since_pawn_move_or_capture: usize,
    move_count: usize,
}

impl Position {
    pub fn empty() -> Self {
        Position {
            piece_side_occupancies: [Bitboard::EMPTY; 2],
            piece_type_occupancies: [Bitboard::EMPTY; 6],
            en_passant_square: Bitboard::EMPTY,
            side_to_move: Side::White,
            castling_rights: CastlingRights::empty(),
            castling_files: File::E.idx() as u16
                | (File::H.idx() as u16) << 3
                | (File::A.idx() as u16) << 6,
            castling_squares: CastlingSquares::new(File::A, File::E, File::H),
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
            castling_files: File::E.idx() as u16
                | (File::H.idx() as u16) << 3
                | (File::A.idx() as u16) << 6,
            castling_squares: CastlingSquares::new(File::A, File::E, File::H),
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

    pub fn king_start_file(&self) -> File {
        File::from_idx((self.castling_files & 0x7) as usize)
    }

    pub fn kingside_rook_start_file(&self) -> File {
        File::from_idx((self.castling_files >> 3 & 0x7) as usize)
    }

    pub fn queenside_rook_start_file(&self) -> File {
        File::from_idx((self.castling_files >> 6 & 0x7) as usize)
    }

    pub fn castling_squares(&self) -> &CastlingSquares {
        &self.castling_squares
    }

    pub fn plies_since_pawn_move_or_capture(&self) -> usize {
        self.plies_since_pawn_move_or_capture
    }

    pub fn move_count(&self) -> usize {
        self.move_count
    }

    // Number of halfmoves played in the game
    //
    // Note: This is not the halfmove clock!
    pub fn halfmove_count(&self) -> usize {
        self.move_count() * 2 - !self.side_to_move() as usize
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

    pub fn set_king_start_file(&mut self, file: File) {
        self.castling_files &= 0b1_1111_1000;
        self.castling_files |= file.idx() as u16;
        self.set_castling_squares();
    }

    pub fn set_kingside_castling_file(&mut self, file: File) {
        self.castling_files &= 0b1_1100_0111;
        self.castling_files |= (file.idx() as u16) << 3;
        self.set_castling_squares();
    }

    pub fn set_queenside_castling_file(&mut self, file: File) {
        self.castling_files &= 0b0_0011_1111;
        self.castling_files |= (file.idx() as u16) << 6;
        self.set_castling_squares();
    }

    pub fn set_plies_since_pawn_move_or_capture(&mut self, plies: usize) {
        self.plies_since_pawn_move_or_capture = plies;
    }

    pub fn set_move_count(&mut self, move_count: usize) {
        debug_assert!(move_count > 0);
        self.move_count = move_count;
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        let square_bit = Bitboard::from(square);
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
        let square_bit = Bitboard::from(square);

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
        let own_king_bb = self.piece_occupancy(side, piece::Type::King);
        let own_king = own_king_bb.to_square();
        let enemy_pawns = self.piece_occupancy(!side, piece::Type::Pawn);
        let enemy_knights = self.piece_occupancy(!side, piece::Type::Knight);
        let enemy_bishops = self.piece_occupancy(!side, piece::Type::Bishop);
        let enemy_rooks = self.piece_occupancy(!side, piece::Type::Rook);
        let enemy_queens = self.piece_occupancy(!side, piece::Type::Queen);
        let occupied = self.occupancy();
        debug_assert_eq!(
            Bitboard::EMPTY,
            {
                let enemy_king = self.piece_occupancy(!side, piece::Type::King);
                King::targets(own_king) & enemy_king
            },
            "Kings attack each other, illegal position:\n{self}",
        );
        Pawn::attack_targets(enemy_pawns, !side) & own_king_bb != Bitboard::EMPTY
            || Knight::targets(own_king) & enemy_knights != Bitboard::EMPTY
            || Bishop::targets(own_king, occupied) & (enemy_bishops | enemy_queens)
                != Bitboard::EMPTY
            || Rook::targets(own_king, occupied) & (enemy_rooks | enemy_queens) != Bitboard::EMPTY
    }

    pub fn gives_check(&self, m: Move) -> bool {
        // Add and remove pieces to get the occupancy after the move
        let origin_bb = Bitboard::from(m.origin());
        let target_bb = Bitboard::from(m.target());
        let mut occupied = self.occupancy() & !origin_bb | target_bb;
        let mut own_diag_sliders = occupied
            & (self.piece_occupancy(self.side_to_move(), piece::Type::Bishop)
                | self.piece_occupancy(self.side_to_move(), piece::Type::Queen));
        let mut own_line_sliders = occupied
            & (self.piece_occupancy(self.side_to_move(), piece::Type::Rook)
                | self.piece_occupancy(self.side_to_move(), piece::Type::Queen));
        if m.is_en_passant() {
            occupied &= !self.en_passant_square();
        }
        // Update the rook position after castling
        if m.is_castle() {
            let start_rank = m.origin().rank();
            let (rook_origin, rook_target) = match m.move_type() {
                MoveType::CASTLE_KINGSIDE => (
                    Bitboard::from(Square::from((self.kingside_rook_start_file(), start_rank))),
                    Bitboard::from(Square::from((File::F, start_rank))),
                ),
                MoveType::CASTLE_QUEENSIDE => (
                    Bitboard::from(Square::from((self.queenside_rook_start_file(), start_rank))),
                    Bitboard::from(Square::from((File::D, start_rank))),
                ),
                _ => unreachable!(),
            };
            occupied = occupied & !rook_origin | rook_target;
            own_line_sliders = own_line_sliders & !rook_origin | rook_target;
        }

        // Get the moved piece type, considering promotions
        let enemy_king_bb = self.piece_occupancy(!self.side_to_move(), piece::Type::King);
        let target_piece_type = if m.is_promotion() {
            m.promotion_piece().unwrap()
        } else {
            self.piece_at(m.origin()).unwrap().piece_type()
        };
        match target_piece_type {
            // If the moved pawn or knight gives check, there is no need to
            // calculate slider attacks and we can return immediately
            piece::Type::Pawn => {
                if enemy_king_bb & Pawn::attack_targets(target_bb, self.side_to_move())
                    != Bitboard::EMPTY
                {
                    return true;
                }
            }
            piece::Type::Knight => {
                if enemy_king_bb & Knight::targets(m.target()) != Bitboard::EMPTY {
                    return true;
                }
            }
            // Add the moved slider to the respective bitboard, so that we can
            // calculate attacks by the moved piece together with discovered
            // attacks
            piece::Type::Bishop => own_diag_sliders |= target_bb,
            piece::Type::Rook => own_line_sliders |= target_bb,
            piece::Type::Queen => {
                own_diag_sliders |= target_bb;
                own_line_sliders |= target_bb;
            }
            // Our king cannot attack the enemy king
            piece::Type::King => {}
        };

        // Check if a slider attacks the enemy king.
        // This includes the moved piece and discovered attacks.
        let enemy_king = enemy_king_bb.to_square();
        Bishop::targets(enemy_king, occupied) & own_diag_sliders != Bitboard::EMPTY
            || Rook::targets(enemy_king, occupied) & own_line_sliders != Bitboard::EMPTY
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
        let removed_castling_rights = match square.rank() {
            Rank::R1 => {
                if square.file() == self.kingside_rook_start_file() {
                    CastlingRights::WHITE_KINGSIDE
                } else if square.file() == self.queenside_rook_start_file() {
                    CastlingRights::WHITE_QUEENSIDE
                } else if square.file() == self.king_start_file() {
                    CastlingRights::WHITE_BOTH
                } else {
                    CastlingRights::empty()
                }
            }
            Rank::R8 => {
                if square.file() == self.kingside_rook_start_file() {
                    CastlingRights::BLACK_KINGSIDE
                } else if square.file() == self.queenside_rook_start_file() {
                    CastlingRights::BLACK_QUEENSIDE
                } else if square.file() == self.king_start_file() {
                    CastlingRights::BLACK_BOTH
                } else {
                    CastlingRights::empty()
                }
            }
            _ => CastlingRights::empty(),
        };
        self.set_castling_rights(self.castling_rights() & !removed_castling_rights);
    }

    pub fn has_minor_or_major_piece(&self, side: Side) -> bool {
        (self.piece_occupancy(side, piece::Type::King)
            | self.piece_occupancy(side, piece::Type::Pawn))
            != self.side_occupancy(side)
    }

    pub fn has_bishop_pair(&self, side: Side) -> bool {
        let bishops = self.piece_occupancy(side, piece::Type::Bishop);
        bishops & Bitboard::LIGHT_SQUARES != Bitboard::EMPTY
            && bishops & Bitboard::DARK_SQUARES != Bitboard::EMPTY
    }

    fn set_castling_squares(&mut self) {
        self.castling_squares = CastlingSquares::new(
            self.queenside_rook_start_file(),
            self.king_start_file(),
            self.kingside_rook_start_file(),
        );
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
                let square = Square::from((File::from_idx(file), Rank::from_idx(rank)));
                squares_in_rank[2 * file] = match self.piece_at(square) {
                    None => EMPTY_SQUARE,
                    Some(piece) => piece.to_ascii(),
                };
            }
            let rank_str = str::from_utf8(&squares_in_rank).unwrap();
            writeln!(f, "{rank_str}")?;
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
        assert_eq!(File::H, pos.kingside_rook_start_file());
        assert_eq!(File::A, pos.queenside_rook_start_file());
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

    #[test]
    fn castling_files() {
        let mut pos = Position::empty();
        assert_eq!(File::E, pos.king_start_file());
        assert_eq!(File::H, pos.kingside_rook_start_file());
        assert_eq!(File::A, pos.queenside_rook_start_file());

        pos.set_king_start_file(File::D);
        assert_eq!(File::D, pos.king_start_file());
        assert_eq!(File::H, pos.kingside_rook_start_file());
        assert_eq!(File::A, pos.queenside_rook_start_file());

        pos.set_kingside_castling_file(File::F);
        assert_eq!(File::D, pos.king_start_file());
        assert_eq!(File::F, pos.kingside_rook_start_file());
        assert_eq!(File::A, pos.queenside_rook_start_file());

        pos.set_queenside_castling_file(File::C);
        assert_eq!(File::D, pos.king_start_file());
        assert_eq!(File::F, pos.kingside_rook_start_file());
        assert_eq!(File::C, pos.queenside_rook_start_file());
    }

    #[test]
    fn has_minor_or_major_piece() {
        let pos = Position::initial();
        assert!(pos.has_minor_or_major_piece(Side::White));
        assert!(pos.has_minor_or_major_piece(Side::Black));

        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        assert!(!pos.has_minor_or_major_piece(Side::White));
        assert!(!pos.has_minor_or_major_piece(Side::Black));

        let mut pos_white_pawn = pos.clone();
        pos_white_pawn.set_piece_at(Square::G1, Some(piece::Piece::WHITE_PAWN));
        assert!(!pos_white_pawn.has_minor_or_major_piece(Side::White));
        assert!(!pos_white_pawn.has_minor_or_major_piece(Side::Black));
        let mut pos_black_pawn = pos.clone();
        pos_black_pawn.set_piece_at(Square::G8, Some(piece::Piece::BLACK_PAWN));
        assert!(!pos_black_pawn.has_minor_or_major_piece(Side::White));
        assert!(!pos_black_pawn.has_minor_or_major_piece(Side::Black));

        let mut pos_white_knight = pos.clone();
        pos_white_knight.set_piece_at(Square::G1, Some(piece::Piece::WHITE_KNIGHT));
        assert!(pos_white_knight.has_minor_or_major_piece(Side::White));
        assert!(!pos_white_knight.has_minor_or_major_piece(Side::Black));
        let mut pos_black_knight = pos.clone();
        pos_black_knight.set_piece_at(Square::G8, Some(piece::Piece::BLACK_KNIGHT));
        assert!(!pos_black_knight.has_minor_or_major_piece(Side::White));
        assert!(pos_black_knight.has_minor_or_major_piece(Side::Black));

        let mut pos_white_bishop = pos.clone();
        pos_white_bishop.set_piece_at(Square::F1, Some(piece::Piece::WHITE_BISHOP));
        assert!(pos_white_bishop.has_minor_or_major_piece(Side::White));
        assert!(!pos_white_bishop.has_minor_or_major_piece(Side::Black));
        let mut pos_black_bishop = pos.clone();
        pos_black_bishop.set_piece_at(Square::F8, Some(piece::Piece::BLACK_BISHOP));
        assert!(!pos_black_bishop.has_minor_or_major_piece(Side::White));
        assert!(pos_black_bishop.has_minor_or_major_piece(Side::Black));

        let mut pos_white_rook = pos.clone();
        pos_white_rook.set_piece_at(Square::H1, Some(piece::Piece::WHITE_ROOK));
        assert!(pos_white_rook.has_minor_or_major_piece(Side::White));
        assert!(!pos_white_rook.has_minor_or_major_piece(Side::Black));
        let mut pos_black_rook = pos.clone();
        pos_black_rook.set_piece_at(Square::H8, Some(piece::Piece::BLACK_ROOK));
        assert!(!pos_black_rook.has_minor_or_major_piece(Side::White));
        assert!(pos_black_rook.has_minor_or_major_piece(Side::Black));

        let mut pos_white_queen = pos.clone();
        pos_white_queen.set_piece_at(Square::D1, Some(piece::Piece::WHITE_QUEEN));
        assert!(pos_white_queen.has_minor_or_major_piece(Side::White));
        assert!(!pos_white_queen.has_minor_or_major_piece(Side::Black));
        let mut pos_black_queen = pos.clone();
        pos_black_queen.set_piece_at(Square::D8, Some(piece::Piece::BLACK_QUEEN));
        assert!(!pos_black_queen.has_minor_or_major_piece(Side::White));
        assert!(pos_black_queen.has_minor_or_major_piece(Side::Black));
    }

    #[test]
    fn has_bishop_pair() {
        let pos = Position::initial();
        assert!(pos.has_bishop_pair(Side::White));
        assert!(pos.has_bishop_pair(Side::Black));

        let mut pos = Position::empty();
        pos.set_piece_at(Square::E1, Some(piece::Piece::WHITE_KING));
        pos.set_piece_at(Square::E8, Some(piece::Piece::BLACK_KING));
        assert!(!pos.has_bishop_pair(Side::White));
        assert!(!pos.has_bishop_pair(Side::Black));

        pos.set_piece_at(Square::A1, Some(piece::Piece::WHITE_BISHOP));
        pos.set_piece_at(Square::A8, Some(piece::Piece::BLACK_BISHOP));
        assert!(!pos.has_bishop_pair(Side::White));
        assert!(!pos.has_bishop_pair(Side::Black));

        pos.set_piece_at(Square::H1, Some(piece::Piece::WHITE_BISHOP));
        pos.set_piece_at(Square::H8, Some(piece::Piece::BLACK_BISHOP));
        assert!(pos.has_bishop_pair(Side::White));
        assert!(pos.has_bishop_pair(Side::Black));

        pos.set_piece_at(Square::A1, None);
        pos.set_piece_at(Square::A8, None);
        assert!(!pos.has_bishop_pair(Side::White));
        assert!(!pos.has_bishop_pair(Side::Black));
    }

    #[test]
    fn halfmove_count() {
        let mut pos = Position::empty();
        assert_eq!(1, pos.move_count());
        assert_eq!(1, pos.halfmove_count());

        pos.set_side_to_move(Side::Black);
        assert_eq!(1, pos.move_count());
        assert_eq!(2, pos.halfmove_count());

        pos.set_move_count(2);
        pos.set_side_to_move(Side::White);
        assert_eq!(2, pos.move_count());
        assert_eq!(3, pos.halfmove_count());

        pos.set_side_to_move(Side::Black);
        assert_eq!(2, pos.move_count());
        assert_eq!(4, pos.halfmove_count());

        pos.set_move_count(10);
        pos.set_side_to_move(Side::White);
        assert_eq!(10, pos.move_count());
        assert_eq!(19, pos.halfmove_count());

        pos.set_side_to_move(Side::Black);
        assert_eq!(10, pos.move_count());
        assert_eq!(20, pos.halfmove_count());
    }
}
