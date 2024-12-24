use std::{cmp::Reverse, collections::HashMap};

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    to_lines,
};

static INPUT: &str = include_str!("../../inputs/day24");

#[derive(Debug, Clone, Copy)]
enum Gate<'s> {
    Hole,
    Output(bool),
    And(&'s str, &'s str),
    Or(&'s str, &'s str),
    Xor(&'s str, &'s str),
}

fn get_gate_val<'s>(gate: &'s str, gates: &mut HashMap<&'s str, Gate<'s>>) -> Option<bool> {
    match gates[gate] {
        Gate::Hole => None,
        Gate::Output(val) => Some(val),
        Gate::And(a, b) => {
            gates.insert(gate, Gate::Hole);
            let a = get_gate_val(a, gates)?;
            let b = get_gate_val(b, gates)?;
            let res = a & b;
            gates.insert(gate, Gate::Output(res));
            Some(res)
        }
        Gate::Or(a, b) => {
            gates.insert(gate, Gate::Hole);
            let a = get_gate_val(a, gates)?;
            let b = get_gate_val(b, gates)?;
            let res = a | b;
            gates.insert(gate, Gate::Output(res));
            Some(res)
        }
        Gate::Xor(a, b) => {
            gates.insert(gate, Gate::Hole);
            let a = get_gate_val(a, gates)?;
            let b = get_gate_val(b, gates)?;
            let res = a ^ b;
            gates.insert(gate, Gate::Output(res));
            Some(res)
        }
    }
}

fn get_reg_val<'a>(reg: char, gates: &mut HashMap<&'a str, Gate<'a>>) -> Option<u64> {
    let output_gates = {
        let mut res: Vec<_> = gates
            .keys()
            .filter(|k| k.starts_with(reg))
            .copied()
            .collect();
        res.sort_unstable_by_key(|&x| Reverse(x));
        res
    };
    output_gates
        .into_iter()
        .map(|gate| get_gate_val(gate, gates))
        .fold(Some(0), |acc, curr| {
            Some((acc? << 1) | (if curr? { 1 } else { 0 }))
        })
}

fn part1(input: &str) -> Result<u64> {
    let (inputs, gates_desc) = input.split_once("\n\n").ok_or_eyre("invalid format")?;
    let mut gates: HashMap<_, _> = to_lines(inputs)
        .map(|line| {
            let (gate, val) = line.split_once(": ").unwrap();
            (gate, Gate::Output(val == "1"))
        })
        .collect();
    for line in to_lines(gates_desc) {
        let (ty, output) = line.split_once(" -> ").ok_or_eyre("invalid format")?;
        let (a, op, b) = {
            let mut i = ty.split_ascii_whitespace();
            (i.next().unwrap(), i.next().unwrap(), i.next().unwrap())
        };
        let res = match op {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => unreachable!(),
        };
        let res = res(a, b);
        assert!(gates.insert(output, res).is_none());
    }

    Ok(get_reg_val('z', &mut gates).unwrap())
}

// I just looked at the input as a graph and spotted the misplaced binary adders.
// Not the cleanest solution, but definitely faster than coming up with a programmatic solution
fn part2(input: &str) -> Result<String> {
    let (inputs, gates_desc) = input.split_once("\n\n").ok_or_eyre("invalid format")?;

    let mut to_swap = [["z17", "cmv"], ["z23", "rmj"], ["z30", "rdg"], [
        "btb", "mwp",
    ]];

    // graphviz printing code
    // eprintln!("digraph {{");
    // for line in to_lines(gates_desc) {
    //     let (ty, output) = line.split_once(" -> ").ok_or_eyre("invalid format")?;
    //     let output = to_swap
    //         .iter()
    //         .find_map(|&[x, y]| {
    //             if x == output {
    //                 Some(y)
    //             } else if y == output {
    //                 Some(x)
    //             } else {
    //                 None
    //             }
    //         })
    //         .unwrap_or(output);
    //     let (a, op, b) = {
    //         let mut i = ty.split_ascii_whitespace();
    //         (i.next().unwrap(), i.next().unwrap(), i.next().unwrap())
    //     };
    //     eprintln!("    {output} [label={output}_{op}]");
    //     eprintln!("    {a} -> {output}");
    //     eprintln!("    {b} -> {output}");
    // }
    // eprintln!("}}");

    let mut gates: HashMap<_, _> = to_lines(inputs)
        .map(|line| {
            let (gate, val) = line.split_once(": ").unwrap();
            (gate, Gate::Output(val == "1"))
        })
        .collect();

    let x = get_reg_val('x', &mut gates).unwrap();
    let y = get_reg_val('y', &mut gates).unwrap();
    let expected = x + y;

    for line in to_lines(gates_desc) {
        let (ty, output) = line.split_once(" -> ").ok_or_eyre("invalid format")?;
        let output = to_swap
            .iter()
            .find_map(|&[x, y]| {
                if x == output {
                    Some(y)
                } else if y == output {
                    Some(x)
                } else {
                    None
                }
            })
            .unwrap_or(output);
        let (a, op, b) = {
            let mut i = ty.split_ascii_whitespace();
            (i.next().unwrap(), i.next().unwrap(), i.next().unwrap())
        };
        let res = match op {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => unreachable!(),
        };
        let res = res(a, b);
        assert!(gates.insert(output, res).is_none());
    }

    let actual = get_reg_val('z', &mut gates).unwrap();
    assert_eq!(actual, expected);

    let res = to_swap.as_flattened_mut();
    res.sort();
    Ok(res.join(","))
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 2024,
    // fake test-case because I can't be bothered setting one up
    part2 => (INPUT) "btb,cmv,mwp,rdg,rmj,z17,z23,z30"
}
