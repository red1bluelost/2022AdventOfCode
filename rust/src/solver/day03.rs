use std::collections::HashSet;
use std::io;

use crate::problem::Solution;

pub fn solve(r: impl io::BufRead) -> io::Result<Solution> {
    let lines: Vec<Vec<i64>> = r
        .lines()
        .flatten()
        .map(|l| {
            l.trim()
                .as_bytes()
                .iter()
                .cloned()
                .map(|c| (c - if c.is_ascii_uppercase() { 38 } else { 96 }).into())
                .collect()
        })
        .collect();

    let part1 = lines
        .iter()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(l, r)| -> (HashSet<i64>, HashSet<i64>) {
            (l.iter().cloned().collect(), r.iter().cloned().collect())
        })
        .map(|(l, r)| l.intersection(&r).sum::<i64>())
        .sum::<i64>()
        .to_string();

    let part2 = lines
        .iter()
        .map(|l| l.iter().cloned().collect())
        .collect::<Vec<HashSet<i64>>>()
        .chunks(3)
        .map(|chunk| match chunk {
            [p1, p2, p3] => p1
                .intersection(&p2.intersection(p3).into_iter().map(i64::clone).collect())
                .last()
                .unwrap()
                .clone(),
            _ => 0,
        })
        .sum::<i64>()
        .to_string();

    Ok(Solution { part1, part2 })
}
