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
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
 */
