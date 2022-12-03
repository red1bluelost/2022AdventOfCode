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
    fn solve(self, mut r: impl io::BufRead) -> io::Result<Solution> {
        let mut v = vec![0];

        let mut buf = String::new();
        loop {
            buf.clear();
            let s = r.read_line(&mut buf)?;
            match s {
                0 => {
                    break;
                }
                1 => {
                    v.push(0);
                }
                _ => {
                    *v.last_mut().unwrap() += buf.trim().parse::<u64>().unwrap();
                }
            }
        }

        v.sort_unstable();

        let part1 = v.iter().last().unwrap().to_string();
        let part2 = v.iter().rev().take(3).sum::<u64>().to_string();
        Ok(Solution { part1, part2 })
    }
}
