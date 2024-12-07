use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day07");

fn is_correct(total: u64, elems: &[u64]) -> bool {
    fn try_op(elems: &[u64], idx: usize, total: u64, target: u64) -> bool {
        let if_was_add = elems[idx] + total;
        let if_was_mul = elems[idx] * total;
        if idx == elems.len() - 1 {
            if_was_add == target || if_was_mul == target
        } else {
            try_op(elems, idx + 1, if_was_add, target) || try_op(elems, idx + 1, if_was_mul, target)
        }
    }
    try_op(elems, 0, 0, total)
}

fn part1(input: &str) -> Result<u64> {
    Ok(to_lines(input)
        .filter_map(|line| {
            let (total, elems) = line.split_once(": ").unwrap();
            let total = total.parse().unwrap();
            let elems: Vec<_> = elems
                .split_ascii_whitespace()
                .map(|elem| elem.parse::<u64>().unwrap())
                .collect();
            is_correct(total, &elems).then_some(total)
        })
        .sum())
}

fn is_correct_with_concat(total: u64, elems: &[u64]) -> bool {
    fn try_op(elems: &[u64], idx: usize, total: u64, target: u64) -> bool {
        let if_was_add = elems[idx] + total;
        let if_was_mul = elems[idx] * total;
        let if_was_concat = {
            let digits = elems[idx].ilog10() + 1;
            total * 10u64.pow(digits) + elems[idx]
        };
        if idx == elems.len() - 1 {
            if_was_add == target || if_was_mul == target || if_was_concat == target
        } else {
            try_op(elems, idx + 1, if_was_add, target)
                || try_op(elems, idx + 1, if_was_mul, target)
                || try_op(elems, idx + 1, if_was_concat, target)
        }
    }
    try_op(elems, 0, 0, total)
}

fn part2(input: &str) -> Result<u64> {
    Ok(to_lines(input)
        .filter_map(|line| {
            let (total, elems) = line.split_once(": ").unwrap();
            let total = total.parse().unwrap();
            let elems: Vec<_> = elems
                .split_ascii_whitespace()
                .map(|elem| elem.parse::<u64>().unwrap())
                .collect();
            is_correct_with_concat(total, &elems).then_some(total)
        })
        .sum())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 3749,
    part2 => (EX_INPUT) 11387
}
