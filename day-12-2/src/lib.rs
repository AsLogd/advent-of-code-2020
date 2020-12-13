
use core::ops::AddAssign;
use core::ops::Mul;




#[derive(Debug)]
pub enum Instr {
    North(u32),
    South(u32),
    East(u32),
    West(u32),

    Left(u32),
    Right(u32),
    Forward(u32),
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    // cos(90) => 0, sin(90) => 1
    fn rotate_rect(&mut self, degrees: i32) {
        let pos_deg = degrees.abs();
        let aux = *self;
        match pos_deg {
            90  => {self.x = -aux.y; self.y =  aux.x},
            180 => {self.x = -aux.x; self.y = -aux.y},
            270 => {self.x =  aux.y; self.y = -aux.x},
            _   => {}
        };
        if degrees < 0 && degrees.abs() != 180{
            *self = *self*-1
        }
    }
    fn manhattan_distance(&self) -> u32 {
        (self.x.abs()+self.y.abs()) as u32
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, a: i32) -> Point {
        Point{
            x: self.x*a,
            y: self.y*a
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, b: Self) {
        *self = Self{
            x: self.x + b.x,
            y: self.y + b.y
        }
    }
}

#[derive(Debug)]
struct Ship {
    pos: Point,
    waypoint: Point
}

impl Ship {
    fn move_to_waypoint(&mut self, times: u32) {
        self.pos += self.waypoint*times as i32
    }
    fn act(&mut self, instr: &Instr) {
        match instr {
            Instr::North(val)   => self.waypoint.y += *val as i32,
            Instr::South(val)   => self.waypoint.y -= *val as i32,
            Instr::East(val)    => self.waypoint.x += *val as i32,
            Instr::West(val)    => self.waypoint.x -= *val as i32,
            Instr::Left(val)    => self.waypoint.rotate_rect(  *val as i32),
            Instr::Right(val)   => self.waypoint.rotate_rect(-(*val as i32)),
            Instr::Forward(val) => self.move_to_waypoint(*val)
        }
    }
    fn manhattan_distance(&self) -> u32 {
        self.pos.manhattan_distance()
    }
}

pub fn parse_input(input: &str) -> Vec<Instr> {
    input.trim()
        .split('\n')
        .map(|x| {
            let mut iter = x.chars();
            let dir = iter.next().unwrap();
            let val = iter.collect::<String>()
                .parse::<u32>()
                .unwrap();
            match dir {
                'N' => Instr::North(val),
                'S' => Instr::South(val),
                'E' => Instr::East(val),
                'W' => Instr::West(val),
                'L' => Instr::Left(val),
                'R' => Instr::Right(val),
                _   => Instr::Forward(val),
            }
        }).collect::<Vec<Instr>>()
}


pub fn solve(input: &[Instr]) -> u32 {
    let mut ship = Ship{
        pos: Point{x:0, y:0},
        waypoint: Point{x:10, y:1}
    };
    
    println!("Status {:#?}", ship );
    for instr in input {
        println!("Acting {:?}", instr);
        ship.act(instr);
        println!("Status {:?}", ship );
    }
    ship.manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn solve_test() {
        let input = String::from("F10
N3
F7
R90
F11
");
        let input = parse_input(&input);
        assert_eq!(286, solve(&input));
    }


    #[test]
    fn small_test() {
        let input = String::from("N2
F1
");
        let input = parse_input(&input);
        assert_eq!(13, solve(&input));
    }

    #[test]
    fn small_test2() {
        let input = String::from("S2
F1
L90
F1
");
        let input = parse_input(&input);
        assert_eq!(20, solve(&input));
    }

    #[test]
    fn go_and_back_again() {
        let input = String::from("F100
L180
F100
");
        let input = parse_input(&input);
        assert_eq!(0, solve(&input));
    }

    #[test]
    fn go_and_back_again2() {
        let input = String::from("F100
R180
F100
");
        let input = parse_input(&input);
        assert_eq!(0, solve(&input));
    }
}


