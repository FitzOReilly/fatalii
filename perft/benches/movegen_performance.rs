use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use movegen::move_generator::MoveGenerator;
use movegen::position::Position;
use perft::performance_tester::PerformanceTester;

fn perft_initial_position(c: &mut Criterion) {
    let min_depth = 0;
    let max_depth = 5;

    let movegen = MoveGenerator::new(Position::initial());
    let mut perft = PerformanceTester::new(movegen);

    let mut group = c.benchmark_group("Perft");
    for depth in min_depth..=max_depth {
        group.throughput(Throughput::Elements(depth as u64));
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &depth| {
            b.iter(|| perft.count_nodes(depth));
        });
    }
    group.finish();
}

criterion_group!(benches, perft_initial_position);
criterion_main!(benches);
