use std::collections::{HashMap, HashSet};

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day22");

fn calc_next(num: i64) -> i64 {
    let next = num;
    let next = (next ^ (next * 64)) % 16777216;
    let next = (next ^ (next / 32)) % 16777216;
    let next = (next ^ (next * 2048)) % 16777216;
    next
}

fn calc_steps(num: i64, steps: usize) -> i64 {
    let mut res = num;
    for _ in 0..steps {
        res = calc_next(res);
    }
    res
}

fn part1(input: &str) -> Result<i64> {
    Ok(to_lines(input)
        .map(|num| calc_steps(num.parse().unwrap(), 2000))
        .sum())
}

fn part2(input: &str) -> Result<i64> {
    let seqs: Vec<_> = to_lines(input)
        .map(|line| {
            let mut num: i64 = line.parse().unwrap();
            let mut prev = num % 10;
            let mut seq = Vec::with_capacity(2000);
            for _ in 0..2000 {
                let next = calc_next(num);
                let now = next % 10;
                let d = now - prev;
                seq.push((now, d));
                num = next;
                prev = now;
            }
            seq
        })
        .collect();
    let changes: Vec<_> = seqs
        .into_iter()
        .map(|seq| {
            let mut res = HashMap::new();
            for w in seq.windows(4) {
                let z: [i64; 4] = std::array::from_fn(|i| w[i].1);
                if !res.contains_key(&z) {
                    res.insert(z, w[3].0);
                }
            }
            res
        })
        .collect();
    let all_seqs: HashSet<_> = changes
        .iter()
        .flat_map(|changes| changes.keys().copied())
        .collect();

    let res = all_seqs
        .into_iter()
        .map(|seq| {
            changes
                .iter()
                .map(|changes| changes.get(&seq).copied().unwrap_or(0))
                .sum()
        })
        .max()
        .unwrap();
    Ok(res)
}

#[allow(dead_code)]
static EX_INPUT_1: &str = r#"
1
10
100
2024
"#;

#[allow(dead_code)]
static EX_INPUT_2: &str = r#"
1
2
3
2024
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT_1) 37327623,
    part2 => (EX_INPUT_2) 23
}
