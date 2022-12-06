mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

use crate::problem;
use crate::problem::SolverImpl;
use day01::Day01Impl;
use day02::Day02Impl;
use day03::Day03Impl;
use day04::Day04Impl;
use day05::Day05Impl;
use std::io;

pub enum Solver {
    Day01(Day01Impl),
    Day02(Day02Impl),
    Day03(Day03Impl),
    Day04(Day04Impl),
    Day05(Day05Impl),
}

impl Solver {
    pub fn get_day(day: i32) -> Solver {
        use Solver::*;
        match day {
            1 => Day01(Day01Impl::new()),
            2 => Day02(Day02Impl::new()),
            3 => Day03(Day03Impl::new()),
            4 => Day04(Day04Impl::new()),
            5 => Day05(Day05Impl::new()),
            _ => todo!(),
        }
    }

    pub fn solve(self, r: impl io::BufRead) -> io::Result<problem::Solution> {
        use Solver::*;
        match self {
            Day01(solver_impl) => solver_impl.solve(r),
            Day02(solver_impl) => solver_impl.solve(r),
            Day03(solver_impl) => solver_impl.solve(r),
            Day04(solver_impl) => solver_impl.solve(r),
            Day05(solver_impl) => solver_impl.solve(r),
        }
    }
}
