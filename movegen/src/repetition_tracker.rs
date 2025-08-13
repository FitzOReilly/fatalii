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

    // Returns true if the position at the horizon has occured
    // - at least twice after the root, or
    // - at least three times in the entire position history
    pub fn is_repetition(&self, plies_from_root: usize) -> bool {
        const REPETITIONS_TO_DRAW: usize = 3;
        let Some(horizon_hash) = self.history.last() else {
            return false;
        };
        // We only need to check every second position because the side to move
        // must be the same in each repetition
        const STEP: usize = 2;
        let mut ply = plies_from_root as isize;
        let mut reps = 1;
        for hash in self
            .history
            .iter()
            .rev()
            .take(self.plies_since_last_irreversible.last().unwrap_or(&0) + 1)
            .skip(STEP)
            .step_by(STEP)
        {
            ply -= STEP as isize;
            if hash == horizon_hash {
                reps += 1;
                if ply > 0 || reps >= REPETITIONS_TO_DRAW {
                    return true;
                }
            }
        }
        false
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

        assert!(!rep_tracker.is_repetition(0));
        rep_tracker.push(key_0, true);
        assert!(!rep_tracker.is_repetition(1));
        rep_tracker.push(key_1, true);
        assert!(!rep_tracker.is_repetition(2));
        rep_tracker.push(key_2, true);
        assert!(!rep_tracker.is_repetition(3));
        rep_tracker.push(key_1, true);
        assert!(rep_tracker.is_repetition(4));
        assert!(rep_tracker.is_repetition(3));
        assert!(!rep_tracker.is_repetition(2));
        rep_tracker.push(key_0, true);
        assert!(rep_tracker.is_repetition(5));
        assert!(!rep_tracker.is_repetition(4));
        rep_tracker.push(key_1, true);
        assert!(rep_tracker.is_repetition(6));
        assert!(rep_tracker.is_repetition(1));
        assert!(rep_tracker.is_repetition(0));
        rep_tracker.push(key_irr, false);
        assert!(!rep_tracker.is_repetition(7));
        rep_tracker.push(key_1, true);
        assert!(!rep_tracker.is_repetition(8));

        rep_tracker.pop();
        assert!(!rep_tracker.is_repetition(7));
        rep_tracker.pop();
        assert!(rep_tracker.is_repetition(6));
        assert!(rep_tracker.is_repetition(1));
        assert!(rep_tracker.is_repetition(0));
        rep_tracker.pop();
        assert!(rep_tracker.is_repetition(5));
        assert!(!rep_tracker.is_repetition(4));
        rep_tracker.pop();
        assert!(rep_tracker.is_repetition(4));
        assert!(rep_tracker.is_repetition(3));
        assert!(!rep_tracker.is_repetition(2));
        rep_tracker.pop();
        assert!(!rep_tracker.is_repetition(3));
        rep_tracker.pop();
        assert!(!rep_tracker.is_repetition(2));
        rep_tracker.pop();
        assert!(!rep_tracker.is_repetition(1));
        rep_tracker.pop();
        assert!(!rep_tracker.is_repetition(0));
    }
}
