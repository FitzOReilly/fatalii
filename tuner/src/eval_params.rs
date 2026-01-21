use std::fmt::Display;

use eval::{
    params::{
        BISHOP_MOB_LEN, DISTANCE_LEN, KNIGHT_MOB_LEN, QUEEN_MOB_LEN, ROOK_MOB_LEN,
        VIRTUAL_DIAG_MOB_LEN, VIRTUAL_LINE_MOB_LEN,
    },
    score_pair::ScorePair,
    Score,
};

use crate::feature_evaluator::{Weight, WeightVector, PST_SIZE};

#[derive(Debug)]
pub struct EvalParams {
    pst_pawn: [ScorePair; PST_SIZE],
    pst_knight: [ScorePair; PST_SIZE],
    pst_bishop: [ScorePair; PST_SIZE],
    pst_rook: [ScorePair; PST_SIZE],
    pst_queen: [ScorePair; PST_SIZE],
    pst_king: [ScorePair; PST_SIZE],
    tempo: ScorePair,
    passed_pawn: [ScorePair; PST_SIZE],
    isolated_pawn: ScorePair,
    backward_pawn: ScorePair,
    doubled_pawn: ScorePair,
    knight_mobility: [ScorePair; KNIGHT_MOB_LEN],
    bishop_mobility: [ScorePair; BISHOP_MOB_LEN],
    rook_mobility: [ScorePair; ROOK_MOB_LEN],
    queen_mobility: [ScorePair; QUEEN_MOB_LEN],
    virtual_diag_mobility: [ScorePair; VIRTUAL_DIAG_MOB_LEN],
    virtual_file_mobility: [ScorePair; VIRTUAL_LINE_MOB_LEN],
    virtual_rank_mobility: [ScorePair; VIRTUAL_LINE_MOB_LEN],
    bishop_pair: ScorePair,
    distance_friendly_pawn: [ScorePair; DISTANCE_LEN],
    distance_enemy_pawn: [ScorePair; DISTANCE_LEN],
    distance_friendly_knight: [ScorePair; DISTANCE_LEN],
    distance_enemy_knight: [ScorePair; DISTANCE_LEN],
    distance_friendly_bishop: [ScorePair; DISTANCE_LEN],
    distance_enemy_bishop: [ScorePair; DISTANCE_LEN],
    distance_friendly_rook: [ScorePair; DISTANCE_LEN],
    distance_enemy_rook: [ScorePair; DISTANCE_LEN],
    distance_friendly_queen: [ScorePair; DISTANCE_LEN],
    distance_enemy_queen: [ScorePair; DISTANCE_LEN],
    distance_friendly_king: [ScorePair; DISTANCE_LEN],
    distance_enemy_king: [ScorePair; DISTANCE_LEN],
}

impl Default for EvalParams {
    fn default() -> Self {
        Self {
            pst_pawn: [ScorePair(0, 0); PST_SIZE],
            pst_knight: [ScorePair(0, 0); PST_SIZE],
            pst_bishop: [ScorePair(0, 0); PST_SIZE],
            pst_rook: [ScorePair(0, 0); PST_SIZE],
            pst_queen: [ScorePair(0, 0); PST_SIZE],
            pst_king: [ScorePair(0, 0); PST_SIZE],
            tempo: ScorePair(0, 0),
            passed_pawn: [ScorePair(0, 0); PST_SIZE],
            isolated_pawn: ScorePair(0, 0),
            backward_pawn: ScorePair(0, 0),
            doubled_pawn: ScorePair(0, 0),
            knight_mobility: [ScorePair(0, 0); KNIGHT_MOB_LEN],
            bishop_mobility: [ScorePair(0, 0); BISHOP_MOB_LEN],
            rook_mobility: [ScorePair(0, 0); ROOK_MOB_LEN],
            queen_mobility: [ScorePair(0, 0); QUEEN_MOB_LEN],
            virtual_diag_mobility: [ScorePair(0, 0); VIRTUAL_DIAG_MOB_LEN],
            virtual_file_mobility: [ScorePair(0, 0); VIRTUAL_LINE_MOB_LEN],
            virtual_rank_mobility: [ScorePair(0, 0); VIRTUAL_LINE_MOB_LEN],
            bishop_pair: ScorePair(0, 0),
            distance_friendly_pawn: [ScorePair(0, 0); DISTANCE_LEN],
            distance_enemy_pawn: [ScorePair(0, 0); DISTANCE_LEN],
            distance_friendly_knight: [ScorePair(0, 0); DISTANCE_LEN],
            distance_enemy_knight: [ScorePair(0, 0); DISTANCE_LEN],
            distance_friendly_bishop: [ScorePair(0, 0); DISTANCE_LEN],
            distance_enemy_bishop: [ScorePair(0, 0); DISTANCE_LEN],
            distance_friendly_rook: [ScorePair(0, 0); DISTANCE_LEN],
            distance_enemy_rook: [ScorePair(0, 0); DISTANCE_LEN],
            distance_friendly_queen: [ScorePair(0, 0); DISTANCE_LEN],
            distance_enemy_queen: [ScorePair(0, 0); DISTANCE_LEN],
            distance_friendly_king: [ScorePair(0, 0); DISTANCE_LEN],
            distance_enemy_king: [ScorePair(0, 0); DISTANCE_LEN],
        }
    }
}

