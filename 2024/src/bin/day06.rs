use std::collections::HashSet;

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    grid::{Grid, Point, PointExt},
};

static INPUT: &str = include_str!("../../inputs/day06");

fn get_visited_cells(grid: &Grid<'_>, starting_pos: Point) -> HashSet<Point> {
    let mut direction = (0, -1);
    let mut pos = starting_pos;

    let mut visited = HashSet::new();

    loop {
        visited.insert(pos);

        loop {
            let next_pos = pos.add(&direction);
            if !grid.is_valid_pos(next_pos) {
                return visited;
            }
            if grid[next_pos] == b'#' {
                direction = (-direction.1, direction.0);
            } else {
                pos = next_pos;
                break;
            }
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let input = input.trim();
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    let pos = input
        .find('^')
        .and_then(|idx| grid.idx_to_pos(idx))
        .ok_or_eyre("invalid input")?;

    Ok(get_visited_cells(&grid, pos).len())
}

fn loops_for_input(
    grid: &Grid<'_>,
    starting_pos: Point,
    check_point: impl Fn(Point) -> bool,
) -> bool {
    let mut visited = HashSet::new();

    let mut direction = (0, -1);
    let mut last_direction = direction;
    let mut pos = starting_pos;

    loop {
        if last_direction != direction {
            if !visited.insert((pos, direction)) {
                return true;
            }
            last_direction = direction;
        }

        loop {
            let next_pos = pos.add(&direction);
            if !grid.is_valid_pos(next_pos) {
                return false;
            }
            if check_point(next_pos) {
                pos = next_pos;
                break;
            } else {
                direction = (-direction.1, direction.0);
            }
        }
    }
}

fn part2(input: &str) -> Result<usize> {
    let input = input.trim();
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;
    let starting_pos = input
        .find('^')
        .and_then(|idx| grid.idx_to_pos(idx))
        .ok_or_eyre("invalid input")?;

    let mut visited = get_visited_cells(&grid, starting_pos);
    visited.remove(&starting_pos);

    Ok(visited
        .into_iter()
        .filter(|&replaced_pos| {
            let check_point = |pos| grid[pos] != b'#' && pos != replaced_pos;
            loops_for_input(&grid, starting_pos, check_point)
        })
        .count())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 41,
    part2 => (EX_INPUT) 6
}
