use std::ops::{Deref, DerefMut};

use aoc_lib::{aoc, color_eyre::eyre::Result};

static INPUT: &str = include_str!("../../inputs/day09");

struct Arr1 {
    len: u8,
    inner: [Option<u16>; 10],
}

impl Deref for Arr1 {
    type Target = [Option<u16>];

    fn deref(&self) -> &Self::Target {
        &self.inner[..self.len as usize]
    }
}

impl DerefMut for Arr1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner[..self.len as usize]
    }
}

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Taken { val: u16, len: u8 },
}

struct Arr2 {
    capacity: u8,
    len: u8,
    inner: [Cell; 10],
}

impl Arr2 {
    fn try_insert(&mut self, val: u16, len: u8) -> bool {
        if self.len + len > self.capacity {
            false
        } else {
            let cell = self
                .inner
                .iter_mut()
                .find(|x| matches!(x, Cell::Empty))
                .unwrap();
            *cell = Cell::Taken { val, len };
            self.len += len;
            true
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let input = input.trim();

    let mut input: Vec<_> = input
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(id, num)| {
            let id = id as u16;
            let len = num - b'0';
            let mut res = Arr1 {
                len,
                inner: [None; 10],
            };
            if id & 1 != 1 {
                for i in 0..res.len as usize {
                    res.inner[i] = Some(id / 2);
                }
            }
            res
        })
        .collect();

    for i in (0..input.len()).rev() {
        for j in (0..input[i].len()).rev() {
            let &Some(elem) = &input[i][j] else {
                continue;
            };
            let Some(empty_place) = (&mut input[..i])
                .iter_mut()
                .find_map(|arr| arr.iter_mut().find(|x| x.is_none()))
            else {
                break;
            };
            *empty_place = Some(elem);
            input[i][j] = None;
        }
    }

    Ok(input
        .iter()
        .scan(0, |state, curr| {
            let res: usize = curr
                .iter()
                .enumerate()
                .filter_map(|(i, e)| e.map(|val| (*state + i) * val as usize))
                .sum();
            *state += curr.len();
            Some(res)
        })
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let input = input.trim();

    let mut input: Vec<_> = input
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(id, num)| {
            let id = id as u16;
            let len = num - b'0';
            let mut res = Arr2 {
                capacity: len,
                len: 0,
                inner: [const { Cell::Empty }; 10],
            };
            if id & 1 != 1 {
                res.inner[0] = Cell::Taken { val: id / 2, len };
                res.len = len;
            }
            res
        })
        .collect();

    for i in (0..input.len()).rev() {
        for j in (0..input[i].len as usize).rev() {
            let &Cell::Taken { val, len } = &input[i].inner[j] else {
                continue;
            };
            if !(&mut input[..i])
                .iter_mut()
                .any(|arr| arr.try_insert(val, len))
            {
                break;
            }
            input[i].inner[j] = Cell::Empty;
            input[i].len -= len;
        }
    }

    Ok(input
        .iter()
        .scan(0, |i, curr| {
            let res: usize = curr
                .inner
                .iter()
                .scan(*i, |j, &e| {
                    let Cell::Taken { val, len } = e else {
                        return None;
                    };
                    let len = len as usize;

                    let mut res = 0;
                    for i in 0..len {
                        res += val as usize * (i + *j);
                    }
                    *j += len;
                    Some(res)
                })
                .sum();
            *i += curr.capacity as usize;
            Some(res)
        })
        .sum())
}

#[allow(dead_code)]
static EX_INPUT: &str = "2333133121414131402";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 1928,
    part2 => (EX_INPUT) 2858
}