impl From<&WeightVector> for EvalParams {
    fn from(weights: &WeightVector) -> Self {
        let mut eval_params = EvalParams::default();
        let mut weight_iter = weights.iter();
        Self::next_params(&mut eval_params.pst_pawn, &mut weight_iter);
        Self::next_params(&mut eval_params.pst_knight, &mut weight_iter);
        Self::next_params(&mut eval_params.pst_bishop, &mut weight_iter);
        Self::next_params(&mut eval_params.pst_rook, &mut weight_iter);
        Self::next_params(&mut eval_params.pst_queen, &mut weight_iter);
        Self::next_params(&mut eval_params.pst_king, &mut weight_iter);
        Self::next_param(&mut eval_params.tempo, &mut weight_iter);
        Self::next_params(&mut eval_params.passed_pawn, &mut weight_iter);
        Self::next_param(&mut eval_params.isolated_pawn, &mut weight_iter);
        Self::next_param(&mut eval_params.backward_pawn, &mut weight_iter);
        Self::next_param(&mut eval_params.doubled_pawn, &mut weight_iter);
        Self::next_params(&mut eval_params.knight_mobility, &mut weight_iter);
        Self::next_params(&mut eval_params.bishop_mobility, &mut weight_iter);
        Self::next_params(&mut eval_params.rook_mobility, &mut weight_iter);
        Self::next_params(&mut eval_params.queen_mobility, &mut weight_iter);
        Self::next_params(&mut eval_params.virtual_diag_mobility, &mut weight_iter);
        Self::next_params(&mut eval_params.virtual_file_mobility, &mut weight_iter);
        Self::next_params(&mut eval_params.virtual_rank_mobility, &mut weight_iter);
        Self::next_param(&mut eval_params.bishop_pair, &mut weight_iter);
        Self::next_params(&mut eval_params.distance_friendly_pawn, &mut weight_iter);
        Self::next_params(&mut eval_params.distance_enemy_pawn, &mut weight_iter);
        Self::next_params(&mut eval_params.distance_friendly_knight, &mut weight_iter);
        Self::next_params(&mut eval_params.distance_enemy_knight, &mut weight_iter);
        Self::next_params(&mut eval_params.distance_friendly_bishop, &mut weight_iter);
        Self::next_params(&mut eval_params.distance_enemy_bishop, &mut weight_iter);
        Self::next_params(&mut eval_params.distance_friendly_rook, &mut weight_iter);
        Self::next_params(&mut eval_params.distance_enemy_rook, &mut weight_iter);
        Self::next_params(&mut eval_params.distance_friendly_queen, &mut weight_iter);
        Self::next_params(&mut eval_params.distance_enemy_queen, &mut weight_iter);
        Self::next_params(&mut eval_params.distance_friendly_king, &mut weight_iter);
        Self::next_params(&mut eval_params.distance_enemy_king, &mut weight_iter);
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
        self.fmt_single_pst(f, "PASSED_PAWN_MG_EG", self.passed_pawn)?;
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

        self.fmt_mobilities(f)?;
        self.fmt_distances(f)?;
        self.fmt_pst(f)?;

        Ok(())
    }
}

impl EvalParams {
    fn next_param<'a, I>(param: &mut ScorePair, weight_iter: &mut I)
    where
        I: Iterator<Item = &'a Weight>,
    {
        param.0 = weight_iter.next().unwrap().round() as Score;
        param.1 = weight_iter.next().unwrap().round() as Score;
    }

