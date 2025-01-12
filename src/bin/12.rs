use pathfinding::matrix::Matrix;
use pathfinding::prelude::directions::DIRECTIONS_4;
use std::collections::BTreeSet;

advent_of_code::solution!(12);

type Pos = (usize, usize);
type IPos = (isize, isize);
type Region = BTreeSet<Pos>;

fn parse(s: &str) -> Matrix<char> {
    Matrix::from_rows(s
        .lines()
        .map(|l| l.chars())
    ).expect("grid")
}

fn find_regions(garden_map: &Matrix<char>) -> Vec<Region> {
    let mut regions = Vec::new();
    for ix in garden_map.keys() {
        if regions.iter().any(|r: &Region| r.contains(&ix)) {
            continue;
        }
        let c = garden_map.get(ix);
        let region = garden_map.bfs_reachable(ix, false, |ix2| {
            c == garden_map.get(ix2)
        });
        regions.push(region);
    }
    regions
}

fn imaginary_neighbours(pos: Pos) -> [IPos; 4] {
    DIRECTIONS_4.map(move |d| (pos.0 as isize + d.0, pos.1 as isize + d.1))
}

fn in_region(region: &Region, pos: IPos) -> bool {
    if pos.0 < 0 || pos.1 < 0 {
        false
    } else {
        region.contains(&(pos.0 as usize, pos.1 as usize))
    }
}

fn perimeter(region: &Region) -> usize {
    region.iter()
        .flat_map(|&p| imaginary_neighbours(p))
        .filter(|&p| !in_region(region, p))
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let garden_map = parse(input);
    let regions = find_regions(&garden_map);
    Some(regions.iter().map(|r|
        perimeter(r) * r.len()
    ).sum())
}

fn sides(region: &Region) -> usize {
    let min_row = region.iter().map(|p| p.0).min().unwrap() as isize;
    let min_col = region.iter().map(|p| p.1).min().unwrap() as isize;
    let max_row = region.iter().map(|p| p.0).max().unwrap() as isize;
    let max_col = region.iter().map(|p| p.1).max().unwrap() as isize;
    let mut sides = 0;
    for r in min_row-1..=max_row {
        let mut top = false;
        let mut bottom = false;
        for c in 0..=max_col {
            let n_top = in_region(region, (r, c));
            let n_bottom = in_region(region, (r+1, c));
            if (top, bottom) == (n_top, n_bottom) {
                continue // same line or outside/inside
            }
            (top, bottom) = (n_top, n_bottom);
            sides += if top ^ bottom {1} else {0};
        }
    }
    for c in min_col-1..=max_col {
        let mut left = false;
        let mut right = false;
        // compare the left and right column
        for r in 0..=max_row {
            let n_left = in_region(region, (r, c));
            let n_right = in_region(region, (r, c+1));
            if (left, right) == (n_left, n_right) {
                continue // same line or outside/inside
            }
            (left, right) = (n_left, n_right);
            sides += if left ^ right {1} else {0};
        }
    }
    sides
}

pub fn part_two(input: &str) -> Option<usize> {
    let garden_map = parse(input);
    let regions = find_regions(&garden_map);
    Some(regions.iter().map(|r|
        sides(r) * r.len()
    ).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }
}
