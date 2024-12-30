use std::collections::HashMap;
use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::One;

advent_of_code::solution!(11);

fn log10(n: &BigUint) -> u32 {
    let mut n = n.clone();
    let mut res = 0;
    while n >= BigUint::from(10u32) {
        n /= 10u32;
        res += 1;
    }
    res
}

fn stone_change<U>(n: U) -> (BigUint, Option<BigUint>)
where U: Into<BigUint>
{
    let n: BigUint = n.into();
    if n == BigUint::ZERO {
        (BigUint::one(), None)
    } else {
        let digits = log10(&n) + 1;
        let half = &BigUint::from(10u32).pow(digits / 2);
        if digits % 2 == 0 {
            (&n / half, Some(&n % half))
        } else {
            (n * 2024u32, None)
        }
    }
}

struct CachedGenerations {
    generations: HashMap<BigUint, Vec<BigUint>>
}

impl CachedGenerations {
    fn new() -> Self {
        Self { generations: HashMap::new() }
    }

    fn get<U>(&mut self, n: U, blinks: usize) -> BigUint
    where U: Into<BigUint>
    {
        let n: BigUint = n.into();
        if blinks == 0 {
            return BigUint::one();
        }
        if !self.generations.contains_key(&n) {
            self.generations.insert(n.clone(), Vec::new());
        }
        let v_len = self.generations[&n].len();
        for blinks in v_len+1..blinks+1 {
            let (n1, on2) = stone_change(n.clone());
            let g1 = self.get(n1, blinks - 1);
            let g2 = on2.map_or(BigUint::ZERO, |n2| self.get(n2, blinks - 1));
            self.generations.get_mut(&n).unwrap().push(g1 + g2);
        }
        self.generations[&n][blinks - 1].clone()
    }
}

pub fn part_one(input: &str) -> Option<BigUint> {
    let numbers = input.split_ascii_whitespace().flat_map(|s| s.parse::<BigUint>()).collect_vec();
    let mut cache = CachedGenerations::new();
    Some(numbers.into_iter().map(|n| cache.get(n, 25)).sum())
}

pub fn part_two(input: &str) -> Option<BigUint> {
    let numbers = input.split_ascii_whitespace().flat_map(|s| s.parse::<BigUint>()).collect_vec();
    let mut cache = CachedGenerations::new();
    Some(numbers.into_iter().map(|n| cache.get(n, 75)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stone() {
        assert_eq!(stone_change(0u32), (1u32.into(), None));
        assert_eq!(stone_change(1u32), (2024u32.into(), None));
        assert_eq!(stone_change(10u32), (1u32.into(), Some(0u32.into())));
        assert_eq!(stone_change(99u32), (9u32.into(), Some(9u32.into())));
        assert_eq!(stone_change(999u32), (2021976u32.into(), None));
    }

    #[test]
    fn test_cache() {
        let mut cache = CachedGenerations::new();
        assert_eq!(cache.get(125u32, 1), 1u32.into());
        assert_eq!(cache.get(125u32, 2), 2u32.into());
        assert_eq!(cache.get(125u32, 3), 2u32.into());
        println!("{:?}", cache.generations);
        assert_eq!(cache.get(125u32, 6), 7u32.into());
        println!("{:?}", cache.generations);
        assert_eq!(cache.get(17u32, 1), 2u32.into()); // 1 7
        assert_eq!(cache.get(17u32, 2), 2u32.into()); // 2024 14168
        assert_eq!(cache.get(17u32, 3), 3u32.into()); // 20 24 28676032
        assert_eq!(cache.get(17u32, 4), 6u32.into()); // 2 0 2 4 2867 6032
        assert_eq!(cache.get(17u32, 5), 8u32.into()); // 4048 1 4048 8096 28 67 60 32
        assert_eq!(cache.get(17u32, 6), 15u32.into()); // 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2
        println!("{:?}", cache.generations);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312u32.into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
