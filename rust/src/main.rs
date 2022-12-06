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
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
 */
