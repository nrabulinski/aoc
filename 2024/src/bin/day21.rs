use std::collections::HashMap;

use aoc_lib::{aoc, color_eyre::eyre::Result, grid::Point, to_lines};

static INPUT: &str = include_str!("../../inputs/day21");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Action {
    Move(Point),
    Press(i64),
}

fn code_char_keypad_pos(c: u8) -> Point {
    match c {
        b'7'..=b'9' => ((c - b'7').into(), 3),
        b'4'..=b'6' => ((c - b'4').into(), 2),
        b'1'..=b'3' => ((c - b'1').into(), 1),
        b'0' => (1, 0),
        b'A' => (2, 0),
        _ => unreachable!(),
    }
}

fn dir_keypad_pos(dir: Point) -> Point {
    match dir {
        (0, 0) => (2, 1), // A button,
        (-1, 0) => (0, 0),
        (0, 1) => (1, 1),
        (0, -1) => (1, 0),
        (1, 0) => (2, 0),
        _ => unreachable!(),
    }
}

fn first(dirs: &[Action], layers: usize, cache: &mut HashMap<(Vec<Action>, usize), i64>) -> i64 {
    dirs.iter()
        .scan(dir_keypad_pos((0, 0)), |pos, &dir| {
            let mut actions = Vec::new();
            let mut move_pos = |pos: &mut Point, to_reach: Point, times_to_press| {
                let d = (to_reach.0 - pos.0, to_reach.1 - pos.1);
                if pos.0 == 0 && to_reach.1 == 1 {
                    // prioritize going right over up if we were to hover over the gap
                    assert_ne!(d.0, 0);
                    assert_ne!(d.1, 0);
                    actions.push(Action::Move((d.0, 0)));
                    actions.push(Action::Move((0, d.1)));
                } else if pos.1 == 1 && to_reach.0 == 0 {
                    // prioritize going down over left if we were to hover over the gap
                    assert_ne!(d.0, 0);
                    assert_ne!(d.1, 0);
                    actions.push(Action::Move((0, d.1)));
                    actions.push(Action::Move((d.0, 0)));
                } else {
                    if d != (0, 0) {
                        actions.push(Action::Move(d));
                    }
                }
                actions.push(Action::Press(times_to_press));
                *pos = to_reach;
            };
            match dir {
                Action::Press(times) => {
                    let to_reach = dir_keypad_pos((0, 0));
                    move_pos(pos, to_reach, times);
                }
                Action::Move(dir) => {
                    let to_reach_hor = if dir.0 < 0 {
                        Some(dir_keypad_pos((-1, 0)))
                    } else if dir.0 > 0 {
                        Some(dir_keypad_pos((1, 0)))
                    } else {
                        None
                    }
                    .map(|to_reach| (to_reach, dir.0.abs()));

                    let to_reach_ver = if dir.1 < 0 {
                        Some(dir_keypad_pos((0, -1)))
                    } else if dir.1 > 0 {
                        Some(dir_keypad_pos((0, 1)))
                    } else {
                        None
                    }
                    .map(|to_reach| (to_reach, dir.1.abs()));

                    // let ops = [to_reach_hor, to_reach_ver];
                    let ops = if to_reach_hor.is_some_and(|(hor, _)| {
                        to_reach_ver.is_none_or(|(ver, _)| {
                            let d1 = (hor.0 - pos.0, hor.1 - pos.1);
                            let d2 = (ver.0 - pos.0, ver.1 - pos.1);
                            d1 < d2
                        })
                    }) {
                        [to_reach_hor, to_reach_ver]
                    } else {
                        [to_reach_ver, to_reach_hor]
                    };

                    for (to_reach, times) in ops.into_iter().flatten() {
                        move_pos(pos, to_reach, times);
                    }
                }
            }
            let x = (actions, layers);
            if let Some(&res) = cache.get(&x) {
                return Some(res);
            }
            let (actions, _) = x;
            let res = if layers > 0 {
                first(&actions, layers - 1, cache)
            } else {
                actions.iter().fold(0, |acc, &curr| {
                    acc + match curr {
                        Action::Move((dx, dy)) => dx.abs() + dy.abs(),
                        Action::Press(times) => times,
                    }
                })
            };
            cache.insert((actions, layers), res);
            Some(res)
        })
        .sum()
}

fn find_complexity(
    line: &str,
    layers: usize,
    cache: &mut HashMap<(Vec<Action>, usize), i64>,
) -> i64 {
    let val: i64 = line[..3].parse().unwrap();
    line.bytes()
        .map(code_char_keypad_pos)
        .scan(code_char_keypad_pos(b'A'), |pos, to_reach| {
            let mut res = Vec::new();
            let d = (to_reach.0 - pos.0, to_reach.1 - pos.1);
            if pos.1 == 0 && to_reach.0 == 0 {
                // prioritize going up over left if we were to hover over the gap
                if d.1 != 0 {
                    res.push(Action::Move((0, d.1)));
                }
                if d.0 != 0 {
                    res.push(Action::Move((d.0, 0)));
                }
            } else if pos.0 == 0 && to_reach.1 == 0 {
                // prioritize going right over down if we were to hover over the gap
                if d.0 != 0 {
                    res.push(Action::Move((d.0, 0)));
                }
                if d.1 != 0 {
                    res.push(Action::Move((0, d.1)));
                }
            } else {
                res.push(Action::Move(d));
            }
            res.push(Action::Press(1));
            *pos = to_reach;
            Some(first(&res, layers - 1, cache))
        })
        .sum::<i64>()
        * val
}

fn part1(input: &str) -> Result<i64> {
    let mut cache = HashMap::new();
    Ok(to_lines(input)
        .map(|line| find_complexity(line, 2, &mut cache))
        .sum())
}

fn part2(input: &str) -> Result<i64> {
    let mut cache = HashMap::new();
    Ok(to_lines(input)
        .map(|line| find_complexity(line, 25, &mut cache))
        .sum())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
029A
980A
179A
456A
379A
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 126384,
    part2 => (EX_INPUT) 154115708116294
}
