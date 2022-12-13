use std::collections::VecDeque;
use std::io;

use crate::problem::Solution;

#[derive(Clone, Debug)]
struct Grid {
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Grid {
    fn shortest_path(&self) -> i64 {
        let mut runner = SPRunner::new(&self.grid, |f, t| t > f + 1);
        runner.mark_start(self.start);
        runner.run_loop();
        runner.get_result(self.end)
    }

    fn shortest_hike(&self) -> i64 {
        let mut runner = SPRunner::new(&self.grid, |f, t| t < f - 1);
        runner.mark_start(self.end);
        runner.run_loop();
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(ri, row)| {
                row.iter().enumerate().flat_map(
                    move |(ci, val)| {
                        if *val == b'a' {
                            Some((ri, ci))
                        } else {
                            None
                        }
                    },
                )
            })
            .map(|(r, c)| runner.results[r][c])
            .min()
            .unwrap()
    }
}

struct SPRunner<'a> {
    results: Vec<Vec<i64>>,
    seen: Vec<Vec<bool>>,
    queue: VecDeque<(usize, usize)>,
    grid: &'a Vec<Vec<u8>>,
    mover: fn(u8, u8) -> bool,
}

impl<'a> SPRunner<'a> {
    fn new(grid: &'a Vec<Vec<u8>>, mover: fn(u8, u8) -> bool) -> Self {
        let rows = grid.len();
        let cols = grid[0].len();
        Self {
            results: vec![vec![i64::MAX; cols]; rows],
            seen: vec![vec![false; cols]; rows],
            queue: Default::default(),
            grid,
            mover,
        }
    }

    fn get_result(&self, (row, col): (usize, usize)) -> i64 {
        self.results[row][col]
    }

    fn up(&self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
        if row > 0 {
            Some((row - 1, col))
        } else {
            None
        }
    }

    fn down(&self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
        if row < self.grid.len() - 1 {
            Some((row + 1, col))
        } else {
            None
        }
    }

    fn left(&self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
        if col > 0 {
            Some((row, col - 1))
        } else {
            None
        }
    }

    fn right(&self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
        if col < self.grid[0].len() - 1 {
            Some((row, col + 1))
        } else {
            None
        }
    }

    fn move_impl(
        &self,
        fpos @ (frow, fcol): (usize, usize),
        tpost: impl FnOnce(&Self, (usize, usize)) -> Option<(usize, usize)>,
    ) -> Option<i64> {
        let tpos @ (trow, tcol) = tpost(self, fpos)?;
        if (self.mover)(self.grid[frow][fcol], self.grid[trow][tcol]) {
            return None;
        }
        Some(self.get_result(tpos))
    }

    fn move_up(&self, pos: (usize, usize)) -> Option<i64> {
        self.move_impl(pos, Self::up)
    }

    fn move_down(&self, pos: (usize, usize)) -> Option<i64> {
        self.move_impl(pos, Self::down)
    }

    fn move_left(&self, pos: (usize, usize)) -> Option<i64> {
        self.move_impl(pos, Self::left)
    }

    fn move_right(&self, pos: (usize, usize)) -> Option<i64> {
        self.move_impl(pos, Self::right)
    }

    fn push_adjacent(&mut self, pos @ (row, col): (usize, usize)) {
        if self.move_up(pos).map_or(false, |v| v == i64::MAX) && !self.seen[row - 1][col] {
            self.seen[row - 1][col] = true;
            self.queue.push_back((row - 1, col));
        }
        if self.move_down(pos).map_or(false, |v| v == i64::MAX) && !self.seen[row + 1][col] {
            self.seen[row + 1][col] = true;
            self.queue.push_back((row + 1, col));
        }
        if self.move_left(pos).map_or(false, |v| v == i64::MAX) && !self.seen[row][col - 1] {
            self.seen[row][col - 1] = true;
            self.queue.push_back((row, col - 1));
        }
        if self.move_right(pos).map_or(false, |v| v == i64::MAX) && !self.seen[row][col + 1] {
            self.seen[row][col + 1] = true;
            self.queue.push_back((row, col + 1));
        }
    }

    fn mark_start(&mut self, pos @ (row, col): (usize, usize)) {
        self.results[row][col] = 0;
        self.seen[row][col] = true;
        self.push_adjacent(pos);
    }

    fn process(&mut self, pos @ (row, col): (usize, usize)) {
        self.results[row][col] = [Self::up, Self::down, Self::left, Self::right]
            .iter()
            .zip([
                Self::move_down,
                Self::move_up,
                Self::move_right,
                Self::move_left,
            ])
            .map(|(dir, move_dir)| {
                let po = dir(self, pos)?;
                move_dir(self, po)?;
                Some(self.get_result(po))
            })
            .flatten()
            .min()
            .map_or(i64::MAX - 1, |v| v + 1);
        self.push_adjacent(pos);
    }

    fn run_loop(&mut self) {
        while !self.queue.is_empty() {
            let pos = self.queue.pop_front().unwrap();
            self.process(pos);
        }
    }
}

pub(super) fn solve(r: impl io::BufRead) -> io::Result<Solution> {
    let grid = {
        let mut start = None;
        let mut end = None;
        Grid {
            grid: r
                .lines()
                .enumerate()
                .map(|(row, l)| {
                    l.unwrap()
                        .as_bytes()
                        .iter()
                        .enumerate()
                        .map(|(col, &b)| match b {
                            b'S' => {
                                start = Some((row, col));
                                b'a'
                            }
                            b'E' => {
                                end = Some((row, col));
                                b'z'
                            }
                            _ => b,
                        })
                        .collect()
                })
                .collect(),
            start: start.unwrap(),
            end: end.unwrap(),
        }
    };

    let part1 = grid.shortest_path().to_string();
    let part2 = grid.shortest_hike().to_string();
    Ok(Solution { part1, part2 })
}
