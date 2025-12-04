use aoc_lib::{
    aoc,
    color_eyre::eyre::Result,
    grid::{Grid, Point},
};

static INPUT: &str = include_str!("../../inputs/day04");

fn reachable(grid: &Grid<'_>) -> impl Iterator<Item = Point> {
    (0..grid.height())
        .flat_map(|y| (0..grid.width()).map(move |x| (x, y)))
        .filter(|&pos| {
            grid[pos] == b'@' && grid.adjacent_pos(pos).filter(|&p| grid[p] == b'@').count() < 4
        })
}

fn part1(input: &str) -> Result<usize> {
    let grid = Grid::for_str(input).unwrap();
    let result = reachable(&grid).count();
    Ok(result)
}

fn part2(input: &str) -> Result<usize> {
    let result = std::iter::repeat(())
        .scan(input.trim().to_string(), |input, _| {
            let grid = Grid::for_str(&input).unwrap();
            let to_remove: Vec<_> = reachable(&grid)
                .map(|pos| grid.pos_to_idx(pos).unwrap())
                .collect();
            // no more rolls available to remove
            if to_remove.is_empty() {
                return None;
            }

            for &idx in &to_remove {
                unsafe {
                    input.as_bytes_mut()[idx] = b'.';
                }
            }
            Some(to_remove.len())
        })
        .fuse()
        .sum();

    Ok(result)
}

#[allow(dead_code)]
static EX_INPUT: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 13,
    part2 => (EX_INPUT) 43
}
