use aoc_lib::{aoc, color_eyre::eyre::Result, rangemap::RangeSet, to_lines};

static INPUT: &str = include_str!("../../inputs/day05");

fn part1(input: &str) -> Result<usize> {
    let mut input = to_lines(input);

    let valid: RangeSet<_> = input
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start: i64 = start.parse().unwrap();
            let end: i64 = end.parse().unwrap();
            start..end + 1
        })
        .collect();

    let result = input
        .map(|line| line.parse().unwrap())
        .filter(|x| valid.contains(x))
        .count();

    Ok(result)
}

fn part2(input: &str) -> Result<i64> {
    let valid: RangeSet<_> = to_lines(input)
        .take_while(|x| !x.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start: i64 = start.parse().unwrap();
            let end: i64 = end.parse().unwrap();
            start..end + 1
        })
        .collect();

    let result = valid.iter().map(|r| r.end - r.start).sum();

    Ok(result)
}

#[allow(dead_code)]
static EX_INPUT: &str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 3,
    part2 => (EX_INPUT) 14
}
