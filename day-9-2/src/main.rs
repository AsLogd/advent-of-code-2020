use std::io::Read;
use std::io;

const TARGET: u64 = 248131121;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let data = aoc::parse_input(&buffer);
    println!("{:?}", aoc::solve(&data, TARGET));
}










