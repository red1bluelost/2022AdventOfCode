use std::collections::HashSet;
use std::convert::identity;
use std::io;
use std::io::BufRead;
use std::ops::Add;

use crate::problem::{Solution, SolverImpl};

#[derive(Default)]
pub struct Day06Impl {}

impl Day06Impl {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run<const N: usize>(line: &str) -> usize {
        line.as_bytes()
            .windows(N)
            .map(|c| c.iter().cloned().collect::<HashSet<_>>().len() == N)
            .position(identity)
            .unwrap()
            .add(N)
    }
}

impl SolverImpl for Day06Impl {
    fn solve(self, r: impl BufRead) -> io::Result<Solution> {
        let line = r.lines().nth(0).unwrap()?;

        let part1 = Self::run::<4>(&line).to_string();
        let part2 = Self::run::<14>(&line).to_string();
        Ok(Solution { part1, part2 })
    }
}
