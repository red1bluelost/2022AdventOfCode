use std::io;

use crate::problem::Solution;

type Elem = (i8, bool, i64);
type Grid = Vec<Vec<Elem>>;

fn reset_grid_max(grid: &mut Grid) {
    grid.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|e| e.2 = e.0.into()));
}

fn reset_grid_vis_level(grid: &mut Grid) {
    grid.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|e| e.2 = 0));
}

fn check_adjecent((_, _, might_cover): Elem, (val, vis, max): &mut Elem) {
    if might_cover < (*val).into() {
        *vis = true;
    } else {
        *max = might_cover;
    }
}

fn check_elem_grid_level(c: usize, r: usize, grid: &mut Grid) -> i64 {
    let width = grid.len();
    let elem_height = grid[c][r].0;

    let mut acc = 1;

    let mut count = 0;
    for ri in r + 1..width {
        count += 1;
        if grid[c][ri].0 >= elem_height {
            break;
        }
    }
    acc *= count;

    let mut count = 0;
    for ri in (0..r).rev() {
        count += 1;
        if grid[c][ri].0 >= elem_height {
            break;
        }
    }
    acc *= count;

    let mut count = 0;
    for ci in c + 1..width {
        count += 1;
        if grid[ci][r].0 >= elem_height {
            break;
        }
    }
    acc *= count;

    let mut count = 0;
    for ci in (0..c).rev() {
        count += 1;
        if grid[ci][r].0 >= elem_height {
            break;
        }
    }
    acc *= count;

    acc
}

pub(super) fn solve(r: impl io::BufRead) -> io::Result<Solution> {
    let mut grid: Grid = r
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .map(|i: i8| (i, false, i.into()))
                .collect()
        })
        .collect();
    let width = grid.len();

    grid.first_mut()
        .unwrap()
        .iter_mut()
        .for_each(|e| e.1 = true);
    grid.last_mut().unwrap().iter_mut().for_each(|e| e.1 = true);
    grid.iter_mut().for_each(|v| {
        v.first_mut().unwrap().1 = true;
        v.last_mut().unwrap().1 = true;
    });

    for row in grid.iter_mut() {
        for i in 0..width - 1 {
            check_adjecent(row[i], &mut row[i + 1]);
        }
    }

    reset_grid_max(&mut grid);

    for row in grid.iter_mut() {
        for i in (1..width).rev() {
            check_adjecent(row[i], &mut row[i - 1]);
        }
    }

    reset_grid_max(&mut grid);

    for c in 0..width - 1 {
        for r in 0..width {
            check_adjecent(grid[c][r], &mut grid[c + 1][r]);
        }
    }

    reset_grid_max(&mut grid);

    for c in (1..width).rev() {
        for r in 0..width {
            check_adjecent(grid[c][r], &mut grid[c - 1][r]);
        }
    }

    let part1 = grid
        .iter()
        .map(|row| row.iter())
        .flatten()
        .filter(|e| e.1)
        .count()
        .to_string();

    reset_grid_vis_level(&mut grid);

    let grid_view = &mut grid;
    (1..width - 1).for_each(|c| {
        (1..width - 1).for_each(|r| {
            grid_view[c][r].2 = check_elem_grid_level(c, r, grid_view);
        })
    });

    let part2 = grid
        .iter()
        .map(|row| row.iter())
        .flatten()
        .map(|e| e.2)
        .max()
        .unwrap()
        .to_string();
    Ok(Solution { part1, part2 })
}
