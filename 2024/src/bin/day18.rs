use std::collections::HashSet;

use aoc_lib::{
    algo::dijkstra,
    aoc,
    color_eyre::eyre::Result,
    grid::{Grid, Point},
    to_lines,
};

static INPUT: &str = include_str!("../../inputs/day18");

const WIDTH: i64 = 70;
const HEIGHT: i64 = 70;

fn part1(input: &str) -> Result<i64> {
    let obstacles: HashSet<Point> = to_lines(input)
        .take(1024)
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();

    let is_valid_pos =
        |(x, y)| x >= 0 && x <= WIDTH && y >= 0 && y <= HEIGHT && !obstacles.contains(&(x, y));

    let (d, _) = dijkstra((0, 0), |&(x, y)| {
        let h = [-1, 1].into_iter().map(move |dx| (x + dx, y));
        let v = [-1, 1].into_iter().map(move |dy| (x, y + dy));

        h.chain(v)
            .filter(|&pos| is_valid_pos(pos))
            .map(|pos| (pos, 1))
    });

    Ok(*d.get(&(WIDTH, HEIGHT)).unwrap())
}

fn part2(input: &str) -> Result<String> {
    let mut iter = to_lines(input).map(|line| {
        let (l, r) = line.split_once(',').unwrap();
        (l.parse().unwrap(), r.parse().unwrap())
    });

    let mut obstacles: HashSet<Point> = iter.by_ref().take(1024).collect();

    for point in iter {
        if !obstacles.insert(point) {
            continue;
        }

        let is_valid_pos =
            |(x, y)| x >= 0 && x <= WIDTH && y >= 0 && y <= HEIGHT && !obstacles.contains(&(x, y));

        let (d, _) = dijkstra((0, 0), |&(x, y)| {
            let h = [-1, 1].into_iter().map(move |dx| (x + dx, y));
            let v = [-1, 1].into_iter().map(move |dy| (x, y + dy));

            h.chain(v)
                .filter(|&pos| is_valid_pos(pos))
                .map(|pos| (pos, 1))
        });

        if !d.contains_key(&(WIDTH, HEIGHT)) {
            return Ok(format!("{},{}", point.0, point.1));
        }
    }

    unreachable!()
}

#[allow(dead_code)]
static EX_INPUT: &str = "EXAMPLE 1 HERE";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 0,
    part2 => (EX_INPUT) ""
}
