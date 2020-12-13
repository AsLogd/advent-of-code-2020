// This problem can be represented as a system of congruences:
//
// x ≡ 0 (mod n_0)
// ...
// x+p_i ≡ 0 (mod n_i) => x ≡ -p_i (mod n_i)
//
// Where:
// · x is the number that we want to find
// · p_i is the position of the i-th number (keeping the 'x's)
// · n_i is the number i
//
// This system defines a number x that: 
// · x is divisible by n_0 (the first line)
// · x+1 is divisible by n_1 (the second line)
// · ...
// · x+i is divisible by n_i (the i-th line)
//
// if a time x is divisible by the bus n, in this context
// this means that the bus n stops at time x.
// then, this system defines an x where:
// · the bus n_0 stops at time x, 
// · the bus n_1 stops at time x+1...


// Based on the rust code on rosettacode for the
// chinese reminder theorem
// fixed for negative numbers and adapted to the input
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b.rem_euclid(a), a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some(x.rem_euclid(n))
    } else {
        None
    }
}
 
fn chinese_remainder(input: &[(i64, i64)]) -> Option<i64> {
    let prod = input.iter().map(|(_, modulii)| modulii).product::<i64>();
 
    let mut sum = 0;
 
    for (residue, modulus) in input {
        let p = prod / modulus;
        sum += residue * mod_inv(p, *modulus)? * p
    }
 
    Some(sum.rem_euclid(prod))
}

pub fn parse_input(input: &str) -> Vec<(i64, i64)> {
    let mut split = input.trim().split('\n');
    split.next();
    split.next().unwrap()
        .split(',')
        .enumerate()
        .filter(|x| x.1 != "x")
        .map(|(i, x)| (-(i as i64), x.parse::<i64>().unwrap()))
        .collect::<Vec<(i64, i64)>>()
}


pub fn solve(input: &[(i64, i64)]) -> i64 {
    //println!("{:?}", &input);
    match chinese_remainder(&input) {
        Some(sol) => sol,
        None      => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn solve_test() {
        let input = String::from("939
7,13,x,x,59,x,31,19
");
        let input = parse_input(&input);
        assert_eq!(1068781, solve(&input));
    }

    #[test]
    fn solve_small() {
        let input = String::from("939
17,x,13,19
");
        let input = parse_input(&input);
        assert_eq!(3417, solve(&input));
    }

    #[test]
    fn solve_small2() {
        let input = String::from("939
67,7,59,61
");
        let input = parse_input(&input);
        assert_eq!(754018, solve(&input));
    }

    #[test]
    fn solve_small3() {
        let input = String::from("939
67,x,7,59,61 
");
        let input = parse_input(&input);
        assert_eq!(779210, solve(&input));
    }

    #[test]
    fn solve_small4() {
        let input = String::from("939
67,7,x,59,61
");
        let input = parse_input(&input);
        assert_eq!(1261476, solve(&input));
    }

    #[test]
    fn solve_small5() {
        let input = String::from("939
1789,37,47,1889 
");
        let input = parse_input(&input);
        assert_eq!(1202161486, solve(&input));
    }
}