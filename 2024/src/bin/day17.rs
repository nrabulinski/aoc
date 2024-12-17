use std::fmt::Write;

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    iter::IterExt,
    to_lines,
};

static INPUT: &str = include_str!("../../inputs/day17");

fn combo_op(op: u8, regs: &[i64; 3]) -> i64 {
    match op {
        b'0'..=b'3' => (op - b'0').into(),
        b'4'..=b'6' => regs[(op - b'4') as usize],
        _ => unreachable!(),
    }
}

fn literal_op(op: u8) -> i64 {
    (op - b'0').into()
}

macro_rules! get_val {
    ($program:expr => $idx:expr) => {{
        if $program.len() > $idx {
            $program[$idx]
        } else {
            break;
        }
    }};
}

fn eval(mut regs: [i64; 3], program: &[u8]) -> String {
    let mut out = String::new();
    let mut pc = 0;

    while pc < program.len() {
        let ins = get_val!(program => pc);
        match ins {
            b'0' => {
                let numerator = regs[0];
                let denominator = combo_op(get_val!(program => pc + 1), &regs);
                regs[0] = numerator >> denominator;

                pc += 2;
            }
            b'1' => {
                let a = regs[1];
                let b = literal_op(get_val!(program => pc + 1));
                regs[1] = a ^ b;

                pc += 2;
            }
            b'2' => {
                let val = combo_op(get_val!(program => pc + 1), &regs);
                regs[1] = val & 7;

                pc += 2;
            }
            b'3' => {
                if regs[0] == 0 {
                    pc += 2;
                } else {
                    pc = literal_op(get_val!(program => pc + 1)) as usize;
                }
            }
            b'4' => {
                let a = regs[1];
                let b = regs[2];
                regs[1] = a ^ b;

                pc += 2;
            }
            b'5' => {
                let val = combo_op(get_val!(program => pc + 1), &regs);
                let val = val & 7;
                write!(out, "{val},").ok();

                pc += 2;
            }
            b'6' => {
                let numerator = regs[0];
                let denominator = combo_op(get_val!(program => pc + 1), &regs);
                regs[1] = numerator >> denominator;

                pc += 2;
            }
            b'7' => {
                let numerator = regs[0];
                let denominator = combo_op(get_val!(program => pc + 1), &regs);
                regs[2] = numerator >> denominator;

                pc += 2;
            }
            _ => unreachable!(),
        }
    }

    out.truncate(out.len() - 1);
    out
}

fn find_ouroboros(program: &[u8]) -> i64 {
    fn find_target(prev_a: i64, idx: usize, program: &[u8]) -> Option<i64> {
        (0..8)
            .filter_map(|maybe_a| {
                let a = maybe_a | (prev_a << 3);

                let mut regs = [a, 0, 0];
                let mut pc = 0;
                let mut out_val = 0;
                loop {
                    let ins = program[pc];
                    match ins {
                        b'0' => {
                            let numerator = regs[0];
                            let denominator = combo_op(program[pc + 1], &regs);
                            regs[0] = numerator >> denominator;

                            pc += 2;
                        }
                        b'1' => {
                            let a = regs[1];
                            let b = literal_op(program[pc + 1]);
                            regs[1] = a ^ b;

                            pc += 2;
                        }
                        b'2' => {
                            let val = combo_op(program[pc + 1], &regs);
                            regs[1] = val & 7;

                            pc += 2;
                        }
                        b'3' => {
                            break;
                        }
                        b'4' => {
                            let a = regs[1];
                            let b = regs[2];
                            regs[1] = a ^ b;

                            pc += 2;
                        }
                        b'5' => {
                            let val = combo_op(program[pc + 1], &regs);
                            out_val = (val & 7) as u8 + b'0';

                            pc += 2;
                        }
                        b'6' => {
                            let numerator = regs[0];
                            let denominator = combo_op(program[pc + 1], &regs);
                            regs[1] = numerator >> denominator;

                            pc += 2;
                        }
                        b'7' => {
                            let numerator = regs[0];
                            let denominator = combo_op(program[pc + 1], &regs);
                            regs[2] = numerator >> denominator;

                            pc += 2;
                        }
                        _ => unreachable!(),
                    }
                }
                if out_val == program[idx] {
                    if idx == 0 {
                        Some(a)
                    } else {
                        find_target(a, idx - 1, program)
                    }
                } else {
                    None
                }
            })
            .min()
    }

    #[cfg(not(test))]
    return find_target(0, program.len() - 1, program).unwrap();

    #[cfg(test)]
    {
        for a in 0..i64::MAX {
            let regs = [a, 0, 0];
            if eval_ouroboros(regs, &program) {
                return a;
            }
        }

        unreachable!()
    }
}

