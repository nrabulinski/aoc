use aoc_lib::{aoc, color_eyre::eyre::Result, iter::IterExt, to_lines};

static INPUT: &str = include_str!("../../inputs/day02");

fn check_report(report: impl Iterator<Item = i64>) -> bool {
    let mut iter = report.peekable2();
    let increasing = {
        let (Some(fst), Some(snd)) = iter.peek_pair() else {
            return true;
        };
        snd > fst
    };
    let mut windows = std::iter::from_fn(move || {
        let a = iter.next()?;
        let b = iter.peek()?;
        Some((a, *b))
    });
    windows.all(|(a, b)| {
        let d = a.abs_diff(b);
        let inc = b > a;
        d > 0 && d < 4 && inc == increasing
    })
}

fn part1(input: &str) -> Result<usize> {
    Ok(to_lines(input)
        .map(|line| line.split_ascii_whitespace().map(|x| x.parse().unwrap()))
        .map(check_report)
        .filter(|&x| x)
        .count())
}

fn check_maybe_report(report: impl Iterator<Item = i64>) -> bool {
    let vals: Vec<_> = report.collect();

    if check_report(vals.iter().copied()) {
        return true;
    }

    for to_remove in 0..vals.len() {
        let (left, right) = vals.split_at(to_remove);
        let iter = left.iter().copied().chain((&right[1..]).iter().copied());
        if check_report(iter) {
            return true;
        }
    }

    false
}

fn part2(input: &str) -> Result<usize> {
    Ok(to_lines(input)
        .map(|line| line.split_ascii_whitespace().map(|x| x.parse().unwrap()))
        .map(check_maybe_report)
        .filter(|&x| x)
        .count())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 2,
    part2 => (EX_INPUT) 4
}
