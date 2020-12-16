use std::collections::HashMap;

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

pub fn solve_fast(input: &InputData) -> u32 {
    let ors = input.get_combined_intervals();
    input.nearby.iter()
        .map(|x| 
            x.iter().filter(|n| !is_valid(&ors,**n)).sum::<u32>()
        )
        .sum()
}

pub fn solve_slow(input: &InputData) -> u32 {
    let mut sum = 0;
    for ticket in input.nearby.iter() {
        'n: for num in ticket.iter() {
            for (_, intervals) in input.rules.iter() {
                for interval in intervals.iter() {
                    if interval.contains(num) {
                        continue 'n;
                    }
                }
            }
            sum += num;
        }
    }
    sum
}

pub fn solve(input: &InputData) -> u32 {
    solve_fast(&input)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn solve_test() {
        let input = String::from("class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12

");
        let input = parse_input(&input);
        assert_eq!(71, solve(&input));
    }
}