use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct RepetitionTracker<K> {
    history: Vec<K>,
    plies_since_last_irreversible: Vec<usize>,
}

impl<K> RepetitionTracker<K>
where
    K: Debug + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            plies_since_last_irreversible: Vec::new(),
        }
    }

    pub fn current_pos_repetitions(&self) -> usize {
        let hash = match self.history.last() {
            Some(h) => h,
            None => return 0,
        };
        // We only need to check every second position because the side to move
        //  must be the same in each repetition
        const STEP: usize = 2;
        1 + self
            .history
            .iter()
            .rev()
            .take(self.plies_since_last_irreversible.last().unwrap_or(&0) + 1)
            .skip(2)
            .step_by(STEP)
            .filter(|&x| x == hash)
            .count()
    }

    pub fn push(&mut self, hash: K, is_reversible: bool) {
        self.history.push(hash);
        self.plies_since_last_irreversible
            .push(match is_reversible {
                true => self.plies_since_last_irreversible.last().unwrap_or(&0) + 1,
                false => 0,
            });
    }

    pub fn pop(&mut self) {
        self.history.pop();
        self.plies_since_last_irreversible.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repetitions() {
        let key_0 = "0";
        let key_1 = "1";
        let key_2 = "2";
        let key_irr = "irr";
        let mut rep_tracker = RepetitionTracker::new();

        rep_tracker.push(key_0, true);
        assert_eq!(1, rep_tracker.current_pos_repetitions());
        rep_tracker.push(key_1, true);
        assert_eq!(1, rep_tracker.current_pos_repetitions());
        rep_tracker.push(key_2, true);
        assert_eq!(1, rep_tracker.current_pos_repetitions());
        rep_tracker.push(key_1, true);
        assert_eq!(2, rep_tracker.current_pos_repetitions());
        rep_tracker.push(key_0, true);
        assert_eq!(2, rep_tracker.current_pos_repetitions());
        rep_tracker.push(key_1, true);
        assert_eq!(3, rep_tracker.current_pos_repetitions());
        rep_tracker.push(key_irr, false);
        assert_eq!(1, rep_tracker.current_pos_repetitions());
        rep_tracker.push(key_1, true);
        assert_eq!(1, rep_tracker.current_pos_repetitions());

        rep_tracker.pop();
        assert_eq!(1, rep_tracker.current_pos_repetitions());
        rep_tracker.pop();
        assert_eq!(3, rep_tracker.current_pos_repetitions());
        rep_tracker.pop();
        assert_eq!(2, rep_tracker.current_pos_repetitions());
        rep_tracker.pop();
        assert_eq!(2, rep_tracker.current_pos_repetitions());
        rep_tracker.pop();
        assert_eq!(1, rep_tracker.current_pos_repetitions());
        rep_tracker.pop();
        assert_eq!(1, rep_tracker.current_pos_repetitions());
        rep_tracker.pop();
        assert_eq!(1, rep_tracker.current_pos_repetitions());
        rep_tracker.pop();
    }
}
