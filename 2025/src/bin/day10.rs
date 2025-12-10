use std::collections::{HashMap, HashSet};

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day10");

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let mut line = line.split_ascii_whitespace();
    let lights = line.next().unwrap();
    let lights = &lights.as_bytes()[1..lights.len() - 1];
    // lights is a bitmap of the target light pattern
    // e.g. for [.#.##.] it returns 0b011010
    let lights: u64 = lights
        .iter()
        .enumerate()
        .map(|(i, &c)| match c {
            b'.' => 0u64,
            b'#' => 1u64 << i,
            _ => unreachable!(),
        })
        .sum();
    let _joltage = line.next_back().unwrap();
    // each button is a bitmask of which lights it toggles
    // e.g. button (1,3) will be 0b1010
    let buttons: Vec<u64> = line
        .map(|s| {
            let s = &s[1..s.len() - 1];
            s.split(',')
                .map(|n| n.parse::<u8>().unwrap())
                .map(|shift| 1u64 << shift)
                .sum()
        })
        .collect();
    (lights, buttons)
}

fn least_button_presses((target, buttons): (u64, Vec<u64>)) -> u64 {
    fn inner(
        cache: &mut HashMap<u64, u64>,
        visiting: &mut HashSet<u64>,
        state: u64,
        buttons: &[u64],
    ) -> Option<u64> {
        if state == 0 {
            return Some(0);
        }

        if let Some(res) = cache.get(&state) {
            return Some(*res);
        }
        if !visiting.insert(state) {
            return None;
        }

        let res = buttons
            .iter()
            .filter_map(|mask| inner(cache, visiting, state ^ mask, buttons))
            .min()?
            + 1;

        visiting.remove(&state);
        cache.insert(state, res);

        Some(res)
    }

    // current button state -> how many presses to all turned off
    let mut cache = HashMap::new();
    // remember which patterns are in the recursive chain to prevent cycles
    let mut visiting = HashSet::new();

    inner(&mut cache, &mut visiting, target, &buttons).unwrap()
}

fn part1(input: &str) -> Result<u64> {
    let result = to_lines(input)
        .map(parse_line)
        .map(least_button_presses)
        .sum();
    Ok(result)
}

#[allow(dead_code)]
static EX_INPUT: &str = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 7
}
