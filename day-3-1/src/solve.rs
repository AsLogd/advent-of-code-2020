const DX: usize = 3;
const DY: usize = 1;

pub fn parse_entries(lines: &Vec<String>) -> Vec<Vec<bool>> {
    lines.iter().map(|l|
        l.trim().chars().map(|c| 
            c == '#'
        ).collect()
    ).collect()
}

pub fn solve(input: &Vec<Vec<bool>>) -> u32 {
    //solve_functional(input)
    solve_imperative(input)
}

pub fn solve_functional(input: &Vec<Vec<bool>>) -> u32 {
    input.iter()
        .enumerate()
        .filter(|(i, _)| i % DY == 0)
        .map(|(_, row)| row)
        .enumerate()
        .fold(0, |sum, (i, row)| 
            sum + row[ (i*DX) % row.len() ] as u32
        )
}

pub fn solve_imperative(input: &Vec<Vec<bool>>) -> u32 {
    let mut sum: u32 = 0;
    let row_len = input[0].len();
    let rows_iter = (0..input.len()).step_by(DY);
    for i in  rows_iter {
        let d = (i*DX) % row_len;
        sum += input[i][d] as u32;
    }
    sum
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