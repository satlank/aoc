// https://adventofcode.com/2021/day/8

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::collections::HashSet;

#[macro_use]
extern crate scan_fmt;

struct Observation {
    patterns: Vec<HashSet<char>>,
    values: Vec<HashSet<char>>,
}

impl Observation {
    fn parse<S: AsRef<str>>(line: S) -> Observation {
        let (o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, v1, v2, v3, v4) = scan_fmt!(
            line.as_ref(),
            "{} {} {} {} {} {} {} {} {} {} | {} {} {} {}",
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String
        )
        .unwrap();
        Observation {
            patterns: vec![
                o1.chars().collect(),
                o2.chars().collect(),
                o3.chars().collect(),
                o4.chars().collect(),
                o5.chars().collect(),
                o6.chars().collect(),
                o7.chars().collect(),
                o8.chars().collect(),
                o9.chars().collect(),
                o10.chars().collect()
            ],
            values: vec![
                v1.chars().collect(), v2.chars().collect(), v3.chars().collect(), v4.chars().collect()],
        }
    }

    fn unscramble(&self) -> usize {
        let p1 = self.patterns.iter().filter(|v| v.len() == 2).next().unwrap();
        let p7 = self.patterns.iter().filter(|v| v.len() == 3).next().unwrap();
        let p4 = self.patterns.iter().filter(|v| v.len() == 4).next().unwrap();
        let p8 = self.patterns.iter().filter(|v| v.len() == 7).next().unwrap();

        let p3 = self.patterns.iter().filter(|v| v.len() == 5 && v.difference(&p1).count() == 3).next().unwrap();

        let p3p4: HashSet<char> = p3.union(p4).copied().collect();
        let p2 = self.patterns.iter().filter(|v| v.len() == 5 && v.difference(&p3p4).count() == 1).next().unwrap();
        let p5 = self.patterns.iter().filter(|v| v.len() == 5 && *v != p3 && v.difference(&p3p4).count() == 0).next().unwrap();

        let p9 = self.patterns.iter().filter(|v| *v == &p3p4).next().unwrap();
        let p0 = self.patterns.iter().filter(|v| v.len() == 6 && *v != p9 && v.difference(p1).count() == 4).next().unwrap();
        let p6 = self.patterns.iter().filter(|v| v.len() == 6 && *v != p9 && *v != p0).next().unwrap();

        let mut digit = 0;
        for i in 0..4 {
            digit *= 10;
            let v = &self.values[i];
            if v == p1 {
                digit += 1
            } else if v == p2 {
                digit += 2
            } else if v == p3 {
                digit += 3
            } else if v == p4 {
                digit += 4
            } else if v == p5 {
                digit += 5
            } else if v == p6 {
                digit += 6
            } else if v == p7 {
                digit += 7
            } else if v == p8 {
                digit += 8
            } else if v == p9 {
                digit += 9
            }
        }
        digit
    }
}

fn read<R: Read>(io: R) -> Vec<Observation> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| Observation::parse(line.unwrap()))
        .collect()
}

fn part_1(obs: &[Observation]) -> usize {
    obs.iter()
        .map(|o| o.values.iter().filter(|v| v.len() == 2 || v.len() == 3 || v.len() == 4 || v.len() == 7).count())
        .sum()
}

fn part_2(obs: &[Observation]) -> usize {
    obs.iter()
        .map(
            |o| o.unscramble()
        )
        .sum()
}

fn main() {
    let obs = read(File::open("input.txt").unwrap());
    let result = part_1(&obs);
    println!("Number of times 1, 4, 7, or 8 appear: {}", result);
    let result = part_2(&obs);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let nums = read(File::open("test1.txt").unwrap());
        assert_eq!(nums.len(), 10);
        let result = part_1(&nums);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part_2() {
        let nums = read(File::open("test1.txt").unwrap());
        assert_eq!(nums.len(), 10);
        let result = part_2(&nums);
        assert_eq!(result, 61229);
    }
}
