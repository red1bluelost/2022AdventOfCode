mod problem;
mod solver;

use std::io;

fn main() -> io::Result<()> {
    let problem::Solution { part1, part2 } =
        solver::Solver::get_day(1).solve(io::stdin().lock())?;
    println!("part 1 = {}", part1);
    println!("part 1 = {}", part2);
    Ok(())
}

/*
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
 */
