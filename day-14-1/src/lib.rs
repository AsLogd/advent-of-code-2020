use std::collections::HashMap;

struct SimChip {
    mask_or: u64,
    mask_and: u64,
    mem: HashMap<u64, u64>
}

impl SimChip {
    fn execute(&mut self, instr: &Instr) {
        match instr {
            Instr::Mask(or, and) => {
                self.mask_or = *or; 
                self.mask_and = *and;
            },
            Instr::Mem(dir, val) => self.insert(*dir, *val) 
        }
    }

    fn insert(&mut self, dir: u64, val: u64) {
        self.mem.insert(
            dir,
            (val & self.mask_and) | self.mask_or
        );
    }

    fn get_mem_sum(&self) -> u64 {
        self.mem.iter()
            .map(|(_, val)| val)
            .sum()
    }
}

pub enum Instr {
    //'or', 'and'
    Mask(u64, u64),
    // dir, val
    Mem(u64, u64)
}

pub fn parse_input(input: &str) -> Vec<Instr> {
    input.trim()
        .split('\n')
        .map(|line| {
            let mut line_iter = line.split('=');
            let lhs = line_iter.next().unwrap().trim();
            let rhs = line_iter.next().unwrap().trim();
            if lhs == "mask" {
                Instr::Mask(
                    // Or will copy 1's
                    u64::from_str_radix(&rhs.replace('X', "0"),2).unwrap(),
                    // And will copy 0's
                    u64::from_str_radix(&rhs.replace('X', "1"),2).unwrap()
                )
            } else {
                let dir = lhs.replace('[', " ")
                    .replace(']', " ")
                    .split(' ')
                    .nth(1).unwrap()
                    .trim()
                    .parse::<u64>().unwrap();
                Instr::Mem(
                    dir,
                    rhs.parse::<u64>().unwrap()
                )
            }
        }).collect::<Vec<Instr>>()
}


pub fn solve(input: &[Instr]) -> u64 {
    let mut chip = SimChip{
        mask_or: 0,
        mask_and: u64::MAX,
        // Is there an easier way to init a hashmap?
        mem: vec![(0,0)].into_iter().collect::<HashMap<u64, u64>>()
    };
    for instr in input {
        chip.execute(instr);
    }
    chip.get_mem_sum()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn solve_test() {
        let input = String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
");
        let input = parse_input(&input);
        assert_eq!(165, solve(&input));
    }
}