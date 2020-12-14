use std::collections::HashMap;

#[derive(Debug)]
struct SimChip {
    mask: u64,
    floating:Vec<u64>,
    mem: HashMap<u64, u64>
}

impl SimChip {
    fn execute(&mut self, instr: &Instr) {
        match instr {
            Instr::Mask(or, floating) => {
                self.mask = *or;
                self.floating = floating.clone();
            },
            Instr::Mem(dir, val) => self.insert(*dir, *val) 
        }
    }

    fn get_floating_mask(&self) -> u64 {
        //println!("gfm: {:?}", self.floating);
        let inv_mask: u64 = self.floating.iter().sum();
        u64::MAX ^ inv_mask
    }

    fn compute_dirs(&self, dir: u64) -> Vec<u64> {
        let floating_mask = self.get_floating_mask();
        let n_float_bits = self.floating.len();
        // Apply 1's mask and Set floating bits to 0
        let masked = (dir | self.mask) & floating_mask;
        let bigger_num = 2u64.pow(n_float_bits as u32);
        //println!("{:?}, m: {:b}, nfb: {:?}, bn: {:?}", &self, floating_mask, n_float_bits, bigger_num);
        let mut res = Vec::new();
        for i in 0..bigger_num {
            let bits = format!("{:0nfb$b}", i, nfb=n_float_bits);
            //println!("i in bits {}", bits);
            let mask_i: u64 =  bits.chars().rev()
                .map(|x| (x == '1') as u64)
                .zip(self.floating.iter())
                .map(|(b, v)| b*v)
                .sum();
            //println!("mask i {:b}", mask_i);
            // Apply current floating num to mask
            res.push(masked | mask_i);
            //println!("{:0nfb$b}", masked | mask_i, nfb=n_float_bits);
        }
        res
    }

    fn insert(&mut self, dir: u64, val: u64) {
        let ndirs = self.compute_dirs(dir);
        for dir in ndirs {
            self.mem.insert(
                dir,
                val 
            );
        }
    }

    fn get_mem_sum(&self) -> u64 {
        self.mem.iter()
            .map(|(_, val)| val)
            .sum()
    }
}

#[derive(Debug)]
pub enum Instr {
    //'or mask', floating mask
    Mask(u64, Vec<u64>),
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
                    rhs.chars().rev()
                        .enumerate()
                        .filter(|(_, c)| *c == 'X')
                        .map(|(i, _)| 2u64.pow(i as u32))
                        .collect::<Vec<u64>>()
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
        mask: 0,
        floating: Vec::new(),
        // Is there an easier way to init a hashmap?
        mem: vec![(0,0)].into_iter().collect::<HashMap<u64, u64>>()
    };
    for instr in input {
        //println!("Executing {:?}", instr);
        chip.execute(instr);
    }
    chip.get_mem_sum()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn solve_test() {
        let input = String::from("mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
");
        let input = parse_input(&input);
        assert_eq!(208, solve(&input));
    }
}