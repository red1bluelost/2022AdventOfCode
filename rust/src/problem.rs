use std::io;

pub struct Solution {
    pub part1: String,
    pub part2: String,
}

pub trait SolverImpl {
    fn solve(self, r: impl io::BufRead) -> io::Result<Solution>;
}
