use std::collections::HashSet;

pub fn parse_input(input: &str) -> Vec<u64> {
    input.trim()
        .split('\n')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

pub fn solve_fast(input: &[u64], pre: usize) -> u64 {
    for (i, n) in input[pre..].iter().enumerate() {
        let base = &input[i..(i+pre)];
        let map = base.iter().cloned().collect::<HashSet<u64>>();
        let subtraction = base.iter().map(|x| 
            i64::abs(((*n as i64) - (*x as i64)) as i64) as u64
        ).collect::<HashSet<u64>>();
        let matches = map.intersection(&subtraction).collect::<HashSet<_>>();
        if matches.is_empty() {
            return *n;
        }
    }
    0
}


pub fn solve_slow(input: &[u64], pre: usize) -> u64 {
    'out: for (i, n) in input[pre..].iter().enumerate() {
        let base = &input[i..(i+pre)];
        for a in base {
            for b in base {
                if a+b == *n {
                    continue 'out;
                }
            }
        }
        return *n;
    }
    0
}

pub fn solve(input: &[u64], pre: usize) -> u64 {
    solve_fast(input, pre)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let input = String::from("35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
");
        let input = parse_input(&input);
        assert_eq!(127, solve(&input, 5));
    }

    #[test]
    fn solve_test2() {
        let input = String::from("1
2
3
4
90
100
");
        let input = parse_input(&input);
        assert_eq!(90, solve(&input, 3));
    }



}

