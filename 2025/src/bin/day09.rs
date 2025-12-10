use aoc_lib::{aoc, color_eyre::eyre::Result, grid::Point, to_lines};

static INPUT: &str = include_str!("../../inputs/day09");

fn part1(input: &str) -> Result<u64> {
    let points: Vec<Point> = to_lines(input)
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            (x, y)
        })
        .collect();

    let result = points
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| points.iter().skip(i + 1).map(move |&b| (a, b)))
        .map(|(a, b)| {
            let w = a.0.abs_diff(b.0) + 1;
            let h = a.1.abs_diff(b.1) + 1;
            w * h
        })
        .max()
        .unwrap();

    Ok(result)
}

#[allow(dead_code)]
static EX_INPUT: &str = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 50
}
