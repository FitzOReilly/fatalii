use std::fmt::Display;

use eval::{
    params::{BISHOP_MOB_LEN, KNIGHT_MOB_LEN, MOB_LEN, QUEEN_MOB_LEN, ROOK_MOB_LEN},
    score_pair::ScorePair,
    Score,
};

use crate::{
    feature_evaluator::WeightVector,
    position_features::{
        PST_SIZE, START_IDX_BACKWARD_PAWN, START_IDX_BISHOP_PAIR, START_IDX_DOUBLED_PAWN,
        START_IDX_ISOLATED_PAWN, START_IDX_MOBILITY, START_IDX_PASSED_PAWN, START_IDX_PST,
        START_IDX_TEMPO,
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
    backward_pawn: ScorePair,
    doubled_pawn: ScorePair,
    mobility: [ScorePair; MOB_LEN],
    bishop_pair: ScorePair,
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
            backward_pawn: ScorePair(0, 0),
            doubled_pawn: ScorePair(0, 0),
            mobility: [ScorePair(0, 0); MOB_LEN],
            bishop_pair: ScorePair(0, 0),
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
        eval_params.backward_pawn.0 = weights[START_IDX_BACKWARD_PAWN].round() as Score;
        eval_params.backward_pawn.1 = weights[START_IDX_BACKWARD_PAWN + 1].round() as Score;
        eval_params.doubled_pawn.0 = weights[START_IDX_DOUBLED_PAWN].round() as Score;
        eval_params.doubled_pawn.1 = weights[START_IDX_DOUBLED_PAWN + 1].round() as Score;

        for idx in 0..MOB_LEN {
            let offset = START_IDX_MOBILITY + 2 * idx;
            eval_params.mobility[idx].0 = weights[offset].round() as Score;
            eval_params.mobility[idx].1 = weights[offset + 1].round() as Score;
        }

        eval_params.bishop_pair.0 = weights[START_IDX_BISHOP_PAIR].round() as Score;
        eval_params.bishop_pair.1 = weights[START_IDX_BISHOP_PAIR + 1].round() as Score;

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
        writeln!(
            f,
            "pub const BACKWARD_PAWN: ScorePair = ScorePair({}, {});",
            self.backward_pawn.0, self.backward_pawn.1
        )?;
        writeln!(
            f,
            "pub const DOUBLED_PAWN: ScorePair = ScorePair({}, {});",
            self.doubled_pawn.0, self.doubled_pawn.1
        )?;

        writeln!(
            f,
            "pub const BISHOP_PAIR: ScorePair = ScorePair({}, {});",
            self.bishop_pair.0, self.bishop_pair.1
        )?;

        self.fmt_mob(f)?;

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

impl EvalParams {
    fn fmt_mob(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut offset = 0;

        // Knights
        write!(
            f,
            "\
const MOBILITY_KNIGHT_MG_EG: ([Score; KNIGHT_MOB_LEN], [Score; KNIGHT_MOB_LEN]) = (
    [
"
        )?;
        write!(f, "       ")?;
        for idx in 0..KNIGHT_MOB_LEN {
            write!(f, " {},", self.mobility[offset + idx].0)?;
        }
        writeln!(f)?;
        write!(
            f,
            "    ],
    [
"
        )?;
        write!(f, "       ")?;
        for idx in 0..KNIGHT_MOB_LEN {
            write!(f, " {},", self.mobility[offset + idx].1)?;
        }
        writeln!(f)?;
        write!(
            f,
            "    ],
);
"
        )?;
        offset += KNIGHT_MOB_LEN;

        // Bishops
        write!(
            f,
            "\
const MOBILITY_BISHOP_MG_EG: ([Score; BISHOP_MOB_LEN], [Score; BISHOP_MOB_LEN]) = (
    [
"
        )?;
        write!(f, "       ")?;
        for idx in 0..BISHOP_MOB_LEN {
            write!(f, " {},", self.mobility[offset + idx].0)?;
        }
        writeln!(f)?;
        write!(
            f,
            "    ],
    [
"
        )?;
        write!(f, "       ")?;
        for idx in 0..BISHOP_MOB_LEN {
            write!(f, " {},", self.mobility[offset + idx].1)?;
        }
        writeln!(f)?;
        write!(
            f,
            "    ],
);
"
        )?;
        offset += BISHOP_MOB_LEN;

        // Rooks
        write!(
            f,
            "\
const MOBILITY_ROOK_MG_EG: ([Score; ROOK_MOB_LEN], [Score; ROOK_MOB_LEN]) = (
    [
"
        )?;
        write!(f, "       ")?;
        for idx in 0..ROOK_MOB_LEN {
            write!(f, " {},", self.mobility[offset + idx].0)?;
        }
        writeln!(f)?;
        write!(
            f,
            "    ],
    [
"
        )?;
        write!(f, "       ")?;
        for idx in 0..ROOK_MOB_LEN {
            write!(f, " {},", self.mobility[offset + idx].1)?;
        }
        writeln!(f)?;
        write!(
            f,
            "    ],
);
"
        )?;
        offset += ROOK_MOB_LEN;

        // Queens
        write!(
            f,
            "\
const MOBILITY_QUEEN_MG_EG: ([Score; QUEEN_MOB_LEN], [Score; QUEEN_MOB_LEN]) = (
    [
"
        )?;
        write!(f, "       ")?;
        for idx in 0..QUEEN_MOB_LEN {
            write!(f, " {},", self.mobility[offset + idx].0)?;
        }
        writeln!(f)?;
        write!(
            f,
            "    ],
    [
"
        )?;
        write!(f, "       ")?;
        for idx in 0..QUEEN_MOB_LEN {
            write!(f, " {},", self.mobility[offset + idx].1)?;
        }
        writeln!(f)?;
        write!(
            f,
            "    ],
);
"
        )?;

        Ok(())
    }
}
