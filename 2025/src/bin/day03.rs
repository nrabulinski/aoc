use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day03");

fn find_largest(i: impl IntoIterator<Item = u8>) -> (usize, u8) {
    i.into_iter().enumerate().fold(
        (usize::MAX, 0),
        |prev @ (_, max), curr @ (_, v)| {
            if v > max { curr } else { prev }
        },
    )
}

fn part1(input: &str) -> Result<i64> {
    let result = to_lines(input)
        .map(|line| {
            let chars = line.as_bytes();
            let (idx, front) = find_largest(chars.iter().take(chars.len() - 1).copied());
            let (_, back) = find_largest(chars.iter().skip(idx + 1).copied());
            let front = (front - b'0') as i64;
            let back = (back - b'0') as i64;
            front * 10 + back
        })
        .sum();

    Ok(result)
}

fn part2(input: &str) -> Result<i64> {
    const DIGITS: usize = 12;

    let result = to_lines(input)
        .map(|line| {
            let chars = line.as_bytes();

            (0..DIGITS)
                .scan(0, |to_skip, i| {
                    let need_to_leave = DIGITS - i - 1;
                    let digits_left = chars.len() - *to_skip;

                    let can_take = digits_left - need_to_leave;

                    let (idx, d) =
                        find_largest(chars.iter().skip(*to_skip).take(can_take).copied());
                    *to_skip += idx + 1;

                    Some((d - b'0') as i64)
                })
                .reduce(|acc, curr| acc * 10 + curr)
                .unwrap()
        })
        .sum();

    Ok(result)
}

#[allow(dead_code)]
static EX_INPUT: &str = "
987654321111111
811111111111119
234234234234278
818181911112111
";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 357,
    part2 => (EX_INPUT) 3121910778619
}
