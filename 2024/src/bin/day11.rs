use std::collections::HashMap;

use aoc_lib::{aoc, color_eyre::eyre::Result};

static INPUT: &str = include_str!("../../inputs/day11");

fn solve(input: &str, blinks: usize) -> Result<usize> {
    let mut groups = HashMap::new();

    fn insert_group(groups: &mut HashMap<usize, usize>, stone: usize, cnt: usize) {
        groups.entry(stone).and_modify(|c| *c += cnt).or_insert(cnt);
    }

    for stone in input.trim().split_ascii_whitespace() {
        let stone: usize = stone.parse()?;
        insert_group(&mut groups, stone, 1);
    }

    for _ in 0..blinks {
        let mut next_groups = HashMap::new();
        for (&stone, &cnt) in &groups {
            if stone == 0 {
                insert_group(&mut next_groups, 1, cnt);
            } else {
                let digits = stone.ilog10() + 1;
                if digits & 1 == 0 {
                    let d = 10usize.pow(digits / 2);
                    let new_stone_a = stone / d;
                    let new_stone_b = stone - new_stone_a * d;
                    insert_group(&mut next_groups, new_stone_a, cnt);
                    insert_group(&mut next_groups, new_stone_b, cnt);
                } else {
                    insert_group(&mut next_groups, stone * 2024, cnt);
                }
            }
        }
        std::mem::swap(&mut groups, &mut next_groups);
    }

    Ok(groups.values().copied().sum())
}

fn part1(input: &str) -> Result<usize> {
    solve(input, 25)
}

fn part2(input: &str) -> Result<usize> {
    solve(input, 75)
}

#[allow(dead_code)]
static EX_INPUT: &str = "125 17";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 55312,
    part2 => (EX_INPUT) 65601038650482
}
