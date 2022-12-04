use crate::problem::{Solution, SolverImpl};
use crate::solver::day02::Moves::{Paper, Rock, Scissors};
use std::io;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum MatchResult {
    Lose,
    Draw,
    Win,
}

impl MatchResult {
    fn score(&self) -> i64 {
        use MatchResult::*;
        match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }
    fn opposite(&self) -> Self {
        use MatchResult::*;
        match self {
            Lose => Win,
            Draw => Draw,
            Win => Lose,
        }
    }
}

impl From<char> for MatchResult {
    fn from(c: char) -> Self {
        use MatchResult::*;
        match c {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => todo!(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

impl Moves {
    fn to_shape_score(&self) -> i64 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn play(&self, other: Self) -> i64 {
        use MatchResult::*;
        match (self, other) {
            (Rock, Scissors) => Win,
            (Scissors, Paper) => Win,
            (Paper, Rock) => Win,
            (Scissors, Rock) => Lose,
            (Paper, Scissors) => Lose,
            (Rock, Paper) => Lose,
            _ => Draw,
        }
        .score()
    }

    fn other_move(&self, res: MatchResult) -> Self {
        use MatchResult::*;
        match (self, res) {
            (m, Draw) => *m,
            (Rock, Win) => Scissors,
            (Paper, Win) => Rock,
            (Scissors, Win) => Paper,
            (Rock, Lose) => Paper,
            (Paper, Lose) => Scissors,
            (Scissors, Lose) => Rock,
        }
    }
}

impl From<char> for Moves {
    fn from(c: char) -> Self {
        use Moves::*;
        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => todo!(),
        }
    }
}

#[derive(Default)]
pub struct Day02Impl {}

impl Day02Impl {
    pub fn new() -> Self {
        Default::default()
    }
}

impl SolverImpl for Day02Impl {
    fn solve(self, r: impl io::BufRead) -> io::Result<Solution> {
        let chars: Vec<[char; 2]> = r
            .lines()
            .flatten()
            .flat_map(|l| {
                l.split_whitespace()
                    .flat_map(|c| c.chars().next())
                    .collect::<Vec<char>>()
                    .try_into()
            })
            .collect();

        let part1 = chars
            .iter()
            .map(|&[f, s]| (Moves::from(f), Moves::from(s)))
            .map(|(o, y)| y.play(o) + y.to_shape_score())
            .sum::<i64>()
            .to_string();

        let part2 = chars
            .iter()
            .map(|&[f, s]| (Moves::from(f), MatchResult::from(s)))
            .map(|(o, r)| r.score() + o.other_move(r.opposite()).to_shape_score())
            .sum::<i64>()
            .to_string();

        Ok(Solution { part1, part2 })
    }
}
