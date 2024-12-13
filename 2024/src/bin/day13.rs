use aoc_lib::{aoc, color_eyre::eyre::Result, grid::PointExt, to_lines};

static INPUT: &str = include_str!("../../inputs/day13");

fn find_optimal<'s>(mut input: impl Iterator<Item = &'s str>) -> Option<usize> {
    fn parse_line(s: &str) -> (i64, i64) {
        let (_, s) = s.split_once(": ").unwrap();
        let (x, y) = s.split_once(", ").unwrap();
        (x[2..].parse().unwrap(), y[2..].parse().unwrap())
    }
    let a = parse_line(input.next().unwrap());
    let b = parse_line(input.next().unwrap());
    let prize = parse_line(input.next().unwrap()).map(|n| n + 10000000000000);

    // px = a * ax + b * bx
    // py = a * ay + b * by
    //
    // b * bx = px - a * ax
    // b = (px - a * ax) / bx
    //
    // py = a * ay + (px - a * ax) / bx * by
    // py * bx = a * ay * bx + (px - a * ax) * by
    // py * bx = a * ay * bx + px * by - a * ax * by
    // py * bx - px * by = a * ay * bx - a * ax * by
    // py * bx - px * by = a(ay * bx - ax * by)
    // (py * bx - px * by) / (ay * bx - ax * by) = a
    let numerator = (prize.1 * b.0 - prize.0 * b.1) as f64;
    let denominator = (a.1 * b.0 - a.0 * b.1) as f64;
    let maybe_a_presses = numerator / denominator;
    if maybe_a_presses.floor() != maybe_a_presses {
        return None;
    }

    let a_presses = maybe_a_presses;
    let b_presses = (prize.0 as f64 - a_presses * a.0 as f64) / b.0 as f64;
    if b_presses.floor() != b_presses {
        return None;
    }

    let a_presses = a_presses as i64;
    let b_presses = b_presses as i64;
    let cost = a_presses * 3 + b_presses;
    Some(cost as usize)
}

fn find_capped<'s>(mut input: impl Iterator<Item = &'s str>) -> Option<usize> {
    fn parse_line(s: &str) -> (i64, i64) {
        let (_, s) = s.split_once(": ").unwrap();
        let (x, y) = s.split_once(", ").unwrap();
        (x[2..].parse().unwrap(), y[2..].parse().unwrap())
    }

    let a = parse_line(input.next().unwrap());
    let b = parse_line(input.next().unwrap());
    let prize = parse_line(input.next().unwrap());

    let mut i = (0..100).filter_map(|a_presses| {
        let x = a_presses * a.0;
        let y = a_presses * a.1;

        let x_remaining = prize.0 - x;
        let y_remaining = prize.1 - y;

        match (x_remaining % b.0, y_remaining % b.1) {
            (0, 0) => {
                let maybe_b_presses = x_remaining / b.0;
                if y_remaining / b.1 != maybe_b_presses {
                    None
                } else {
                    let cost = 3 * a_presses as usize;
                    Some(cost + maybe_b_presses as usize)
                }
            }
            _ => None,
        }
    });

    let fst = i.next()?;
    let Some(snd) = i.next() else {
        return Some(fst);
    };

    if fst < snd {
        Some(fst)
    } else {
        Some(i.next_back().unwrap_or(snd))
    }
}

fn part1(input: &str) -> Result<usize> {
    Ok(input
        .trim()
        .split("\n\n")
        .filter_map(|lines| find_capped(to_lines(lines)))
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    Ok(input
        .trim()
        .split("\n\n")
        .filter_map(|lines| find_optimal(to_lines(lines)))
        .sum())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 480,
    part2 => (EX_INPUT) 875318608908
}
