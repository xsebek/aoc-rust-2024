use nom::bytes::complete::tag;
use nom::character::complete as nom_c;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;

advent_of_code::solution!(13);

struct Pos {
    x: i64,
    y: i64,
}

struct Machine {
    a: Pos,
    b: Pos,
    prize: Pos,
}

fn parse_pos(c: char, input: &str) -> IResult<&str, Pos> {
    let (input, (_,_,x)) = tuple((tag("X"), nom_c::char(c), nom_c::i64))(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, (_,_,y)) = tuple((tag("Y"), nom_c::char(c), nom_c::i64))(input)?;
    Ok((input, Pos { x, y }))
}

// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, a) = delimited(tag("Button A: "), |i| parse_pos('+', i), newline)(input)?;
    let (input, b) = delimited(tag("Button B: "), |i| parse_pos('+', i), newline)(input)?;
    let (input, prize) = delimited(tag("Prize: "), |i| parse_pos('=', i), newline)(input)?;
    Ok((input, Machine {a, b, prize}))
}

fn parse(s: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(newline, parse_machine)(s)
}

// X = A * ax + B * bx
// Y = A * ay + B * by
// -----
// A = (X - B * bx) / ax = (X / ax) - B * (bx / ax)
// -----
// Y = ((X / ax) - B * (bx / ax)) * ay + B * by
// Y - (X * ay / ax) = B * (- bx * ay / ax) + B * by
// Y - (X * ay / ax) = B * (by - (bx * ay / ax))
// -----
// B = (Y - (X * ay / ax)) / (by - (bx * ay / ax))
// B = (Y - (X * ay / ax)) / (by - (bx * ay / ax))
// B = (Y * ax / ax - (X * ay / ax)) / (by - (bx * ay / ax))
// B = (Y * ax - X * ay) / (ax * (by - (bx * ay / ax)))
// B = (Y * ax - X * ay) / (by * ax - bx * ay)
// -----
// X = A * ax + B * bx
// X - B * bx = A * ax
// A = (X - B * bx) / ax
fn solve(claw: Machine) -> Option<(i64, i64)> {
    // B = (Y * ax - X * ay) / (by * ax - bx * ay)
    let b_dividend = claw.prize.y * claw.a.x - claw.prize.x * claw.a.y;
    let b_divisor = claw.b.y * claw.a.x - claw.b.x * claw.a.y;
    if b_divisor == 0 || b_dividend % b_divisor != 0 {
        return None;
    }
    let b = b_dividend / b_divisor;
    
    // A = (X - B * bx) / ax
    let a_dividend = claw.prize.x - b * claw.b.x;
    let a_divisor = claw.a.x;
    if a_divisor == 0 || a_dividend % a_divisor != 0 {
        None
    } else {
        Some((a_dividend / a_divisor, b))
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let (rest, machines) = parse(input).expect("can parse input");
    assert_eq!(rest.len(), 0);
    Some(machines.into_iter().flat_map(solve).map(|(a, b)| 3 * a + b).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let (rest, machines) = parse(input).expect("can parse input");
    assert_eq!(rest.len(), 0);
    const HIGHER: i64 = 10000000000000;
    Some(machines.into_iter()
        .map(|m| Machine {prize: Pos {x: m.prize.x + HIGHER, y: m.prize.y + HIGHER}, ..m})
        .flat_map(solve)
        .map(|(a, b)| 3 * a + b)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.unwrap_or(0) > 480);
    }
}
