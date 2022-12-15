use std::fmt::{Display, Formatter, Write};
use std::{cmp, io};

use crate::problem::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        if end.x < start.x || end.y < start.y {
            Self::new(end, start)
        } else {
            Self { start, end }
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.x != self.end.x && self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x && self.start.y != self.end.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
    Empty,
    Wall,
    Sand,
}

impl Display for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Slot::Empty => '.',
            Slot::Wall => '#',
            Slot::Sand => 'o',
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SandMove {
    Move,
    Stop,
    Gone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid(Vec<Vec<Slot>>);

impl Grid {
    fn new(Point { x, y }: Point) -> Grid {
        Self(vec![
            vec![Slot::Empty; usize::try_from(x).unwrap()];
            usize::try_from(y).unwrap()
        ])
    }

    fn get(&self, Point { x, y }: Point) -> Slot {
        self.0[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()]
    }

    fn try_get(&self, p @ Point { x, y }: Point) -> Option<Slot> {
        if x < 0
            || y < 0
            || x >= self.0[0].len().try_into().unwrap()
            || y >= self.0.len().try_into().unwrap()
        {
            None
        } else {
            Some(self.get(p))
        }
    }

    fn set(&mut self, Point { x, y }: Point, s: Slot) {
        self.0[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()] = s;
    }

    fn fill_line(&mut self, l: Line) {
        if l.is_horizontal() {
            for c in l.start.x..=l.end.x {
                self.set(Point::new(c, l.start.y), Slot::Wall);
            }
        } else if l.is_vertical() {
            for r in l.start.y..=l.end.y {
                self.set(Point::new(l.start.x, r), Slot::Wall);
            }
        } else {
            panic!("can't handle diagonal line: {:?}", l);
        }
    }

    fn try_move(&self, sand_point: Point) -> (Point, SandMove) {
        for next in [
            Point::new(sand_point.x, sand_point.y + 1),
            Point::new(sand_point.x - 1, sand_point.y + 1),
            Point::new(sand_point.x + 1, sand_point.y + 1),
        ] {
            match self.try_get(next) {
                None => return (next, SandMove::Gone),
                Some(Slot::Empty) => return (next, SandMove::Move),
                Some(Slot::Wall | Slot::Sand) => {}
            }
        }
        (sand_point, SandMove::Stop)
    }

    /// true if the sand landed on the board
    fn drop_sand(&mut self, drop_point: Point) -> Option<Point> {
        let mut pos = drop_point;
        loop {
            let (new_pos, sand_move) = self.try_move(pos);
            match sand_move {
                SandMove::Move => pos = new_pos,
                SandMove::Stop => {
                    self.set(new_pos, Slot::Sand);
                    return Some(new_pos);
                }
                SandMove::Gone => return None,
            }
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for v in row {
                write!(f, "{}", v)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn simulate_part1(mut grid: Grid, pour_point: Point) -> i64 {
    let mut count = 0;
    while grid.drop_sand(pour_point).is_some() {
        count += 1;
    }
    count
}

fn simulate_part2(mut grid: Grid, pour_point: Point) -> i64 {
    let mut count = 0;
    while grid.drop_sand(pour_point).map(|p| p != pour_point).unwrap() {
        count += 1;
    }
    count + 1
}

pub(super) fn solve(r: impl io::BufRead) -> io::Result<Solution> {
    let mut max_point = Point {
        x: i64::MIN,
        y: i64::MIN,
    };
    let mut lines = vec![];

    for line in r.lines() {
        let points: Vec<_> = line?
            .split_whitespace()
            .flat_map(|s| {
                if s == "->" {
                    None
                } else {
                    let (l, r) = s.split_at(s.chars().position(|c| c == ',').unwrap());
                    Some(Point::new(l.parse().unwrap(), (&r[1..]).parse().unwrap()))
                }
            })
            .collect();

        for &p in points.iter() {
            max_point.x = cmp::max(max_point.x, p.x);
            max_point.y = cmp::max(max_point.y, p.y);
        }

        for ps in points.windows(2) {
            if let &[l, r] = ps {
                lines.push(Line::new(l, r))
            }
        }
    }

    let pour_point = Point::new(500, 0);

    let grid_size = Point::new(max_point.y + max_point.x + 1, max_point.y + 3);

    let mut grid = Grid::new(grid_size);
    lines.iter().for_each(|l| grid.fill_line(*l));

    let part1 = simulate_part1(grid.clone(), pour_point).to_string();

    let bottom_line = Line::new(
        Point::new(0, max_point.y + 2),
        Point::new(max_point.y + max_point.x, max_point.y + 2),
    );
    grid.fill_line(bottom_line);

    let part2 = simulate_part2(grid, pour_point).to_string();
    Ok(Solution { part1, part2 })
}
