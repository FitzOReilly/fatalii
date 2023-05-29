use std::fmt::Display;

use eval::{score_pair::ScorePair, Score};

use crate::{
    feature_evaluator::WeightVector,
    position_features::{
        PST_SIZE, START_IDX_ISOLATED_PAWN, START_IDX_PASSED_PAWN, START_IDX_PST, START_IDX_TEMPO,
    },
};

#[derive(Debug)]
pub struct EvalParams {
    pst_pawn: [ScorePair; 32],
    pst_knight: [ScorePair; 32],
    pst_bishop: [ScorePair; 32],
    pst_rook: [ScorePair; 32],
    pst_queen: [ScorePair; 32],
    pst_king: [ScorePair; 32],
    tempo: ScorePair,
    passed_pawn: ScorePair,
    isolated_pawn: ScorePair,
}

impl Default for EvalParams {
    fn default() -> Self {
        Self {
            pst_pawn: [ScorePair(0, 0); 32],
            pst_knight: [ScorePair(0, 0); 32],
            pst_bishop: [ScorePair(0, 0); 32],
            pst_rook: [ScorePair(0, 0); 32],
            pst_queen: [ScorePair(0, 0); 32],
            pst_king: [ScorePair(0, 0); 32],
            tempo: ScorePair(0, 0),
            passed_pawn: ScorePair(0, 0),
            isolated_pawn: ScorePair(0, 0),
        }
    }
}

impl From<&WeightVector> for EvalParams {
    fn from(weights: &WeightVector) -> Self {
        let mut eval_params = EvalParams::default();

        let mut pst_idx = START_IDX_PST;
        for pst in [
            &mut eval_params.pst_pawn,
            &mut eval_params.pst_knight,
            &mut eval_params.pst_bishop,
            &mut eval_params.pst_rook,
            &mut eval_params.pst_queen,
            &mut eval_params.pst_king,
        ] {
            for square_idx in 0..PST_SIZE {
                pst[square_idx].0 = weights[pst_idx + 2 * square_idx].round() as Score;
                pst[square_idx].1 = weights[pst_idx + 2 * square_idx + 1].round() as Score;
            }
            pst_idx += 2 * PST_SIZE;
        }

        eval_params.tempo.0 = weights[START_IDX_TEMPO].round() as Score;
        eval_params.tempo.1 = weights[START_IDX_TEMPO + 1].round() as Score;

        eval_params.passed_pawn.0 = weights[START_IDX_PASSED_PAWN].round() as Score;
        eval_params.passed_pawn.1 = weights[START_IDX_PASSED_PAWN + 1].round() as Score;
        eval_params.isolated_pawn.0 = weights[START_IDX_ISOLATED_PAWN].round() as Score;
        eval_params.isolated_pawn.1 = weights[START_IDX_ISOLATED_PAWN + 1].round() as Score;

        eval_params
    }
}

impl Display for EvalParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "pub const TEMPO: ScorePair = ScorePair({}, {});",
            self.tempo.0, self.tempo.1
        )?;
        writeln!(
            f,
            "pub const PASSED_PAWN: ScorePair = ScorePair({}, {});",
            self.passed_pawn.0, self.passed_pawn.1
        )?;
        writeln!(
            f,
            "pub const ISOLATED_PAWN: ScorePair = ScorePair({}, {});",
            self.isolated_pawn.0, self.isolated_pawn.1
        )?;

        for (pst, piece) in [
            (self.pst_pawn, "PAWN"),
            (self.pst_knight, "KNIGHT"),
            (self.pst_bishop, "BISHOP"),
            (self.pst_rook, "ROOK"),
            (self.pst_queen, "QUEEN"),
            (self.pst_king, "KING"),
        ] {
            write!(
                f,
                "\
#[rustfmt::skip]
const PST_{piece}_MG_EG: ([Score; 32], [Score; 32]) = (
    [
"
            )?;
            for rank in (0..8).rev() {
                write!(f, "       ")?;
                for file in 0..4 {
                    let idx = file * 8 + rank;
                    write!(f, " {:4},", pst[idx].0)?;
                }
                writeln!(f)?;
            }
            write!(
                f,
                "    ],
    [
"
            )?;
            for rank in (0..8).rev() {
                write!(f, "       ")?;
                for file in 0..4 {
                    let idx = file * 8 + rank;
                    write!(f, " {:4},", pst[idx].1)?;
                }
                writeln!(f)?;
            }
            write!(
                f,
                "    ],
);
"
            )?;
        }

        Ok(())
    }
}
