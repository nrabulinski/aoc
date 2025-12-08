use std::{cmp::Reverse, collections::HashSet};

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day08");

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
struct NoNaN(f64);

impl PartialOrd for NoNaN {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NoNaN {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0.is_nan() || other.0.is_nan() {
            panic!();
        }
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl PartialEq for NoNaN {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() || other.0.is_nan() {
            panic!();
        }
        self.0 == other.0
    }
}

impl Eq for NoNaN {}

trait NoNaNExt {
    fn no_nan(self) -> NoNaN;
}

impl NoNaNExt for f64 {
    fn no_nan(self) -> NoNaN {
        NoNaN(self)
    }
}

type Vec3 = (i64, i64, i64);

fn parse(input: &str) -> Vec<Vec3> {
    to_lines(input)
        .map(|line| {
            let mut line = line.split(',').map(|x| x.parse().unwrap());
            (
                line.next().unwrap(),
                line.next().unwrap(),
                line.next().unwrap(),
            )
        })
        .collect()
}

fn all_distances(points: &[Vec3]) -> Vec<(Vec3, Vec3)> {
    let mut all_distances: Vec<_> = points
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| points.iter().skip(i + 1).map(move |&b| (a, b)))
        .collect();
    all_distances.sort_by_key(|&(a, b)| distance(a, b).no_nan());
    all_distances
}

fn distance(a: Vec3, b: Vec3) -> f64 {
    let x = (a.0 - b.0).pow(2) as f64;
    let y = (a.1 - b.1).pow(2) as f64;
    let z = (a.2 - b.2).pow(2) as f64;
    (x + y + z).sqrt()
}

fn merge_circuits(circuits: &mut Vec<HashSet<Vec3>>, a: Vec3, b: Vec3) {
    let idx_a = circuits.iter().position(|c| c.contains(&a)).unwrap();
    if circuits[idx_a].contains(&b) {
        return;
    }
    let idx_b = circuits.iter().position(|c| c.contains(&b)).unwrap();
    let (i, j) = if idx_a > idx_b {
        (idx_b, idx_a)
    } else {
        (idx_a, idx_b)
    };
    let other = circuits.swap_remove(j);
    let og = &mut circuits[i];
    for p in other {
        og.insert(p);
    }
}

fn part1(input: &str) -> Result<usize> {
    #[cfg(test)]
    const CONN_COUNT: usize = 10;
    #[cfg(not(test))]
    const CONN_COUNT: usize = 1_000;

    let points = parse(input);
    let all_distances = all_distances(&points);

    let mut circuits: Vec<_> = points
        .into_iter()
        .map(|p| {
            let mut res = HashSet::new();
            res.insert(p);
            res
        })
        .collect();

    for &(a, b) in &all_distances[..CONN_COUNT] {
        merge_circuits(&mut circuits, a, b);
    }

    circuits.sort_by_key(|c| Reverse(c.len()));

    let result = circuits
        .iter()
        .take(3)
        .map(|c| c.len())
        .reduce(|acc, curr| acc * curr)
        .unwrap();

    Ok(result)
}

fn part2(input: &str) -> Result<i64> {
    let points = parse(input);
    let all_distances = all_distances(&points);

    let circuits: Vec<_> = points
        .into_iter()
        .map(|p| {
            let mut res = HashSet::new();
            res.insert(p);
            res
        })
        .collect();

    let (_, a, b) = all_distances
        .into_iter()
        .scan(circuits, |circuits, (a, b)| {
            merge_circuits(circuits, a, b);
            Some((circuits.len(), a, b))
        })
        .find(|(len, ..)| *len == 1)
        .unwrap();

    Ok(a.0 * b.0)
}

#[allow(dead_code)]
static EX_INPUT: &str = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

aoc! {
    INPUT:
    part1 => (EX_INPUT) 40,
    part2 => (EX_INPUT) 25272
}