#[allow(dead_code)]
fn eval_ouroboros(mut regs: [i64; 3], program: &[u8]) -> bool {
    let mut out = Vec::new();
    let mut pc = 0;

    while pc < program.len() {
        let ins = get_val!(program => pc);
        match ins {
            b'0' => {
                let numerator = regs[0];
                let denominator = combo_op(get_val!(program => pc + 1), &regs);
                regs[0] = numerator >> denominator;

                pc += 2;
            }
            b'1' => {
                let a = regs[1];
                let b = literal_op(get_val!(program => pc + 1));
                regs[1] = a ^ b;

                pc += 2;
            }
            b'2' => {
                let val = combo_op(get_val!(program => pc + 1), &regs);
                regs[1] = val & 7;

                pc += 2;
            }
            b'3' => {
                if regs[0] == 0 {
                    pc += 2;
                } else {
                    pc = literal_op(get_val!(program => pc + 1)) as usize;
                }
            }
            b'4' => {
                let a = regs[1];
                let b = regs[2];
                regs[1] = a ^ b;

                pc += 2;
            }
            b'5' => {
                let val = combo_op(get_val!(program => pc + 1), &regs);
                let val = (val & 7) as u8 + b'0';
                out.push(val);

                if out.len() > program.len() || val != program[out.len() - 1] {
                    return false;
                }

                pc += 2;
            }
            b'6' => {
                let numerator = regs[0];
                let denominator = combo_op(get_val!(program => pc + 1), &regs);
                regs[1] = numerator >> denominator;

                pc += 2;
            }
            b'7' => {
                let numerator = regs[0];
                let denominator = combo_op(get_val!(program => pc + 1), &regs);
                regs[2] = numerator >> denominator;

                pc += 2;
            }
            _ => unreachable!(),
        }
    }

    out == program
}

fn part1(input: &str) -> Result<String> {
    let (regs, program) = input
        .trim()
        .split_once("\n\n")
        .ok_or_eyre("invalid format")?;

    let mut regs = to_lines(regs).map(|line| line.split_once(": ").unwrap().1.parse().unwrap());
    let regs: [i64; 3] = std::array::from_fn(|_| regs.next().unwrap());

    let program: Vec<_> = program
        .split_once(": ")
        .ok_or_eyre("invalid format")?
        .1
        .split(',')
        .map(|s| s.as_bytes()[0])
        .collect();

    Ok(eval(regs, &program))
}

fn part2(input: &str) -> Result<i64> {
    let (_, program) = input
        .trim()
        .split_once("\n\n")
        .ok_or_eyre("invalid format")?;

    let program: Vec<_> = program
        .split_once(": ")
        .ok_or_eyre("invalid format")?
        .1
        .split(',')
        .map(|s| s.as_bytes()[0])
        .collect();

    return Ok(find_ouroboros(&program));
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#;

#[allow(dead_code)]
static EX_INPUT_B: &str = r#"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0 
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) "4,6,3,5,6,3,5,2,1,0",
    part2 => (EX_INPUT_B) 117440
}