    fn next_params<'a, I>(params: &mut [ScorePair], weight_iter: &mut I)
    where
        I: Iterator<Item = &'a Weight>,
    {
        for param in params {
            param.0 = weight_iter.next().unwrap().round() as Score;
            param.1 = weight_iter.next().unwrap().round() as Score;
        }
    }

    fn fmt_mobilities(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (const_name, len_name, mob) in [
            (
                "MOBILITY_KNIGHT_MG_EG",
                "KNIGHT_MOB_LEN",
                &self.knight_mobility[..],
            ),
            (
                "MOBILITY_BISHOP_MG_EG",
                "BISHOP_MOB_LEN",
                &self.bishop_mobility[..],
            ),
            (
                "MOBILITY_ROOK_MG_EG",
                "ROOK_MOB_LEN",
                &self.rook_mobility[..],
            ),
            (
                "MOBILITY_QUEEN_MG_EG",
                "QUEEN_MOB_LEN",
                &self.queen_mobility[..],
            ),
            (
                "VIRTUAL_DIAG_MOBILITY_MG_EG",
                "VIRTUAL_DIAG_MOB_LEN",
                &self.virtual_diag_mobility[..],
            ),
            (
                "VIRTUAL_FILE_MOBILITY_MG_EG",
                "VIRTUAL_LINE_MOB_LEN",
                &self.virtual_file_mobility[..],
            ),
            (
                "VIRTUAL_RANK_MOBILITY_MG_EG",
                "VIRTUAL_LINE_MOB_LEN",
                &self.virtual_rank_mobility[..],
            ),
        ] {
            self.fmt_weights(f, const_name, len_name, mob)?;
        }
        Ok(())
    }

    fn fmt_distances(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (const_name, distance) in [
            ("DISTANCE_FRIENDLY_PAWN_MG_EG", &self.distance_friendly_pawn),
            ("DISTANCE_ENEMY_PAWN_MG_EG", &self.distance_enemy_pawn),
            (
                "DISTANCE_FRIENDLY_KNIGHT_MG_EG",
                &self.distance_friendly_knight,
            ),
            ("DISTANCE_ENEMY_KNIGHT_MG_EG", &self.distance_enemy_knight),
            (
                "DISTANCE_FRIENDLY_BISHOP_MG_EG",
                &self.distance_friendly_bishop,
            ),
            ("DISTANCE_ENEMY_BISHOP_MG_EG", &self.distance_enemy_bishop),
            ("DISTANCE_FRIENDLY_ROOK_MG_EG", &self.distance_friendly_rook),
            ("DISTANCE_ENEMY_ROOK_MG_EG", &self.distance_enemy_rook),
            (
                "DISTANCE_FRIENDLY_QUEEN_MG_EG",
                &self.distance_friendly_queen,
            ),
            ("DISTANCE_ENEMY_QUEEN_MG_EG", &self.distance_enemy_queen),
            ("DISTANCE_FRIENDLY_KING_MG_EG", &self.distance_friendly_king),
            ("DISTANCE_ENEMY_KING_MG_EG", &self.distance_enemy_king),
        ] {
            self.fmt_weights(f, const_name, "DISTANCE_LEN", distance)?;
        }
        Ok(())
    }

    fn fmt_weights(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        const_name: &str,
        len_name: &str,
        weights: &[ScorePair],
    ) -> std::fmt::Result {
        let (mg, eg): (Vec<_>, Vec<_>) = weights.iter().map(|sp| (sp.0, sp.1)).unzip();
        writeln!(
            f,
            "const {const_name}: ([Score; {len_name}], [Score; {len_name}]) = ({mg:?}, {eg:?});",
        )?;
        Ok(())
    }

    fn fmt_pst(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (const_name, pst) in [
            ("PST_PAWN_MG_EG", self.pst_pawn),
            ("PST_KNIGHT_MG_EG", self.pst_knight),
            ("PST_BISHOP_MG_EG", self.pst_bishop),
            ("PST_ROOK_MG_EG", self.pst_rook),
            ("PST_QUEEN_MG_EG", self.pst_queen),
            ("PST_KING_MG_EG", self.pst_king),
        ] {
            self.fmt_single_pst(f, const_name, pst)?;
        }
        Ok(())
    }

    fn fmt_single_pst(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        const_name: &str,
        pst: [ScorePair; 32],
    ) -> std::fmt::Result {
        write!(
            f,
            "\
#[rustfmt::skip]
const {const_name}: ([Score; 32], [Score; 32]) = (
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
        Ok(())
    }
}
