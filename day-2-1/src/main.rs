use std::io;

#[derive(PartialEq, Debug)]
struct PasswordEntry{
    min: u32,
    max: u32,
    character: char,
    password: String
}

impl PasswordEntry{
    fn is_valid(&self) -> bool {
        let count = self.password
            .chars()
            .fold(0, |sum, c| {
                if c == self.character {
                    return sum+1;
                }
                sum
            });
        count >= self.min && count <= self.max
    }
}

fn parse_entries(lines: &Vec<String>) -> Vec<PasswordEntry> {
    let mut entries = Vec::new();
    for line in lines {
        let spaced_parts: Vec<&str> = line.split_whitespace().collect();
        let interval_parts: Vec<&str> = spaced_parts[0].split("-").collect();    
        let entry = PasswordEntry {
            min: interval_parts[0].parse::<u32>().unwrap(),
            max: interval_parts[1].parse::<u32>().unwrap(),
            character: spaced_parts[1].chars().nth(0).unwrap(),
            password: spaced_parts[2].to_string()
        };
        entries.push(entry);
    }
    entries
}

fn solve(input: &Vec<PasswordEntry>) -> u32 {
    input.iter().fold(0, |sum, entry| if entry.is_valid() {sum+1} else {sum})
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
                input.push(buffer.clone());
            }
            Err(error) => println!("error: {}", error),
        }
    }
    println!("{}", solve(&parse_entries(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_input() {
        let input = vec![
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        ];
        let entries = parse_entries(&input);
        assert_eq!(entries, [
            PasswordEntry {
                min: 1,
                max: 3,
                character: 'a',
                password: String::from("abcde")
            },
            PasswordEntry {
                min: 1,
                max: 3,
                character: 'b',
                password: String::from("cdefg")
            },
            PasswordEntry {
                min: 2,
                max: 9,
                character: 'c',
                password: String::from("ccccccccc")
            },
        ])
        //assert_eq!(solve(entries), 2);
    }
    #[test]
    fn solve_input() {
        let input = vec![
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        ];
        let entries = parse_entries(&input);
        assert_eq!(solve(&entries), 2);
    }
}