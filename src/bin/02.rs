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

fn diff<'a, I>(numbers: I) -> impl Iterator<Item=i32> + use<'a, I>
where I: IntoIterator<Item=&'a i32>,{
    numbers.into_iter()
        .tuple_windows()
        .map(|(a, b)| a - b)
}

fn inside(l: i32, r: i32) -> impl Fn(i32) -> bool {
    move |n| n >= l && n <= r
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(parse(input)
        .into_iter()
        .filter(|l| diff(l).all(inside(1, 3)) || diff(l).all(inside(-3, -1)))
        .count())
}

fn without(i: usize, numbers: &[i32]) -> impl Iterator<Item = &i32> + '_ {
    let (l,r) = numbers.split_at(i);
    l.iter().chain(r.iter().dropping(1))
}

fn dampen<F>(f: F, numbers: &[i32]) -> bool 
where F: Fn(i32) -> bool
{
    (0..numbers.len()).any(|i| diff(without(i, numbers)).all(|n| f(n)))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(parse(input)
        .into_iter()
        .filter(|l| dampen(inside(1,3), l) || dampen(inside(-3,-1), l))
        .count())
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
        assert_eq!(result, Some(4));
    }
}
