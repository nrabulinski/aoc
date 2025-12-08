use std::{array, ops::Not};

use aoc_lib::{aoc, color_eyre::eyre::Result};

static INPUT: &str = include_str!("../../inputs/day06");

fn do_op(a: i64, b: i64, op: &str) -> i64 {
    match op {
        "+" => a + b,
        "*" => a * b,
        _ => todo!("{op}"),
    }
}

fn part1(input: &str) -> Result<i64> {
    let mut input = input.trim().lines().map(|line| line.trim());

    let operators: Vec<_> = input
        .next_back()
        .unwrap()
        .split_ascii_whitespace()
        .collect();

    let mut input = input.map(|line| line.split_ascii_whitespace().map(|n| n.parse().unwrap()));
    let init: Vec<i64> = input.next().unwrap().collect();

    let columns = input.fold(init, |mut acc, curr| {
        curr.zip(acc.iter_mut())
            .zip(operators.iter())
            .for_each(|((a, b), op)| *b = do_op(a, *b, op));
        acc
    });

    let result = columns.into_iter().sum();

    Ok(result)
}

fn do_one_column(input: &str, total_width: usize, x: usize) -> i64 {
    input
        .as_bytes()
        .iter()
        .skip(x)
        .step_by(total_width)
        .filter(|&b| b.is_ascii_whitespace().not())
        .map(|b| (b - b'0') as i64)
        .reduce(|acc, curr| acc * 10 + curr)
        .unwrap()
}

fn do_one_group(input: &str, total_width: usize, start: usize, width: usize, op: &str) -> i64 {
    (0..width)
        .rev()
        .map(|x| do_one_column(input, total_width, x + start))
        .reduce(|acc, curr| do_op(acc, curr, op))
        .unwrap()
}

fn part2(input: &str) -> Result<i64> {
    let input = input.trim();

    let mut lines = input.lines();
    let width = lines.next().unwrap().len() + 1;

    let operators = lines.next_back().unwrap();
    let height = input.len() / width;
    let operators: Vec<_> = operators
        .as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(idx, b)| {
            b.is_ascii_whitespace()
                .not()
                .then_some((idx, str::from_utf8(array::from_ref(b)).unwrap()))
        })
        .collect();
    let operators = operators.iter().enumerate().map(|(idx, &(start, op))| {
        let end = operators
            .get(idx + 1)
            .map_or_else(|| width - 1, |&(s, _)| s - 1);
        let w = end - start;
        (start, w, op)
    });

    let input = &input[..width * height];

    let result = operators
        .map(|(start, curr_width, op)| do_one_group(input, width, start, curr_width, op))
        .sum();

    Ok(result)
}

#[allow(dead_code)]
static EX_INPUT: &str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 4277556,
    part2 => (EX_INPUT) 3263827
}
