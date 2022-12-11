use std::collections::VecDeque;
use std::io;
use std::ops::Mul;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::problem::Solution;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operand {
    Old,
    Num(i64),
}

impl FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            return Ok(Operand::Old);
        }
        if let Ok(num) = s.parse() {
            return Ok(Operand::Num(num));
        }
        Err(s.to_string())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operator {
    Add,
    Mul,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "*" => Operator::Mul,
            "+" => Operator::Add,
            _ => Err(s.to_string())?,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Operation {
    lhs: Operand,
    op: Operator,
    rhs: Operand,
}

impl Operation {
    fn apply(&self, val: i64) -> i64 {
        let &Operation { lhs, op, rhs } = self;
        let get_operand = |opnd| match opnd {
            Operand::Old => val,
            Operand::Num(n) => n,
        };
        let lhs = get_operand(lhs);
        let rhs = get_operand(rhs);
        match op {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test: i64,
    if_true: usize,
    if_false: usize,
}

impl TryFrom<&[String]> for Monkey {
    type Error = String;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref MONKEY: Regex = Regex::new(r"^Monkey \d+:$").unwrap();
            static ref OPERATION: Regex =
                Regex::new(r"^  Operation: new = (.*) (.) (.*)$").unwrap();
        }
        assert!(MONKEY.is_match(&lines[0]));
        let items = lines[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split_whitespace()
            .map(|ns| {
                if ns.ends_with(",") {
                    &ns[..ns.len() - 1]
                } else {
                    ns
                }
                .parse()
                .unwrap()
            })
            .collect();

        let operation = {
            let matches = OPERATION.captures(&lines[2]).unwrap();
            Operation {
                lhs: matches.get(1).unwrap().as_str().parse().unwrap(),
                op: matches.get(2).unwrap().as_str().parse().unwrap(),
                rhs: matches.get(3).unwrap().as_str().parse().unwrap(),
            }
        };

        let test = lines[3]
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();
        let if_true = lines[4]
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let if_false = lines[5]
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        Ok(Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
        })
    }
}

pub(super) fn solve(r: impl io::BufRead) -> io::Result<Solution> {
    let monkeys: Vec<Monkey> = r
        .lines()
        .collect::<io::Result<Vec<String>>>()?
        .chunks(7)
        .map(|c| c.try_into().unwrap())
        .collect();
    let num_monkeys = monkeys.len();

    let part1 = {
        let mut monkeys = monkeys.clone();
        let mut seen = vec![0i64; num_monkeys];

        for _ in 0..20 {
            for midx in 0..num_monkeys {
                while !monkeys[midx].items.is_empty() {
                    seen[midx] += 1;
                    let worry_level = monkeys[midx].items.pop_front().unwrap();
                    let worry_level = monkeys[midx].operation.apply(worry_level);
                    let worry_level = worry_level / 3;
                    let nidx = if worry_level % monkeys[midx].test == 0 {
                        monkeys[midx].if_true
                    } else {
                        monkeys[midx].if_false
                    };
                    monkeys[nidx].items.push_back(worry_level);
                }
            }
        }
        seen.sort();
        seen.iter()
            .rev()
            .take(2)
            .cloned()
            .fold(1, i64::mul)
            .to_string()
    };

    let part2 = {
        let mut monkeys = monkeys.clone();
        let mut seen = vec![0i64; num_monkeys];
        let reducer = monkeys.iter().map(|m| m.test).reduce(i64::mul).unwrap();

        for _ in 0..10000 {
            for midx in 0..num_monkeys {
                while !monkeys[midx].items.is_empty() {
                    seen[midx] += 1;
                    let worry_level = monkeys[midx].items.pop_front().unwrap();
                    let worry_level = monkeys[midx].operation.apply(worry_level);
                    let nidx = if worry_level % monkeys[midx].test == 0 {
                        monkeys[midx].if_true
                    } else {
                        monkeys[midx].if_false
                    };
                    monkeys[nidx].items.push_back(worry_level);
                }
            }
            monkeys
                .iter_mut()
                .for_each(|m| m.items.iter_mut().for_each(|n| *n = *n % reducer));
        }
        seen.sort();
        seen.iter()
            .rev()
            .take(2)
            .cloned()
            .fold(1, i64::mul)
            .to_string()
    };
    Ok(Solution { part1, part2 })
}
