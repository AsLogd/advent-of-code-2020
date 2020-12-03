use std::io;

const DX: usize = 3;
const DY: usize = 1;

fn parse_entries(lines: &Vec<String>) -> Vec<Vec<bool>> {
    lines.iter().map(|l|
        l.trim().chars().map(|c| 
            c == '#'
        ).collect()
    ).collect()
}

fn solve(input: &Vec<Vec<bool>>) -> u32 {
    input.iter()
        .enumerate()
        .filter(|(i, _)| i % DY == 0)
        .map(|(_, row)| row)
        .enumerate()
        .fold(0, |sum, (i, row)| 
            sum + row[ (i*DX) % row.len() ] as u32
        )
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
    fn read_input() {
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
        assert_eq!(entries, [
            [false,false,true,true,false,false,false,false,false,false,false],
            [true,false,false,false,true,false,false,false,true,false,false],
            [false,true,false,false,false,false,true,false,false,true,false],
            [false,false,true,false,true,false,false,false,true,false,true],
            [false,true,false,false,false,true,true,false,false,true,false],
            [false,false,true,false,true,true,false,false,false,false,false],
            [false,true,false,true,false,true,false,false,false,false,true],
            [false,true,false,false,false,false,false,false,false,false,true],
            [true,false,true,true,false,false,false,true,false,false,false],
            [true,false,false,false,true,true,false,false,false,false,true],
            [false,true,false,false,true,false,false,false,true,false,true],
        ])
    }
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
        assert_eq!(solve(&entries), 7);
    }

    #[test]
    fn input_excerpt() {
        let input = vec![
            String::from(".#..........#......#..#.....#.."),
            String::from("....#.............#.#....#..#.."),
            String::from(".....##...###....#..#.......#.."),
            String::from(".#....#..#......#........#....."),
            String::from(".#.........###.#..........##..."),
            String::from("...............##........#....."),
        ];
        let entries = parse_entries(&input);
        assert_eq!(solve(&entries), 4);
    }

    
    #[test]
    fn input_excerpt_noloop() {
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
        ];
        let entries = parse_entries(&input);
        assert_eq!(solve(&entries), 7);
    }

    #[test]
    fn rows_four() {
        let input = vec![
            String::from("...."),
            String::from("...#"),
            String::from("..#."),
            String::from(".#.."),
            String::from("#..."),
            String::from("...#"),
            String::from("..#."),
            String::from(".#.."),
            String::from("#..."),
        ];
        let entries = parse_entries(&input);
        assert_eq!(solve(&entries), 8);
    }
    #[test]
    fn spaces() {
        let input = vec![
            String::from("...."),
            String::from("###."),
            String::from("##.#"),
            String::from("#.##"),
            String::from(".###"),
            String::from("###."),
            String::from("##.#"),
            String::from("#.##"),
            String::from(".###"),
        ];
        let entries = parse_entries(&input);
        assert_eq!(solve(&entries), 0);
    }

    #[test]
    fn input_excerpt_loop() {
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
        let entries = parse_entries(&input);
        assert_eq!(solve(&entries), 9);
    }
}









