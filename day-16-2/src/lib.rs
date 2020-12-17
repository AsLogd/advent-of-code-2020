use std::collections::HashSet;
use std::collections::HashMap;

// TODO: don't suck

type OrIntervals = Vec<Interval>;

type Rules = HashMap<String, OrIntervals>;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    min: u32,
    max: u32
}

impl Interval {
    fn intersects(&self, b: &Interval) -> bool {
        !(self.max < b.min || self.min > b.max)
    }
    // pre:
    // self is smaller than b, 
    // self intersects/contains b
    fn combine(&mut self, b: &Interval) {
        self.max = b.max;
    }

    fn contains(&self, a: &u32) -> bool {
        self.min <= *a && *a <= self.max
    }
}

#[derive(Debug)]
pub struct InputData {
    rules: Rules,
    my: Vec<u32>,
    nearby: Vec<Vec<u32>>
}

impl InputData {
    fn get_combined_intervals(&self) -> OrIntervals {
        let mut all: Vec<Interval> = Vec::new();
        for (_, intervals) in self.rules.iter() {
            all.extend(intervals);
        }
        all.sort_by_key(|x| x.min);
        let mut processed: OrIntervals= Vec::new();
        processed.push(all[0]);
        let tail = &all[1..];
        for interval in tail.iter() {
            let last = processed.last_mut().unwrap();
            if interval.intersects(&last) {
                last.combine(interval);
            } else {
                processed.push(*interval);
            }
        }
        processed
    }
}

fn parse_rule(text: &str) -> (String, OrIntervals) {
    let mut parts_iter = text.split(':');
    let key = String::from(parts_iter.next().unwrap());
    let intervals = parts_iter.next().unwrap()
        .trim()
        .split("or")
        .map(|x| {
            let mut inter_iter = x.trim()
                .split('-')
                .map(|x| {
                    x.parse::<u32>().unwrap()
                });
            Interval{
                min: inter_iter.next().unwrap(),
                max: inter_iter.next().unwrap(),
            }
        }).collect::<Vec<Interval>>();
    (key, intervals)
}

pub fn parse_input(input: &str) -> InputData {
    let mut parts_iter = input.trim().split("\n\n");
    let rules = parts_iter.next().unwrap()
        .split('\n')
        .map(parse_rule)
        .collect::<Rules>();
    let my = parts_iter.next().unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let nearby = parts_iter.next().unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split('\n')
        .map(|x| 
            x.split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
        ).collect::<Vec<Vec<u32>>>();
    InputData{
        rules,
        my,
        nearby,
    }
}

fn is_valid(intervals: &OrIntervals, num: u32) -> bool {
    for interval in intervals.iter().rev() {
        if interval.contains(&num) {
            return true;
        }
    }
    false
}

fn generate_check_list(keys: &Vec<&String>) -> Vec<HashSet<String>> {
    let mut cl = Vec::new();
    for _ in keys.iter() {
        let mut t = HashSet::new();
        for k in keys.iter() {
            t.insert(String::from(*k));
        }
        cl.push(t);
    }
    cl
}

// It's late and I just wanted rust to compile
fn clean_check_list(cl: &Vec<HashSet<String>>) -> Vec<String> {
    let mut sorted = Vec::new();
    for (i, a) in cl.iter().enumerate() {
        sorted.push((i, a.clone()));
    }
    sorted.sort_by_key(|(_, m)| m.len());
    let mut res: Vec<(usize, String)>= Vec::new();
    for i in 0..sorted.len() {
        // Unsolvable othenwise
        assert_eq!(sorted[i].1.len(), 1);
        let locked_field = String::from(sorted[i].1.iter().next().unwrap().clone());
        res.push((sorted[i].0, locked_field.clone()));
        for j in i+1..sorted.len() {
            let a = &mut sorted[j];
            a.1.retain(|x| *x != locked_field)
        }
    }
    res.sort_by_key(|(oi, _)| oi.clone());
    res.iter().map(|(_, x)| x.clone()).collect::<Vec<String>>()
}

pub fn solve(input: &InputData) -> u64 {
    let ors = input.get_combined_intervals();
    let names = input.rules.keys().collect::<Vec<&String>>();
    let mut check_list = generate_check_list(&names);
    let valid_nearbys = input.nearby.iter()
        .filter(|x| 
            x.iter().all(|n| is_valid(&ors,*n))
        );
    for ticket in valid_nearbys {
        for (i_field, field) in ticket.iter().enumerate() {
            check_list[i_field].retain(|rule_name| 
                is_valid(input.rules.get(rule_name).unwrap(), *field)
            );
        }
    }
    let field_names = clean_check_list(&mut check_list);
    println!("{:?}", field_names);
    field_names.iter()
        .enumerate()
        .filter(|(_, name)| name.contains("departure"))
        .map(|(i, _)| input.my[i] as u64)
        .product()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn solve_test() {
        let input = String::from("class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
");
        let input = parse_input(&input);
        assert_eq!(71, solve(&input));
    }
}