use std::{
    iter,
    ops::{Deref, DerefMut},
};

use movegen::{
    file::File,
    piece::{self, Piece},
    rank::Rank,
    side::Side,
    square::Square,
};

use crate::{params, Score};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Coeff(pub i8);

impl Deref for Coeff {
    type Target = i8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Coeff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<i8> for Coeff {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl From<Score> for Coeff {
    fn from(value: Score) -> Self {
        Self(value as i8)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HandCraftedEvalCoeffs {
    pub game_phase: usize,
    pub pst_pawn: [Coeff; 32],
    pub pst_knight: [Coeff; 32],
    pub pst_bishop: [Coeff; 32],
    pub pst_rook: [Coeff; 32],
    pub pst_queen: [Coeff; 32],
    pub pst_king: [Coeff; 32],
    pub tempo: Coeff,
    pub passed_pawn: [Coeff; 32],
    pub isolated_pawn: Coeff,
    pub backward_pawn: Coeff,
    pub doubled_pawn: Coeff,
    pub knight_mobility: [Coeff; params::KNIGHT_MOB_LEN],
    pub bishop_mobility: [Coeff; params::BISHOP_MOB_LEN],
    pub rook_mobility: [Coeff; params::ROOK_MOB_LEN],
    pub queen_mobility: [Coeff; params::QUEEN_MOB_LEN],
    pub bishop_pair: Coeff,
    pub pawn_square_relative_to_friendly_king: [Coeff; params::SQUARE_RELATIVE_TO_KING_LEN],
    pub pawn_square_relative_to_enemy_king: [Coeff; params::SQUARE_RELATIVE_TO_KING_LEN],
    pub knight_square_relative_to_friendly_king: [Coeff; params::SQUARE_RELATIVE_TO_KING_LEN],
    pub knight_square_relative_to_enemy_king: [Coeff; params::SQUARE_RELATIVE_TO_KING_LEN],
    pub bishop_square_relative_to_friendly_king: [Coeff; params::SQUARE_RELATIVE_TO_KING_LEN],
    pub bishop_square_relative_to_enemy_king: [Coeff; params::SQUARE_RELATIVE_TO_KING_LEN],
    pub rook_square_relative_to_friendly_king: [Coeff; params::SQUARE_RELATIVE_TO_KING_LEN],
    pub rook_square_relative_to_enemy_king: [Coeff; params::SQUARE_RELATIVE_TO_KING_LEN],
    pub queen_square_relative_to_friendly_king: [Coeff; params::SQUARE_RELATIVE_TO_KING_LEN],
    pub queen_square_relative_to_enemy_king: [Coeff; params::SQUARE_RELATIVE_TO_KING_LEN],
}

impl Default for HandCraftedEvalCoeffs {
    fn default() -> Self {
        Self {
            game_phase: Default::default(),
            pst_pawn: Default::default(),
            pst_knight: Default::default(),
            pst_bishop: Default::default(),
            pst_rook: Default::default(),
            pst_queen: Default::default(),
            pst_king: Default::default(),
            tempo: Default::default(),
            passed_pawn: Default::default(),
            isolated_pawn: Default::default(),
            backward_pawn: Default::default(),
            doubled_pawn: Default::default(),
            knight_mobility: Default::default(),
            bishop_mobility: Default::default(),
            rook_mobility: Default::default(),
            queen_mobility: Default::default(),
            bishop_pair: Default::default(),
            pawn_square_relative_to_friendly_king: [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN],
            pawn_square_relative_to_enemy_king: [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN],
            knight_square_relative_to_friendly_king: [Coeff(0);
                params::SQUARE_RELATIVE_TO_KING_LEN],
            knight_square_relative_to_enemy_king: [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN],
            bishop_square_relative_to_friendly_king: [Coeff(0);
                params::SQUARE_RELATIVE_TO_KING_LEN],
            bishop_square_relative_to_enemy_king: [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN],
            rook_square_relative_to_friendly_king: [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN],
            rook_square_relative_to_enemy_king: [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN],
            queen_square_relative_to_friendly_king: [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN],
            queen_square_relative_to_enemy_king: [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN],
        }
    }
}

impl HandCraftedEvalCoeffs {
    pub fn add_pst(&mut self, p: Piece, square: Square, diff: i8) {
        let pst = match p.piece_type() {
            piece::Type::Pawn => &mut self.pst_pawn,
            piece::Type::Knight => &mut self.pst_knight,
            piece::Type::Bishop => &mut self.pst_bishop,
            piece::Type::Rook => &mut self.pst_rook,
            piece::Type::Queen => &mut self.pst_queen,
            piece::Type::King => &mut self.pst_king,
        };
        match p.piece_side() {
            Side::White => *pst[square.fold_to_queenside().idx()] += diff,
            Side::Black => *pst[square.flip_vertical().fold_to_queenside().idx()] -= diff,
        }
    }

    pub fn clear_squares_relative_to_king(&mut self) {
        self.pawn_square_relative_to_friendly_king =
            [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN];
        self.pawn_square_relative_to_enemy_king = [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN];
        self.knight_square_relative_to_friendly_king =
            [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN];
        self.knight_square_relative_to_enemy_king = [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN];
        self.bishop_square_relative_to_friendly_king =
            [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN];
        self.bishop_square_relative_to_enemy_king = [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN];
        self.rook_square_relative_to_friendly_king =
            [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN];
        self.rook_square_relative_to_enemy_king = [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN];
        self.queen_square_relative_to_friendly_king =
            [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN];
        self.queen_square_relative_to_enemy_king = [Coeff(0); params::SQUARE_RELATIVE_TO_KING_LEN];
    }

    pub fn add_squares_relative_to_king(
        &mut self,
        p: Piece,
        square: Square,
        kings: &[Square; 2],
        diff: i8,
    ) {
        let (friendly_square_relative, enemy_square_relative) = match p.piece_type() {
            piece::Type::Pawn => (
                &mut self.pawn_square_relative_to_friendly_king,
                &mut self.pawn_square_relative_to_enemy_king,
            ),
            piece::Type::Knight => (
                &mut self.knight_square_relative_to_friendly_king,
                &mut self.knight_square_relative_to_enemy_king,
            ),
            piece::Type::Bishop => (
                &mut self.bishop_square_relative_to_friendly_king,
                &mut self.bishop_square_relative_to_enemy_king,
            ),
            piece::Type::Rook => (
                &mut self.rook_square_relative_to_friendly_king,
                &mut self.rook_square_relative_to_enemy_king,
            ),
            piece::Type::Queen => (
                &mut self.queen_square_relative_to_friendly_king,
                &mut self.queen_square_relative_to_enemy_king,
            ),
            piece::Type::King => unreachable!(),
        };
        const OFFSET: i8 = ((Rank::NUM_RANKS - 1) * File::NUM_FILES) as i8;
        match p.piece_side() {
            Side::White => {
                *friendly_square_relative
                    [(OFFSET + square.relative_to(kings[Side::White as usize])) as usize] += diff;
                *enemy_square_relative[(OFFSET
                    + square
                        .flip_vertical()
                        .relative_to(kings[Side::Black as usize].flip_vertical()))
                    as usize] -= diff;
            }
            Side::Black => {
                *friendly_square_relative[(OFFSET
                    + square
                        .flip_vertical()
                        .relative_to(kings[Side::Black as usize].flip_vertical()))
                    as usize] -= diff;
                *enemy_square_relative
                    [(OFFSET + square.relative_to(kings[Side::White as usize])) as usize] += diff;
            }
        }
    }

    pub fn coeff_iter(&self) -> impl Iterator<Item = &Coeff> {
        self.pst_pawn
            .iter()
            .chain(self.pst_knight.iter())
            .chain(self.pst_bishop.iter())
            .chain(self.pst_rook.iter())
            .chain(self.pst_queen.iter())
            .chain(self.pst_king.iter())
            .chain(iter::once(&self.tempo))
            .chain(self.passed_pawn.iter())
            .chain(iter::once(&self.isolated_pawn))
            .chain(iter::once(&self.backward_pawn))
            .chain(iter::once(&self.doubled_pawn))
            .chain(self.knight_mobility.iter())
            .chain(self.bishop_mobility.iter())
            .chain(self.rook_mobility.iter())
            .chain(self.queen_mobility.iter())
            .chain(iter::once(&self.bishop_pair))
            .chain(self.pawn_square_relative_to_friendly_king.iter())
            .chain(self.pawn_square_relative_to_enemy_king.iter())
            .chain(self.knight_square_relative_to_friendly_king.iter())
            .chain(self.knight_square_relative_to_enemy_king.iter())
            .chain(self.bishop_square_relative_to_friendly_king.iter())
            .chain(self.bishop_square_relative_to_enemy_king.iter())
            .chain(self.rook_square_relative_to_friendly_king.iter())
            .chain(self.rook_square_relative_to_enemy_king.iter())
            .chain(self.queen_square_relative_to_friendly_king.iter())
            .chain(self.queen_square_relative_to_enemy_king.iter())
    }
}
