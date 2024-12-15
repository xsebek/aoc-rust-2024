use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete as nom_chars;
use nom::branch::alt;
use nom::combinator::{value, map};
use nom::multi::fold_many0;
use nom::sequence::{separated_pair, delimited};
use crate::Op2::Do;

advent_of_code::solution!(3);

#[derive(Clone)]
enum Op {
    Mul { l: i32, r: i32 },
}

fn eval(op: Op) -> i32 {
    match op {
        Op::Mul { l, r } => l * r,
    }
}

fn parse_op(s: &str) -> IResult<&str, Op> {
    let (following, _) = tag("mul")(s)?;
    let (remain, (l, r)) = delimited(
        tag("("),
        separated_pair(
            nom_chars::i32,
            tag(","),
            nom_chars::i32
        ),
        tag(")")
    )(following)?;
    Ok((remain, Op::Mul { l, r }))
}

fn parse_one(s: &str) -> IResult<&str, Option<Op>> {
    alt((
        map(parse_op, Some),
        value(None, nom_chars::anychar),
    ))(s)
}

fn parse_all<F, R>(parser: F, s: &str) -> Vec<R>
  where F: Fn(&str) -> IResult<&str, Option<R>>
{
    let Ok((rest, ops)) = fold_many0(
        parser,
        Vec::new,
        |mut acc, item| {
            match item { Some(o) => acc.push(o), _ => {} };
            acc
        }
    )(s) else {
        panic!("Failed to parse input");
    };
    assert_eq!(rest.len(), 0);
    ops
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(parse_all(parse_one, input).into_iter().map(eval).sum())
}

#[derive(Clone)]
enum Op2 {
    Do { enabled : bool },
    Op { op : Op }
}

fn parse_do(s: &str) -> IResult<&str, Op2> {
    alt((
        value(Do {enabled: true}, tag("do()")),
        value(Do {enabled: false}, tag("don't()"))
        ))(s)
}

fn parse_two(s: &str) -> IResult<&str, Option<Op2>> {
    alt((
        map(parse_do, Some),
        map(parse_one, |o| o.map(|o| Op2::Op { op: o })),
    ))(s)
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(parse_all(parse_two, input)
        .into_iter()
        .fold((true, 0),
              |(e, s), o2| {
                  match o2 {
                    Op2::Do { enabled } => (enabled, s),
                    Op2::Op { op } => if e { (e, s + eval(op)) } else { (e, s) }
                }
            })
        .1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
