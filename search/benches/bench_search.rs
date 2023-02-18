use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use crossbeam_channel::{unbounded, Receiver};
use eval::complex::Complex;
use eval::Eval;
use movegen::fen::Fen;
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use search::alpha_beta::AlphaBeta;
use search::negamax::Negamax;
use search::search::{Search, SearchInfo, SearchResult};
use search::searcher::Searcher;
use search::SearchOptions;
use std::time::Duration;

const TIMEOUT_PER_BENCH: Duration = Duration::from_millis(10000);

fn evaluator() -> impl Eval {
    Complex::new()
}

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
        let search_options = SearchOptions {
            depth: Some(depth),
            ..Default::default()
        };
        self.searcher.search(pos_hist, search_options);
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

fn negamax(c: &mut Criterion, group_name: &str, pos: Position, min_depth: usize, max_depth: usize) {
    let table_size = 16 * 1024 * 1024;
    let pos_history = PositionHistory::new(pos);

    let mut group = c.benchmark_group(group_name);
    for depth in min_depth..=max_depth {
        group.throughput(Throughput::Elements(depth as u64));
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &depth| {
            b.iter_batched(
                || SearchBencher::new(Negamax::new(Box::new(evaluator()), table_size)),
                |mut searcher| searcher.search(pos_history.clone(), depth),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn alpha_beta(
    c: &mut Criterion,
    group_name: &str,
    pos: Position,
    min_depth: usize,
    max_depth: usize,
) {
    let table_size = 16 * 1024 * 1024;
    let pos_history = PositionHistory::new(pos);

    let mut group = c.benchmark_group(group_name);
    for depth in min_depth..=max_depth {
        group.throughput(Throughput::Elements(depth as u64));
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &depth| {
            b.iter_batched(
                || SearchBencher::new(AlphaBeta::new(Box::new(evaluator()), table_size)),
                |mut searcher| searcher.search(pos_history.clone(), depth),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn negamax_initial_position(c: &mut Criterion) {
    let group_name = "Negamax initial position";
    let pos = Position::initial();
    let min_depth = 1;
    let max_depth = 3;

    negamax(c, group_name, pos, min_depth, max_depth);
}

#[allow(dead_code)]
fn negamax_middlegame_position(c: &mut Criterion) {
    let group_name = "Negamax middlegame position";
    // Position from https://www.chessprogramming.org/Perft_Results
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let pos = Fen::str_to_pos(fen).unwrap();
    let min_depth = 1;
    let max_depth = 4;

    negamax(c, group_name, pos, min_depth, max_depth);
}

fn alpha_beta_initial_position(c: &mut Criterion) {
    let group_name = "Alpha-Beta initial position";
    let pos = Position::initial();
    let min_depth = 1;
    let max_depth = 6;

    alpha_beta(c, group_name, pos, min_depth, max_depth);
}

fn alpha_beta_middlegame_position(c: &mut Criterion) {
    let group_name = "Alpha-Beta middlegame position";
    // Position from https://www.chessprogramming.org/Perft_Results
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let pos = Fen::str_to_pos(fen).unwrap();
    let min_depth = 1;
    let max_depth = 6;

    alpha_beta(c, group_name, pos, min_depth, max_depth);
}

criterion_group!(
    benches,
    negamax_initial_position,
    alpha_beta_initial_position,
    alpha_beta_middlegame_position,
);
criterion_main!(benches);
