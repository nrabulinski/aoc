use std::collections::HashSet;

use aoc_lib::{
    algo::dijkstra,
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    grid::{Grid, Point},
};

static INPUT: &str = include_str!("../../inputs/day10");

fn part1(input: &str) -> Result<usize> {
    let input = input.trim();
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    let end_points: Vec<_> = input
        .as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| (c == b'9').then(|| grid.idx_to_pos(i).unwrap()))
        .collect();

    Ok(input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == b'0')
        .map(|(pos, _)| {
            let (dist, _) = dijkstra(grid.idx_to_pos(pos).unwrap(), |&pos| {
                let val = grid[pos];
                grid.orthogonal_pos(pos)
                    .filter(move |&next_pos| {
                        let next_val = grid[next_pos];
                        next_val > val && next_val - val == 1
                    })
                    .map(|x| (x, 1))
            });
            end_points.iter().filter_map(|end| dist.get(end)).count()
        })
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let input = input.trim();
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    fn dfs(grid: &Grid<'_>, pos: Point, visited: &HashSet<Point>) -> usize {
        if grid[pos] == b'9' {
            return 1;
        }
        let mut new_visited = visited.clone();
        new_visited.insert(pos);
        let val = grid[pos];
        grid.orthogonal_pos(pos)
            .filter(|next_pos| {
                let next_val = grid[*next_pos];
                next_val > val && next_val - val == 1 && !new_visited.contains(next_pos)
            })
            .map(|next_pos| dfs(grid, next_pos, &new_visited))
            .sum()
    }

    Ok(input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == b'0')
        .map(|(pos, _)| {
            let pos = grid.idx_to_pos(pos).unwrap();
            let mut visited = HashSet::new();
            visited.insert(pos);
            dfs(&grid, pos, &visited)
        })
        .sum())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 36,
    part2 => (EX_INPUT) 81
}
