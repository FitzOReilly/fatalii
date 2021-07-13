use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use search::alpha_beta::AlphaBeta;
use search::negamax::Negamax;
use search::search::Search;

fn negamax_initial_position(c: &mut Criterion) {
    let min_depth = 0;
    let max_depth = 3;
    let table_idx_bits = 20;

    let mut pos_history = PositionHistory::new(Position::initial());

    let mut group = c.benchmark_group("Negamax");
    for depth in min_depth..=max_depth {
        group.throughput(Throughput::Elements(depth as u64));
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &depth| {
            b.iter_batched(
                || Negamax::new(table_idx_bits),
                |mut negamax| negamax.search(&mut pos_history, depth),
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

    let mut pos_history = PositionHistory::new(Position::initial());

    let mut group = c.benchmark_group("Alpha-Beta");
    for depth in min_depth..=max_depth {
        group.throughput(Throughput::Elements(depth as u64));
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &depth| {
            b.iter_batched(
                || AlphaBeta::new(table_idx_bits),
                |mut alpha_beta| alpha_beta.search(&mut pos_history, depth),
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
