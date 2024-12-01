use std::collections::HashMap;

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
};

static INPUT: &str = include_str!("../../inputs/day01");

fn parse(input: &str) -> Result<(Vec<i64>, Vec<i64>)> {
    aoc_lib::to_lines(input)
        .map(|x| -> Result<(i64, i64)> {
            let (a, b) = x.split_once("   ").ok_or_eyre("invalid format")?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect()
}

fn part1(input: &str) -> Result<u64> {
    let (mut a, mut b) = parse(input)?;
    a.sort_unstable();
    b.sort_unstable();
    Ok(a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum())
}

fn part2(input: &str) -> Result<i64> {
    let (a, b) = parse(input)?;
    let b = b.into_iter().fold(HashMap::new(), |mut acc, curr| {
        acc.entry(curr).and_modify(|cnt| *cnt += 1).or_insert(1);
        acc
    });
    Ok(a.into_iter()
        .map(|x| b.get(&x).copied().unwrap_or(0) * x)
        .sum())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 11,
    part2 => (EX_INPUT) 31
}
