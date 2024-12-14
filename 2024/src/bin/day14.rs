use std::collections::HashSet;

use aoc_lib::{
    aoc,
    color_eyre::eyre::Result,
    grid::{Point, PointExt},
    to_lines,
};

static INPUT: &str = include_str!("../../inputs/day14");

#[cfg(test)]
const WIDTH: i64 = 11;
#[cfg(test)]
const HEIGHT: i64 = 7;
#[cfg(not(test))]
const WIDTH: i64 = 101;
#[cfg(not(test))]
const HEIGHT: i64 = 103;

fn parse_line(line: &str) -> (Point, Point) {
    fn parse(s: &str) -> Point {
        let (x, y) = s[2..].split_once(",").unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }
    let (pos, vel) = line.split_once(" ").unwrap();
    (parse(pos), parse(vel))
}

fn pos_after_steps((pos, vel): (Point, Point), steps: i64) -> Point {
    let (x, y) = vel.map(|c| c * steps).add(&pos);
    (x.rem_euclid(WIDTH), y.rem_euclid(HEIGHT))
}

fn part1(input: &str) -> Result<i64> {
    let (a, b, c, d) = to_lines(input)
        .map(|line| pos_after_steps(parse_line(line), 100))
        .fold((0, 0, 0, 0), |mut acc, pos| {
            if pos.0 < WIDTH / 2 {
                if pos.1 < HEIGHT / 2 {
                    acc.0 += 1;
                } else if pos.1 > HEIGHT / 2 {
                    acc.2 += 1;
                }
            } else if pos.0 > WIDTH / 2 {
                if pos.1 < HEIGHT / 2 {
                    acc.1 += 1;
                } else if pos.1 > HEIGHT / 2 {
                    acc.3 += 1;
                }
            }
            acc
        });
    Ok(a * b * c * d)
}

fn part2(input: &str) -> Result<i64> {
    let robots: Vec<_> = to_lines(input).map(parse_line).collect();

    Ok((1..)
        .find(|&steps| {
            let positions: HashSet<_> = robots
                .iter()
                .map(|&state| pos_after_steps(state, steps))
                .collect();

            positions.len() == robots.len()
        })
        .unwrap())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 12,
    part2 => (EX_INPUT) 1
}
