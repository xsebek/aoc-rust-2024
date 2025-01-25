use std::collections::HashMap;
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

fn debug_map(view: D, space: V, robot_map: HashMap<V, usize>) {
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
                    robot_map.get(&V{x, y}).map_or(
                        String::from(if dots {'.'} else {'0'}),
                        |n| n.to_string()
                    )
                })
            .collect();
        println!("{line}");
    }
}

fn debug_lobby(view: D, space: V, robots: &[Robot]) {
    debug_map(view, space, robots.iter().map(|robot| robot.position).counts())
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
    let mut quads = [[0; 3]; 3];
    for robot in lobby {
        let xi = mx.cmp(&robot.position.x) as i8;
        let yi = my.cmp(&robot.position.y) as i8;
        quads[(xi + 1) as usize][(yi + 1) as usize] += 1;
    }
    Some(quads[0][0] * quads[0][2] * quads[2][0] * quads[2][2])
}

const LOBBY: V = V {x: 101, y: 103};

pub fn part_one(input: &str) -> Option<usize> {
    const SECONDS: i32 = 100;
    part_one_parametrised(input, LOBBY, SECONDS, false)
}

fn longest_sequence(input: &[i32]) -> usize {
    let mut max = 0;
    let mut seq = 1;
    for (&previous, &current) in input.iter().zip(input.iter().skip(1)) {
        if previous + 1 == current {
            seq += 1;
        } else {
            max = max.max(seq);
            seq = 1;
        }
    }
    max.max(seq)
}

fn robots_in_line(robots: &[Robot], space: V, seconds: i32) -> (HashMap<V, usize>, usize) {
    let robot_map = robots
        .iter()
        .map(|r| simulate(*r, space, seconds).position)
        .counts();

    let lines = robot_map.iter()
        .sorted_by_key(|(v, _)| v.y)
        .chunk_by(|(v, _)| v.y)
        .into_iter()
        .map(|(_, g)| g.map(|(v, _)| v.x).sorted().collect_vec())
        .collect_vec();

    (robot_map, lines.iter().map(|l| longest_sequence(l)).max().unwrap())
}

fn interact(query: String) -> bool {
    print!("{query} (y/n)?\n> ");
    stdout().flush().unwrap();
    let mut line = String::new();
    stdin().read_line(&mut line).is_ok() && line.to_lowercase().contains("y")
}

fn part_two_parametrised(input: &str, space: V, interactive: Option<usize>) -> Option<i32> {
    let (rest, robots) = parse(input).unwrap();
    assert_eq!(rest, "");
    let mut max_line = 0;
    let mut result: Option<i32> = None;

    for i in 1..(space.x * space.y) {
        if interactive.is_some() && i % 1000 == 0 {
            println!("== {i:6} ============================")
        }
        let (robot_map, line_len) = robots_in_line(&robots, LOBBY, i);

        if line_len > max_line {
            max_line = line_len;
            result = Some(i);
        }

        if interactive.is_none_or(|threshold| line_len < threshold) {
            continue
        }

        debug_map(D::DOTS, space, robot_map);
        if interact(format!("{i}: Do you see a tree")) {
            return Some(i);
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<i32> {
    part_two_parametrised(input, LOBBY, None) // set Some(6) to search interactively
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

    #[test]
    fn test_longest_sequence() {
        assert_eq!(longest_sequence(&vec![1, 2, 4, 5, 6, 8, 9]), 3);
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two_parametrised(
            &advent_of_code::template::read_file("examples", DAY), 
            V {x: 11, y: 7}, 
            None);
        assert_eq!(result, Some(1));
    }
}
