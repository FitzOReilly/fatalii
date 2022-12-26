use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use movegen::fen::Fen;
use movegen::position::Position;
use movegen::position_history::PositionHistory;
use perft::PerformanceTester;

fn perft(c: &mut Criterion, group_name: &str, pos: Position, min_depth: usize, max_depth: usize) {
    let bytes = 32 * 64 * 1024;

    let mut group = c.benchmark_group(group_name);
    for depth in min_depth..=max_depth {
        group.throughput(Throughput::Elements(depth as u64));
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &depth| {
            b.iter_batched(
                || PerformanceTester::new(PositionHistory::new(pos.clone()), bytes),
                |mut perft| perft.count_nodes(depth),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn perft_initial_position(c: &mut Criterion) {
    let group_name = "Perft initial position";
    let pos = Position::initial();
    let min_depth = 0;
    let max_depth = 5;

    perft(c, group_name, pos, min_depth, max_depth);
}

fn perft_middlegame_position(c: &mut Criterion) {
    let group_name = "Perft middlegame position";
    // Position from https://www.chessprogramming.org/Perft_Results
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let pos = Fen::str_to_pos(fen).unwrap();
    let min_depth = 0;
    let max_depth = 4;

    perft(c, group_name, pos, min_depth, max_depth);
}

criterion_group!(benches, perft_initial_position, perft_middlegame_position);
criterion_main!(benches);
