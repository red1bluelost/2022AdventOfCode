use std::io;

use crate::problem::Solution;

pub(super) fn solve(r: impl io::BufRead) -> io::Result<Solution> {
    let lines: Vec<[i64; 4]> = r
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .split(',')
                .flat_map(|p| p.split('-'))
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>()
                .try_into()
        })
        .collect();

    let part1 = lines
        .iter()
        .filter(|&&[a, b, c, d]| (a <= c && b >= d) || (c <= a && d >= b))
        .count()
        .to_string();

    let part2 = lines
        .iter()
        .filter(|&&[a, b, c, d]| {
            (a >= c && a <= d) || (b >= c && b <= d) || (c >= a && c <= b) || (d >= a && d <= b)
        })
        .count()
        .to_string();
    Ok(Solution { part1, part2 })
}
