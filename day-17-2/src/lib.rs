
use std::collections::HashMap;

const CYCLES: u32 = 6;

type Grid = HashMap<String, bool>;

fn is_alive_next_turn(g: &Grid, coords: &(i32, i32, i32, i32)) -> bool {
    let mut sum = 0;
    let mut current = false;
    //println!("for {:?} considering:", coords);
    for x in coords.0-1..coords.0+2 {
        for y in coords.1-1..coords.1+2 {
            for z in coords.2-1..coords.2+2 {
                for w in coords.3-1..coords.3+2 {
                    let coords_str = format!("{},{},{},{}",x,y,z,w);

                    let cell = g.get(&coords_str).unwrap_or(&false);
                    //println!("cell {:?} which holds {}", coords_str, cell);
                    if x == coords.0 && y == coords.1 
                    && z == coords.2 && w == coords.3 {
                        current = *cell;
                    } else if *cell {
                        sum += 1;
                    }
                }
            }
        }
    }
    //println!("sum: {:?}", sum);
    sum == 3 || (current && sum == 2) 
}

fn next_state(g: &Grid) -> Grid {
    let mut next = g.clone();
    for (key, value) in g.iter() {
        if !value {
            continue;
        }
        let mut coords_iter = key.split(',');
        let coords: (i32, i32, i32, i32) = (
            coords_iter.next().unwrap().parse().unwrap(),
            coords_iter.next().unwrap().parse().unwrap(),
            coords_iter.next().unwrap().parse().unwrap(),
            coords_iter.next().unwrap().parse().unwrap(),
        );
        for x in coords.0-1..coords.0+2 {
            for y in coords.1-1..coords.1+2 {
                for z in coords.2-1..coords.2+2 {
                    for w in coords.3-1..coords.3+2 {
                        let coords_str = format!("{},{},{},{}",x,y,z,w);
                        let cell = next.entry(coords_str.to_string()).or_insert(false);
                        *cell = is_alive_next_turn(&g, &(x,y,z,w));
                    }
                }
            }
        }
    }
    next
}

fn get_cubes_count(g: &Grid) -> u32 {
    g.iter().map(|(_, x)| *x as u32).sum()
}

pub fn parse_input(input: &str) -> Grid {
    let lines = input.trim().split('\n').enumerate();
    let mut g = Grid::new();
    for (i, line) in lines {
        let y = i;
        for (x, c) in line.chars().enumerate() {
            let val = matches!(c, '#');
            if val {
                g.insert(format!("{},{},{},{}",x,y,0,0), val);
            }
        }
    }
    g
}

pub fn solve(input: &Grid) -> u32 {
    let mut current = input.clone();

    for _i in 0..CYCLES {
        current = next_state(&current);
    }
    get_cubes_count(&current)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn solve_test() {
        let input = String::from(".#.
..#
###
");
        let input = parse_input(&input);
        assert_eq!(848, solve(&input));
    }
}