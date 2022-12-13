use std::io;

use crate::problem;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

pub struct Solver(i32);

impl Solver {
    pub fn get_day(day: i32) -> Solver {
        Solver(day)
    }

    pub fn solve(self, r: impl io::BufRead) -> io::Result<problem::Solution> {
        match self.0 {
            1 => day01::solve(r),
            2 => day02::solve(r),
            3 => day03::solve(r),
            4 => day04::solve(r),
            5 => day05::solve(r),
            6 => day06::solve(r),
            7 => day07::solve(r),
            8 => day08::solve(r),
            9 => day09::solve(r),
            10 => day10::solve(r),
            11 => day11::solve(r),
            12 => day12::solve(r),
            _ => todo!(),
        }
    }
}
