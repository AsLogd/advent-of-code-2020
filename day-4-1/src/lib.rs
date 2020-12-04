use std::collections::HashSet;
//use regex::Regex;

const FIELDS: [&str; 7]=  [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
];

pub fn solve(input: &str) -> u32 {
    //Regex::new("^\n$").unwrap().split(input) why this didnt work?
    input.split("\n\n")
        //Map entries into field sets
        .map(|entry| 
            entry.split(&[' ', '\n'][..]).map(|field| 
                field.split(':').next().unwrap()
            ).collect::< HashSet<&str> >()
        )
        //Map entries to bool (missing field exists?)
        .map(|set|
            FIELDS.iter().cloned()
                .collect::< HashSet<&str> >()
                .difference(&set).next() == None
        )
        //Count valid entries
        .fold(0, |sum, entry| sum + entry as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split() {
        let input = String::from("asdf
asdf

asdf
asdf

a");
        let f = input.split("\n\n").collect::<Vec<&str>>().len();
        assert_eq!(f, 3);
    }

    #[test]
    fn test_solve() {
        let input = String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in");
        assert_eq!(solve(&input), 2);
        
    }

}