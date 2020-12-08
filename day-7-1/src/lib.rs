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

/* TRY 2: I totally misread the problem. solved contained colors

use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ColorEntry {
    color_set: HashSet<String>,
    rules: Option<Vec<(u32, String)>>
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
                    color_set: HashSet::new(),
                    rules: None
                });
            }
            let mut rules = Vec::new();
            for bag_rule in contain[1].trim().split(',') {
                //println!("{:?}", bag_rule);

                let parts = bag_rule.trim().split(' ').collect::<Vec<_>>();
                //println!("{:?}", parts);

                let n = parts[0].parse::<u32>().unwrap();
                let color = parts[1..3].join(" ");
                rules.push((n, color));
            }
            let entry = ColorEntry{
                color_set: HashSet::new(),
                rules: Some(rules)
            };
            (def_color, entry)
        }).collect::<HashMap<&str, ColorEntry>>()
}
// Obtains the set of colors found on children of the node 'color'
fn get_color_subset(mut rules: &mut HashMap<&str, ColorEntry>, color:&str) -> HashSet<String> {
    // hmmmmm...
    let hmcopy = rules.clone();
    //println!("{:#?} , {:?}", hmcopy, color);
    let entry = hmcopy.get(&color[..]).unwrap();
    let mut result = HashSet::new();
    match &entry.rules {
        None =>  {},
        Some(list) => list.iter().for_each(|(_, c)| {
            result.insert(c.to_string());
            result.extend(get_color_subset(&mut rules, c));
        })
    }
    //println!("Subset for {:?}: {:?}", color, result);
    result
}

pub fn solve(mut rules: &mut HashMap<&str, ColorEntry>) -> usize {
    get_color_subset(&mut rules, &MY_BAG).len()
}
*/



use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ColorEntry {
    contained_by: HashSet<String>,
    rules: Option<Vec<(u32, String)>>
}

pub fn parse_input(input: &str) -> HashMap<String, ColorEntry> {
    let mut result: HashMap<String, ColorEntry> = HashMap::new();
    for rule in input.trim().split('\n') {
        let contain = rule.split("bags contain").collect::<Vec<&str>>(); 
        let def_color = contain[0].trim();
        let first_rule = contain[1].trim().split(',').next().unwrap();
        //println!("{:?}", first_rule);
        if &first_rule[..2] == "no" {
            if !result.contains_key(def_color) {
                result.insert(def_color.to_string(), ColorEntry{
                    contained_by: HashSet::new(),
                    rules: None
                });
            }
            continue;
        }
        let mut rules = Vec::new();

        for bag_rule in contain[1].trim().split(',') {
            //println!("{:?}", bag_rule);

            let parts = bag_rule.trim().split(' ').collect::<Vec<_>>();
            //println!("{:?}", parts);

            let n = parts[0].parse::<u32>().unwrap();
            let color = parts[1..3].join(" ");
            rules.push((n, color.clone()));
            if !result.contains_key(&color[..]) {
                let mut hs: HashSet<String> = HashSet::new();
                hs.insert(def_color.to_string());
                result.insert(color, ColorEntry{
                    contained_by: hs,
                    rules: None
                });
            } else {
                let entry = result.get_mut(&color[..]).unwrap();
                entry.contained_by.insert(def_color.to_string());
            }    
        }
        if !result.contains_key(def_color) {
            result.insert(def_color.to_string(), ColorEntry{
                contained_by: HashSet::new(),
                rules: Some(rules)
            });
        } else {
            let entry = result.get_mut(def_color).unwrap();
            entry.rules = Some(rules);
        }
        
    }
    result
}

fn get_contained_set(entries: &HashMap<String, ColorEntry>, color: &str) -> HashSet<String> {
    let set_read = &entries.get(color).unwrap().contained_by;
    let mut set = entries.get(color).unwrap().contained_by.clone();
    for c in set_read.iter() {
        set.extend(get_contained_set(entries, &c[..]));
    }
    set
}

pub fn solve(entries: &HashMap<String, ColorEntry>) -> usize {
    get_contained_set(&entries, MY_BAG).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let input = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.");
        let d = parse_input(&input);
        assert_eq!(4, solve(&d));
    }

    #[test]
    fn test_contained() {
        let input = String::from("vibrant plum bags contain 5 shiny gold bags, 6 dotted black bags.
shiny gold bags contain no other bags.
red red bags contain 1 vibrant plum bag.
dotted black bags contain no other bags.");
        let d = parse_input(&input);
        println!("{:?}", d);
        assert_eq!(2, solve(&d));
    }

    /* TRY 2
    #[test]
    fn solve_small_test() {
        let input = String::from("shiny gold bags contain 1 dotted black bag.
dotted black bags contain no other bags.

");
        let mut d = parse_input(&input);
        assert_eq!(1, solve(&mut d));
    }

    #[test]
    fn solve_test2() {
        let input = String::from("shiny gold bags contain 1 dotted black bag, 1 tan tan bag.
dotted black bags contain no other bags.
tan tan bags contain 1 black pink bag, 1 dark green bag.
black pink bags contain 1 dark green bag.
dark green bags contain no other bags

");
        let mut d = parse_input(&input);
        assert_eq!(4, solve(&mut d));
    }
    */

}

