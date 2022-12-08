use std::collections::HashSet;
use std::convert::identity;
use std::io;
use std::io::BufRead;
use std::ops::Add;

use crate::problem::Solution;

fn run<const N: usize>(line: &str) -> usize {
    line.as_bytes()
        .windows(N)
        .map(|c| c.iter().cloned().collect::<HashSet<_>>().len() == N)
        .position(identity)
        .unwrap()
        .add(N)
}

pub fn solve(r: impl BufRead) -> io::Result<Solution> {
    let line = r.lines().nth(0).unwrap()?;

    let part1 = run::<4>(&line).to_string();
    let part2 = run::<14>(&line).to_string();
    Ok(Solution { part1, part2 })
}
