use std::io::{stdin, stdout, Write};
use bitflags::bitflags;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete as nom_c;
use nom::character::complete::newline;
use nom::multi::many0;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

advent_of_code::solution!(14);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct V {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Robot {
    position: V,
    velocity: V,
}

fn parse_vector(input: &str) -> IResult<&str, V> {
    let (input, (x, y)) = separated_pair(nom_c::i32, tag(","), nom_c::i32)(input)?;
    Ok((input, V { x, y }))
}

// p=0,4 v=3,-3
fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, position) = preceded(tag("p="), parse_vector)(input)?;
    let (input, velocity) = preceded(tag(" v="), parse_vector)(input)?;
    Ok((input, Robot { position, velocity }))
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    many0(terminated(parse_robot, newline))(input)
}

fn simulate(robot: Robot, space: V, seconds: i32) -> Robot {
    
    Robot {
        position: V {
            x: (robot.position.x + robot.velocity.x * seconds).rem_euclid(space.x),
            y: (robot.position.y + robot.velocity.y * seconds).rem_euclid(space.y),
        },
        ..robot
    }
}

bitflags! {
    struct D: u8 {
        const QUADS = 1 << 0;
        const DOTS = 1 << 1;
    }
}

fn debug_lobby(view: D, space: V, robots: &[Robot]) {
    let positions = robots.iter().map(|robot| robot.position).counts();
    let mx = space.x / 2;
    let my = space.y / 2;
    let quadrants = view.contains(D::QUADS);
    let dots = view.contains(D::DOTS);
    
    for y in 0..space.y {
        if quadrants && y == my {
            println!("{0:1$}", ' ', space.x as usize);
            continue;
        }
        let line: String = (0..space.x)
            .map(|x|
                if quadrants && x == mx {
                    String::from(' ')
                } else {
                    positions.get(&V{x, y}).map_or(
                        String::from(if dots {'.'} else {'0'}),
                        |n| n.to_string()
                    )
                })
            .collect();
        println!("{line}");
    }
}

fn part_one_parametrised(input: &str, space: V, seconds: i32, debug: bool) -> Option<usize> {
    let (rest, robots) = parse(input).unwrap();
    assert_eq!(rest, "");

    if debug {
        println!("Initial lobby:");
        debug_lobby(D::DOTS, space, &robots);
    }
    
    let lobby = robots
        .into_iter()
        .map(|r| simulate(r, space, seconds))
        .collect_vec();

    if debug {
        println!("After simulating {seconds}sec:");
        debug_lobby(D::DOTS | D::QUADS, space, &lobby);
    }

    let mx = space.x / 2;
    let my = space.y / 2;
    let ul = lobby.iter().filter(|robot| robot.position.x < mx && robot.position.y < my).count();
    let ur = lobby.iter().filter(|robot| robot.position.x > mx && robot.position.y < my).count();
    let ll = lobby.iter().filter(|robot| robot.position.x < mx && robot.position.y > my).count();
    let lr = lobby.iter().filter(|robot| robot.position.x > mx && robot.position.y > my).count();
    Some(ul * ur * ll * lr)
}

const LOBBY: V = V {x: 101, y: 103};

pub fn part_one(input: &str) -> Option<usize> {
    const SECONDS: i32 = 100;
    part_one_parametrised(input, LOBBY, SECONDS, false)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (rest, robots) = parse(input).unwrap();
    assert_eq!(rest, "");
    // TODO: search for "111111111111111111111"
    for i in 4000.. {
        let ri = robots.iter().map(|r| simulate(*r, LOBBY, i)).collect_vec();
        debug_lobby(D::DOTS, LOBBY, &ri);
        print!("{i}: Do you see a tree (y/n)?\n> ");
        stdout().flush().unwrap();
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) if line == "y" => return Some(i),
            _ => {},
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_parametrised(
            &advent_of_code::template::read_file("examples", DAY),
            V {x: 11, y: 7}, 
            100,
            true,
        );
        assert_eq!(result, Some(12));
    }
    
    // part two is interactive
}
