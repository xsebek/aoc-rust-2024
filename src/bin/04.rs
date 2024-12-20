use itertools::{chain, Itertools};
use std::cmp::max;
use std::collections::HashSet;
use advent_of_code::{Grid};

advent_of_code::solution!(4);

fn find_word<'a, 'b>(word: &'a str, s: &'b str) -> impl Iterator<Item=usize> + use<'a, 'b>
{
    (0..max(s.len(), word.len()) - word.len() + 1)
        .filter(move |&i| s[i..].starts_with(word))
}

fn count_word(word: &str, s: &str) -> usize
{
    find_word(word, s).count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::new(input);
    let cols = (0..grid.cols).map(|c| grid.col(c));
    let rows = (0..grid.rows).map(|r| grid.row(r));
    let lr_diagonals_t = (0..grid.cols).map(|c| grid.lr_diagonal(0, c));
    let lr_diagonals_l = (1..grid.rows).map(|r| grid.lr_diagonal(r, 0));
    let rl_diagonals_t = (0..grid.cols).map(|c| grid.rl_diagonal(0, c));
    let rl_diagonals_r = (1..grid.rows).map(|r| grid.rl_diagonal(r, grid.cols - 1));
    
    Some(chain![cols, rows, lr_diagonals_t, rl_diagonals_t, lr_diagonals_l, rl_diagonals_r]
        .map(|s| {
            //println!("'{s}'");
            count_word("XMAS", &s) + count_word("SAMX", &s)
        })
        .sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::new(input);

    let lr_diagonal_pos: HashSet<(usize, usize)> = chain![
        (0..grid.cols).map(|c| (0, c)),
         (1..grid.rows).map(|r| (r, 0)),
    ].flat_map(|(r, c)| {
        let diag = grid.lr_diagonal(r, c);
        chain![find_word("MAS", &diag), find_word("SAM", &diag)].map(|i| (r + i + 1, c + i + 1)).collect_vec()
    }).collect();

    let rl_diagonal_pos: HashSet<(usize, usize)> = chain![
        (0..grid.cols).map(|c| (0, c)),
         (1..grid.rows).map(|r| (r, grid.cols - 1)),
    ].flat_map(|(r, c)| {
        let diag = grid.rl_diagonal(r, c);
        chain![find_word("MAS", &diag), find_word("SAM", &diag)].map(|i| (r + i + 1, c - i - 1)).collect_vec()
    }).collect();

    Some(lr_diagonal_pos.intersection(&rl_diagonal_pos).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
