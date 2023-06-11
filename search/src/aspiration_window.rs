use std::cmp;

use eval::{Score, NEG_INF, POS_INF};

const STEPS: [i32; 4] = [40, 160, 640, u16::MAX as i32];

#[derive(Debug)]
pub struct AspirationWindow {
    score: i32,
    idx_alpha: usize,
    idx_beta: usize,
}

impl AspirationWindow {
    pub fn infinite() -> Self {
        Self {
            score: 0,
            idx_alpha: STEPS.len() - 1,
            idx_beta: STEPS.len() - 1,
        }
    }

    pub fn new(s: Score) -> Self {
        Self {
            score: s as i32,
            idx_alpha: 0,
            idx_beta: 0,
        }
    }

    pub fn alpha(&self) -> Score {
        (self.score - STEPS[self.idx_alpha]).clamp(NEG_INF as i32, POS_INF as i32) as Score
    }

    pub fn beta(&self) -> Score {
        (self.score + STEPS[self.idx_beta]).clamp(NEG_INF as i32, POS_INF as i32) as Score
    }

    pub fn widen_down(&mut self) {
        self.idx_alpha = cmp::min(STEPS.len() - 1, self.idx_alpha + 1);
    }

    pub fn widen_up(&mut self) {
        self.idx_beta = cmp::min(STEPS.len() - 1, self.idx_beta + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infinite() {
        let aw = AspirationWindow::infinite();
        assert_eq!(NEG_INF, aw.alpha());
        assert_eq!(POS_INF, aw.beta());
    }

    #[test]
    fn widen() {
        let score = 200;
        let mut aw = AspirationWindow::new(score);
        assert_eq!(score - STEPS[0] as Score, aw.alpha());
        assert_eq!(score + STEPS[0] as Score, aw.beta());

        aw.widen_down();
        assert_eq!(score - STEPS[1] as Score, aw.alpha());
        assert_eq!(score + STEPS[0] as Score, aw.beta());

        aw.widen_up();
        assert_eq!(score - STEPS[1] as Score, aw.alpha());
        assert_eq!(score + STEPS[1] as Score, aw.beta());

        for _ in 0..STEPS.len() {
            aw.widen_down();
            aw.widen_up();
        }
        assert_eq!(NEG_INF, aw.alpha());
        assert_eq!(POS_INF, aw.beta());

        let neg_score = -1000;
        let mut neg_aw = AspirationWindow::new(neg_score);
        for _ in 0..STEPS.len() {
            neg_aw.widen_down();
            neg_aw.widen_up();
        }
        assert_eq!(NEG_INF, neg_aw.alpha());
        assert_eq!(POS_INF, neg_aw.beta());
    }
}
