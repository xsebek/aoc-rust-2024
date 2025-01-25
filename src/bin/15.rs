use itertools::Itertools;
use pathfinding::matrix::Matrix;
use pathfinding::matrix::directions as directions;

advent_of_code::solution!(15);

/// region Parse input

fn parse(input: &str) -> (Matrix<char>, &str) {
    let (grid, movements) = input.split_once("\n\n").expect("two input parts");
    let grid = Matrix::from_rows(
        grid.lines().map(|l| l.chars())
    ).expect("rectangle grid");
    (grid, movements)
}

/// endregion

/// region Part one

fn find_pos(grid: &Matrix<char>) -> (usize, usize) {
    grid.items().find(|(_, &c)| c == '@').unwrap().0
}

fn move_dir(grid: &Matrix<char>, start: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
    for pos in grid.in_direction(start, dir) {
        match grid.get(pos) {
            Some('.') => return Some(pos),
            Some('O') => continue,
            _ => break
        }
    }
    None
}

fn to_dir(c: char) -> Option<(isize, isize)> {
    match c {
        '^' => Some(directions::N),
        '>' => Some(directions::E),
        'v' => Some(directions::S),
        '<' => Some(directions::W),
        _ => None,
    }
}

fn attempt_move(grid: &mut Matrix<char>, pos: (usize, usize), dir: char) -> Option<(usize, usize)> {
    let dir = to_dir(dir)?;
    let next = grid.move_in_direction(pos, dir)?;
    
    let last_pos = move_dir(grid, pos, dir)?;
    if last_pos != next {
        grid.swap(next, last_pos); // move the column of same elements by one
    }
    grid.swap(pos, next);
    Some(next)
}

fn gps(grid: &Matrix<char>) -> usize {
    grid.items().map(|(k, c)| match c {
        'O' => 100 * k.0 + k.1,
        _ => 0,
    }).sum()
}

#[allow(dead_code)]
fn debug(grid: &Matrix<char>) {
    for r in grid.iter() {
        println!("{}", r.iter().format(""))
    }
    println!();
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut grid, moves) = parse(input);
    let mut pos = find_pos(&grid);
    for m in moves.chars() {
        pos = attempt_move(&mut grid, pos, m).unwrap_or(pos);
        //debug(&grid);
    }
    Some(gps(&grid))
}

/// endregion Part one

/// region Part two

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

/// endregion Part two

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
