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

fn part1(input_str: &str) -> Result<usize> {
    let input_str = input_str.trim();
    let mut input = input_str.as_bytes().to_vec();

    let mut res = 0;

    let mut idx = (input[0] - b'0') as usize;
    let mut left = 1;
    let mut right = input.len() - if input.len() & 1 == 1 { 1 } else { 2 };

    while left < right {
        let a = input[left] - b'0';
        let b = input[right] - b'0';

        if a > b {
            input[left] -= b;
            let id = right / 2;
            let len = b as usize;
            // len (idx + (idx + len - 1)) / 2
            // len (2idx + len - 1) / 2
            // len * 2idx + len^2 - len / 2
            res += len * (2 * idx + len - 1) / 2 * id;
            idx += len;
            right -= 2;
        } else {
            input[right] -= a;
            let id = right / 2;
            let len = a as usize;
            res += len * (2 * idx + len - 1) / 2 * id;
            idx += len;

            let i = left + 1;
            let id = i / 2;
            let len = (input[i] - b'0') as usize;
            res += len * (2 * idx + len - 1) / 2 * id;
            idx += len;

            left += 2;
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
