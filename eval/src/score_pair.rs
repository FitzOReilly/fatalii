use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use crate::Score;

#[derive(Debug, Clone, Copy)]
pub struct ScorePair(pub Score, pub Score);

impl Mul<Score> for ScorePair {
    type Output = Self;

    fn mul(self, rhs: Score) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<&Score> for ScorePair {
    type Output = Self;

    fn mul(self, rhs: &Score) -> Self::Output {
        self * *rhs
    }
}

impl Mul<ScorePair> for Score {
    type Output = ScorePair;

    fn mul(self, rhs: ScorePair) -> Self::Output {
        ScorePair(self * rhs.0, self * rhs.1)
    }
}

impl Mul<&ScorePair> for Score {
    type Output = ScorePair;

    fn mul(self, rhs: &ScorePair) -> Self::Output {
        self * *rhs
    }
}

impl Add for ScorePair {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for ScorePair {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for ScorePair {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for ScorePair {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
