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
            let digits = id.ilog10() + 1;
            // leading zeros don't count,
            // so there can't be any invalid
            // ids with odd number of digits
            if digits & 1 == 1 {
                return false;
            }

            // We're checking if a number has a form of xyzxyz
            // meaning, we can just check if it's divisible by 1001.
            // Similarly, for abcdabcd, we need to check divisibility
            // by 10001, and so on.
            let d = 10i64.pow(digits / 2) + 1;
            id % d == 0
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
            let digits = id.ilog10() + 1;

            // The general approach is:
            // For any number x with digits n, find the divisors of n,
            // and for each of them calculate the "mask",
            // which will then be checked whether it can divide x.
            //
            // E.g. for an 8 digit number abcdefgh, we need to check whether:
            // a = b = c = d = e = f = g = h
            // ab = cd = ef = gh
            // abcd = efgh
            //
            // We can easily notice, that this will be true if one of those is a divisor of x:
            // 11111111, 1010101, 10001.
            // And those divisors represent 1, 2, and 4 - the divisors of 8.
            // The amount of 1s is n / divisor, and the "gap" between each 1 (or the size of each "chunk")
            // is the divisor itself.
            (1..=digits / 2)
                .filter(|&n| digits % n == 0)
                .map(|n| (0..digits / n).map(|i| 10i64.pow(i * n)).sum::<i64>())
                .any(|d| id % d == 0)
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
