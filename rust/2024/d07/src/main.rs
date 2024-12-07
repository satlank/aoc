// https://adventofcode.com/2024/day/7

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use itertools::Itertools;

#[derive(Debug)]
enum Ops {
    Mul,
    Add,
    Concat,
}

impl ::std::fmt::Display for Ops {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Ops::Add => write!(f, "+"),
            Ops::Mul => write!(f, "*"),
            Ops::Concat => write!(f, "||"),
        }
    }
}

impl Ops {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Ops::Add => a + b,
            Ops::Mul => a * b,
            Ops::Concat => format!("{}{}", a, b).parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Calibration {
    target: usize,
    values: Vec<usize>,
}

impl Calibration {
    fn is_valid(&self, base_ops: &[Ops]) -> bool {
        // NB: NOT .combinations_with_replacement or .permutations or any of that, that produces
        // too many equivalent results and it takes forever to run
        let ops =
            std::iter::repeat_n(base_ops.iter(), self.values.len() - 1).multi_cartesian_product();
        for attempt in ops {
            let mut res = self.values[0];
            for (i, op) in attempt.iter().enumerate() {
                res = op.apply(res, self.values[i + 1]);
                if res > self.target {
                    break;
                }
            }
            if res == self.target {
                return true;
            }
        }

        false
    }
}

impl ::std::str::FromStr for Calibration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let target = parts.next().unwrap().parse().unwrap();
        let values = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|p| p.parse().unwrap())
            .collect();
        Ok(Calibration { target, values })
    }
}

fn read<R: Read>(io: R) -> Vec<Calibration> {
    let br = BufReader::new(io);
    br.lines()
        .map_while(Result::ok)
        .map(|s| s.parse::<Calibration>().unwrap())
        .collect()
}

fn part_1(input: &[Calibration]) -> usize {
    input
        .iter()
        .filter_map(|c| {
            if c.is_valid(&[Ops::Mul, Ops::Add]) {
                Some(c.target)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(input: &[Calibration]) -> usize {
    input
        .iter()
        .filter_map(|c| {
            if c.is_valid(&[Ops::Mul, Ops::Add, Ops::Concat]) {
                Some(c.target)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input);
    println!("Part 1: {}", p1);
    let p2 = part_2(&input);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input), 3749);
        assert_eq!(part_2(&input), 11387);
    }
}
