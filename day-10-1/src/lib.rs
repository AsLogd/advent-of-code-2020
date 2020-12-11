
use std::convert::TryInto;

pub fn parse_input(input: &str) -> Vec<i32> {
    input.trim()
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}


pub fn solve(input: &[i32]) -> i32 {
    let mut sorted = Vec::from(input);
    sorted.sort();
    sorted.push(sorted.last().unwrap()+3);
    let mut shifted = sorted.clone();
    shifted.insert(0,0);
    let differences = sorted.iter()
        .zip(shifted.iter())
        .map(|(curr, pre)| curr - pre).collect::<Vec<i32>>();
    let ones = differences.clone().iter()
        .filter(|x| **x == 1 as i32)
        .count();
    let threes = differences.clone().iter()
        .filter(|x| **x == 3 as i32)
        .count();
    //println!("{:#?}", &differences);
    //println!("{:?}, {:?}", &ones, &threes);
    (ones*threes).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let input = String::from("28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
");
        let input = parse_input(&input);
        assert_eq!(220, solve(&input));
    }
}

