use crate::problem::{Solution, SolverImpl};
use std::collections::HashSet;
use std::io;

#[derive(Default)]
pub struct Day03Impl {}

impl Day03Impl {
    pub fn new() -> Self {
        Default::default()
    }
}

fn to_priority(&c: &u8) -> i64 {
    if c <= b'Z' {
        (c - b'A' + 27) as i64
    } else {
        (c - b'a' + 1) as i64
    }
}

impl SolverImpl for Day03Impl {
    fn solve(self, r: impl io::BufRead) -> io::Result<Solution> {
        let lines: Vec<_> = r
            .lines()
            .map(|l| l.unwrap().trim().to_string().into_bytes())
            .collect();

        let part1 = lines
            .iter()
            .map(|l| l.split_at(l.len() / 2))
            .map(|(l, r)| {
                let m = |s: &[u8]| -> HashSet<i64> { s.iter().map(to_priority).collect() };
                (m(l), m(r))
            })
            .map(|(l, r)| l.intersection(&r).sum::<i64>())
            .sum::<i64>()
            .to_string();

        let part2 = lines
            .iter()
            .map(|l| l.iter().map(to_priority).collect())
            .collect::<Vec<HashSet<i64>>>()
            .chunks(3)
            .map(|chunk| {
                if let [p1, p2, p3] = chunk {
                    p1.intersection(&p2.intersection(p3).into_iter().map(i64::clone).collect())
                        .last()
                        .unwrap()
                        .clone()
                } else {
                    0
                }
            })
            .sum::<i64>()
            .to_string();

        Ok(Solution { part1, part2 })
    }
}
