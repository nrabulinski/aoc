use aoc_lib::{aoc, color_eyre::eyre::Result};

static INPUT: &str = include_str!("../../inputs/day03");

fn part1(input: &str) -> Result<u64> {
    let mut s = &input[..];
    let mut res = 0;

    while !s.is_empty() {
        let Some(idx) = s.find("mul(") else {
            break;
        };
        let fst = idx + 4;
        s = &s[fst..];
        let Some(comma) = s.find(',') else {
            continue;
        };
        if comma > 3 {
            continue;
        }
        let a = &s[..comma];
        let Ok(a) = u64::from_str_radix(a, 10) else {
            continue;
        };
        s = &s[comma + 1..];
        let Some(end) = s.find(')') else {
            continue;
        };
        if end > 3 {
            continue;
        }
        let b = &s[..end];
        let Ok(b) = u64::from_str_radix(b, 10) else {
            continue;
        };
        s = &s[end + 1..];
        res += a * b;
    }
    Ok(res)
}

fn part2(input: &str) -> Result<u64> {
    let mut s = &input[..];
    let mut enabled = true;
    let mut res = 0;

    while !s.is_empty() {
        if enabled {
            let idx = match (s.find("mul("), s.find("don't()")) {
                (Some(a), Some(b)) if a < b => a,
                (Some(a), Some(b)) if b < a => {
                    enabled = false;
                    s = &s[b + 7..];
                    continue;
                }
                _ => break,
            };
            let Some(idx) = s.find("mul(") else {
                break;
            };
            let fst = idx + 4;
            s = &s[fst..];
            let Some(comma) = s.find(',') else {
                continue;
            };
            if comma > 3 {
                continue;
            }
            let a = &s[..comma];
            let Ok(a) = u64::from_str_radix(a, 10) else {
                continue;
            };
            s = &s[comma + 1..];
            let Some(end) = s.find(')') else {
                continue;
            };
            if end > 3 {
                continue;
            }
            let b = &s[..end];
            let Ok(b) = u64::from_str_radix(b, 10) else {
                continue;
            };
            s = &s[end + 1..];
            res += a * b;
        } else {
            let Some(idx) = s.find("do()") else {
                break;
            };
            enabled = true;
            s = &s[idx + 4..];
        }
    }
    Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = "EXAMPLE 1 HERE";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 0,
    part2 => (EX_INPUT) 0
}
