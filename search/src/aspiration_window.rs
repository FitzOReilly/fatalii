use crate::search_params::SearchParams;
use eval::{Score, NEG_INF, POS_INF};

#[derive(Debug)]
pub struct AspirationWindow {
    score: i32,
    width_down: i32,
    width_up: i32,
    grow_rate: i32,
    alpha: Score,
    beta: Score,
}

impl AspirationWindow {
    pub fn infinite() -> Self {
        Self {
            score: 0,
            width_up: POS_INF as i32,
            width_down: POS_INF as i32,
            grow_rate: SearchParams::default().aspiration_window_grow_rate,
            alpha: NEG_INF,
            beta: POS_INF,
        }
    }

    pub fn new(s: Score, initial_width: i32, grow_rate: i32) -> Self {
        Self {
            score: s as i32,
            width_up: initial_width,
            width_down: initial_width,
            grow_rate,
            alpha: calc_alpha(s as i32, initial_width),
            beta: calc_beta(s as i32, initial_width),
        }
    }

    pub fn alpha(&self) -> Score {
        self.alpha
    }

    pub fn beta(&self) -> Score {
        self.beta
    }

    pub fn widen_down(&mut self) {
        self.width_down = (self.width_down * self.grow_rate).clamp(0, self.score - NEG_INF as i32);
        self.alpha = (self.score - self.width_down) as Score;
    }

    pub fn widen_up(&mut self) {
        self.width_up = (self.width_up * self.grow_rate).clamp(0, POS_INF as i32 - self.score);
        self.beta = (self.score + self.width_up) as Score;
    }
}

fn calc_alpha(score: i32, width_down: i32) -> Score {
    (score - width_down).clamp(NEG_INF as i32, POS_INF as i32) as Score
}

fn calc_beta(score: i32, width_up: i32) -> Score {
    (score + width_up).clamp(NEG_INF as i32, POS_INF as i32) as Score
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
        let mut aw = AspirationWindow::new(
            score,
            SearchParams::default().aspiration_window_initial_width,
            SearchParams::default().aspiration_window_grow_rate,
        );
        assert_eq!(
            score - SearchParams::default().aspiration_window_initial_width as Score,
            aw.alpha()
        );
        assert_eq!(
            score + SearchParams::default().aspiration_window_initial_width as Score,
            aw.beta()
        );

        aw.widen_down();
        assert_eq!(
            score
                - (SearchParams::default().aspiration_window_initial_width
                    * SearchParams::default().aspiration_window_grow_rate)
                    as Score,
            aw.alpha()
        );
        assert_eq!(
            score + SearchParams::default().aspiration_window_initial_width as Score,
            aw.beta()
        );

        aw.widen_up();
        assert_eq!(
            score
                - (SearchParams::default().aspiration_window_initial_width
                    * SearchParams::default().aspiration_window_grow_rate)
                    as Score,
            aw.alpha()
        );
        assert_eq!(
            score
                + (SearchParams::default().aspiration_window_initial_width
                    * SearchParams::default().aspiration_window_grow_rate)
                    as Score,
            aw.beta()
        );

        let mut prev_alpha = aw.alpha() + 1;
        while aw.alpha() != prev_alpha {
            prev_alpha = aw.alpha();
            aw.widen_down();
        }
        assert_eq!(NEG_INF, aw.alpha());
        let mut prev_beta = aw.beta() - 1;
        while aw.beta() != prev_beta {
            prev_beta = aw.beta();
            aw.widen_up();
        }
        assert_eq!(POS_INF, aw.beta());

        let neg_score = -1000;
        let mut neg_aw = AspirationWindow::new(
            neg_score,
            SearchParams::default().aspiration_window_initial_width,
            SearchParams::default().aspiration_window_grow_rate,
        );
        let mut prev_alpha = neg_aw.alpha() + 1;
        while neg_aw.alpha() != prev_alpha {
            prev_alpha = neg_aw.alpha();
            neg_aw.widen_down();
        }
        assert_eq!(NEG_INF, neg_aw.alpha());
        let mut prev_beta = neg_aw.beta() - 1;
        while neg_aw.beta() != prev_beta {
            prev_beta = neg_aw.beta();
            neg_aw.widen_up();
        }
        assert_eq!(POS_INF, neg_aw.beta());
    }
}
