use std::collections::HashSet;

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    grid::{Grid, Point, PointExt},
    to_lines,
};

static INPUT: &str = include_str!("../../inputs/day15");

fn char_to_dir(c: u8) -> Point {
    match c {
        b'<' => (-1, 0),
        b'>' => (1, 0),
        b'^' => (0, -1),
        b'v' => (0, 1),
        c => unreachable!("{} ({})", c as char, c),
    }
}

fn move_box_narrow(grid: &Grid<'_>, boxes: &mut HashSet<Point>, pos: Point, dir: Point) -> bool {
    if grid[pos] == b'#' {
        return false;
    }
    if !boxes.contains(&pos) {
        return true;
    }
    let next_pos = pos.add(&dir);
    if move_box_narrow(grid, boxes, next_pos, dir) {
        boxes.remove(&pos);
        boxes.insert(next_pos);
        true
    } else {
        false
    }
}

fn maybe_move_box_wide(
    grid: &Grid<'_>,
    boxes: &mut HashSet<Point>,
    pos: Point,
    dir: Point,
) -> bool {
    fn can_move_box_wide(grid: &Grid<'_>, boxes: &HashSet<Point>, pos: Point, dir: Point) -> bool {
        if dir.1 != 0 {
            let next = pos.add(&dir);
            if grid[(next.0 / 2, next.1)] == b'#' || grid[((next.0 + 1) / 2, next.1)] == b'#' {
                return false;
            }
            if boxes.contains(&(next.0 - 1, next.1)) {
                if !can_move_box_wide(grid, boxes, (next.0 - 1, next.1), dir) {
                    return false;
                }
            }
            if boxes.contains(&pos.add(&dir)) {
                if !can_move_box_wide(grid, boxes, next, dir) {
                    return false;
                }
            }
            if boxes.contains(&(pos.0 + 1, pos.1).add(&dir)) {
                if !can_move_box_wide(grid, boxes, (next.0 + 1, next.1), dir) {
                    return false;
                }
            }
        } else {
            let obs_check = pos.add(&dir.map(|d| if pos.0 & 1 == 0 { d * 2 } else { d }));
            if grid[(obs_check.0 / 2, obs_check.1)] == b'#' {
                return false;
            }
            let next = pos.add(&dir.map(|d| d * 2));
            if boxes.contains(&next) {
                return can_move_box_wide(grid, boxes, next, dir);
            }
        }
        true
    }
    fn move_box_wide(grid: &Grid<'_>, boxes: &mut HashSet<Point>, pos: Point, dir: Point) {
        if !boxes.contains(&pos) {
            return;
        }
        if dir.1 != 0 {
            move_box_wide(grid, boxes, (pos.0 - 1, pos.1).add(&dir), dir);
            move_box_wide(grid, boxes, (pos.0, pos.1).add(&dir), dir);
            move_box_wide(grid, boxes, (pos.0 + 1, pos.1).add(&dir), dir);
        } else {
            move_box_wide(grid, boxes, pos.add(&dir.map(|d| d * 2)), dir);
        }
        boxes.remove(&pos);
        boxes.insert(pos.add(&dir));
    }

    if grid[(pos.0 / 2, pos.1)] == b'#' {
        return false;
    }
    let real_pos = if boxes.contains(&pos) {
        pos
    } else if boxes.contains(&(pos.0 - 1, pos.1)) {
        (pos.0 - 1, pos.1)
    } else {
        return true;
    };

    if can_move_box_wide(grid, boxes, real_pos, dir) {
        move_box_wide(grid, boxes, real_pos, dir);
        true
    } else {
        false
    }
}

fn part1(input: &str) -> Result<i64> {
    let (grid, moves) = input.split_once("\n\n").ok_or_eyre("invalid format")?;
    let grid_str = grid.trim();
    let grid = Grid::for_str(grid_str).ok_or_eyre("invalid format")?;

    let mut pos = (0, 0);
    let mut boxes = HashSet::new();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let p = (x, y);
            match grid[p] {
                b'O' => _ = boxes.insert(p),
                b'@' => pos = p,
                _ => (),
            }
        }
    }

    for dir in to_lines(moves).flat_map(|line| line.as_bytes().iter().copied().map(char_to_dir)) {
        let next_pos = pos.add(&dir);
        if grid[next_pos] == b'#' {
            continue;
        }
        if move_box_narrow(&grid, &mut boxes, next_pos, dir) {
            pos = next_pos;
        }
    }

    Ok(boxes.into_iter().map(|(x, y)| x + 100 * y).sum())
}

fn part2(input: &str) -> Result<i64> {
    let (grid, moves) = input.split_once("\n\n").ok_or_eyre("invalid format")?;
    let grid_str = grid.trim();
    let grid = Grid::for_str(grid_str).ok_or_eyre("invalid format")?;

    let mut pos = (0, 0);
    let mut boxes = HashSet::new();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let p = (x, y);
            let p_actual = (x * 2, y);
            match grid[p] {
                b'O' => _ = boxes.insert(p_actual),
                b'@' => pos = p_actual,
                _ => (),
            }
        }
    }

    for dir in to_lines(moves).flat_map(|line| line.as_bytes().iter().copied().map(char_to_dir)) {
        let next_pos = pos.add(&dir);
        if grid[(next_pos.0 / 2, next_pos.1)] == b'#' {
            continue;
        }
        if maybe_move_box_wide(&grid, &mut boxes, next_pos, dir) {
            pos = next_pos;
        }
    }

    Ok(boxes.into_iter().map(|(x, y)| x + 100 * y).sum())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;

#[allow(dead_code)]
static SMALL_1: &str = r#"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"#;

#[allow(dead_code)]
static SMALL_2: &str = r#"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 10092,
    part2 => (EX_INPUT) 9021
}
