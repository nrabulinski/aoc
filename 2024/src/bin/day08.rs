use std::collections::HashSet;

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    grid::{Grid, Point, PointExt},
};

static INPUT: &str = include_str!("../../inputs/day08");

fn get_antinode_pos(grid: &Grid<'_>, k: Point, l: Point) -> Option<Point> {
    let d = (k.0 - l.0, k.1 - l.1);
    let anti = k.add(&d);

    grid.is_valid_pos(anti).then_some(anti)
}

fn part1(input: &str) -> Result<usize> {
    let input = input.trim();
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    let points: Vec<_> = input
        .as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| {
            if c == b'.' {
                None
            } else {
                grid.idx_to_pos(i).map(|pos| (pos, c))
            }
        })
        .collect();

    let mut res = HashSet::new();

    for i in 0..points.len() {
        let a = points[i];
        for j in (i + 1)..points.len() {
            let b = points[j];
            if a.1 != b.1 {
                continue;
            }
            if let Some(pos) = get_antinode_pos(&grid, a.0, b.0) {
                res.insert(pos);
            }
            if let Some(pos) = get_antinode_pos(&grid, b.0, a.0) {
                res.insert(pos);
            }
        }
    }

    Ok(res.len())
}

fn part2(input: &str) -> Result<usize> {
    let input = input.trim();
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    let points: Vec<_> = input
        .as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| {
            if c == b'.' {
                None
            } else {
                grid.idx_to_pos(i).map(|pos| (pos, c))
            }
        })
        .collect();

    let mut res = HashSet::new();

    for i in 0..points.len() {
        let a = points[i];
        for j in (i + 1)..points.len() {
            let b = points[j];
            if a.1 != b.1 {
                continue;
            }
            res.insert(a.0);
            res.insert(b.0);
            let mut last = (a.0, b.0);
            loop {
                if let Some(pos) = get_antinode_pos(&grid, last.0, last.1) {
                    res.insert(pos);
                    last = (pos, last.0);
                } else {
                    break;
                }
            }
            last = (b.0, a.0);
            loop {
                if let Some(pos) = get_antinode_pos(&grid, last.0, last.1) {
                    res.insert(pos);
                    last = (pos, last.0);
                } else {
                    break;
                }
            }
        }
    }

    Ok(res.len())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 14,
    part2 => (EX_INPUT) 34
}
