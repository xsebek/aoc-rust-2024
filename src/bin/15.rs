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
        'O' | '[' => 100 * k.0 + k.1,
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

fn expand(c: char) -> [char; 2] {
    match c {
        'O' => ['[', ']'],
        '@' => ['@', '.'],
        c => [c, c],
    }
}

fn expand_grid(grid: &Matrix<char>) -> Matrix<char> {
    Matrix::from_rows(grid.into_iter().map(|r|
        r.iter().flat_map(|&c| expand(c))
    )).expect("rectangle grid")
}

fn rec_move_single(grid: &mut Matrix<char>, start: (usize, usize), dir: (isize, isize)) -> bool {
    if let Some(next) = grid.move_in_direction(start, dir) {
        let move_res = match grid[next] {
            '#' => false,
            '.' => true,
            'O' | '[' | ']' => rec_move_dir(grid, next, dir),
            c => panic!("Unknown entity '{c}'"),
        };
        if move_res {
            grid.swap(start, next);
        }
        move_res
    } else {
        false
    }
}

fn other_pos(pos: (usize, usize), c: char) -> (usize, usize) {
    match c {
        '[' => (pos.0, pos.1 + 1),
        ']' => (pos.0, pos.1 - 1),
        c => panic!("No matching position for {c}"),
    }
}

fn rec_move_dir(grid: &mut Matrix<char>, start: (usize, usize), dir: (isize, isize)) -> bool {
    match grid[start] {
        c@('[' | ']') if dir == directions::S || dir == directions::N =>
            rec_move_single(grid, start, dir) && rec_move_single(grid, other_pos(start, c), dir),
        _ => rec_move_single(grid, start, dir),
    }
}

fn rec_move(grid: Matrix<char>, start: (usize, usize), dir: char) -> ((usize, usize), Matrix<char>) {
    match to_dir(dir) {
        Some(d) => {
            let mut temp = grid.clone();
            if rec_move_dir(&mut temp, start, d) {
                let next = grid.move_in_direction(start, d).unwrap();
                (next, temp)
            } else {
                (start, grid)
            }
        },
        None => (start, grid),
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, moves) = parse(input);
    let mut grid = expand_grid(&grid);
    // debug(&grid);
    
    let mut pos = find_pos(&grid);
    for m in moves.chars() {
        (pos, grid) = rec_move(grid, pos, m);
        // debug(&grid);
    }
    Some(gps(&grid))
}

/// endregion Part two

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));

        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(105 + 207 + 306));

        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(9021));
    }
}
