use criterion::{criterion_group, criterion_main, Criterion};

const INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

// Fast version is about an order of magnitude faster
pub fn fast_benchmark(_c: &mut Criterion) {
	let _input = aoc::parse_input(&INPUT);
    //c.bench_function("fast", |b| b.iter(|| aoc::solve_fast(&input, 5)));

}

pub fn slow_benchmark(_c: &mut Criterion) {
	let _input = aoc::parse_input(&INPUT);
    //c.bench_function("slow", |b| b.iter(|| aoc::solve_slow(&input, 5)));
}

criterion_group!(benches, fast_benchmark, slow_benchmark);
criterion_main!(benches);

