use criterion::{criterion_group, criterion_main, Criterion};

pub fn imperative_benchmark(_c: &mut Criterion) {
    //c.bench_function("imperative", |b| b.iter(|| aoc::solve_imperative(&INPUT)));

}

pub fn functional_benchmark(_c: &mut Criterion) {
    //c.bench_function("functional", |b| b.iter(|| aoc::solve_functional(&INPUT)));
}

criterion_group!(benches, imperative_benchmark, functional_benchmark);
criterion_main!(benches);

