use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeCounter {
    node_counts: Vec<Vec<(u64, u64)>>,
    eval_count: Vec<u64>,
    max_depth: usize,
}

impl NodeCounter {
    pub fn new() -> Self {
        Self {
            node_counts: Vec::new(),
            eval_count: Vec::new(),
            max_depth: 0,
        }
    }

    pub fn increment_nodes(&mut self, search_depth: usize, plies_from_end: usize) {
        self.reserve(search_depth);
        self.node_counts[search_depth - 1][plies_from_end].0 += 1;
    }

    pub fn increment_cache_hits(&mut self, search_depth: usize, plies_from_end: usize) {
        self.reserve(search_depth);
        self.node_counts[search_depth - 1][plies_from_end].1 += 1;
    }

    pub fn increment_eval_calls(&mut self, search_depth: usize) {
        self.reserve(search_depth);
        self.eval_count[search_depth - 1] += 1;
    }

    pub fn sum_nodes(&self) -> u64 {
        self.node_counts
            .iter()
            .map(|nc| nc.iter().map(|x| x.0).sum::<u64>())
            .sum()
    }

    fn reserve(&mut self, search_depth: usize) {
        if search_depth > self.max_depth {
            debug_assert!(search_depth == self.max_depth + 1);
            self.node_counts.push(vec![(0, 0); search_depth + 1]);
            self.eval_count.push(0);
            self.max_depth += 1;
        }
    }
}

impl fmt::Display for NodeCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for d in 1..=self.max_depth {
            writeln!(f, "Search depth {d}:")?;
            writeln!(f, "\tEvaluate calls: {}", self.eval_count[d - 1])?;
            for p in 0..=d {
                let nc = &self.node_counts[d - 1][p];
                writeln!(
                    f,
                    "\tPlies from end of PV / moves made / cache hits: {} / {} / {}",
                    p, nc.0, nc.1,
                )?;
            }
        }
        Ok(())
    }
}
