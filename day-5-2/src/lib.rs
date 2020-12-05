const D: usize = 7;

#[derive(PartialEq, Debug)]
pub struct Seat {
    row: u32,
    column: u32
}

impl Seat{
    fn id(&self) -> u32 {
        self.row*8 + self.column
    }
}

pub fn parse_input(input: &str) -> Vec<Seat> {
    input.trim().split('\n').map(|line| {
        let row_str = &line[..D];
        let row_bin = row_str.chars().map(|c| 
            if c == 'F' {'0'} else {'1'}
        ).collect::<String>();
        //println!("{:?} -> {:?}", row_str, row_bin);

        let col_str = &line[D..];
        let col_bin = col_str.chars().map(|c| 
            if c == 'L' {'0'} else {'1'}
        ).collect::<String>();
        //println!("{:?} -> {:?}", col_str, col_bin);
        Seat {
            row: u32::from_str_radix(&row_bin, 2).unwrap(),
            column: u32::from_str_radix(&col_bin, 2).unwrap()
        }
    }).collect::<Vec<Seat>>()
}

pub fn solve(seats: &Vec<Seat>) -> u32 {
    let mut ids= seats.iter()
        .map(|x| x.id())
        .collect::<Vec<u32>>();
    ids.sort();
    for i in 1..seats.len() {
        if ids[i] - ids[i-1] == 2 {
            return ids[i]-1
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input() {
        let input = String::from("BFFFBBFRRR");
        assert_eq!(parse_input(&input), [Seat{
            row: 70,
            column: 7
        }]);
    }

    #[test]
    fn solve_test() {
        let input = String::from("BFFFFFFLLL
BFFFFFFLRR
BFFFFFFLLR
");
        assert_eq!(solve(&parse_input(&input)), 64*8+2);

    }
}