use crate::problem::{Solution, SolverImpl};
use std::io;

#[derive(Default)]
pub struct Day01Impl {}

impl Day01Impl {
    pub fn new() -> Self {
        Default::default()
    }
}

impl SolverImpl for Day01Impl {
    fn solve(self, r: impl io::BufRead) -> io::Result<Solution> {
        let mut v = vec![0];

        for line in r.lines().flatten() {
            if line.is_empty() {
                v.push(0);
            } else {
                *v.last_mut().unwrap() += line.trim().parse::<u64>().unwrap();
            }
        }

        v.sort_unstable();

        let part1 = v.iter().last().unwrap().to_string();
        let part2 = v.iter().rev().take(3).sum::<u64>().to_string();
        Ok(Solution { part1, part2 })
    }
}
