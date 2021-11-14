use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use crossbeam_channel::{unbounded, Receiver};
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use search::alpha_beta::AlphaBeta;
use search::negamax::Negamax;
use search::search::{Search, SearchInfo, SearchResult};
use search::searcher::Searcher;
use std::time::Duration;

const TIMEOUT_PER_BENCH: Duration = Duration::from_millis(10000);

struct SearchBencher {
    searcher: Searcher,
    result_receiver: Receiver<SearchInfo>,
}

impl SearchBencher {
    fn new(search_algo: impl Search + Send + 'static) -> SearchBencher {
        let (sender, receiver) = unbounded();
        let info_callback = Box::new(move |search_info| {
            sender.send(search_info).unwrap();
        });

        SearchBencher {
            searcher: Searcher::new(search_algo, info_callback),
            result_receiver: receiver,
        }
    }

    fn search(&mut self, pos_hist: PositionHistory, depth: usize) -> SearchResult {
        self.searcher.search(pos_hist, depth);
        loop {
            let search_result = match self.result_receiver.recv_timeout(TIMEOUT_PER_BENCH) {
                Ok(SearchInfo::DepthFinished(res)) => res,
                unexp => panic!("Expected SearchInfo::DepthFinished(_), got {:?}", unexp),
            };
            assert!(
                search_result.depth() <= depth,
                "Expected max depth: {}, actual depth: {}",
                depth,
                search_result.depth()
            );
            if search_result.depth() == depth {
                self.searcher.stop();
                loop {
                    if let Ok(SearchInfo::Stopped) = self.result_receiver.recv() {
                        break;
                    }
                }
                return search_result;
            }
        }
    }
}

fn negamax_initial_position(c: &mut Criterion) {
    let min_depth = 0;
    let max_depth = 3;
    let table_idx_bits = 20;

    let pos_history = PositionHistory::new(Position::initial());

    let mut group = c.benchmark_group("Negamax");
    for depth in min_depth..=max_depth {
        group.throughput(Throughput::Elements(depth as u64));
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &depth| {
            b.iter_batched(
                || SearchBencher::new(Negamax::new(table_idx_bits)),
                |mut searcher| searcher.search(pos_history.clone(), depth),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn alpha_beta_initial_position(c: &mut Criterion) {
    let min_depth = 0;
    let max_depth = 5;
    let table_idx_bits = 20;

    let pos_history = PositionHistory::new(Position::initial());

    let mut group = c.benchmark_group("Alpha-Beta");
    for depth in min_depth..=max_depth {
        group.throughput(Throughput::Elements(depth as u64));
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &depth| {
            b.iter_batched(
                || SearchBencher::new(AlphaBeta::new(table_idx_bits)),
                |mut searcher| searcher.search(pos_history.clone(), depth),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    negamax_initial_position,
    alpha_beta_initial_position
);
criterion_main!(benches);
