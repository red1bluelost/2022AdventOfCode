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
        let lines: Vec<Vec<i64>> = r
            .lines()
            .flatten()
            .map(|l| l.trim().as_bytes().iter().map(to_priority).collect())
            .collect();

        let part1 = lines
            .iter()
            .map(|l| l.split_at(l.len() / 2))
            .map(|(l, r)| -> (HashSet<i64>, HashSet<i64>) {
                (l.iter().cloned().collect(), r.iter().cloned().collect())
            })
            .map(|(l, r)| l.intersection(&r).sum::<i64>())
            .sum::<i64>()
            .to_string();

        let part2 = lines
            .iter()
            .map(|l| l.iter().cloned().collect())
            .collect::<Vec<HashSet<i64>>>()
            .chunks(3)
            .map(|chunk| match chunk {
                [p1, p2, p3] => p1
                    .intersection(&p2.intersection(p3).into_iter().map(i64::clone).collect())
                    .last()
                    .unwrap()
                    .clone(),
                _ => 0,
            })
            .sum::<i64>()
            .to_string();

        Ok(Solution { part1, part2 })
    }
}
