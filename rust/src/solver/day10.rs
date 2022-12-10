use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::problem::Solution;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }
        if s.starts_with("addx ") {
            return Ok(Instruction::Addx(
                s.split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse()
                    .map_err(|err: ParseIntError| err.to_string())?,
            ));
        }
        Err(format!("Invalid instruction: {}", s))
    }
}

pub(super) fn solve(r: impl io::BufRead) -> io::Result<Solution> {
    let insts: Vec<Instruction> = r.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    let cycles = {
        let mut cycles = vec![1];
        for &inst in insts.iter() {
            let &cur_val = cycles.last().unwrap();
            match inst {
                Instruction::Noop => {
                    cycles.push(cur_val);
                }
                Instruction::Addx(inst_val) => {
                    cycles.push(cur_val);
                    cycles.push(cur_val + inst_val);
                }
            }
        }
        cycles
    };

    let part1 = cycles
        .iter()
        .enumerate()
        .filter(|&(idx, _)| (idx + 21) % 40 == 0)
        .map(|(idx, &val): (usize, &i64)| -> i64 { i64::try_from(idx + 1).unwrap() * val })
        .sum::<i64>()
        .to_string();
    let part2 = cycles
        .chunks(40)
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(idx, &val)| (i64::try_from(idx).unwrap(), val))
                .map(|(cycle, val)| cycle >= val - 1 && cycle <= val + 1)
                .map(|b| if b { '#' } else { '.' })
                .collect::<String>()
        })
        .fold("\n".to_string(), |acc, s| acc + &s + "\n");
    Ok(Solution { part1, part2 })
}
