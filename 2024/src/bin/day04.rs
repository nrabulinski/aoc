use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    grid::{Grid, Point, PointExt},
};

static INPUT: &str = include_str!("../../inputs/day04");

fn is_xmas(grid: &Grid<'_>, point: Point, direction: Point) -> bool {
    b"MAS"
        .iter()
        .enumerate()
        .map(|(i, c)| (point.add(&direction.map(|a| a * (i as i64 + 1))), c))
        .all(|(pos, c)| grid.get_pos(pos).is_some_and(|v| v == c))
}

fn find_xmas(grid: &Grid<'_>, point: Point) -> usize {
    grid.adjacent_pos(point)
        .map(|(x, y)| (x - point.0, y - point.1))
        .filter(|&dir| is_xmas(grid, point, dir))
        .count()
}

fn part1(input: &str) -> Result<usize> {
    let input = input.trim();
    let grid = Grid::for_str(input).ok_or_eyre("invalid input")?;

    Ok(input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == b'X')
        .map(|(idx, _)| find_xmas(&grid, grid.idx_to_pos(idx).unwrap()))
        .sum())
}

fn is_x_mas(grid: &Grid<'_>, point: Point) -> bool {
    [-1, 1]
        .into_iter()
        .flat_map(|dx| [(dx, -1), (dx, 1)])
        .filter(|&d| {
            let pos_a = point.add(&d);
            let dd = d.map(|a| -a);
            let pos_b = point.add(&dd);
            matches!(
                (grid.get_pos(pos_a).copied(), grid.get_pos(pos_b).copied()),
                (Some(b'M'), Some(b'S'))
            )
        })
        .count()
        == 2
}

fn part2(input: &str) -> Result<usize> {
    let input = input.trim();
    let grid = Grid::for_str(input).ok_or_eyre("invalid input")?;

    Ok(input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == b'A')
        .filter(|&(idx, _)| is_x_mas(&grid, grid.idx_to_pos(idx).unwrap()))
        .count())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 18,
    part2 => (EX_INPUT) 9
}
