use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete as nom_chars;
use nom::character::complete::newline;
use nom::combinator::eof;
use nom::multi::{separated_list1, many1};
use nom::sequence::separated_pair;
use nom::IResult;

advent_of_code::solution!(5);

fn parse_page_order(s: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(
        nom_chars::u32,
        tag("|"),
        nom_chars::u32
    )(s)
}

fn parse_pages(s: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), nom_chars::u32)(s)
}

fn parse(s: &str) -> IResult<&str, (Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    let (s, page_orders) = separated_list1(newline, parse_page_order)(s)?;
    let (s, _) = many1(newline)(s)?;
    let (s, pages) = separated_list1(newline, parse_pages)(s)?;
    let (s, _) = many1(newline)(s)?;
    eof(s)?;
    Ok((s, (page_orders, pages)))
}

fn pre_post(page_orders: &[(u32, u32)]) -> HashMap<u32, (HashSet<u32>, HashSet<u32>)> {
    page_orders.iter().fold(
        HashMap::new(),
        |m, &(l, r)| {
            let mut m = m;
            m.entry(l).or_insert((HashSet::new(), HashSet::new())).1.insert(r);
            m.entry(r).or_insert((HashSet::new(), HashSet::new())).0.insert(l);
            m
    })
}

fn sort_pages(pages: &[u32], pre_post: &HashMap<u32, (HashSet<u32>, HashSet<u32>)>) -> Vec<u32> {
    pages.iter().cloned().sorted_by(|l, r| {
        if let Some((l_pre, l_post)) = pre_post.get(&l) {
            if l_pre.contains(r) {
                return Ordering::Greater;
            }
            if l_post.contains(r) {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }).collect()
}

fn solve(input: &str) -> Option<(u32, u32)> {
    let (_, (page_orders, pages_lists)) = parse(input)
        .expect("input must be fully parsed");
    
    let pages_pp = pre_post(&page_orders);
    let mut sum1 = 0;
    let mut sum2 = 0;
    'page: for pages in pages_lists {
        let mut pre: HashSet<u32> = HashSet::new();
        let mut post: HashSet<u32> = HashSet::from_iter(pages.iter().skip(1).cloned());
        assert_eq!(post.len() + 1, pages.len(), "repeated page updates break logic");
        for page in &pages {
            if let Some((l, r)) = pages_pp.get(page) {
                if r.intersection(&pre).next().is_some() || l.intersection(&post).next().is_some() {
                    sum2 += sort_pages(&pages, &pages_pp)[pages.len() / 2];
                    continue 'page;
                }
            }
            pre.insert(*page);
            post.remove(page);
        }
        sum1 += pages[pages.len() / 2]
    }
    Some((sum1, sum2))
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input).map(|(sum1, _)| sum1)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input).map(|(_, sum2)| sum2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
