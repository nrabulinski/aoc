use std::collections::{HashMap, HashSet};

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    grid::{Grid, Point},
};

static INPUT: &str = include_str!("../../inputs/day16");

fn part1(input: &str) -> Result<i64> {
    let input = input.trim();
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    let start = input
        .as_bytes()
        .iter()
        .position(|&x| x == b'S')
        .and_then(|i| grid.idx_to_pos(i))
        .ok_or_eyre("invalid format")?;
    let end = input
        .as_bytes()
        .iter()
        .position(|&x| x == b'E')
        .and_then(|i| grid.idx_to_pos(i))
        .ok_or_eyre("invalid format")?;

    fn dfs(
        grid: &Grid<'_>,
        curr: Point,
        dir: Point,
        points: i64,
        end: Point,
        visited: &mut HashMap<Point, i64>,
    ) -> i64 {
        visited.insert(curr, points);
        if curr == end {
            return points;
        }

        let h = [-1, 1]
            .into_iter()
            .map(move |dx| ((dx, 0), (curr.0 + dx, curr.1)));
        let v = [-1, 1]
            .into_iter()
            .map(move |dy| ((0, dy), (curr.0, curr.1 + dy)));

        let mut min_cost = i64::MAX;
        for (neighbor, next_dir, cost) in h
            .chain(v)
            .filter(|&(_, pos)| grid.is_valid_pos(pos) && grid[pos] != b'#')
            .map(move |(next_dir, next_pos)| {
                let cost = 1i64 + if next_dir == dir { 0 } else { 1000 };
                (next_pos, next_dir, cost)
            })
        {
            let total_cost = points + cost;
            if visited.get(&neighbor).is_some_and(|&c| c < total_cost) {
                continue;
            }
            min_cost = min_cost.min(dfs(grid, neighbor, next_dir, total_cost, end, visited));
        }

        min_cost
    }

    Ok(dfs(&grid, start, (1, 0), 0, end, &mut HashMap::new()))
}

fn part2(input: &str) -> Result<usize> {
    let input = input.trim();
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    let start = input
        .as_bytes()
        .iter()
        .position(|&x| x == b'S')
        .and_then(|i| grid.idx_to_pos(i))
        .ok_or_eyre("invalid format")?;
    let end = input
        .as_bytes()
        .iter()
        .position(|&x| x == b'E')
        .and_then(|i| grid.idx_to_pos(i))
        .ok_or_eyre("invalid format")?;

    let actual_min_cost = part1(input)?;

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        real_min: i64,
        grid: &Grid<'_>,
        curr: Point,
        dir: Point,
        points: i64,
        end: Point,
        visited: &mut HashMap<Point, i64>,
        parents: &mut HashMap<Point, Vec<Point>>,
    ) -> i64 {
        if curr == end {
            return points;
        }
        if real_min < points {
            return i64::MAX;
        }

        let h = [-1, 1]
            .into_iter()
            .map(move |dx| ((dx, 0), (curr.0 + dx, curr.1)));
        let v = [-1, 1]
            .into_iter()
            .map(move |dy| ((0, dy), (curr.0, curr.1 + dy)));

        let mut min_cost = i64::MAX;
        for (neighbor, next_dir, cost) in h
            .chain(v)
            .filter(|&(_, pos)| grid.is_valid_pos(pos) && grid[pos] != b'#')
            .map(move |(next_dir, next_pos)| {
                let cost = 1i64 + if next_dir == dir { 0 } else { 1000 };
                (next_pos, next_dir, cost)
            })
        {
            let total_cost = points + cost;
            if visited.get(&neighbor).is_some_and(|&c| c < total_cost) {
                continue;
            }
            if next_dir != dir {
                visited.insert(neighbor, total_cost);
            }
            let next_cost = dfs(
                real_min, grid, neighbor, next_dir, total_cost, end, visited, parents,
            );
            if next_cost == real_min {
                parents.entry(neighbor).or_default().push(curr);
            }
            min_cost = min_cost.min(next_cost);
        }

        min_cost
    }

    let mut parents = HashMap::new();

    let min_cost = dfs(
        actual_min_cost,
        &grid,
        start,
        (1, 0),
        0,
        end,
        &mut HashMap::new(),
        &mut parents,
    );

    assert_eq!(min_cost, actual_min_cost);

    let mut path = HashSet::new();
    let mut queue = Vec::new();

    queue.push(end);

    while let Some(pos) = queue.pop() {
        path.insert(pos);
        queue.extend(
            parents
                .get(&pos)
                .into_iter()
                .flatten()
                .filter(|pos| !path.contains(pos)),
        );
    }

    Ok(path.len())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#;

#[allow(dead_code)]
static BIG: &str = r#"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 7036,
    part2 => (EX_INPUT) 45
}
