use criterion::{criterion_group, criterion_main, Criterion};


pub fn imperative_benchmark(c: &mut Criterion) {
    let input = vec![
        String::from(".#..........#......#..#.....#.."),
        String::from("....#.............#.#....#..#.."),
        String::from(".....##...###....#..#.......#.."),
        String::from(".#....#..#......#........#....."),
        String::from(".#.........###.#..........##..."),
        String::from("...............##........#....."),
        String::from("#..#..........#..##..#....#.#.."),
        String::from("....#.##....#..#...#.#....#...."),
        String::from("...###...#............#.#......"),
        String::from("#.........#..#...............#."),
        String::from("#.#...........#...............#"),
        String::from("..#.#......#..###.#...#..##...."),
        String::from("..#......#...........#..#..#.#."),
        String::from(".....##.....#.####....#........"),

    ];
    let entries = aoc::parse_entries(&input);
    c.bench_function("imperative", |b| b.iter(|| aoc::solve_imperative(&entries)));

}

// Wow, this code is faster than imperative. didn't expect it
pub fn functional_benchmark(c: &mut Criterion) {
    let input = vec![
        String::from(".#..........#......#..#.....#.."),
        String::from("....#.............#.#....#..#.."),
        String::from(".....##...###....#..#.......#.."),
        String::from(".#....#..#......#........#....."),
        String::from(".#.........###.#..........##..."),
        String::from("...............##........#....."),
        String::from("#..#..........#..##..#....#.#.."),
        String::from("....#.##....#..#...#.#....#...."),
        String::from("...###...#............#.#......"),
        String::from("#.........#..#...............#."),
        String::from("#.#...........#...............#"),
        String::from("..#.#......#..###.#...#..##...."),
        String::from("..#......#...........#..#..#.#."),
        String::from(".....##.....#.####....#........"),

    ];
    let entries = aoc::parse_entries(&input);
    c.bench_function("functional", |b| b.iter(|| aoc::solve_functional(&entries)));
}



criterion_group!(benches, imperative_benchmark, functional_benchmark);
criterion_main!(benches);