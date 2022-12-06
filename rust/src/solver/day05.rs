use std::borrow::Borrow;
use std::io;
use std::iter;
use std::ops::{Add, Div, Sub};

use regex::Regex;

use crate::problem::{Solution, SolverImpl};

#[derive(Clone)]
struct Boxes(Vec<Vec<char>>);

impl Boxes {
    fn tops(&self) -> String {
        self.0.iter().map(|b| b.last().unwrap().clone()).collect()
    }

    fn simulate_part1(mut self, moves: &[Move]) -> Boxes {
        for &[count, from, to] in moves {
            for _ in 0..count {
                let val = self.0[from - 1].pop().unwrap();
                self.0[to - 1].push(val);
            }
        }
        self
    }

    fn simulate_part2(mut self, moves: &[Move]) -> Boxes {
        for &[count, from, to] in moves {
            let mut buf = vec![];
            for _ in 0..count {
                buf.push(self.0[from - 1].pop().unwrap());
            }
            buf.iter().rev().for_each(|&val| self.0[to - 1].push(val));
        }
        self
    }
}

impl From<Vec<Vec<char>>> for Boxes {
    fn from(v: Vec<Vec<char>>) -> Self {
        Self(v)
    }
}

type Move = [usize; 3];

#[derive(Default)]
pub struct Day05Impl {}

impl Day05Impl {
    pub fn new() -> Self {
        Default::default()
    }

    fn parse_boxes(boxes: &[String]) -> Boxes {
        let length = boxes.last().unwrap().len().add(1).div(4);
        let boxes_re = Regex::new(
            iter::repeat(r"(   |\[.])")
                .take(length - 1)
                .fold(r"^(   |\[.])".to_string(), |l, r| l + " " + r)
                .borrow(),
        )
        .unwrap();

        boxes
            .iter()
            .take(boxes.len().sub(1))
            .map(|s| {
                boxes_re
                    .captures(s)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .flatten()
                    .map(|i| i.as_str().chars().nth(1).unwrap())
                    .collect()
            })
            .rev()
            .fold(
                iter::repeat(vec![]).take(length).collect(),
                |mut acc: Vec<_>, row: Vec<char>| {
                    for (i, &c) in row.iter().enumerate().filter(|(_, &c)| c != ' ') {
                        acc[i].push(c);
                    }
                    acc
                },
            )
            .into()
    }

    fn parse_moves(moves: &[String]) -> Vec<Move> {
        let moves_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        moves
            .iter()
            .map(|s| {
                moves_re
                    .captures(s)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|s| s.unwrap().as_str().parse().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect()
    }

    fn parse_lines(lines: Vec<String>) -> (Boxes, Vec<Move>) {
        let (boxes, moves) = lines.split_at(lines.iter().position(String::is_empty).unwrap());
        (Self::parse_boxes(boxes), Self::parse_moves(&moves[1..]))
    }
}

impl SolverImpl for Day05Impl {
    fn solve(self, r: impl io::BufRead) -> io::Result<Solution> {
        let (boxes, moves) = Self::parse_lines(r.lines().collect::<io::Result<_>>()?);

        let part1 = boxes.clone().simulate_part1(&moves).tops();
        let part2 = boxes.simulate_part2(&moves).tops();
        Ok(Solution { part1, part2 })
    }
}
