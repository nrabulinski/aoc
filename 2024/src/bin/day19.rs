use std::collections::HashMap;

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    to_lines,
};

static INPUT: &str = include_str!("../../inputs/day19");

fn find_available_patterns(patterns: &[&str], wants: &[&str]) -> usize {
    fn is_possible(target: &str, patterns: &[&str]) -> bool {
        if target.is_empty() {
            return true;
        }

        patterns.iter().copied().any(|pat| {
            if !target.starts_with(pat) {
                return false;
            }
            is_possible(&target[pat.len()..], patterns)
        })
    }

    wants
        .iter()
        .copied()
        .filter(|&want| is_possible(want, patterns))
        .count()
}

fn part1(input: &str) -> Result<usize> {
    let (patterns, wants) = input
        .trim()
        .split_once("\n\n")
        .ok_or_eyre("invalid format")?;

    let patterns: Vec<_> = patterns.split(", ").collect();
    let wants: Vec<_> = to_lines(wants).collect();

    Ok(find_available_patterns(&patterns, &wants))
}

fn find_all_available_patterns(patterns: &[&str], wants: &[&str]) -> usize {
    fn inner<'a>(
        want: &'a str,
        patterns: &[&'a str],
        cache: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if want.is_empty() {
            return 1;
        }

        if let Some(&res) = cache.get(&want) {
            return res;
        }
        let res = patterns
            .iter()
            .copied()
            .map(|pat| {
                if !want.starts_with(pat) {
                    return 0;
                }

                inner(&want[pat.len()..], patterns, cache)
            })
            .sum();
        cache.insert(want, res);
        res
    }

    let mut cache = HashMap::new();
    wants
        .iter()
        .copied()
        .map(|want| inner(want, patterns, &mut cache))
        .sum()
}

fn part2(input: &str) -> Result<usize> {
    let (patterns, wants) = input
        .trim()
        .split_once("\n\n")
        .ok_or_eyre("invalid format")?;

    let patterns: Vec<_> = patterns.split(", ").collect();
    let wants: Vec<_> = to_lines(wants).collect();

    Ok(find_all_available_patterns(&patterns, &wants))
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 6,
    part2 => (EX_INPUT) 16
}
