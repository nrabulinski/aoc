use aoc_lib::{aoc, color_eyre::eyre::Result};

static INPUT: &str = include_str!("../../inputs/day09");

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Taken { val: u16, len: u8 },
}

struct Arr {
    capacity: u8,
    len: u8,
    inner: [Cell; 10],
}

impl Arr {
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
    let input = input.trim().as_bytes();

    let mut res = 0;
    let mut idx = (input[0] - b'0') as usize;
    let mut add_to_res = |id, len| {
        let len = len as usize;
        res += len * (2 * idx + len - 1) / 2 * id;
        idx += len;
    };

    let mut left_idx = 1;
    let mut right_idx = input.len() - if input.len() & 1 == 1 { 1 } else { 2 };
    let mut left = input[left_idx] - b'0';
    let mut right = input[right_idx] - b'0';

    while left_idx < right_idx {
        if left > right {
            left -= right;
            add_to_res(right_idx / 2, right);
            right_idx -= 2;
            right = input[right_idx] - b'0';
        } else {
            right -= left;
            add_to_res(right_idx / 2, left);
            let i = left_idx + 1;
            add_to_res(
                i / 2,
                if i == right_idx {
                    right
                } else {
                    input[i] - b'0'
                },
            );

            left_idx += 2;
            left = input[left_idx] - b'0';
        }
    }

    Ok(res)
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
            let mut res = Arr {
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
            if !input[..i]
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
