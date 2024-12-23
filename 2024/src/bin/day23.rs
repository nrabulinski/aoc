use std::collections::{HashMap, HashSet};

use aoc_lib::{
    aoc,
    color_eyre::eyre::{OptionExt, Result},
    to_lines,
};

static INPUT: &str = include_str!("../../inputs/day23");

fn part1(input: &str) -> Result<usize> {
    let mut conns = HashMap::<&str, Vec<&str>>::new();
    for line in to_lines(input) {
        let (a, b) = line.split_once('-').ok_or_eyre("invalid format")?;
        conns.entry(a).or_default().push(b);
        conns.entry(b).or_default().push(a);
    }
    let conns = &conns;
    let triples: HashSet<[&str; 3]> = conns
        .iter()
        .flat_map(|(&a, bs)| {
            bs.iter().flat_map(move |&b| {
                conns[b]
                    .iter()
                    .filter(move |&&c| c != a && conns[c].contains(&a))
                    .map(move |&c| {
                        let mut res = [a, b, c];
                        res.sort();
                        res
                    })
            })
        })
        .collect();
    Ok(triples
        .into_iter()
        .filter(|triple| triple.iter().any(|puter| puter.starts_with('t')))
        .count())
}

fn part2(input: &str) -> Result<String> {
    let mut conns = HashMap::<&str, HashSet<&str>>::new();
    let mut puters = Vec::new();
    for line in to_lines(input) {
        let (a, b) = line.split_once('-').ok_or_eyre("invalid format")?;
        conns.entry(a).or_default().insert(b);
        conns.entry(b).or_default().insert(a);
        puters.push(a);
    }
    for &puter in &puters {
        loop {
            let this_conns = &conns[puter];
            // get the computer with the most and the least common connections with us
            let (min, max) = this_conns.iter().fold(
                (("", usize::MAX), ("", usize::MIN)),
                |mut res @ ((_, min), (_, max)), &curr| {
                    let cnt = conns[curr].intersection(this_conns).count();

                    if cnt < min {
                        res.0 = (curr, cnt);
                    }
                    if cnt > max {
                        res.1 = (curr, cnt);
                    }

                    res
                },
            );

            if min.1 != max.1 && min.0 != max.0 {
                // and remove it
                conns.get_mut(&puter).unwrap().remove(&min.0);
                conns.get_mut(&min.0).unwrap().remove(&puter);
            } else {
                // unless every computer has the same amount of connections in common
                break;
            }
        }
    }

    let tail = conns
        .into_iter()
        .map(|(head, tail)| {
            let mut tail: Vec<_> = tail.into_iter().collect();
            tail.push(head);
            tail.sort_unstable();
            tail
        })
        .max_by_key(|group| group.len())
        .unwrap();

    Ok(tail.join(","))
}

#[allow(dead_code)]
static EX_INPUT_1: &str = r#"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"#;

#[allow(dead_code)]
static EX_INPUT_2: &str = r#"
ka-co
ta-co
de-co
ta-ka
de-ta
ka-de
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT_1) 7,
    part2 => (EX_INPUT_2) "co,de,ka,ta"
}
