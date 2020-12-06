use std::iter::FromIterator;
use std::collections::HashSet;

fn to_set(data: impl Iterator<Item = char>) -> HashSet<char> {
    HashSet::from_iter(data)
}
    
pub fn solve(input: &str) -> u32 {
    input.split("\n\n")
        .map(|group|
            to_set(
                group.split('\n')
                    .collect::<String>()
                    .chars()
            ).len()
        ).fold(0, |sum, curr| sum + curr as u32)
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let input = String::from("abc

a
b
c

ab
ac

a
a
a
a

b");
        assert_eq!(11, solve(&input));
    }
}