use std::iter::once;
use itertools::chain;
use pathfinding::prelude::*;
use pathfinding::prelude::directions::*;
use pathfinding::matrix::Matrix;

advent_of_code::solution!(16);

type Pos = (usize, usize);
type Dir = (isize, isize);

/// region Parse input

fn parse(input: &str) -> Matrix<char> {
    Matrix::from_rows(
        input.lines().map(|l| l.chars())
    ).expect("rectangle grid")
}

fn find_pos(grid: &Matrix<char>, elem: char) -> Pos {
    grid.items().find(|(_, &c)| c == elem).unwrap().0
}

/// endregion

fn rotate90(dir: Dir, count: i8) -> Dir {
    match count.rem_euclid(4) {
        0 => ( dir.0,  dir.1), // ( 1,  0)  ( 0,  1)
        1 => ( dir.1, -dir.0), // ( 0, -1)  ( 1,  0)
        2 => (-dir.0, -dir.1), // (-1,  0)  ( 0, -1)
        3 => (-dir.1,  dir.0), // ( 0,  1)  (-1,  0)
        _ => unreachable!(),
    }
}

fn reindeer_move(grid: &Matrix<char>, start: Pos, dir: Dir)
    -> impl IntoIterator<Item=((Pos, Dir), u32)>
{
    chain![
        once(((start, rotate90(dir, 1)), 1000)),
        once(((start, rotate90(dir, -1)), 1000)),
        grid.move_in_direction(start, dir)
            .filter(|p| grid[*p] != '#')
            .map(|p| ((p, dir), 1)),
    ]
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let start = find_pos(&grid, 'S');
    let end = find_pos(&grid, 'E');
    dijkstra(
        &(start, E),
        |(pos, dir)| reindeer_move(&grid, *pos, *dir),
        |(p, _)| *p == end,
    ).map(|res| res.1)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(7036));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
