use std::io;

fn solve(input: &Vec<u32>) -> u32 {
    for a in input {
        for b in input {
            for c in input {
                let sum = a+b+c;
                if sum == 2020 {
                    return a*b*c;
                }
            }
        }
    }
    0
}



fn main() {
    let mut input = Vec::new();
    let mut buffer = String::new();
    loop {
        buffer.clear();
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                //println!("{}", buffer);
                input.push(buffer.trim().parse::<u32>().unwrap());
            }
            Err(error) => println!("error: {}", error),
        }
    }
    println!("{}", solve(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = vec![
            1721,
            979,
            366,
            299,
            675,
            1456,
        ];
        assert_eq!(solve(&input), 241861950);
    }
}