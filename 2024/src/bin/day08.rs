use std::{collections::HashSet, iter::once};

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    grid::{Grid, Point, PointExt},
};

static INPUT: &str = include_str!("../../inputs/day08");

fn parse(input: &str) -> Result<(Grid<'_>, Vec<(Point, u8)>)> {
    let input = input.trim();
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    let points = input
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

    Ok((grid, points))
}

fn get_antinode_pos(grid: &Grid<'_>, k: Point, l: Point) -> impl Iterator<Item = Point> {
    std::iter::successors(Some((k, l)), |&(k, l)| {
        let d = (k.0 - l.0, k.1 - l.1);
        let next = k.add(&d);
        grid.is_valid_pos(next).then_some((next, k))
    })
    .skip(1)
    .map(|(pos, _)| pos)
}

fn part1(input: &str) -> Result<usize> {
    let (grid, ref points) = parse(input)?;

    let res: HashSet<_> = (0..points.len())
        .flat_map(|i| ((i + 1)..points.len()).map(move |j| (points[i], points[j])))
        .filter_map(|(a, b)| (a.1 == b.1).then_some((a.0, b.0)))
        .flat_map(|(a, b)| {
            [
                get_antinode_pos(&grid, a, b).next(),
                get_antinode_pos(&grid, b, a).next(),
            ]
            .into_iter()
            .flatten()
        })
        .collect();

    Ok(res.len())
}

fn part2(input: &str) -> Result<usize> {
    let (grid, ref points) = parse(input)?;

    let res: HashSet<_> = (0..points.len())
        .flat_map(|i| ((i + 1)..points.len()).map(move |j| (points[i], points[j])))
        .filter_map(|(a, b)| (a.1 == b.1).then_some((a.0, b.0)))
        .flat_map(|(a, b)| {
            once(a)
                .chain(once(b))
                .chain(get_antinode_pos(&grid, a, b))
                .chain(get_antinode_pos(&grid, b, a))
        })
        .collect();

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
