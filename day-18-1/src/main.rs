use std::io::Read;
use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let data = aoc::tokenize(&buffer);
    println!("{:?}", aoc::solve(&data));
}










