use std::collections::HashSet;
use itertools::{chain, iproduct, Itertools};
use advent_of_code::{Grid, GridIx};

advent_of_code::solution!(8);

fn get_opposite_antinodes(grid: &Grid, a1: GridIx, a2: GridIx) -> Vec<GridIx> {
    // distance from A1 ---> A2
    let delta = GridIx {
        row: a2.row - a1.row,
        col: a2.col - a1.col,
    };
    [a2.add(&delta), a1.add(&GridIx {row: -delta.row, col: -delta.col})]
        .into_iter()
        .filter(|ix| ix.is_within(grid))
        .collect()
}

fn collect_antinodes<F>(grid: &Grid, get_antinodes: F) -> HashSet<GridIx>
  where F: Fn(&Grid, GridIx, GridIx) -> Vec<GridIx>
{
    let chars: HashSet<char> = HashSet::from_iter(grid.data.chars().filter(|c| c.is_alphanumeric()));
    //   for each character get all positions
    chars
     .into_iter()
     .flat_map(|c| {
         // for each pair get antinodes position
         iproduct!(0..grid.rows, 0..grid.cols)
             .filter(move |&(row, col)| c == grid.get(row, col))
             .map(|(row, col)| GridIx::new_u(row, col) )
             .tuple_combinations()
             .flat_map(|(a1, a2)| get_antinodes(&grid, a1, a2))
     })
     // get the size of the set of unique antinodes positions
     .collect::<HashSet<GridIx>>()
} 

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::new(input);
    Some(collect_antinodes(&grid, get_opposite_antinodes).len())
}

fn get_resonant_antinodes(grid: &Grid, a1: GridIx, a2: GridIx) -> Vec<GridIx> {
    // distance from A1 ---> A2
    let d = GridIx {
        row: a2.row - a1.row,
        col: a2.col - a1.col,
    };
    chain![
        (0..).map(|n| a2.add(&GridIx {row: n * d.row, col: n * d.col})).take_while(|&a| a.is_within(grid)),
        (0..).map(|n| a1.add(&GridIx {row: -n * d.row, col: -n * d.col})).take_while(|&a| a.is_within(grid)),
    ].collect()
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::new(input);
    Some(collect_antinodes(&grid, get_resonant_antinodes).len())}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antinodes() {
        let grid = Grid::new(
            "..........\n\
            ...#......\n\
            ..........\n\
            ....a.....\n\
            ..........\n\
            .....a....\n\
            ..........\n\
            ......#...\n\
            ..........\n\
            ..........");
        assert_eq!(get_opposite_antinodes(&grid, GridIx::new(3, 4), GridIx::new(5, 5)),
                   [GridIx::new(7, 6), GridIx::new(1, 3)]);

        assert_eq!(part_one(grid.data), Some(2));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
