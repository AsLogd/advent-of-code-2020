const SLOPES: [(usize, usize); 5] = [
    (1,1),
    (3,1),
    (5,1),
    (7,1),
    (1,2),
];

pub fn parse_entries(lines: &Vec<String>) -> Vec<Vec<bool>> {
    lines.iter().map(|l|
        l.trim().chars().map(|c| 
            c == '#'
        ).collect()
    ).collect()
}

// Wow, this code is faster than imperative. didn't expect it
pub fn solve_slope_functional(x: &usize, y: &usize,input: &Vec<Vec<bool>>) -> u32 {
    input.iter()
        .enumerate()
        .filter(|(i, _)| i % y == 0)
        .map(|(_, row)| row)
        .enumerate()
        .fold(0, |sum, (i, row)| 
            sum + row[ (i*x) % row.len() ] as u32
        )
}

pub fn solve_slope_imperative(x: &usize, y: &usize,input: &Vec<Vec<bool>>) -> u32 {
    let mut sum: u32 = 0;
    let row_len = input[0].len();
    let rows_iter = (0..input.len()).step_by(*y);
    for i in  rows_iter {
        let d = (i*x) % row_len;
        sum += input[i][d] as u32;
    }
    sum
}


pub fn solve_functional(input: &Vec<Vec<bool>>) -> u32 {
    SLOPES.iter()
        .map(|(x,y)| solve_slope_functional(x, y, input))
        .fold(1, |prod, x| prod*x)
}

pub fn solve_imperative(input: &Vec<Vec<bool>>) -> u32 {
    SLOPES.iter()
        .map(|(x,y)| solve_slope_imperative(x, y, input))
        .fold(1, |prod, x| prod*x)
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
        assert_eq!(solve_imperative(&entries), 336);
        
    }

}