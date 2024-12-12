use std::collections::HashSet;

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    grid::{Grid, Point, PointExt},
};

static INPUT: &str = include_str!("../../inputs/day12");

fn part1(input: &str) -> Result<usize> {
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    let mut visited = HashSet::new();
    let mut plots = Vec::new();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let start = (x, y);
            if visited.contains(&start) {
                continue;
            }
            let curr = grid[start];
            let mut state = (0, 0);

            let mut queue = Vec::new();
            queue.push(start);

            while let Some(pos) = queue.pop() {
                if !visited.insert(pos) {
                    continue;
                }
                state.0 += 1usize;
                state.1 += 4 - grid
                    .orthogonal_pos(pos)
                    .filter(|&cell| grid[cell] == curr)
                    .count();
                queue.extend(
                    grid.orthogonal_pos(pos)
                        .filter(|cell| grid[*cell] == curr && !visited.contains(cell)),
                );
            }
            plots.push(state);
        }
    }

    Ok(plots
        .into_iter()
        .map(|(area, perimeter)| area * perimeter)
        .sum())
}

fn calc_perim(mut edges: Vec<(Point, Point)>) -> usize {
    // Order edges by the type first (is it above a cell, below, left of it, or right of it),
    // then, if it's a horizontal edge (above or below), by y component first, then x,
    // and if it's a vertical edge, by x component first, then y.
    edges.sort_unstable_by_key(|&(d, pos)| (d, if d.0 == 0 { (pos.1, pos.0) } else { pos }));
    edges
        .windows(2)
        .filter(|w| {
            if w[0].0 != w[1].0 {
                return false;
            }
            let d = (w[0].0.1, w[0].0.0).map(i64::abs);
            w[0].1.add(&d) != w[1].1
        })
        .count()
        + 4
}

fn part2(input: &str) -> Result<usize> {
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    let mut visited = HashSet::new();
    let mut plots = Vec::new();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let start = (x, y);
            if visited.contains(&start) {
                continue;
            }
            let curr = grid[start];
            let mut state = (0, Vec::new());

            let mut queue = Vec::new();
            queue.push(start);

            while let Some(pos) = queue.pop() {
                if !visited.insert(pos) {
                    continue;
                }
                state.0 += 1usize;
                state.1.extend(
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

            plots.push(state);
        }
    }

    Ok(plots
        .into_iter()
        .map(|(area, edges)| area * calc_perim(edges))
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
