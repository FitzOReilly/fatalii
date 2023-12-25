use std::fmt;

use movegen::side::Side;

pub type Score = i16;

// Avoid overflow of -NEGATIVE_INF
pub const NEG_INF: Score = Score::MIN + 1;
pub const POS_INF: Score = Score::MAX;
pub const EQ_POSITION: Score = 0;
// We must be able to distinguish these values from +/-inf
pub const BLACK_WIN: Score = NEG_INF + 1;
pub const WHITE_WIN: Score = POS_INF - 1;

const MAX_MATE_DIST: Score = 255;
const MIN_CP: Score = BLACK_WIN + MAX_MATE_DIST + 1;
const MAX_CP: Score = WHITE_WIN - MAX_MATE_DIST - 1;

pub fn is_valid(s: Score) -> bool {
    (BLACK_WIN..=WHITE_WIN).contains(&s)
}

pub fn is_centipawns(s: Score) -> bool {
    (MIN_CP..=MAX_CP).contains(&s)
}

pub fn is_mating(s: Score) -> bool {
    is_white_mating(s) || is_black_mating(s)
}

pub fn is_white_mating(s: Score) -> bool {
    (MAX_CP + 1..=WHITE_WIN).contains(&s)
}

pub fn is_black_mating(s: Score) -> bool {
    (BLACK_WIN..=MIN_CP - 1).contains(&s)
}

pub fn mate_dist(s: Score) -> Score {
    if is_white_mating(s) {
        WHITE_WIN - s
    } else if is_black_mating(s) {
        BLACK_WIN - s
    } else {
        NEG_INF
    }
}

pub fn inc_mate_dist(s: Score) -> Score {
    inc_mate_dist_by(s, 1)
}

pub fn inc_mate_dist_by(s: Score, plies: usize) -> Score {
    if is_white_mating(s) {
        s - (plies as Score).min(s - MAX_CP)
    } else if is_black_mating(s) {
        s + (plies as Score).min(-MAX_CP - s)
    } else {
        s
    }
}

pub fn dec_mate_dist(s: Score) -> Score {
    dec_mate_dist_by(s, 1)
}

