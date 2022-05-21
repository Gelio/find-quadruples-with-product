use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use find_quadruples_with_products::{division, naive, pruning};
use rand::{prelude::SmallRng, Rng, SeedableRng};

fn bench_solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Solutions");
    let mut rng = SmallRng::seed_from_u64(1337);
    for i in [5, 50, 500, 1000, 2000] {
        let array: Vec<i64> = (0..i).map(|_| rng.gen()).collect();

        if i <= 1000 {
            // NOTE: it takes too long to run the naive solution with large inputs
            group.bench_with_input(BenchmarkId::new("Naive O(n^4)", i), &i, |b, _i| {
                b.iter(|| naive::solve(&array))
            });
        }

        group.bench_with_input(BenchmarkId::new("Using pruning O(n^3)", i), &i, |b, _i| {
            b.iter(|| pruning::solve(&array))
        });
        group.bench_with_input(
            BenchmarkId::new("Using division O(n^2 * log n)", i),
            &i,
            |b, _i| b.iter(|| division::solve(&array)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_solutions);
criterion_main!(benches);
