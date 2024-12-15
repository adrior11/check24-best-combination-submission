use benchmarks::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_set_cover_comparison(c: &mut Criterion) {
    let (universe, subsets) = util::build_test_data();

    let mut group = c.benchmark_group("set_cover_comparison");

    group.bench_function("iterative", |b| {
        b.iter(|| {
            let result = iterative_set_cover(black_box(&universe), black_box(&subsets));
            black_box(result);
        })
    });

    group.bench_function("recursive", |b| {
        b.iter(|| {
            let result = recursive_set_covers(black_box(&universe), black_box(&subsets), 1);
            black_box(result);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_set_cover_comparison);
criterion_main!(benches);
