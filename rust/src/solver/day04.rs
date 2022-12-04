use std::io;

use crate::problem::{Solution, SolverImpl};

#[derive(Default)]
pub struct Day04Impl {}

impl Day04Impl {
    pub fn new() -> Self {
        Default::default()
    }
}

impl SolverImpl for Day04Impl {
    fn solve(self, r: impl io::BufRead) -> io::Result<Solution> {
        let part1 = "TODO".to_string();
        let part2 = "TODO".to_string();
        Ok(Solution { part1, part2 })
    }
}
