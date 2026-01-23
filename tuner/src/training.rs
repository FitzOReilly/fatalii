use eval::HandCraftedEvalCoeffs;

use crate::{eval_coeffs::EvalCoeffs, feature_evaluator::EvalType};

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    WhiteWin,
    Draw,
    BlackWin,
}

#[derive(Debug, Clone)]
pub struct TrainingPosition {
    pub eval_coeffs: HandCraftedEvalCoeffs,
    pub outcome: Outcome,
}

#[derive(Debug, Clone)]
pub struct TrainingCoeffs {
    pub coeffs: EvalCoeffs,
    pub outcome: Outcome,
}

impl From<&TrainingPosition> for TrainingCoeffs {
    fn from(tp: &TrainingPosition) -> Self {
        let coeffs = EvalCoeffs::from(&tp.eval_coeffs);
        Self {
            coeffs,
            outcome: tp.outcome,
        }
    }
}

impl From<&Outcome> for EvalType {
    fn from(o: &Outcome) -> Self {
        match o {
            Outcome::WhiteWin => 1.0,
            Outcome::Draw => 0.5,
            Outcome::BlackWin => 0.0,
        }
    }
}

impl From<Outcome> for EvalType {
    fn from(o: Outcome) -> Self {
        Self::from(&o)
    }
}

impl TrainingCoeffs {
    pub fn grad(&self) -> &EvalCoeffs {
        self.coeffs.grad()
    }
}
