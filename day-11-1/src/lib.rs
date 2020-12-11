

use std::convert::TryInto;

type Grid = Vec<Vec<Cell>>;
type Point= (usize, usize);

#[derive(Clone, PartialEq)]
pub enum Cell {
    None,
    Seated,
    Free
}

pub fn parse_input(input: &str) -> Grid {
    input.trim()
        .split('\n')
        .map(|x| 
            x.chars().map(|c| 
                if c == 'L' {Cell::Free}
                else        {Cell::None}
            ).collect::<Vec<Cell>>()
        ).collect::<Grid>()
}

fn is_valid_cell(s: &Grid, p: (i32, i32)) -> bool {
       0 <= p.0 && (p.0 as usize) < s.len()
    && 0 <= p.1 && (p.1 as usize) < s[0].len()
}

fn count_seats(grid: &Grid) -> u32 {
    grid.iter()
        .flatten()
        .filter(|x| Cell::Seated == **x )
        .count().try_into().unwrap()
}

fn get_adjacent(grid: &Grid, p: Point) -> Vec<&Cell> {
    [
        // Top row
        (-1, -1) as (i32, i32),
        ( 0, -1),
        ( 1, -1),
        // Same row
        (-1,  0),
        ( 1,  0),
        // Bottom row
        (-1,  1),
        ( 0,  1),
        ( 1,  1),
    ].iter()
     .map(|(x, y)| (x+p.0 as i32, y+p.1 as i32))
     .filter(|x| is_valid_cell(&grid, *x))
     .map(|(x, y)| &grid[x as usize][y as usize])
     .collect()
}

fn next_state(grid: &Grid, p: Point) -> Cell {
    match &grid[p.0][p.1] {
        Cell::Free   => {
            let adj_seated = get_adjacent(grid, (p.0, p.1))
                .iter()
                .filter(|x| ***x == Cell::Seated)
                .count();

            if adj_seated > 0 {Cell::Free} else {Cell::Seated} 
        },
        Cell::Seated => {
            let adj_seated = get_adjacent(grid, (p.0, p.1))
                .iter()
                .filter(|x| ***x == Cell::Seated)
                .count();

            if adj_seated > 3 {Cell::Free} else {Cell::Seated}
        },
        Cell::None => Cell::None
    }
}

fn next_grid(grid: &mut Grid) -> Result<(),()> {
    let read = grid.clone();
    let mut changed = Err(());
    for (i, row) in read.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            grid[i][j] = next_state(&read, (i, j));
            if grid[i][j] != *cell {
                changed = Ok(())
            }
        }
    }
    changed
}

pub fn solve(input: &Grid) -> u32 {
    let mut grid = input.to_vec();
    while next_grid(&mut grid).is_ok() {}
    count_seats(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn solve_test() {
        let input = String::from("L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
");
        let input = parse_input(&input);
        assert_eq!(37, solve(&input));
    }
}