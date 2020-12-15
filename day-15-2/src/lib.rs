use std::collections::HashMap;

const Q_NUM: u32 = 30000000;

pub fn parse_input(input: &str) -> Vec<u32> {
    input.trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}


pub fn solve(input: &[u32]) -> u32 {
    let mut memory: HashMap<u32, u32> = HashMap::new();
    let mut spoken;
    let n_start = input.len() as u32;
    for i in 0..n_start-1 {
        memory.insert(input[i as usize], 1+i as u32);
        //println!("Inserted {}", input[i]);
    }
    spoken = *input.last().unwrap();
    for i in n_start..Q_NUM {
        let turn = i;
        let last_turn = memory.entry(spoken).or_insert(0);
        //println!("Turn {}: spoken {}, saves {}, will speak {}", turn, spoken, turn, if *last_turn == 0 {0} else {turn - *last_turn} );
        spoken = if *last_turn == 0 {0} else {turn - *last_turn};
        *last_turn = turn;

    };
    spoken
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn solve_test() {
        let input = String::from("0,3,6");
        let input = parse_input(&input);
        assert_eq!(175594, solve(&input));
    }

    #[test]
    fn solve_extra_1() {
        let input = String::from("1,3,2");
        let input = parse_input(&input);
        assert_eq!(2578, solve(&input));
    }
    #[test]
    fn solve_extra_2() {
        let input = String::from("1,2,3
");
        let input = parse_input(&input);
        assert_eq!(261214, solve(&input));
    }
    #[test]
    fn solve_extra_3() {
        let input = String::from("3,2,1");
        let input = parse_input(&input);
        assert_eq!(18, solve(&input));
    }
    #[test]
    fn solve_extra_4() {
        let input = String::from("3,1,2");
        let input = parse_input(&input);
        assert_eq!(362, solve(&input));
    }
}