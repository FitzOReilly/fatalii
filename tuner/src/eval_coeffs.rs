use eval::{GamePhase, HandCraftedEvalCoeffs};
use nalgebra_sparse::{CooMatrix, CsrMatrix};

use crate::feature_evaluator::{EvalType, NUM_FEATURES};

pub type CoeffType = f64;
pub type CoeffVector = CsrMatrix<CoeffType>;

#[derive(Debug, Clone)]
pub struct EvalCoeffs {
    pub coeff_vec: CoeffVector,
}

impl EvalCoeffs {
    pub fn new(coeffs: CooMatrix<CoeffType>) -> Self {
        Self {
            coeff_vec: CoeffVector::from(&coeffs),
        }
    }

    pub fn grad(&self) -> CoeffVector {
        self.coeff_vec.clone()
    }
}

impl From<&HandCraftedEvalCoeffs> for EvalCoeffs {
    fn from(coeffs: &HandCraftedEvalCoeffs) -> Self {
        let mg_factor = coeffs.game_phase as EvalType / GamePhase::MAX as EvalType;
        let eg_factor = 1.0 - mg_factor;
        let mut coeff_coo = CooMatrix::new(1, NUM_FEATURES);
        for (idx, &coeff) in coeffs
            .coeff_iter()
            .enumerate()
            .filter(|&(_, &coeff)| *coeff != 0)
        {
            // Middlegame
            coeff_coo.push(0, 2 * idx, mg_factor * *coeff as EvalType);
            // Endgame
            coeff_coo.push(0, 2 * idx + 1, eg_factor * *coeff as EvalType);
        }
        Self::new(coeff_coo)
    }
}
