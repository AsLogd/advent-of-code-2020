
use std::collections::HashSet;
    
pub fn solve_functional(input: &str) -> u32 {
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


pub fn solve_imperative(input: &str) -> u32 {
    let groups = input.split("\n\n");
    let mut sum = 0;
    for g in groups {
        let smallest = g.split('\n').min_by_key(|person| person.len()).unwrap();
        'cl: for c in smallest.chars() {
            for person in g.split('\n') {
                if !person.chars().any(|x| x == c) {
                    continue 'cl;
                }
            }
            sum += 1;
        }
    }
    sum
}

pub fn solve(input: &str) -> u32 {
    solve_imperative(input)
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