pub fn dec_mate_dist_by(s: Score, plies: usize) -> Score {
    if is_white_mating(s) && s != WHITE_WIN {
        s + (plies as Score).min(WHITE_WIN - s)
    } else if is_black_mating(s) && s != BLACK_WIN {
        s - (plies as Score).min(s - BLACK_WIN)
    } else {
        s
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ScoreVariant {
    Centipawns(i16),
    Mate(Side, i16),
}

impl From<Score> for ScoreVariant {
    fn from(s: Score) -> Self {
        if is_white_mating(s) {
            Self::Mate(Side::White, (WHITE_WIN - s + 1) / 2)
        } else if is_black_mating(s) {
            Self::Mate(Side::Black, (BLACK_WIN - s - 1) / 2)
        } else if (MIN_CP..=MAX_CP).contains(&s) {
            Self::Centipawns(s)
        } else {
            panic!("Invalid score: {s}");
        }
    }
}

impl fmt::Display for ScoreVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScoreVariant::Centipawns(cp) => write!(f, "{:.2}", (*cp as f32) / 100.0),
            ScoreVariant::Mate(Side::White, dist) => write!(f, "M{dist}"),
            ScoreVariant::Mate(Side::Black, dist) => write!(f, "-M{}", dist.abs()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_valid_or_invalid() {
        assert!(!is_valid(Score::MIN));
        assert!(!is_valid(NEG_INF));
        assert!(is_valid(BLACK_WIN));
        assert!(is_valid(MIN_CP - 1));
        assert!(is_valid(MIN_CP));
        assert!(is_valid(EQ_POSITION));
        assert!(is_valid(MAX_CP));
        assert!(is_valid(MAX_CP + 1));
        assert!(is_valid(WHITE_WIN));
        assert!(!is_valid(POS_INF));
    }

    #[test]
    fn mating_scores() {
        assert!(!is_mating(Score::MIN));
        assert!(!is_centipawns(Score::MIN));
        assert!(!is_mating(NEG_INF));
        assert!(!is_centipawns(NEG_INF));
        assert!(is_black_mating(BLACK_WIN));
        assert_eq!(0, mate_dist(BLACK_WIN));
        assert!(!is_white_mating(BLACK_WIN));
        assert!(!is_centipawns(BLACK_WIN));
        assert!(is_black_mating(MIN_CP - 1));
        assert!(!is_white_mating(MIN_CP - 1));
        assert_eq!(-MAX_MATE_DIST, mate_dist(MIN_CP - 1));
        assert!(!is_centipawns(MIN_CP - 1));
        assert!(!is_mating(MIN_CP));
        assert!(is_centipawns(MIN_CP));
        assert!(!is_mating(EQ_POSITION));
        assert!(is_centipawns(EQ_POSITION));
        assert!(!is_mating(MAX_CP));
        assert!(is_centipawns(MAX_CP));
        assert!(is_white_mating(MAX_CP + 1));
        assert_eq!(MAX_MATE_DIST, mate_dist(MAX_CP + 1));
        assert!(!is_black_mating(MAX_CP + 1));
        assert!(!is_centipawns(MAX_CP + 1));
        assert!(is_white_mating(WHITE_WIN));
        assert_eq!(0, mate_dist(WHITE_WIN));
        assert!(!is_black_mating(WHITE_WIN));
        assert!(!is_centipawns(WHITE_WIN));
        assert!(!is_mating(POS_INF));
        assert!(!is_centipawns(POS_INF));
    }

    #[test]
    fn increase_and_decrease_mate_distance() {
        assert_eq!(Score::MIN, inc_mate_dist(Score::MIN));
        assert_eq!(Score::MIN, inc_mate_dist_by(Score::MIN, 2));
        assert_eq!(Score::MIN, dec_mate_dist(Score::MIN));
        assert_eq!(Score::MIN, dec_mate_dist_by(Score::MIN, 2));
        assert_eq!(NEG_INF, inc_mate_dist(NEG_INF));
        assert_eq!(NEG_INF, inc_mate_dist_by(NEG_INF, 2));
        assert_eq!(NEG_INF, dec_mate_dist(NEG_INF));
        assert_eq!(NEG_INF, dec_mate_dist_by(NEG_INF, 2));
        assert_eq!(BLACK_WIN + 1, inc_mate_dist(BLACK_WIN));
        assert_eq!(BLACK_WIN + 2, inc_mate_dist_by(BLACK_WIN, 2));
        assert_eq!(BLACK_WIN, dec_mate_dist(BLACK_WIN));
        assert_eq!(BLACK_WIN, dec_mate_dist_by(BLACK_WIN, 2));
        assert_eq!(MIN_CP, inc_mate_dist(MIN_CP - 1));
        assert_eq!(MIN_CP, inc_mate_dist_by(MIN_CP - 1, 2));
        assert_eq!(MIN_CP - 2, dec_mate_dist(MIN_CP - 1));
        assert_eq!(MIN_CP - 3, dec_mate_dist_by(MIN_CP - 1, 2));
        assert_eq!(MIN_CP, inc_mate_dist(MIN_CP));
        assert_eq!(MIN_CP, inc_mate_dist_by(MIN_CP, 2));
        assert_eq!(MIN_CP, dec_mate_dist(MIN_CP));
        assert_eq!(MIN_CP, dec_mate_dist_by(MIN_CP, 2));
        assert_eq!(EQ_POSITION, inc_mate_dist(EQ_POSITION));
        assert_eq!(EQ_POSITION, inc_mate_dist_by(EQ_POSITION, 2));
        assert_eq!(EQ_POSITION, dec_mate_dist(EQ_POSITION));
        assert_eq!(EQ_POSITION, dec_mate_dist_by(EQ_POSITION, 2));
        assert_eq!(MAX_CP, inc_mate_dist(MAX_CP));
        assert_eq!(MAX_CP, inc_mate_dist_by(MAX_CP, 2));
        assert_eq!(MAX_CP, dec_mate_dist(MAX_CP));
        assert_eq!(MAX_CP, dec_mate_dist_by(MAX_CP, 2));
        assert_eq!(MAX_CP, inc_mate_dist(MAX_CP + 1));
        assert_eq!(MAX_CP, inc_mate_dist_by(MAX_CP + 1, 2));
        assert_eq!(MAX_CP + 2, dec_mate_dist(MAX_CP + 1));
        assert_eq!(MAX_CP + 3, dec_mate_dist_by(MAX_CP + 1, 2));
        assert_eq!(WHITE_WIN - 1, inc_mate_dist(WHITE_WIN));
        assert_eq!(WHITE_WIN - 2, inc_mate_dist_by(WHITE_WIN, 2));
        assert_eq!(WHITE_WIN, dec_mate_dist(WHITE_WIN));
        assert_eq!(WHITE_WIN, dec_mate_dist_by(WHITE_WIN, 2));
        assert_eq!(POS_INF, inc_mate_dist(POS_INF));
        assert_eq!(POS_INF, inc_mate_dist_by(POS_INF, 2));
        assert_eq!(POS_INF, dec_mate_dist(POS_INF));
        assert_eq!(POS_INF, dec_mate_dist_by(POS_INF, 2));
    }

    #[test]
    #[should_panic]
    fn invalid_score_min() {
        let _ = ScoreVariant::from(Score::MIN);
    }

    #[test]
    #[should_panic]
    fn invalid_score_neg_inf() {
        let _ = ScoreVariant::from(NEG_INF);
    }

    #[test]
    #[should_panic]
    fn invalid_score_pos_inf() {
        let _ = ScoreVariant::from(POS_INF);
    }

    #[test]
    fn score_conversion() {
        let s = ScoreVariant::from(0);
        assert_eq!(ScoreVariant::Centipawns(0), s);
        assert_eq!("0.00", format!("{s}"));

        let s = ScoreVariant::from(WHITE_WIN);
        assert_eq!(ScoreVariant::Mate(Side::White, 0), s);
        assert_eq!("M0", format!("{s}"));
        let s = ScoreVariant::from(WHITE_WIN - 1);
        assert_eq!(ScoreVariant::Mate(Side::White, 1), s);
        assert_eq!("M1", format!("{s}"));
        let s = ScoreVariant::from(WHITE_WIN - 2);
        assert_eq!(ScoreVariant::Mate(Side::White, 1), s);
        assert_eq!("M1", format!("{s}"));
        let s = ScoreVariant::from(WHITE_WIN - 3);
        assert_eq!(ScoreVariant::Mate(Side::White, 2), s);
        assert_eq!("M2", format!("{s}"));
        let s = ScoreVariant::from(MAX_CP + 1);
        assert_eq!(ScoreVariant::Mate(Side::White, (MAX_MATE_DIST + 1) / 2), s);
        assert_eq!("M128", format!("{s}"));
        let s = ScoreVariant::from(MAX_CP);
        assert_eq!(ScoreVariant::Centipawns(MAX_CP), s);
        assert_eq!("325.10", format!("{s}"));

        let s = ScoreVariant::from(BLACK_WIN);
        assert_eq!(ScoreVariant::Mate(Side::Black, 0), s);
        assert_eq!("-M0", format!("{s}"));
        let s = ScoreVariant::from(BLACK_WIN + 1);
        assert_eq!(ScoreVariant::Mate(Side::Black, -1), s);
        assert_eq!("-M1", format!("{s}"));
        let s = ScoreVariant::from(BLACK_WIN + 2);
        assert_eq!(ScoreVariant::Mate(Side::Black, -1), s);
        assert_eq!("-M1", format!("{s}"));
        let s = ScoreVariant::from(BLACK_WIN + 3);
        assert_eq!(ScoreVariant::Mate(Side::Black, -2), s);
        assert_eq!("-M2", format!("{s}"));
        let s = ScoreVariant::from(MIN_CP - 1);
        assert_eq!(ScoreVariant::Mate(Side::Black, -(MAX_MATE_DIST + 1) / 2), s);
        assert_eq!("-M128", format!("{s}"));
        let s = ScoreVariant::from(MIN_CP);
        assert_eq!(ScoreVariant::Centipawns(MIN_CP), s);
        assert_eq!("-325.10", format!("{s}"));
    }
}
