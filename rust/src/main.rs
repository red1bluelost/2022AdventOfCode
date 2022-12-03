mod problem;
mod solver;

use std::io;

fn main() -> io::Result<()> {
    let problem::Solution { part1, part2 } =
        solver::Solver::get_day(2).solve(io::stdin().lock())?;
    println!("part 1 = {}", part1);
    println!("part 2 = {}", part2);
    Ok(())
}

/*
A Y
B X
C Z
 */
