// https://adventofcode.com/2024/day/2

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Clone)]
struct Report {
    levels: Vec<usize>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut prev = self.levels[0];
        match prev.cmp(&self.levels[1]) {
            std::cmp::Ordering::Less => {
                for level in &self.levels[1..] {
                    if prev >= *level || *level - prev > 3 {
                        return false;
                    }
                    prev = *level;
                }
                true
            }
            std::cmp::Ordering::Greater => {
                for level in &self.levels[1..] {
                    if prev <= *level || prev - *level > 3 {
                        return false;
                    }
                    prev = *level;
                }
                true
            }
            std::cmp::Ordering::Equal => false,
        }
    }

    fn remove_level(&self, index: usize) -> Report {
        let mut levels = self.levels.clone();
        levels.remove(index);
        Report { levels }
    }
}

impl ::std::str::FromStr for Report {
    type Err = ::std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
        Ok(Report { levels })
    }
}

fn read<R: Read>(io: R) -> Vec<Report> {
    let br = BufReader::new(io);
    br.lines()
        .map_while(Result::ok)
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part_1(input: &[Report]) -> usize {
    input.iter().filter(|report| report.is_safe()).count()
}

fn part_2(input: &[Report]) -> usize {
    input
        .iter()
        .filter(|report| {
            if !report.is_safe() {
                for i in 0..report.levels.len() {
                    if report.remove_level(i).is_safe() {
                        return true;
                    }
                }
                return false;
            }
            true
        })
        .count()
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
        assert_eq!(part_1(&input), 2);
        assert_eq!(part_2(&input), 4);
    }
}
