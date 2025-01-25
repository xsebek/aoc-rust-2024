use std::iter::once;
use itertools::{chain, Itertools};
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

fn reindeer_move(grid: &Matrix<char>, start: Pos, end: Pos, dir: Dir)
    -> impl IntoIterator<Item=((Pos, Dir), usize)>
{
    let rotation_cost = if start == end { 0 } else { 1000 };
    chain![
        grid.move_in_direction(start, dir)
            .filter(|p| grid[*p] != '#')
            .map(|p| ((p, dir), 1)),
        once(((start, rotate90(dir, 1)), rotation_cost)),
        once(((start, rotate90(dir, -1)), rotation_cost)),
    ]
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    let start = find_pos(&grid, 'S');
    let end = find_pos(&grid, 'E');
    dijkstra(
        &(start, E),
        |(pos, dir)| reindeer_move(&grid, *pos, end, *dir),
        |(p, _)| *p == end,
    ).map(|res| res.1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    let start = find_pos(&grid, 'S');
    let end = find_pos(&grid, 'E');
    let lowest_score = part_one(input)?;
    let reachable = dijkstra_reach(
        &(start, E),
        |(pos, dir), _c| reindeer_move(&grid, *pos, end, *dir));

    let reverse = dijkstra_all(
        &(end, S),
        |(pos, dir)| reindeer_move(&grid, *pos, end, *dir));

    // let mut reached = HashSet::<Pos>::new();
    // for r in reachable
    //     .filter(|r| r.total_cost <= lowest_score)
    // {
    //     println!("-------------------------------------");
    //     println!("NODE: {:2?} cost: {}", r.node, r.total_cost);
    //     let rev_k = (r.node.0, rotate90(r.node.1, 2));
    //     let rev_r = reverse.get(&rev_k);
    //     println!("REV:  {:2?} cost: {:?}", rev_k, rev_r.map(|rr| rr.1));
    //     if rev_r.is_some_and(|rr| rr.1 + r.total_cost <= lowest_score) {
    //         reached.insert(r.node.0);
    //         println!("--> OK: new count: {}", reached.len())
    //     }
    // 
    //     let mut grid2 = grid.clone();
    //     grid2[r.node.0] = 'O';
    //     debug(&grid2);
    // }
    // Some(reached.len())
    Some([end].len() + reachable
        .take_while(|r| r.total_cost < lowest_score)
        .filter(|r| reverse
            .get(&(r.node.0, rotate90(r.node.1, 2)))
            .is_some_and(|(_, c)| c + r.total_cost <= lowest_score))
        .map(|r| r.node.0)
        .unique()
        .count())
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(45));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(64));
    }
}
