use std::collections::HashSet;
use std::io;
use std::ops::{Add, AddAssign, Sub};
use std::str::FromStr;

use crate::problem::Solution;

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
struct Point(i32, i32);

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(mut self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => self.0 += 1,
            Direction::Down => self.0 -= 1,
            Direction::Left => self.1 += 1,
            Direction::Right => self.1 -= 1,
        }
        self
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

fn pull_tail(head: Point, tail: Point) -> Point {
    let Point(vtd, hrd) = head - tail;
    if vtd.abs() <= 1 && hrd.abs() <= 1 {
        return tail;
    }

    let calc_dim = |dis: i32, pos: i32| -> i32 {
        pos + match dis.abs() {
            0 => 0,
            1 => dis,
            2 => dis / 2,
            _ => panic!(
                "invalid distance: head({:?}), tail({:?}), dist({:?})",
                head,
                tail,
                Point(vtd, hrd)
            ),
        }
    };

    Point(calc_dim(vtd, tail.0), calc_dim(hrd, tail.1))
}

type Move = (Direction, i32);

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        Ok(match s {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => Err(format!("invalid direction char {}", s))?,
        })
    }
}

pub(super) fn solve(r: impl io::BufRead) -> io::Result<Solution> {
    let moves: Vec<Move> = r
        .lines()
        .flatten()
        .map(|l| {
            let mut words = l.split_whitespace();
            (
                words.next().unwrap().parse().unwrap(),
                words.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let part1 = {
        let mut head_loc = Point::default();
        let mut tail_loc = Point::default();

        let mut location_set = HashSet::from([Point::default()]);
        for &(dir, count) in moves.iter() {
            for _ in 0..count {
                head_loc += dir;
                tail_loc = pull_tail(head_loc, tail_loc);
                location_set.insert(tail_loc);
            }
        }
        location_set.len().to_string()
    };
    let part2 = {
        let mut locs: [Point; 10] = Default::default();

        let mut location_set = HashSet::from([Point::default()]);
        for &(dir, count) in moves.iter() {
            for _ in 0..count {
                locs[0] += dir;
                for i in 0..locs.len() - 1 {
                    locs[i + 1] = pull_tail(locs[i], locs[i + 1]);
                }
                location_set.insert(locs[locs.len() - 1]);
            }
        }
        location_set.len().to_string()
    };
    Ok(Solution { part1, part2 })
}
