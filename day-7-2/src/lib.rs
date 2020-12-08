const MY_BAG: &str = "shiny gold";
/* TRY 1
use std::collections::HashMap;


pub fn parse_input(input: &str) -> HashMap<&str, (i32, Option<Vec<(i32, &str)>>)> {
    input.split('\n')
        .map(|def| {
            let contain = def.split("bags contain").collect::<Vec<&str>>();
            let def_color = contain[0].trim();
            let contents = contain[1].split(',').map(|x| {
                let parts = x.split(' ').collect::<Vec<_>>();
                if parts[0] == "no" {
                    return None
                }
                let n = parts[0].parse::<i32>().unwrap();
                let color = &parts[1..3].join("")[..];
                Some((n, color))
            }).collect();
            (def_color, (-1, contents))
        }).collect::<HashMap<_, _>>()
}

fn containing_colors(defs: &mut HashMap<&str, (i32, Option<Vec<(i32, &str)>>)>, color: &str) -> i32{
    // ..HashMap<&str,.. and ...color: &str... declared with different timelines
    let (n, content) = defs.get_mut(&color).unwrap();
    // but data from color flows into defs here
    if *n > -1 {
        return *n;
    }
    *n = match content {
        Some(content_list) => content_list.iter().map(|(quant, color)| 
            quant*containing_colors(&mut defs, &color)
        ).sum(),
        None => 0
    };
    *n

}

pub fn solve<'a>(defs: &mut HashMap<&str, (i32, Option<Vec<(i32, &str)>>)>) -> i32 {
    containing_colors(&mut defs, &MY_BAG)
}
*/
// I just went by to TRY 2 from day-7-1

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ColorEntry {
    rules: Option<Vec<(i32, String)>>
}

pub fn parse_input(input: &str) -> HashMap<&str, ColorEntry> {
    input.trim().split('\n')
        .map(|def| {
            let contain = def.split("bags contain").collect::<Vec<&str>>(); 
            let def_color = contain[0].trim();
            let first_rule = contain[1].trim().split(',').next().unwrap();
            //println!("{:?}", first_rule);
            if &first_rule[..2] == "no" {
                return (def_color, ColorEntry {
                    rules: None
                });
            }
            let mut rules = Vec::new();
            for bag_rule in contain[1].trim().split(',') {
                //println!("{:?}", bag_rule);

                let parts = bag_rule.trim().split(' ').collect::<Vec<_>>();
                //println!("{:?}", parts);

                let n = parts[0].parse::<i32>().unwrap();
                let color = parts[1..3].join(" ");
                rules.push((n, color));
            }
            let entry = ColorEntry{
                rules: Some(rules)
            };
            (def_color, entry)
        }).collect::<HashMap<&str, ColorEntry>>()
}

fn get_containing_bags(rules: &HashMap<&str, ColorEntry>, color: &str) -> i32 {
    let res = match &rules.get(color).unwrap().rules {
        None => 0,
        Some(list) => list.iter().map(|(mult, c)| 
            (get_containing_bags(rules, c)+1)*mult
        ).sum::<i32>()
    };
    println!("Result for color {:?}: {:?}", color, res);
    res
}

pub fn solve(rules: &HashMap<&str, ColorEntry>) -> i32 {
    get_containing_bags(rules, &MY_BAG)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let input = String::from("shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
");
        let d = parse_input(&input);
        assert_eq!(126, solve(&d));
    }

        #[test]
    fn solve_test_initial() {
        let input = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
");
        let d = parse_input(&input);
        assert_eq!(32, solve(&d));
    }

    #[test]
    fn solve_small_test() {
        let input = String::from("shiny gold bags contain 1 dotted black bag.
dotted black bags contain no other bags.

");
        let d = parse_input(&input);
        assert_eq!(1, solve(&d));
    }

    #[test]
    fn solve_test2() {
        let input = String::from("shiny gold bags contain 1 dotted black bag, 1 tan tan bag.
dotted black bags contain no other bags.
tan tan bags contain 1 black pink bag, 1 dark green bag.
black pink bags contain 1 dark green bag.
dark green bags contain no other bags

");
        let d = parse_input(&input);
        assert_eq!(5, solve(&d));
    }

}

