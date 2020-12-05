use std::io::Read;
use std::io;



fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    //Damn, row length was wrong because of '\n's
    //println!("{:#?}", input);
    println!("{}", aoc::solve(
    	&aoc::parse_input(&buffer)
    ));
}










