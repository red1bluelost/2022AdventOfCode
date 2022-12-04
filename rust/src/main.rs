use std::io;

mod problem;
mod solver;

fn main() -> io::Result<()> {
    let day = std::env::args().nth(1).unwrap().parse().unwrap();

    let problem::Solution { part1, part2 } =
        solver::Solver::get_day(day).solve(io::stdin().lock())?;
    println!("part 1 = {}", part1);
    println!("part 2 = {}", part2);
    Ok(())
}

/*
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
 */
