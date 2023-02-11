use eval::{Score, ScoreVariant};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct UciScore(ScoreVariant);

impl From<ScoreVariant> for UciScore {
    fn from(s: ScoreVariant) -> Self {
        Self(s)
    }
}

impl From<Score> for UciScore {
    fn from(s: Score) -> Self {
        Self(ScoreVariant::from(s))
    }
}

impl fmt::Display for UciScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ScoreVariant::Centipawns(cp) => write!(f, "cp {cp}"),
            ScoreVariant::Mate(_, dist) => write!(f, "mate {dist}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use movegen::side::Side;

    #[test]
    fn score_conversion() {
        let s = UciScore::from(0);
        assert_eq!(ScoreVariant::Centipawns(0), s.0);
        assert_eq!("cp 0", format!("{s}"));

        let s = UciScore::from(ScoreVariant::Mate(Side::White, 0));
        assert_eq!(ScoreVariant::Mate(Side::White, 0), s.0);
        assert_eq!("mate 0", format!("{s}"));
        let s = UciScore::from(ScoreVariant::Mate(Side::White, 1));
        assert_eq!(ScoreVariant::Mate(Side::White, 1), s.0);
        assert_eq!("mate 1", format!("{s}"));
        let s = UciScore::from(167);
        assert_eq!(ScoreVariant::Centipawns(167), s.0);
        assert_eq!("cp 167", format!("{s}"));

        let s = UciScore::from(ScoreVariant::Mate(Side::Black, 0));
        assert_eq!(ScoreVariant::Mate(Side::Black, 0), s.0);
        assert_eq!("mate 0", format!("{s}"));
        let s = UciScore::from(ScoreVariant::Mate(Side::Black, -1));
        assert_eq!(ScoreVariant::Mate(Side::Black, -1), s.0);
        assert_eq!("mate -1", format!("{s}"));
        let s = UciScore::from(-225);
        assert_eq!(ScoreVariant::Centipawns(-225), s.0);
        assert_eq!("cp -225", format!("{s}"));
    }
}
