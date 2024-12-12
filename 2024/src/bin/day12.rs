use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    grid::{Grid, Point},
};

static INPUT: &str = include_str!("../../inputs/day12");

fn part1(input: &str) -> Result<usize> {
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    let mut visited = HashSet::new();
    let mut plots = HashMap::new();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let start = (x, y);
            if visited.contains(&start) {
                continue;
            }
            let curr = grid[start];

            let mut queue = Vec::new();
            queue.push(start);

            while let Some(pos) = queue.pop() {
                if !visited.insert(pos) {
                    continue;
                }
                let entry = plots.entry(start).or_insert((0, 0));
                entry.0 += 1usize;
                entry.1 += 4 - grid
                    .orthogonal_pos(pos)
                    .filter(|&cell| grid[cell] == curr)
                    .count();
                queue.extend(
                    grid.orthogonal_pos(pos)
                        .filter(|cell| grid[*cell] == curr && !visited.contains(cell)),
                );
            }
        }
    }

    Ok(plots
        .into_values()
        .map(|(area, perimeter)| area * perimeter)
        .sum())
}

fn calc_perim(edges: &[(Point, Point)]) -> usize {
    let mut above: Vec<_> = edges
        .iter()
        .filter_map(|&(d, pos)| (d == (0, -1)).then_some(pos))
        .collect();
    above.sort_unstable_by(|a, b| match a.1.cmp(&b.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        res => res,
    });
    let mut below: Vec<_> = edges
        .iter()
        .filter_map(|&(d, pos)| (d == (0, 1)).then_some(pos))
        .collect();
    below.sort_unstable_by(|a, b| match a.1.cmp(&b.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        res => res,
    });

    let horizontal = above
        .windows(2)
        .chain(below.windows(2))
        .filter(|win| win[0].1 != win[1].1 || win[1].0 - win[0].0 != 1)
        .count()
        + 2;

    let mut left: Vec<_> = edges
        .iter()
        .filter_map(|&(d, pos)| (d == (-1, 0)).then_some(pos))
        .collect();
    left.sort_unstable_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Equal => a.1.cmp(&b.1),
        res => res,
    });
    let mut right: Vec<_> = edges
        .iter()
        .filter_map(|&(d, pos)| (d == (1, 0)).then_some(pos))
        .collect();
    right.sort_unstable_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Equal => a.1.cmp(&b.1),
        res => res,
    });

    let vertical = left
        .windows(2)
        .chain(right.windows(2))
        .filter(|win| win[0].0 != win[1].0 || win[1].1 - win[0].1 != 1)
        .count()
        + 2;

    horizontal + vertical
}

fn part2(input: &str) -> Result<usize> {
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    let mut visited = HashSet::new();
    let mut plots = HashMap::new();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let start = (x, y);
            if visited.contains(&start) {
                continue;
            }
            let curr = grid[start];

            let mut queue = Vec::new();
            queue.push(start);

            while let Some(pos) = queue.pop() {
                if !visited.insert(pos) {
                    continue;
                }
                let entry = plots.entry(start).or_insert((0, Vec::new()));
                entry.0 += 1usize;
                entry.1.extend(
                    [-1, 1]
                        .into_iter()
                        .map(|dy| ((0, dy), (pos.0, pos.1 + dy)))
                        .chain([-1, 1].into_iter().map(|dx| ((dx, 0), (pos.0 + dx, pos.1))))
                        .filter(|&(_, cell)| !grid.is_valid_pos(cell) || grid[cell] != curr),
                );

                queue.extend(
                    grid.orthogonal_pos(pos)
                        .filter(|cell| grid[*cell] == curr && !visited.contains(cell)),
                );
            }
        }
    }

    Ok(plots
        .into_values()
        .map(|(area, edges)| area * calc_perim(&edges))
        .sum())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;

#[allow(dead_code)]
static E: &str = r#"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 1930,
    part2 => (EX_INPUT) 1206
}
