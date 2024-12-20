use advent_of_code;
use advent_of_code::Grid;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::from_fn;

advent_of_code::solution!(6);

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct GridIx {
    row: isize,
    col: isize,
}

impl GridIx {
    fn rot90(&self) -> GridIx {
        GridIx {
            row: self.col,
            col: -self.row,
        }
    }

    fn add(&self, other: &GridIx) -> GridIx {
        GridIx {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

fn find_start(grid: &Grid) -> GridIx {
    (0..grid.rows)
        .cartesian_product(0..grid.cols)
        .find(|&(row, col)| '^' == grid.get(row, col))
        .map(|(row, col)| GridIx {
            row: row as isize,
            col: col as isize,
        })
        .unwrap()
}

fn move_guard(grid: &Grid, pos: GridIx, dir: GridIx) -> Option<(GridIx, GridIx)> {
    let next = pos.add(&dir);
    let ahead = grid.try_get(next.row, next.col)?;
    if ahead == '#' {
        return Some((pos, dir.rot90()));
    }
    Some((next, dir))
}

fn patrol(grid: &Grid, pos: GridIx) -> (bool, HashSet<GridIx>) {
    let mut loc = pos;
    let mut dir = GridIx { row: -1, col: 0 };
    let mut pos_dir: HashSet<(GridIx, GridIx)> = HashSet::from([(loc, dir)]);
    let mut cycle: bool = false;
    let mut positions: HashSet<GridIx> = from_fn(|| {
        (loc, dir) = move_guard(&grid, loc, dir)?;
        if pos_dir.contains(&(loc, dir)) {
            cycle = true;
            return None;
        }
        pos_dir.insert((loc, dir));
        Some(loc)
    })
    .collect();
    positions.insert(pos);
    (cycle, positions)
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::new(input);
    let loc = find_start(&grid);
    let (cycle, path) = patrol(&grid, loc);
    assert_eq!(cycle, false);
    Some(path.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::new(input);
    let loc = find_start(&grid);
    let (_cycle, path) = patrol(&grid, loc);
    let mut input2 = String::from(input);
    Some(path
        .into_iter()
        .filter(|p| {
            let raw_ix = grid.raw_index(p.row, p.col);
            input2.replace_range(raw_ix..raw_ix+1, "#");
            let (cycle, _pos) = patrol(&Grid {data: &input2, ..grid}, loc);
            input2.replace_range(raw_ix..raw_ix+1, ".");
            cycle
        })
        .count())
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
        assert_eq!(result, Some(6));
    }
}
