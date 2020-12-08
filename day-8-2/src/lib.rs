#[derive(Clone)]
pub enum Instr{
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

pub type Program = Vec<Instr>;


pub fn parse_input(input: &str) -> Program {
    input.trim()
        .split('\n')
        .map(|ln|{
            let line = ln.split(' ').collect::<Vec<&str>>();
            let n = line[1].parse::<i32>().unwrap();
            match line[0] {
                "acc" => Instr::Acc(n),
                "jmp" => Instr::Jmp(n),
                _     => Instr::Nop(n),
            }
        }).collect::<Program>()
}

fn run_program(prog: &Program, fix: u32) -> Result<i32, &str> {
    let mut pc: usize = 0;
    let mut acu: i32 = 0;
    let mut fix_ptr: u32 = 0;
    let mut executed = vec![false; prog.len()];
    loop {
        if pc == prog.len() {
            return Ok(acu);
        }
        if pc > prog.len() {
            return Err("Overflow");
        }
        let mut instr = prog[pc].clone();
        let ex = &mut executed[pc];
        if *ex { 
            return Err("Infinite loop");
        }
        *ex = true;
        if fix_ptr == fix {
            match instr {
                Instr::Nop(n) => {instr = Instr::Jmp(n)},
                Instr::Jmp(n) => {instr = Instr::Nop(n)},
                _ => {}
            }
        }
        match instr {
            Instr::Acc(n) => {acu += n; pc += 1},
            Instr::Jmp(n) => {
                pc = (pc as i32+n) as usize; 
                fix_ptr += 1;
            },
            _             => {pc += 1; fix_ptr += 1;}
        }
    }
}

pub fn solve(prog: &Program) -> i32 {
    let acu;
    let mut fix_n = 0;
    loop {
        match run_program(prog, fix_n) {
            Ok(r)  => {acu = r; break;},
            Err(_) => {fix_n += 1;}
        }
    }
    acu
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let input = String::from("nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
");
        let prog = parse_input(&input);
        assert_eq!(8, solve(&prog));
    }


}

