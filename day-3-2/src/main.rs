use std::io;



fn main() {
    let mut input = Vec::new();
    let mut buffer = String::new();
    loop {
        buffer.clear();
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                // Probably would be faster to parse
                // the line here, without cloning
                input.push(buffer.clone());
            }
            Err(error) => println!("error: {}", error),
        }
    }
    //Damn, row length was wrong because of '\n's
    //println!("{:#?}", input);
    println!("{}", aoc::solve_imperative(&aoc::parse_entries(&input)));
}










