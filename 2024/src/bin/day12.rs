use std::collections::{HashMap, HashSet};

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
    // Only find horizontal edges, i.e. those that are above or below a cell
    let mut hor: Vec<_> = edges.iter().filter(|&(d, _)| matches!(d, (0, _))).collect();
    // Sort them first by the difference, then by y, then by x.
    // This means we will get first all the edges that were above, sorted by their y component, and finally by x,
    // followed by edges that were below.
    // E.g. for this shape:
    // XX
    // X <- this cell is counted twice, it's both above and below an X
    // XX
    // Where the horizontal edges are (0, -1), (1, -1), (0, 3), (1, 3), and (1, 1) twice
    // `hor` would be [(0, -1), (1, -1), (1, 1), (1, 1), (0, 3), (1, 3)]
    hor.sort_unstable_by_key(|(d, pos)| (d, pos.1, pos.0));
    // Finally, calculate how many horizontal edges we have.
    // We do this by counting how many neighboring entries have the same difference (i.e. both are above or below)
    // and then checking if their y component is different or x component is different by 1.
    // In other words, a cell is part of the same edge if the previous cell (remember, the cells are sorted)
    // is on the same y level, and its x is smaller by 1.
    // And then we add 2 because we don't account for the first edge above and below (we're only counting differences)
    let hor = hor
        .windows(2)
        .filter(|w| w[0].0 == w[1].0 && (w[0].1.1 != w[1].1.1 || w[1].1.0 - w[0].1.0 != 1))
        .count()
        + 2;
    // Now do the same, but for horizontal edges, i.e. we switch up x and y.
    let mut ver: Vec<_> = edges.iter().filter(|&(d, _)| matches!(d, (_, 0))).collect();
    ver.sort_unstable_by_key(|(d, pos)| (d, pos.0, pos.1));
    let ver = ver
        .windows(2)
        .filter(|w| w[0].0 == w[1].0 && (w[0].1.0 != w[1].1.0 || w[1].1.1 - w[0].1.1 != 1))
        .count()
        + 2;

    hor + ver
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
