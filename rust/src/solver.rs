mod day01;
mod day02;
mod day03;

use crate::problem;
use crate::problem::SolverImpl;
use day01::Day01Impl;
use day02::Day02Impl;
use day03::Day03Impl;
use std::io;

pub enum Solver {
    Day01(Day01Impl),
    Day02(Day02Impl),
    Day03(Day03Impl),
}

impl Solver {
    pub fn get_day(day: i32) -> Solver {
        use Solver::*;
        match day {
            1 => Day01(Day01Impl::new()),
            2 => Day02(Day02Impl::new()),
            3 => Day03(Day03Impl::new()),
            _ => todo!(),
        }
    }

    pub fn solve(self, r: impl io::BufRead) -> io::Result<problem::Solution> {
        use Solver::*;
        match self {
            Day01(solver_impl) => solver_impl.solve(r),
            Day02(solver_impl) => solver_impl.solve(r),
            Day03(solver_impl) => solver_impl.solve(r),
        }
    }
}
