use std::{
    iter,
    ops::{Deref, DerefMut},
};

use movegen::{
    piece::{self, Piece},
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

#[derive(Debug, Clone, Default, PartialEq)]
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
    pub distance_friendly_pawn: [Coeff; params::DISTANCE_LEN],
    pub distance_enemy_pawn: [Coeff; params::DISTANCE_LEN],
    pub distance_friendly_knight: [Coeff; params::DISTANCE_LEN],
    pub distance_enemy_knight: [Coeff; params::DISTANCE_LEN],
    pub distance_friendly_bishop: [Coeff; params::DISTANCE_LEN],
    pub distance_enemy_bishop: [Coeff; params::DISTANCE_LEN],
    pub distance_friendly_rook: [Coeff; params::DISTANCE_LEN],
    pub distance_enemy_rook: [Coeff; params::DISTANCE_LEN],
    pub distance_friendly_queen: [Coeff; params::DISTANCE_LEN],
    pub distance_enemy_queen: [Coeff; params::DISTANCE_LEN],
    pub distance_friendly_king: [Coeff; params::DISTANCE_LEN],
    pub distance_enemy_king: [Coeff; params::DISTANCE_LEN],
}

impl HandCraftedEvalCoeffs {
    pub fn add_piece(
        &mut self,
        p: Piece,
        square: Square,
        white_king: Square,
        black_king: Square,
        diff: i8,
    ) {
        self.add_pst(p, square, diff);
        self.add_distance(p, square, white_king, black_king, diff);
    }

    fn add_pst(&mut self, p: Piece, square: Square, diff: i8) {
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

    pub fn clear_distance(&mut self) {
        self.distance_friendly_pawn = Default::default();
        self.distance_enemy_pawn = Default::default();
        self.distance_friendly_knight = Default::default();
        self.distance_enemy_knight = Default::default();
        self.distance_friendly_bishop = Default::default();
        self.distance_enemy_bishop = Default::default();
        self.distance_friendly_rook = Default::default();
        self.distance_enemy_rook = Default::default();
        self.distance_friendly_queen = Default::default();
        self.distance_enemy_queen = Default::default();
        self.distance_friendly_king = Default::default();
        self.distance_enemy_king = Default::default();
    }

    pub fn add_distance(
        &mut self,
        p: Piece,
        square: Square,
        white_king: Square,
        black_king: Square,
        diff: i8,
    ) {
        let (friendly_dist, enemy_dist) = match p.piece_type() {
            piece::Type::Pawn => (
                &mut self.distance_friendly_pawn,
                &mut self.distance_enemy_pawn,
            ),
            piece::Type::Knight => (
                &mut self.distance_friendly_knight,
                &mut self.distance_enemy_knight,
            ),
            piece::Type::Bishop => (
                &mut self.distance_friendly_bishop,
                &mut self.distance_enemy_bishop,
            ),
            piece::Type::Rook => (
                &mut self.distance_friendly_rook,
                &mut self.distance_enemy_rook,
            ),
            piece::Type::Queen => (
                &mut self.distance_friendly_queen,
                &mut self.distance_enemy_queen,
            ),
            piece::Type::King => (
                &mut self.distance_friendly_king,
                &mut self.distance_enemy_king,
            ),
        };
        match p.piece_side() {
            Side::White => {
                *friendly_dist[square.distance(white_king)] += diff;
                *enemy_dist[square.distance(black_king)] -= diff;
            }
            Side::Black => {
                *friendly_dist[square.distance(black_king)] -= diff;
                *enemy_dist[square.distance(white_king)] += diff;
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
            .chain(self.distance_friendly_pawn.iter())
            .chain(self.distance_enemy_pawn.iter())
            .chain(self.distance_friendly_knight.iter())
            .chain(self.distance_enemy_knight.iter())
            .chain(self.distance_friendly_bishop.iter())
            .chain(self.distance_enemy_bishop.iter())
            .chain(self.distance_friendly_rook.iter())
            .chain(self.distance_enemy_rook.iter())
            .chain(self.distance_friendly_queen.iter())
            .chain(self.distance_enemy_queen.iter())
            .chain(self.distance_friendly_king.iter())
            .chain(self.distance_enemy_king.iter())
    }
}
