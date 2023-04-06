use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pathseeker::{simple, AStar, Map, Position};

fn criterion_benchmark(c: &mut Criterion) {
    let map = Map::new(16, 16);
    let start = Position { x: 0, y: 0 };
    let goal = Position { x: 15, y: 15 };

    c.bench_function("Astar Search 16x16", |b| {
        b.iter(|| {
            AStar::search(
                black_box(&map),
                black_box(start),
                black_box(goal),
                black_box(simple),
            )
        })
    });

    c.bench_function("Astar Search 256x256", |b| {
        b.iter(|| {
            AStar::search(
                black_box(&Map::new(256, 256)),
                black_box(Position { x: 0, y: 0 }),
                black_box(Position { x: 255, y: 255 }),
                black_box(simple),
            )
        })
    });

    c.bench_function("Astar Search 512x512", |b| {
        b.iter(|| {
            AStar::search(
                black_box(&Map::new(512, 512)),
                black_box(Position { x: 0, y: 0 }),
                black_box(Position { x: 511, y: 511 }),
                black_box(simple),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
