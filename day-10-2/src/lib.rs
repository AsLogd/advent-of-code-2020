use std::convert::TryInto;

pub fn parse_input(input: &str) -> Vec<i32> {
    input.trim()
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
// Combinations with at least 3 consecutive
// 0s are not allowed because they would
// increment the difference to more than 3
fn prohibited_nums(bits: isize) -> isize {
    // 3 is the maximum difference
    let free_bits: u32 = (bits - 3).try_into().unwrap();
    // There are 3 locked bits which are 0.
    // We get all the combinations of the 
    // restant digits
    //     (2^free_bits)
    // Then, we shift the locked bits 1 bit
    // changing one locked bit for one free,
    // allowing for new combinations
    // (Only half new combinations because
    // we already counted those with value 0)
    //      + ((2^(free_bits -1))
    // We can shift as many times as free
    // bits we have
    //      * free_bits)
    isize::pow(2, free_bits)
    + isize::pow(2, free_bits-1)*free_bits as isize
}


fn to_section_factor(section: &[i32]) -> u64 {
    match section.len() as isize {
        0 | 1 => 1,
        2     => 2,
        3     => 4,
        4     => 7,
        n     => {
            // Last 1 cannot change
            let m = n-1;

            // All possible combinations
            // in m bits minus where
            // we would take out 
            // 3 adapters in a row
            // (which would make a 
            // difference bigger than 3)
            (2^m - prohibited_nums(m)).try_into().unwrap()
        }
    }
}

pub fn solve(input: &[i32]) -> u64 {
    let mut sorted = Vec::from(input);
    sorted.sort();
    sorted.push(sorted.last().unwrap()+3);
    let mut shifted = sorted.clone();
    shifted.insert(0,0);
    let differences = sorted.iter()
        .zip(shifted.iter())
        .map(|(curr, pre)| curr - pre)
        .collect::<Vec<i32>>();
    // Where the difference is 3, the path is
    // critical and cannot change. We can only
    // change sections with multiple 1's
    differences
        .split(|x| *x == 3)
        .map(to_section_factor)
        .product()

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_small_test() {
        let input = String::from("16
10
15
5
1
11
7
19
6
12
4
");
        let input = parse_input(&input);
        assert_eq!(8, solve(&input));
    }

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
        assert_eq!(19208, solve(&input));
    }
}