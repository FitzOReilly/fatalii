use std::iter;

use eval::params::{self, MOB_LEN, SQUARE_RELATIVE_TO_KING_LEN};
use nalgebra::SVector;

use crate::eval_coeffs::EvalCoeffs;

// Middlegame and endgame weights for each feature
const NUM_WEIGHTS: usize = 2 * NUM_FEATURES;

pub type Weight = f64;
pub type WeightVector = SVector<Weight, NUM_WEIGHTS>;

pub type EvalType = f64;

pub const PST_SIZE: usize = 32;
const NUM_SIDES: usize = 2;
const NUM_PIECE_TYPES: usize = 6;
const NUM_PST_FEATURES: usize = NUM_PIECE_TYPES * PST_SIZE;
const NUM_TEMPO_FEATURES: usize = 1;
const NUM_PASSED_PAWN_FEATURES: usize = PST_SIZE;
const NUM_ISOLATED_PAWN_FEATURES: usize = 1;
const NUM_BACKWARD_PAWN_FEATURES: usize = 1;
const NUM_DOUBLED_PAWN_FEATURES: usize = 1;
const NUM_MOBILITY_FEATURES: usize = MOB_LEN;
const NUM_BISHOP_PAIR_FEATURES: usize = 1;
const NUM_SQUARE_RELATIVE_TO_KING_FEATURES: usize =
    NUM_SIDES * (NUM_PIECE_TYPES - 1) * SQUARE_RELATIVE_TO_KING_LEN;
pub const NUM_FEATURES: usize = NUM_PST_FEATURES
    + NUM_TEMPO_FEATURES
    + NUM_PASSED_PAWN_FEATURES
    + NUM_ISOLATED_PAWN_FEATURES
    + NUM_BACKWARD_PAWN_FEATURES
    + NUM_DOUBLED_PAWN_FEATURES
    + NUM_MOBILITY_FEATURES
    + NUM_BISHOP_PAIR_FEATURES
    + NUM_SQUARE_RELATIVE_TO_KING_FEATURES;

pub const START_IDX_PST: usize = 0;

#[derive(Debug, Clone)]
pub struct FeatureEvaluator {
    weights: WeightVector,
}

impl Default for FeatureEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&WeightVector> for FeatureEvaluator {
    fn from(weights: &WeightVector) -> Self {
        Self { weights: *weights }
    }
}

impl FeatureEvaluator {
    pub fn new() -> Self {
        Self {
            weights: engine_weights(),
        }
    }

    pub fn update_weights(&mut self, weights: &WeightVector) {
        for (w, new) in self.weights.iter_mut().zip(weights.iter()) {
            *w = *new;
        }
    }

    pub fn eval(&self, coeffs: &EvalCoeffs) -> EvalType {
        coeffs
            .coeff_vec
            .col_indices()
            .iter()
            .zip(coeffs.coeff_vec.values())
            .map(|(idx, val)| {
                coeffs.mg_factor * *val as f64 * self.weights.index(2 * idx)
                    + coeffs.eg_factor * *val as f64 * self.weights.index(2 * idx + 1)
            })
            .sum()
    }
}

pub fn default_weights() -> WeightVector {
    let mut weights = WeightVector::from_element(0.0);
    let mut pst_idx = START_IDX_PST;
    for material_value in [
        100, // pawn
        300, // knight
        300, // bishop
        500, // rook
        900, // queen
        0,   // king
    ] {
        for square_idx in 0..PST_SIZE {
            weights[pst_idx + 2 * square_idx] = material_value.into();
            weights[pst_idx + 2 * square_idx + 1] = material_value.into();
        }
        pst_idx += 2 * PST_SIZE;
    }
    // Write zeros to first and eighth ranks for pawns
    for square_idx in (0..PST_SIZE).step_by(8) {
        weights[START_IDX_PST + 2 * square_idx] = 0.0;
        weights[START_IDX_PST + 2 * square_idx + 1] = 0.0;
        weights[START_IDX_PST + 2 * (square_idx + 7)] = 0.0;
        weights[START_IDX_PST + 2 * (square_idx + 7) + 1] = 0.0;
    }
    weights
}

