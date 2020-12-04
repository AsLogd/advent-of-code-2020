use regex::Regex;

const FIELDS: [&str; 7]=  [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
];

fn byr_is_valid(field: &str) -> bool {
    match field.parse::<u32>() {
        Ok(val) => val >= 1920 && val <= 2002,
        Err(_) => false
    }
}
fn iyr_is_valid(field: &str) -> bool {
    match field.parse::<u32>() {
        Ok(val) => val >= 2010 && val <= 2020,
        Err(_) => false
    }
}
fn eyr_is_valid(field: &str) -> bool {
    match field.parse::<u32>() {
        Ok(val) => val >= 2020 && val <= 2030,
        Err(_) => false
    }
}
fn hgt_is_valid(field: &str) -> bool {
    let l = field.len();
    let n = match field[..l-2].parse::<u32>() {
        Ok(val) => val,
        Err(_) => return false
    };
    match &field[l-2..] {
        "cm" => n >= 150 && n <= 193,
        "in" => n >= 59 && n <= 76,
        _ => false
    }
}
fn hcl_is_valid(field: &str) -> bool {
    Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(field)
}
fn ecl_is_valid(field: &str) -> bool {
    match field {
          "amb" | "blu" | "brn"
        | "gry" | "grn" | "hzl"| "oth" => true,
        _ => false
    }
}
fn pid_is_valid(field: &str) -> bool {
    Regex::new(r"^[0-9]{9}$").unwrap().is_match(field)
}

fn field_is_valid(field: &Vec<&str>) -> bool {
    match field[0] {
        "byr" => byr_is_valid(field[1]),
        "iyr" => iyr_is_valid(field[1]),
        "eyr" => eyr_is_valid(field[1]),
        "hgt" => hgt_is_valid(field[1]),
        "hcl" => hcl_is_valid(field[1]),
        "ecl" => ecl_is_valid(field[1]),
        "pid" => pid_is_valid(field[1]),
        _ => true
    }
}

fn field_is_valid_imp(name: &str, val: &str) -> bool {
    match name {
        "byr" => byr_is_valid(val),
        "iyr" => iyr_is_valid(val),
        "eyr" => eyr_is_valid(val),
        "hgt" => hgt_is_valid(val),
        "hcl" => hcl_is_valid(val),
        "ecl" => ecl_is_valid(val),
        "pid" => pid_is_valid(val),
        _ => true
    }
}

pub fn solve_functional(input: &str) -> u32 {
    input.split("\n\n")
        //Map entries into key, value iterator
        .map(|entry| 
            entry.split(&[' ', '\n'][..]).map(|field| 
                field.split(':').collect::<Vec<&str>>()
            ).collect::<Vec<Vec<&str>>>()
        )
        .map(|entry|
            FIELDS.iter().all(|field|
                entry.iter().any(|ef|
                    ef[0] == *field
                )
                && entry.iter().all(|ef|
                    field_is_valid(ef)
                )
            )
        )
        
        //Count valid entries
        .fold(0, |sum, entry| sum + entry as u32)
}

fn field_to_pos(field: &str) -> usize {
    match field {
        "byr" => 0,
        "iyr" => 1,
        "eyr" => 2,
        "hgt" => 3,
        "hcl" => 4,
        "ecl" => 5,
        "pid" => 6,
        _ => 7
    }
}

pub fn solve_imperative(input: &str) -> u32 {
    let entries = input.split("\n\n");
    let mut valid_entries = 0;
    'el: for entry in entries {
        let pairs = entry.split(&[' ', '\n'][..]);
        let mut mandatory_field_present = [
            false, false, false,
            false, false, false, false
        ];
        for p in pairs {
            let mut field_value = p.split(':');
            let field = field_value.next().unwrap_or("err");
            let val =  field_value.next().unwrap_or("err");
            if !field_is_valid_imp(field, val) {
                continue 'el;
            }
            let i = field_to_pos(field);
            if i < 7 {
                mandatory_field_present[i] = true;
            }
        }
        for f in mandatory_field_present.iter() {
            if !f {
                continue 'el;
            }
        }
        valid_entries += 1;
    }
    valid_entries
}

pub fn solve(input: &str) -> u32 {
    solve_imperative(input)
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

    #[test]
    fn test_invalid() {
        let input = String::from("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007");
        assert_eq!(solve(&input), 0);
        
    }

    #[test]
    fn test_valid() {
        let input = String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719");
        assert_eq!(solve(&input), 4);
        
    }


    #[test]
    fn test_byr() {
        assert_eq!(byr_is_valid("1937"), true);
        assert_eq!(byr_is_valid("1920"), true);
        assert_eq!(byr_is_valid("2002"), true);
        assert_eq!(byr_is_valid("1910"), false);
        assert_eq!(byr_is_valid("2100"), false);
        assert_eq!(byr_is_valid("1937a"), false);
        assert_eq!(byr_is_valid("19"), false);
        assert_eq!(byr_is_valid("19370"), false);
    }

    #[test]
    fn test_iyr() {
        assert_eq!(iyr_is_valid("2015"), true);
        assert_eq!(iyr_is_valid("2010"), true);
        assert_eq!(iyr_is_valid("2020"), true);
        assert_eq!(iyr_is_valid("2000"), false);
        assert_eq!(iyr_is_valid("2025"), false);
        assert_eq!(iyr_is_valid("19"), false);
        assert_eq!(iyr_is_valid("2015a"), false);
        assert_eq!(iyr_is_valid("19370"), false);
    }

    #[test]
    fn test_eyr() {
        assert_eq!(eyr_is_valid("2025"), true);
        assert_eq!(eyr_is_valid("2020"), true);
        assert_eq!(eyr_is_valid("2030"), true);
        assert_eq!(eyr_is_valid("2000"), false);
        assert_eq!(eyr_is_valid("2035"), false);
        assert_eq!(eyr_is_valid("19"), false);
        assert_eq!(eyr_is_valid("2025a"), false);
        assert_eq!(eyr_is_valid("19370"), false);
    }

    #[test]
    fn test_hgt() {
        assert_eq!(hgt_is_valid("150cm"), true);
        assert_eq!(hgt_is_valid("193cm"), true);
        assert_eq!(hgt_is_valid("59in"), true);
        assert_eq!(hgt_is_valid("76in"), true);
        assert_eq!(hgt_is_valid("200cm"), false, "above cm");
        assert_eq!(hgt_is_valid("78ina"), false, "wrong format (ina)");
        assert_eq!(hgt_is_valid("a195cm"), false, "wrong format (a195cm)");
    }

    #[test]
    fn test_hcl() {
        assert_eq!(hcl_is_valid("#a1b2c3"), true);
        assert_eq!(hcl_is_valid("#afe123"), true);
        assert_eq!(hcl_is_valid("abc123"), false);
        assert_eq!(hcl_is_valid("a#bc123"), false);
        assert_eq!(hcl_is_valid("#abc12"), false);
        assert_eq!(hcl_is_valid("#stf900"), false);
    }


    #[test]
    fn test_ecl() {
        assert_eq!(ecl_is_valid("amb"), true);
        assert_eq!(ecl_is_valid("blu"), true);
        assert_eq!(ecl_is_valid("brn"), true);
        assert_eq!(ecl_is_valid("zaa"), false);
        assert_eq!(ecl_is_valid("1brn"), false);
        assert_eq!(ecl_is_valid("blue"), false);
    }

    #[test]
    fn test_pid() {
        assert_eq!(pid_is_valid("000000001"), true);
        assert_eq!(pid_is_valid("100000001"), true);
        assert_eq!(pid_is_valid("0100000001"), false);
    }

}