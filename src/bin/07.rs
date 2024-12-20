use nom::bytes::complete::tag;
use nom::character::complete as nom_chars;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

advent_of_code::solution!(7);

enum Op {
    Add,
    Mul,
    Cat,
}

impl Op {
    fn eval(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Cat => a * i64::pow(10, i64::ilog10(b) + 1) + b,
        }
    }
}

fn parse_line(s: &str) -> IResult<&str, (i64, Vec<i64>)> {
    separated_pair(
        nom_chars::i64,
        tag(": "),
        separated_list1(tag(" "), nom_chars::i64),
    )(s)
}

fn parse(s: &str) -> Vec<(i64, Vec<i64>)> {
    s.lines()
        .flat_map(parse_line)
        .map(|(rest, b)| {
            assert_eq!(rest.len(), 0);
            //println!("{b:?}");
            b
        })
        .collect()
}

fn solvable(ops: &[Op], result: i64, current: i64, numbers: &[i64]) -> bool {
    if current > result {
        return false;
    }
    match numbers.first() {
        None => current == result,
        Some(&n) => ops
            .into_iter()
            .any(|op| solvable(ops, result, op.eval(current, n), &numbers[1..])),
    }
}

fn sum_solvable(ops: &[Op], equations: &[(i64, Vec<i64>)]) -> i64 {
    equations
        .into_iter()
        .filter(|(res, nums)| solvable(ops, *res, nums[0], &nums[1..]))
        .map(|(res, _nums)| res)
        .sum()
}

pub fn part_one(input: &str) -> Option<i64> {
    let equations = parse(input);
    Some(sum_solvable(&[Op::Add, Op::Mul], &equations))
}

pub fn part_two(input: &str) -> Option<i64> {
    let equations = parse(input);
    Some(sum_solvable(&[Op::Add, Op::Mul, Op::Cat], &equations))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_op() {
        assert_eq!(Op::Add.eval(5, 5), 10);
        assert_eq!(Op::Mul.eval(5, 5), 25);
        assert_eq!(Op::Cat.eval(5, 5), 55);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
