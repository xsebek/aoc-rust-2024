use itertools::Itertools;

advent_of_code::solution!(1);

fn parse(input: &str) -> impl Iterator<Item=(i64, i64)> + use<'_> {
    input.lines()
        .map(|l| {
            let p = l.split_once(' ').expect("two numbers");
            (p.0.parse::<i64>().expect("n1"), p.1.trim().parse::<i64>().expect("n2"))
        })
}

pub fn part_one(input: &str) -> Option<i64> {
    let (l, r): (Vec<_>, Vec<_>) = parse(input).unzip();
    Some(l.into_iter().sorted()
        .zip(r.into_iter().sorted())
        .map(|(l, r)| i64::abs(l - r))
        .sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (l, r): (Vec<_>, Vec<_>) = parse(input).unzip();
    let l_c = l.into_iter().counts();
    let r_c = r.into_iter().counts();
    Some(l_c.iter()
        .map(|(l, lc)| r_c.get(l).map(|rc| *l as usize * lc * rc).unwrap_or(0))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
