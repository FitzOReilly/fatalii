use movegen::position::Position;

use crate::position_features::{EvalType, FeatureVector, PositionFeatures};

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    WhiteWin,
    Draw,
    BlackWin,
}

#[derive(Debug, Clone)]
pub struct TrainingPosition {
    pub pos: Position,
    pub outcome: Outcome,
}

#[derive(Debug, Clone)]
pub struct TrainingFeatures {
    pub features: PositionFeatures,
    pub grad: FeatureVector,
    pub outcome: Outcome,
}

impl From<&TrainingPosition> for TrainingFeatures {
    fn from(tp: &TrainingPosition) -> Self {
        let features = PositionFeatures::from(&tp.pos);
        let grad = features.grad();
        Self {
            features,
            grad,
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
