
use std::collections::HashSet;
    
pub fn solve(input: &str) -> u32 {
    input.split("\n\n")
        .map(|group| {
            let mut people = group.split('\n');
            people.next()
                .map(|first| 
                    people.fold(first.chars().collect::<HashSet<char>>(), 
                        |inter, curr| inter.intersection(&curr.chars().collect::<HashSet<char>>()).cloned().collect()
                    )
                ).unwrap()
        }

        ).fold(0, |sum, curr| sum + curr.len() as u32)
    
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
        assert_eq!(6, solve(&input));
    }

    #[test]
    fn solve_tes2() {
        let input = String::from("efhlq
flaq
qflwt
fqmpl
ltqf

yznp
np

i
i
i
i

a
");
        assert_eq!(7, solve(&input.trim()));
    }
}

