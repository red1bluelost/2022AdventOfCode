mod day03;
mod day1;
mod day2;

use crate::problem;
use crate::problem::SolverImpl;
use day03::Day03Impl;
use day1::Day1Impl;
use day2::Day2Impl;
use std::io;

pub enum Solver {
    Day1(Day1Impl),
    Day2(Day2Impl),
    Day03(Day03Impl),
}

impl Solver {
    pub fn get_day(day: i32) -> Solver {
        use Solver::*;
        match day {
            1 => Day1(Day1Impl::new()),
            2 => Day2(Day2Impl::new()),
            3 => Day03(Day03Impl::new()),
            _ => todo!(),
        }
    }

    pub fn solve(self, r: impl io::BufRead) -> io::Result<problem::Solution> {
        use Solver::*;
        match self {
            Day1(solver_impl) => solver_impl.solve(r),
            Day2(solver_impl) => solver_impl.solve(r),
            Day03(solver_impl) => solver_impl.solve(r),
        }
    }
}
