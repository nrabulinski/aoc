use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    grid::Grid,
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
                let entry = plots.entry(start).or_insert((
                    0,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ));
                entry.0 += 1usize;

                let above = (pos.0, pos.1 - 1);
                if !grid.is_valid_pos(above) || grid[above] != curr {
                    entry.1.push(above);
                }
                let below = (pos.0, pos.1 + 1);
                if !grid.is_valid_pos(below) || grid[below] != curr {
                    entry.2.push(below);
                }
                let left = (pos.0 - 1, pos.1);
                if !grid.is_valid_pos(left) || grid[left] != curr {
                    entry.3.push(left);
                }
                let right = (pos.0 + 1, pos.1);
                if !grid.is_valid_pos(right) || grid[right] != curr {
                    entry.4.push(right);
                }

                queue.extend(
                    grid.orthogonal_pos(pos)
                        .filter(|cell| grid[*cell] == curr && !visited.contains(cell)),
                );
            }
        }
    }

    Ok(plots
        .into_values()
        .map(|(area, mut a, mut b, mut l, mut r)| {
            a.sort_unstable_by(|a, b| match a.1.cmp(&b.1) {
                Ordering::Equal => a.0.cmp(&b.0),
                res => res,
            });
            b.sort_unstable_by(|a, b| match a.1.cmp(&b.1) {
                Ordering::Equal => a.0.cmp(&b.0),
                res => res,
            });
            l.sort_unstable_by(|a, b| match a.0.cmp(&b.0) {
                Ordering::Equal => a.1.cmp(&b.1),
                res => res,
            });
            r.sort_unstable_by(|a, b| match a.0.cmp(&b.0) {
                Ordering::Equal => a.1.cmp(&b.1),
                res => res,
            });
            let a = a
                .windows(2)
                .filter(|win| win[0].1 != win[1].1 || win[1].0 - win[0].0 != 1)
                .count()
                + if !a.is_empty() { 1 } else { 0 };
            let b = b
                .windows(2)
                .filter(|win| win[0].1 != win[1].1 || win[1].0 - win[0].0 != 1)
                .count()
                + if !b.is_empty() { 1 } else { 0 };
            let l = l
                .windows(2)
                .filter(|win| win[0].0 != win[1].0 || win[1].1 - win[0].1 != 1)
                .count()
                + if !l.is_empty() { 1 } else { 0 };
            let r = r
                .windows(2)
                .filter(|win| win[0].0 != win[1].0 || win[1].1 - win[0].1 != 1)
                .count()
                + if !r.is_empty() { 1 } else { 0 };

            area * (a + b + l + r)
        })
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
