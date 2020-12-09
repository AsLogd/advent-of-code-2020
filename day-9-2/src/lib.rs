use core::cmp::Reverse;
use std::iter::FromIterator;
use std::collections::BinaryHeap;

pub fn parse_input(input: &str) -> Vec<u64> {
    input.trim()
        .split('\n')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}


pub fn solve(input: &[u64], target: u64) -> u64 {
    let list = input.split(|x| *x == target).next().unwrap();
    let mut r = (0,2);
    let mut sum: u64 = list[r.0..r.1].iter().sum();
    let mut min: BinaryHeap<Reverse<u64>> = BinaryHeap::from_iter(list[r.0..r.1].iter().cloned().map(Reverse));
    let mut max: BinaryHeap<u64> = BinaryHeap::from_iter(list[r.0..r.1].iter().cloned());
    while sum != target {
        //println!("--Sum {:?} with set {:?}, is not {:?}", sum, &list[r.0..r.1], target);
        //println!("-min is {:?}, max is {:?}", min.peek().unwrap(), max.peek().unwrap());
        if sum < target {
            //println!("sum is too small. adding number {:?}", list[r.1]);

            sum += list[r.1];
            min.push(Reverse(list[r.1]));
            max.push(  list[r.1] );
            //println!("new sum is {:?}, min is {:?}, max is {:?}", sum, min.peek().unwrap(), max.peek().unwrap());
            r.1 += 1;
        } else {
            //println!("sum is too big. removing number {:?}", list[r.0]);

            sum -= list[r.0];       
            min = min.iter()
                .filter(|x| **x != Reverse(list[r.0]))
                .cloned().collect::<BinaryHeap<Reverse<u64>>>();
            max = max.iter()
                .filter(|x| **x != list[r.0])
                .cloned().collect::<BinaryHeap<u64>>();
            //println!("new sum is {:?}, min is {:?}, max is {:?}", sum, min.peek().unwrap(), max.peek().unwrap());

            r.0 += 1;
        }
    }
    //println!("Finished! sum {:?} is equal to {:?} with set {:?}, min is {:?} and max {:?}", sum, target, &list[r.0..r.1], min.peek().unwrap(), max.peek().unwrap());

    let min_n = if let Some(Reverse(n)) = min.pop() {
        n
    } else {0};
    let max_n =  max.pop().unwrap() as u64;
    min_n + max_n
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
        assert_eq!(62, solve(&input, 127));
    }

    #[test]
    fn solve_test2() {
        let input = String::from("1
2
3
4
10
100
");
        let input = parse_input(&input);
        assert_eq!(5, solve(&input, 10));
    }



}

