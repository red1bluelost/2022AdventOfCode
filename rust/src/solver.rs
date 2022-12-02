mod day1;

use crate::problem;
use crate::problem::SolverImpl;
use day1::Day1Impl;
use std::io;

pub enum Solver {
    Day1(Day1Impl),
}

impl Solver {
    pub fn get_day(day: i32) -> Solver {
        use Solver::*;
        match day {
            1 => Day1(Day1Impl::new()),
            _ => todo!(),
        }
    }

    pub fn solve(self, r: impl io::BufRead) -> io::Result<problem::Solution> {
        use Solver::*;
        match self {
            Day1(solver_impl) => solver_impl.solve(r),
        }
    }
}
