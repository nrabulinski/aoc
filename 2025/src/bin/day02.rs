use aoc_lib::{aoc, color_eyre::eyre::Result};

static INPUT: &str = include_str!("../../inputs/day02");

fn part1(input: &str) -> Result<i64> {
    let result = input
        .split(',')
        .flat_map(|r| {
            let (start, end) = r.trim().split_once('-').unwrap();
            let start: i64 = start.parse().unwrap();
            let end: i64 = end.parse().unwrap();
            start..=end
        })
        .filter(|&id| {
            let digits = id.ilog10();
            // leading zeros don't count,
            // so there can't be any invalid
            // ids with odd number of digits
            if digits % 2 == 0 {
                return false;
            }
            let midpoint = (digits + 2) / 2;
            let mask = 10i64.pow(midpoint);
            let front = id / mask;
            let back = id % mask;
            front == back
        })
        .sum();
    Ok(result)
}

fn part2(input: &str) -> Result<i64> {
    let result = input
        .split(',')
        .flat_map(|r| {
            let (start, end) = r.trim().split_once('-').unwrap();
            let start: i64 = start.parse().unwrap();
            let end: i64 = end.parse().unwrap();
            start..=end
        })
        .filter(|&id| {
            let id = id.to_string();
            let mut divisors = (1..=id.len() / 2).filter(|len| id.len() % len == 0);
            divisors.any(|sz| {
                let mut i = id.as_bytes().chunks(sz);
                let fst = i.next().unwrap();
                i.all(|curr| curr == fst)
            })
        })
        .sum();
    Ok(result)
}

#[allow(dead_code)]
static EX_INPUT: &str = "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 1227775554,
    part2 => (EX_INPUT) 4174379265
}
