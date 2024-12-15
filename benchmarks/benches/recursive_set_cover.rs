use benchmarks::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_recursive_set_cover(c: &mut Criterion) {
    let (universe, subsets) = util::build_test_data();

    let mut group = c.benchmark_group("recursive_set_cover");
    group.measurement_time(Duration::from_secs(30));
    group.sample_size(50);

    for i in 1..=10 {
        group.bench_function(BenchmarkId::new("recursive", i), |b| {
            b.iter(|| {
                let result =
                    recursive_set_covers(black_box(&universe), black_box(&subsets), black_box(i));
                black_box(result);
            })
        });
    }
}

criterion_group!(benches, bench_recursive_set_cover);
criterion_main!(benches);
