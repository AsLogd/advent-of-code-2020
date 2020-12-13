
pub fn parse_input(input: &str) -> (u32, Vec<u32>) {
    let mut split = input.trim().split('\n');
    let arrival = split.next().unwrap().parse::<u32>().unwrap();
    let bus_lines = split.next().unwrap()
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    (arrival, bus_lines)
}


pub fn solve(input: (u32, Vec<u32>)) -> u32 {
    let (arrival, bus_lines) = input;
    let fastest_line = bus_lines.iter().map(|x|
        (*x, x - (arrival % x)) 
    ).min_by_key(|x| x.1).unwrap();
    fastest_line.0 * fastest_line.1
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
        assert_eq!(295, solve(input));
    }
}


