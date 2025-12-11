use std::collections::HashMap;

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day11");

fn parse_line(line: &str) -> (&str, Vec<&str>) {
    let (from, to) = line.split_once(": ").unwrap();
    let to = to.split_ascii_whitespace().collect();
    (from, to)
}

fn part1(input: &str) -> Result<usize> {
    fn conns_to_out<'a>(
        cache: &mut HashMap<&'a str, usize>,
        from: &'a str,
        connections: &HashMap<&'a str, Vec<&'a str>>,
    ) -> usize {
        if let Some(res) = cache.get(&from) {
            return *res;
        }
        if from == "out" {
            return 1;
        }

        let conns = connections
            .get(from)
            .unwrap()
            .iter()
            .map(|&new_from| conns_to_out(cache, new_from, connections))
            .sum();

        cache.insert(from, conns);

        conns
    }

    let connections: HashMap<_, _> = to_lines(input).map(parse_line).collect();

    Ok(conns_to_out(&mut HashMap::new(), "you", &connections))
}

fn part2(input: &str) -> Result<usize> {
    fn conns_to_out<'a>(
        cache: &mut HashMap<(&'a str, bool, bool), usize>,
        from: &'a str,
        connections: &HashMap<&'a str, Vec<&'a str>>,
        dac_seen: bool,
        fft_seen: bool,
    ) -> usize {
        let dac_seen = dac_seen || from == "dac";
        let fft_seen = fft_seen || from == "fft";

        if let Some(res) = cache.get(&(from, dac_seen, fft_seen)) {
            return *res;
        }
        if from == "out" {
            if dac_seen && fft_seen {
                return 1;
            } else {
                return 0;
            }
        }

        let conns = connections
            .get(from)
            .unwrap()
            .iter()
            .map(|&new_from| conns_to_out(cache, new_from, connections, dac_seen, fft_seen))
            .sum();

        cache.insert((from, dac_seen, fft_seen), conns);

        conns
    }

    let connections: HashMap<_, _> = to_lines(input).map(parse_line).collect();

    Ok(conns_to_out(
        &mut HashMap::new(),
        "svr",
        &connections,
        false,
        false,
    ))
}

#[allow(dead_code)]
static EX_INPUT_1: &str = "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

#[allow(dead_code)]
static EX_INPUT_2: &str = "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

aoc! {
    INPUT:
    part1 => (EX_INPUT_1) 5,
    part2 => (EX_INPUT_2) 2
}
