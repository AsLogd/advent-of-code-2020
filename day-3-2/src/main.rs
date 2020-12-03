use std::io;

const SLOPES: [(usize, usize); 5] = [
    (1,1),
    (3,1),
    (5,1),
    (7,1),
    (1,2),
];

fn parse_entries(lines: &Vec<String>) -> Vec<Vec<bool>> {
    lines.iter().map(|l|
        l.trim().chars().map(|c| 
            c == '#'
        ).collect()
    ).collect()
}

fn solve_slope(x: &usize, y: &usize,input: &Vec<Vec<bool>>) -> u32 {
    input.iter()
        .enumerate()
        .filter(|(i, _)| i % y == 0)
        .map(|(_, row)| row)
        .enumerate()
        .fold(0, |sum, (i, row)| 
            sum + row[ (i*x) % row.len() ] as u32
        )
}

fn solve(input: &Vec<Vec<bool>>) -> u32 {
    SLOPES.iter()
        .map(|(x,y)| solve_slope(x, y, input))
        .fold(1, |prod, x| prod*x)
}

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
    println!("{}", solve(&parse_entries(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_input() {
        let input = vec![
            String::from("..##......."),
            String::from("#...#...#.."),
            String::from(".#....#..#."),
            String::from("..#.#...#.#"),
            String::from(".#...##..#."),
            String::from("..#.##....."),
            String::from(".#.#.#....#"),
            String::from(".#........#"),
            String::from("#.##...#..."),
            String::from("#...##....#"),
            String::from(".#..#...#.#"),
        ];
        let entries = parse_entries(&input);
        assert_eq!(solve(&entries), 336);
    }

}









