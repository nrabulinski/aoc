use aoc_lib::{
    algo::dijkstra,
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    grid::{Grid, PointExt},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

static INPUT: &str = include_str!("../../inputs/day20");

fn part1(input: &str) -> Result<usize> {
    let input = input.trim();
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    let start = input
        .bytes()
        .position(|x| x == b'S')
        .ok_or_eyre("invalid input")?;
    let start = grid.idx_to_pos(start).unwrap();
    let end = input
        .bytes()
        .position(|x| x == b'E')
        .ok_or_eyre("invalid input")?;
    let end = grid.idx_to_pos(end).unwrap();

    let baseline = {
        let (d, _) = dijkstra(start, |&pos| {
            grid.orthogonal_pos(pos)
                .filter(|&next| grid[next] != b'#')
                .map(|pos| (pos, 1))
        });
        *d.get(&end).unwrap()
    };

    let res = (1..grid.height() - 1)
        .into_par_iter()
        .flat_map(|y| (1..grid.width() - 1).into_par_iter().map(move |x| (x, y)))
        .filter(|&cheat| {
            if grid[cheat] != b'#' {
                return false;
            }
            let (d, _) = dijkstra(start, |&pos| {
                grid.orthogonal_pos(pos)
                    .filter(|&next| grid[next] != b'#' || next == cheat)
                    .map(|pos| (pos, 1))
            });
            d.get(&end).is_some_and(|d| baseline - d >= 100)
        })
        .count();

    Ok(res)
}

fn part2(input: &str) -> Result<usize> {
    let input = input.trim();
    let grid = Grid::for_str(input).ok_or_eyre("invalid format")?;

    let start = input
        .bytes()
        .position(|x| x == b'S')
        .ok_or_eyre("invalid input")?;
    let start = grid.idx_to_pos(start).unwrap();
    let end = input
        .bytes()
        .position(|x| x == b'E')
        .ok_or_eyre("invalid input")?;
    let end = grid.idx_to_pos(end).unwrap();

    let baseline = {
        let (d, _) = dijkstra(start, |&pos| {
            grid.orthogonal_pos(pos)
                .filter(|&next| grid[next] != b'#')
                .map(|pos| (pos, 1))
        });
        *d.get(&end).unwrap()
    };

    let res = (1..grid.height() - 1)
        .into_par_iter()
        .flat_map(|y| (1..grid.width() - 1).into_par_iter().map(move |x| (x, y)))
        .filter(|&pos| grid[pos] != b'#')
        .flat_map(|pos| {
            (-20i64..=20).into_par_iter().flat_map(move |dx| {
                let z = 20 - dx.abs();
                (-z..=z)
                    .into_par_iter()
                    .map(move |dy| pos.add(&(dx, dy)))
                    .filter(move |&target| {
                        grid.get_pos(target).is_some_and(|&c| c != b'#') && target != pos
                    })
                    .map(move |target| (pos, target))
            })
        })
        .filter(|&(cheat_from, cheat_to)| {
            let d = cheat_from.0.abs_diff(cheat_to.0) + cheat_from.1.abs_diff(cheat_to.1);

            let (d, _) = dijkstra(start, |&pos| {
                if pos == cheat_from {
                    vec![(cheat_to, d)].into_iter()
                } else {
                    grid.orthogonal_pos(pos)
                        .filter(|&next| grid[next] != b'#')
                        .map(|pos| (pos, 1))
                        .collect::<Vec<_>>()
                        .into_iter()
                }
            });
            d.get(&end).is_some_and(|d| baseline - d >= 100)
        })
        .count();

    Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = "EXAMPLE 1 HERE";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 0,
    part2 => (EX_INPUT) 0
}
