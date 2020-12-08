
pub enum Instr{
    Nop,
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
                _     => Instr::Nop,
            }
        }).collect::<Program>()
}


pub fn solve(prog: &Program) -> i32 {
    let mut pc: usize = 0;
    let mut acu: i32 = 0;
    let mut executed = vec![false; prog.len()];
    loop {
        let ex = &mut executed[pc % prog.len()];
        if *ex { break; }
        *ex = true;
        match prog[pc] {
            Instr::Acc(n) => {acu += n; pc += 1},
            Instr::Jmp(n) => {pc = (pc as i32+n) as usize},
            _      => {pc += 1}
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
        assert_eq!(5, solve(&prog));
    }


}

