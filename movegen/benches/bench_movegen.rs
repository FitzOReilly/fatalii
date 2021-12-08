use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use perft::PerformanceTester;

fn perft_initial_position(c: &mut Criterion) {
    let min_depth = 0;
    let max_depth = 5;
    let table_idx_bits = 16;

    let mut group = c.benchmark_group("Perft");
    for depth in min_depth..=max_depth {
        group.throughput(Throughput::Elements(depth as u64));
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &depth| {
            b.iter_batched(
                || {
                    PerformanceTester::new(
                        PositionHistory::new(Position::initial()),
                        table_idx_bits,
                    )
                },
                |mut perft| perft.count_nodes(depth),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, perft_initial_position);
criterion_main!(benches);
