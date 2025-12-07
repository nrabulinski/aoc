use std::collections::{HashMap, HashSet, VecDeque};

use aoc_lib::{
    aoc,
    color_eyre::eyre::Result,
    grid::{Grid, Point, PointExt},
};

static INPUT: &str = include_str!("../../inputs/day07");

fn part1(input: &str) -> Result<i64> {
    let grid = Grid::for_str(input).unwrap();

    let start = (0..grid.height())
        .flat_map(|y| (0..grid.width()).map(move |x| (x, y)))
        .find(|&pos| grid[pos] == b'S')
        .unwrap();

    let mut result = 0;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(start);

    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        let next = pos.add(&(0, 1));
        if !grid.is_valid_pos(next) {
            continue;
        }
        if grid[next] == b'^' {
            result += 1;
            for d in [(1, 0), (-1, 0)] {
                let next = next.add(&d);
                if grid.is_valid_pos(next) {
                    queue.push_back(next);
                }
            }
        } else {
            queue.push_back(next);
        }
    }

    Ok(result)
}

fn timelines_from_pos(cache: &mut HashMap<Point, i64>, grid: &Grid<'_>, pos: Point) -> i64 {
    if let Some(&res) = cache.get(&pos) {
        return res;
    }
    let next = pos.add(&(0, 1));
    if !grid.is_valid_pos(next) {
        return 1;
    }
    let res = if grid[next] == b'^' {
        let left = Some(next.add(&(-1, 0)))
            .filter(|&p| grid.is_valid_pos(p))
            .map_or(0, |next| timelines_from_pos(cache, grid, next));
        let right = Some(next.add(&(1, 0)))
            .filter(|&p| grid.is_valid_pos(p))
            .map_or(0, |next| timelines_from_pos(cache, grid, next));
        left + right
    } else {
        timelines_from_pos(cache, grid, next)
    };
    cache.insert(pos, res);
    res
}

fn part2(input: &str) -> Result<i64> {
    let grid = Grid::for_str(input).unwrap();

    let start = (0..grid.height())
        .flat_map(|y| (0..grid.width()).map(move |x| (x, y)))
        .find(|&pos| grid[pos] == b'S')
        .unwrap();

    let result = timelines_from_pos(&mut HashMap::new(), &grid, start);

    Ok(result)
}

#[allow(dead_code)]
static EX_INPUT: &str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 21,
    part2 => (EX_INPUT) 40
}
