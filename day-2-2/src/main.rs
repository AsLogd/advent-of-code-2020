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
        let first_match = self.password.chars().nth(self.min as usize) == Some(self.character);
        let second_match = self.password.chars().nth(self.max as usize) == Some(self.character);
        first_match ^ second_match
    }
}

fn parse_entries(lines: &Vec<String>) -> Vec<PasswordEntry> {
    let mut entries = Vec::new();
    for line in lines {
        let spaced_parts: Vec<&str> = line.split_whitespace().collect();
        let interval_parts: Vec<&str> = spaced_parts[0].split("-").collect();    
        let entry = PasswordEntry {
            min: interval_parts[0].parse::<u32>().unwrap()-1,
            max: interval_parts[1].parse::<u32>().unwrap()-1,
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
                min: 0,
                max: 2,
                character: 'a',
                password: String::from("abcde")
            },
            PasswordEntry {
                min: 0,
                max: 2,
                character: 'b',
                password: String::from("cdefg")
            },
            PasswordEntry {
                min: 1,
                max: 8,
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
        assert_eq!(solve(&entries), 1);
    }
    #[test]
    fn two_matches() {
        let input = vec![
            String::from("1-3 a: abade")
        ];
        let entries = parse_entries(&input);
        assert_eq!(solve(&entries), 0);
    }
    #[test]
    fn match_last() {
        let input = vec![
            String::from("1-5 a: ebada")
        ];
        let entries = parse_entries(&input);
        assert_eq!(solve(&entries), 1);
    }
}