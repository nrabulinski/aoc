use std::cmp::Ordering;

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    to_lines,
};

static INPUT: &str = include_str!("../../inputs/day05");

fn parse(input: &str) -> Result<([Vec<u8>; 100], impl Iterator<Item = Vec<u8>>)> {
    let mut iter = to_lines(input);
    let mut res = [const { Vec::new() }; 100];

    for line in iter.by_ref().take_while(|line| !line.is_empty()) {
        let (idx, val) = line.split_once('|').ok_or_eyre("invalid format")?;
        res[idx.parse::<usize>()?].push(val.parse()?);
    }

    let iter = iter.map(|line| {
        line.split(',')
            .map(|num| num.parse::<u8>().unwrap())
            .collect::<Vec<_>>()
    });

    Ok((res, iter))
}

fn is_correctly_ordered(nums: &[u8], checks: &[Vec<u8>; 100]) -> bool {
    let mut iter = nums.iter().peekable();
    let mut iter = std::iter::from_fn(|| {
        let a = iter.next()?;
        let b = iter.peek()?;
        Some((a, *b))
    });
    iter.all(|(a, b)| !checks[*b as usize].contains(a))
}

fn part1(input: &str) -> Result<u32> {
    let (checks, lines) = parse(input)?;

    Ok(lines
        .filter(|nums| is_correctly_ordered(nums, &checks))
        .map(|nums| nums[nums.len() / 2] as u32)
        .sum())
}

fn fix_ordering(mut nums: Vec<u8>, checks: &[Vec<u8>; 100]) -> Vec<u8> {
    nums.sort_unstable_by(|a, b| {
        if a == b {
            Ordering::Equal
        } else if checks[*a as usize].contains(b) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    nums
}

fn part2(input: &str) -> Result<u32> {
    let (checks, lines) = parse(input)?;

    Ok(lines
        .filter(|nums| !is_correctly_ordered(nums, &checks))
        .map(|nums| fix_ordering(nums, &checks))
        .map(|nums| nums[nums.len() / 2] as u32)
        .sum())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 143,
    part2 => (EX_INPUT) 123
}
