use std::collections::BTreeSet;
use itertools::Itertools;
use pathfinding::matrix::Matrix;
use pathfinding::prelude::{bfs_reach, count_paths};

advent_of_code::solution!(10);

const WALL: i8 = -111;

fn parse(s: &str) -> Matrix<i8> {
    Matrix::from_rows(s
        .lines()
        .map(|l| l
            .chars()
            .map(|c| c.to_digit(10).map_or(WALL, |n| n as i8)))
    ).expect("grid")
}

#[allow(dead_code)]
fn pretty_pos(i: i8) -> char {
    if i == WALL {
        '.'
    } else {
        char::from_digit(i as u32, 10).unwrap()
    }
}

#[allow(dead_code)]
fn pretty(grid: &Matrix<i8>) -> String {
    Itertools::intersperse(
        grid.iter().map(|r| r.iter().cloned().map(pretty_pos).collect()),
        String::from("\n")
    ).collect()
}

fn is_nice(matrix: &Matrix<i8>, start: (usize, usize), end: (usize, usize)) -> bool {
    match (matrix.get(start), matrix.get(end)) {
        (Some(&s), Some(&e)) => s + 1 == e,
        _ => false,
    }
}

fn trailhead_score(matrix: &Matrix<i8>, start: (usize, usize), ends: &BTreeSet<(usize,usize)>) -> usize {
    let reachable: BTreeSet<(usize,usize)> = bfs_reach(start, |&n1| {
        matrix.neighbours(n1, false)
            .filter(|&n2| is_nice(matrix, n1, n2))
            .collect_vec()
    }).collect();
    reachable.intersection(ends).count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = parse(input);
    //println!("{}", pretty(&matrix));
    let starts = matrix.keys().filter(|&p| matrix[p] == 0).collect_vec();
    //println!("starts: {:?}", starts);
    let ends: BTreeSet<(usize,usize)> = matrix.keys().filter(|&p| matrix[p] == 9).collect();
    //println!("ends: {:?}", ends);
    Some(starts.into_iter()
        .map(|s| trailhead_score(&matrix, s, &ends))
        .sum())
}

fn trailhead_rating(matrix: &Matrix<i8>, start: (usize, usize), ends: &BTreeSet<(usize,usize)>) -> usize {
    count_paths(
        start,
        |&n1| {
            matrix.neighbours(n1, false)
                .filter(|&n2| is_nice(matrix, n1, n2))
                .collect_vec()
        },
        |e| ends.contains(e))
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = parse(input);
    let starts = matrix.keys().filter(|&p| matrix[p] == 0).collect_vec();
    let ends: BTreeSet<(usize,usize)> = matrix.keys().filter(|&p| matrix[p] == 9).collect();
    Some(starts.into_iter()
        .map(|s| trailhead_rating(&matrix, s, &ends))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 0));
        assert_eq!(result, Some(1));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(1 + 2));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
