use algorithmic_toolbox::math::{fib, fib_naive};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark1(c: &mut Criterion) {
    c.bench_function("fib 10", |b| b.iter(|| fib(black_box(10))));
}

fn benchmark2(c: &mut Criterion) {
    c.bench_function("fib_naive 10", |b| b.iter(|| fib_naive(black_box(10))));
}

criterion_group!(benches, benchmark1, benchmark2);
criterion_main!(benches);
