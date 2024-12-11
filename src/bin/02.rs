use itertools::Itertools;

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|l| l
            .split_ascii_whitespace()
            .map(|w| w.parse().expect("number"))
            .collect())
        .collect()
}

fn diff(numbers: &[i32]) -> impl Iterator<Item=i32> + use<'_> {
    numbers.iter().tuple_windows().map(|(a, b)| a - b)
}

fn inside(l: i32, r: i32) -> impl Fn(i32) -> bool {
    move |n| n >= l && n <= r
}

pub fn part_one(input: &str) -> Option<usize> {
    let m = parse(input);
    Some(m.iter()
        .filter(|l| diff(l).all(inside(1,3)) || diff(l).all(inside(-3,-1)))
        .count())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
