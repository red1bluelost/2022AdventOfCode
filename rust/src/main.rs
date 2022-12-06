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
mjqjpqmgbljsphdztnvjfqwrcgsmlb    7 19
bvwbjplbgvbhsrlpgdmjqwftvncz      5 23
nppdvjthqldpwncqszvftbrmjlhg      6 23
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg 10 29
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw  11 26
 */
