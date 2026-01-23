use eval::{GamePhase, HandCraftedEvalCoeffs};
use nalgebra_sparse::{CooMatrix, CsrMatrix};

use crate::feature_evaluator::{EvalType, NUM_FEATURES};

pub type CoeffType = i8;
pub type CoeffVector = CsrMatrix<CoeffType>;

#[derive(Debug, Clone)]
pub struct EvalCoeffs {
    pub mg_factor: EvalType,
    pub eg_factor: EvalType,
    pub coeff_vec: CoeffVector,
}

impl EvalCoeffs {
    pub fn new(mg_factor: EvalType, eg_factor: EvalType, coeffs: CooMatrix<CoeffType>) -> Self {
        Self {
            mg_factor,
            eg_factor,
            coeff_vec: CoeffVector::from(&coeffs),
        }
    }

    pub fn grad(&self) -> &EvalCoeffs {
        self
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
            coeff_coo.push(0, idx, *coeff);
        }
        Self::new(mg_factor, eg_factor, coeff_coo)
    }
}
