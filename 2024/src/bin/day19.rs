use std::collections::HashMap;

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    to_lines,
};

static INPUT: &str = include_str!("../../inputs/day19");

fn find_available_patterns<'a>(
    patterns: &[&'a str],
    wants: impl Iterator<Item = &'a str>,
) -> usize {
    fn is_possible<'a>(
        target: &'a str,
        patterns: &[&'a str],
        cache: &mut HashMap<&'a str, bool>,
    ) -> bool {
        if target.is_empty() {
            true
        } else if let Some(&res) = cache.get(&target) {
            res
        } else {
            let res = patterns
                .iter()
                .filter(|&pat| target.starts_with(pat))
                .any(|pat| is_possible(&target[pat.len()..], patterns, cache));
            cache.insert(target, res);
            res
        }
    }

    let mut cache = HashMap::new();
    wants
        .filter(|want| is_possible(want, patterns, &mut cache))
        .count()
}

fn part1(input: &str) -> Result<usize> {
    let (patterns, wants) = input
        .trim()
        .split_once("\n\n")
        .ok_or_eyre("invalid format")?;

    let patterns: Vec<_> = patterns.split(", ").collect();

    Ok(find_available_patterns(&patterns, to_lines(wants)))
}

fn find_all_available_patterns<'a>(
    patterns: &[&'a str],
    wants: impl Iterator<Item = &'a str>,
) -> usize {
    fn inner<'a>(
        want: &'a str,
        patterns: &[&'a str],
        cache: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if want.is_empty() {
            1
        } else if let Some(&res) = cache.get(&want) {
            res
        } else {
            let res = patterns
                .iter()
                .map(|pat| {
                    // somehow this is faster than doing filter + map or filter_map
                    // also, using strip_prefix is slower
                    #[allow(clippy::manual_strip)]
                    if want.starts_with(pat) {
                        inner(&want[pat.len()..], patterns, cache)
                    } else {
                        0
                    }
                })
                .sum();
            cache.insert(want, res);
            res
        }
    }

    let mut cache = HashMap::new();
    wants.map(|want| inner(want, patterns, &mut cache)).sum()
}

fn part2(input: &str) -> Result<usize> {
    let (patterns, wants) = input
        .trim()
        .split_once("\n\n")
        .ok_or_eyre("invalid format")?;

    let patterns: Vec<_> = patterns.split(", ").collect();

    Ok(find_all_available_patterns(&patterns, to_lines(wants)))
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
