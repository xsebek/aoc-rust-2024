use std::collections::HashSet;
use std::iter::from_fn;
use itertools::{Itertools};
use advent_of_code;
use advent_of_code::Grid;

advent_of_code::solution!(6);

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct GridIx {
    row: isize,
    col: isize,
}

impl GridIx {
    fn rot90(&self) -> GridIx {
        GridIx { row: self.col, col: -self.row}
    }

    fn add(&self, other: &GridIx) -> GridIx {
        GridIx { row: self.row + other.row, col: self.col + other.col }
    }
}

fn find_start(grid: &Grid) -> GridIx {
    (0..grid.rows)
        .cartesian_product(0..grid.cols)
        .find(|&(row, col)| '^' == grid.get(row, col))
        .map(|(row, col)| GridIx { row: row as isize, col: col as isize})
        .unwrap()
}

fn move_guard(grid: &Grid, pos: GridIx, dir: GridIx) -> Option<(GridIx, GridIx)> {
    let next = pos.add(&dir);
    let ahead = grid.try_get(next.row, next.col)?;
    if ahead == '#' {
        return Some((pos, dir.rot90()))
    }
    Some((next, dir))
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::new(input);
    let mut dir = GridIx{ row: -1, col: 0};
    let mut loc = find_start(&grid);
    
    let distinct: HashSet<GridIx> =
        from_fn(move || { 
            let prev = (loc, dir);
            (loc, dir) = move_guard(&grid, loc, dir)?;
            Some(prev)
        })
        .map(|(p, _d)| { /*println!("{p:?}");*/ p})
        .collect();
    
    Some(distinct.len() + 1)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
