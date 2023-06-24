use eval::params;
use nalgebra::SVector;

use crate::position_features::{
    EvalType, PositionFeatures, NUM_FEATURES, PST_SIZE, START_IDX_BACKWARD_PAWN,
    START_IDX_BISHOP_PAIR, START_IDX_DOUBLED_PAWN, START_IDX_ISOLATED_PAWN, START_IDX_MOBILITY,
    START_IDX_PASSED_PAWN, START_IDX_PST, START_IDX_TEMPO,
};

type Weight = f64;
pub type WeightVector = SVector<Weight, NUM_FEATURES>;

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
            weights: initialize_weights(),
        }
    }

    pub fn update_weights(&mut self, weights: &WeightVector) {
        for (w, new) in self.weights.iter_mut().zip(weights.iter()) {
            *w = *new;
        }
    }

    pub fn eval(&self, features: &PositionFeatures) -> EvalType {
        (&features.feature_vec * self.weights)[0]
    }
}

pub fn initialize_weights() -> WeightVector {
    let mut weights = WeightVector::from_element(0.0);

    let mut pst_idx = START_IDX_PST;
    for pst in [
        params::PST_PAWN,
        params::PST_KNIGHT,
        params::PST_BISHOP,
        params::PST_ROOK,
        params::PST_QUEEN,
        params::PST_KING,
    ] {
        for square_idx in 0..PST_SIZE {
            weights[pst_idx + 2 * square_idx] = pst[square_idx].0.into();
            weights[pst_idx + 2 * square_idx + 1] = pst[square_idx].1.into();
        }
        pst_idx += 2 * PST_SIZE;
    }

    weights[START_IDX_TEMPO] = params::TEMPO.0.into();
    weights[START_IDX_TEMPO + 1] = params::TEMPO.1.into();

    weights[START_IDX_PASSED_PAWN] = params::PASSED_PAWN.0.into();
    weights[START_IDX_PASSED_PAWN + 1] = params::PASSED_PAWN.1.into();
    weights[START_IDX_ISOLATED_PAWN] = params::ISOLATED_PAWN.0.into();
    weights[START_IDX_ISOLATED_PAWN + 1] = params::ISOLATED_PAWN.1.into();
    weights[START_IDX_BACKWARD_PAWN] = params::BACKWARD_PAWN.0.into();
    weights[START_IDX_BACKWARD_PAWN + 1] = params::BACKWARD_PAWN.1.into();
    weights[START_IDX_DOUBLED_PAWN] = params::DOUBLED_PAWN.0.into();
    weights[START_IDX_DOUBLED_PAWN + 1] = params::DOUBLED_PAWN.1.into();

    initialize_mobility(&mut weights);

    weights[START_IDX_BISHOP_PAIR] = params::BISHOP_PAIR.0.into();
    weights[START_IDX_BISHOP_PAIR + 1] = params::BISHOP_PAIR.1.into();

    weights
}

pub fn initialize_mobility(weights: &mut WeightVector) {
    let mut idx = START_IDX_MOBILITY;
    for mob in params::MOBILITY_KNIGHT {
        weights[idx] = mob.0.into();
        weights[idx + 1] = mob.1.into();
        idx += 2;
    }
    for mob in params::MOBILITY_BISHOP {
        weights[idx] = mob.0.into();
        weights[idx + 1] = mob.1.into();
        idx += 2;
    }
    for mob in params::MOBILITY_ROOK {
        weights[idx] = mob.0.into();
        weights[idx + 1] = mob.1.into();
        idx += 2;
    }
    for mob in params::MOBILITY_QUEEN {
        weights[idx] = mob.0.into();
        weights[idx + 1] = mob.1.into();
        idx += 2;
    }
}

#[cfg(test)]
mod tests {
    use eval::{complex::Complex, Eval};
    use movegen::fen::Fen;

    use crate::{feature_evaluator::EvalType, position_features::PositionFeatures};

    use super::FeatureEvaluator;

    #[test]
    fn tuner_eval_matches_actual_eval() {
        let fens = ["8/6pk/5p2/P1R4P/1P5P/5K2/2P5/6r1 b - - 2 70"];

        let mut evaluator = Complex::new();
        let feature_evaluator = FeatureEvaluator::new();

        for fen in fens {
            let pos = Fen::str_to_pos(fen).unwrap();
            let exp_eval = evaluator.eval(&pos);
            let features = PositionFeatures::from(&pos);
            let act_eval = feature_evaluator.eval(&features);
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