pub fn engine_weights() -> WeightVector {
    let weight_iter = params::PST_PAWN[..PST_SIZE]
        .iter()
        .chain(params::PST_KNIGHT[..PST_SIZE].iter())
        .chain(params::PST_BISHOP[..PST_SIZE].iter())
        .chain(params::PST_ROOK[..PST_SIZE].iter())
        .chain(params::PST_QUEEN[..PST_SIZE].iter())
        .chain(params::PST_KING[..PST_SIZE].iter())
        .chain(iter::once(&params::TEMPO))
        .chain(params::PASSED_PAWN[..PST_SIZE].iter())
        .chain(iter::once(&params::ISOLATED_PAWN))
        .chain(iter::once(&params::BACKWARD_PAWN))
        .chain(iter::once(&params::DOUBLED_PAWN))
        .chain(params::MOBILITY_KNIGHT.iter())
        .chain(params::MOBILITY_BISHOP.iter())
        .chain(params::MOBILITY_ROOK.iter())
        .chain(params::MOBILITY_QUEEN.iter())
        .chain(iter::once(&params::BISHOP_PAIR))
        .chain(params::PAWN_SQUARE_RELATIVE_TO_FRIENDLY_KING.iter())
        .chain(params::PAWN_SQUARE_RELATIVE_TO_ENEMY_KING.iter())
        .chain(params::KNIGHT_SQUARE_RELATIVE_TO_FRIENDLY_KING.iter())
        .chain(params::KNIGHT_SQUARE_RELATIVE_TO_ENEMY_KING.iter())
        .chain(params::BISHOP_SQUARE_RELATIVE_TO_FRIENDLY_KING.iter())
        .chain(params::BISHOP_SQUARE_RELATIVE_TO_ENEMY_KING.iter())
        .chain(params::ROOK_SQUARE_RELATIVE_TO_FRIENDLY_KING.iter())
        .chain(params::ROOK_SQUARE_RELATIVE_TO_ENEMY_KING.iter())
        .chain(params::QUEEN_SQUARE_RELATIVE_TO_FRIENDLY_KING.iter())
        .chain(params::QUEEN_SQUARE_RELATIVE_TO_ENEMY_KING.iter());
    let mut weights = WeightVector::from_element(0.0);
    for (idx, &weight) in weight_iter.enumerate() {
        // Middlegame
        weights[2 * idx] = weight.0.into();
        // Endgame
        weights[2 * idx + 1] = weight.1.into();
    }
    weights
}

#[cfg(test)]
mod tests {
    use eval::{Eval, HandCraftedEval};
    use movegen::fen::Fen;

    use crate::{eval_coeffs::EvalCoeffs, feature_evaluator::EvalType};

    use super::FeatureEvaluator;

    #[test]
    fn tuner_eval_matches_actual_eval() {
        let fens = ["8/6pk/5p2/P1R4P/1P5P/5K2/2P5/6r1 b - - 2 70"];

        let mut evaluator = HandCraftedEval::new();
        let feature_evaluator = FeatureEvaluator::new();

        for fen in fens {
            let pos = Fen::str_to_pos(fen).unwrap();
            let exp_eval = evaluator.eval(&pos);
            let coeffs = EvalCoeffs::from(evaluator.coeffs());
            let act_eval = feature_evaluator.eval(&coeffs);
            // There may be small differences in the evaluations:
            // - The position evaluator rounds down the result, the feature evaluator doesn't
            // - Rounding errors
            assert!(
                ((exp_eval as EvalType) - act_eval).abs() < 1.0,
                "Evaluations don't match\nExpected: {exp_eval}\nActual: {act_eval}",
            );
        }
    }
}
