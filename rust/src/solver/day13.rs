use std::cmp::Ordering;
use std::io;
use std::ops::Add;
use std::str::FromStr;

use crate::problem::Solution;

#[derive(Eq, PartialEq, Debug, Clone)]
enum Elem {
    Num(i8),
    List(Vec<Elem>),
}

impl Elem {
    fn parse_str(s: &str) -> (Elem, &str) {
        use Elem::*;
        if s.chars().nth(0).unwrap() == '[' {
            let mut list = vec![];
            let mut rem = &s[1..];
            while rem.chars().nth(0).map_or(false, |c| c != ']') {
                let (e, rs) = Self::parse_str(rem);
                rem = if rs.chars().nth(0).map_or(false, |c| c == ',') {
                    &rs[1..]
                } else {
                    rs
                };
                list.push(e);
            }
            assert!(rem.chars().nth(0).map_or(false, |c| c == ']'));
            (List(list), &rem[1..])
        } else {
            let (l, r) = s.split_at(s.chars().take_while(|&c| c.is_digit(10)).count());
            (Num(l.parse().unwrap()), r)
        }
    }
}

impl ToString for Elem {
    fn to_string(&self) -> String {
        match self {
            Elem::Num(v) => v.to_string(),
            Elem::List(l) => {
                "[".to_string()
                    + &l.iter()
                        .map(Elem::to_string)
                        .reduce(|acc, r| acc + "," + &r)
                        .unwrap_or("".to_string())
                    + "]"
            }
        }
    }
}

impl FromStr for Elem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Input is empty".to_string());
        }
        let (e, rs) = Elem::parse_str(s);
        if rs.is_empty() {
            Ok(e)
        } else {
            Err(rs.to_string())
        }
    }
}

fn ordering(lhs: &Elem, rhs: &Elem) -> Ordering {
    use Elem::*;
    match (lhs, rhs) {
        (Num(l), Num(r)) => l.cmp(r),
        (l @ Num(_), r @ List(_)) => ordering(&List(vec![l.clone()]), r),
        (l @ List(_), r @ Num(_)) => ordering(l, &List(vec![r.clone()])),
        (List(l), List(r)) => {
            for (l, r) in l.iter().zip(r.iter()) {
                match ordering(l, r) {
                    Ordering::Equal => continue,
                    ord => return ord,
                }
            }
            l.len().cmp(&r.len())
        }
    }
}

pub(super) fn solve(r: impl io::BufRead) -> io::Result<Solution> {
    let lines = r.lines().collect::<io::Result<Vec<_>>>()?;

    let mut packets: Vec<Elem> = lines
        .chunks(3)
        .flat_map(|l| l.iter().take(2).map(|l| l.parse().unwrap()))
        .collect();

    let part1 = packets
        .chunks(2)
        .map(|s| ordering(&s[0], &s[1]))
        .enumerate()
        .filter(|(_, o)| *o == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum::<usize>()
        .to_string();

    let key0: Elem = "[[2]]".parse().unwrap();
    packets.push(key0.clone());
    let key1: Elem = "[[6]]".parse().unwrap();
    packets.push(key1.clone());
    packets.sort_by(ordering);

    let key0loc = packets.iter().position(|e| e == &key0).unwrap().add(1);
    let key1loc = packets.iter().position(|e| e == &key1).unwrap().add(1);
    let part2 = (key0loc * key1loc).to_string();

    Ok(Solution { part1, part2 })
}
