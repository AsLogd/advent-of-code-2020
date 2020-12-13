use criterion::{criterion_group, criterion_main, Criterion};

// Fast version is about an order of magnitude faster
pub fn fast_benchmark(_c: &mut Criterion) {
	let input = aoc::parse_input(&INPUT);
    //c.bench_function("fast", |b| b.iter(|| aoc::solve_fast(&input, 5)));

}

pub fn slow_benchmark(_c: &mut Criterion) {
	let input = aoc::parse_input(&INPUT);
    //c.bench_function("slow", |b| b.iter(|| aoc::solve_slow(&input, 5)));
}

criterion_group!(benches, fast_benchmark, slow_benchmark);
criterion_main!(benches);

