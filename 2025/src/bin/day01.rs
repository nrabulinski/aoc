use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day01");

fn part1(input: &str) -> Result<i64> {
    let result = to_lines(input)
        .map(|line| {
            let cnt: i64 = line[1..].parse().unwrap();
            if line.starts_with('L') { -cnt } else { cnt }
        })
        .scan(50, |state, curr| {
            *state = (*state + curr).rem_euclid(100);
            Some(*state)
        })
        .filter(|&x| x == 0)
        .count();
    Ok(result as i64)
}

fn part2(input: &str) -> Result<i64> {
    let result = to_lines(input)
        .map(|line| {
            let cnt: i64 = line[1..].parse().unwrap();
            if line.starts_with('L') { -cnt } else { cnt }
        })
        .scan(50, |dial, d| {
            let new_dial = (*dial + d).clamp(0, 100);

            // if we didn't cross the barrier, exit early
            if !matches!(new_dial, 0 | 100) {
                *dial += d;
                return Some(0);
            }

            // if we started at zero, don't count that as crossing
            let crossed_early = if *dial != new_dial { 1 } else { 0 };

            let first_d = new_dial - *dial;
            let remaining = d - first_d;

            *dial = (*dial + d).rem_euclid(100);

            Some(remaining.abs() / 100 + crossed_early)
        })
        .sum();

    Ok(result)
}

#[allow(dead_code)]
static EX_INPUT: &str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 3,
    part2 => (EX_INPUT) 6
}
