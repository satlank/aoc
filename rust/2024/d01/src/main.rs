// https://adventofcode.com/2024/day/1

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct LocationId(usize);

impl LocationId {
    fn as_isize(&self) -> usize {
        self.0
    }

    fn distance(&self, other: &LocationId) -> usize {
        match self.0.cmp(&other.0) {
            std::cmp::Ordering::Less => other.0 - self.0,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => self.0 - other.0,
        }
    }
}

impl ::std::str::FromStr for LocationId {
    type Err = ::std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LocationId(s.parse()?))
    }
}

fn read<R: Read>(io: R) -> (Vec<LocationId>, Vec<LocationId>) {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(|line| {
            let locs = line
                .as_ref()
                .ok()?
                .trim()
                .split("   ")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<LocationId>>();
            Some((locs[0], locs[1]))
        })
        .collect()
}

fn part_1(first: &[LocationId], second: &[LocationId]) -> usize {
    let mut first = first.to_vec();
    let mut second = second.to_vec();
    first.sort();
    second.sort();
    assert_eq!(first.len(), second.len());
    first
        .iter()
        .zip(second.iter())
        .map(|(f, s)| f.distance(s))
        .sum()
}

fn part_2(first: &[LocationId], second: &[LocationId]) -> usize {
    let freq = second.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    first
        .iter()
        .map(|x| freq.get(x).unwrap_or(&0) * x.as_isize())
        .sum()
}

fn main() {
    let (first, second) = read(File::open("input.txt").unwrap());
    let p1 = part_1(&first, &second);
    println!("Part 1: {}", p1);
    let p2 = part_2(&first, &second);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let (first, second) = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&first, &second), 11);
    }

    #[test]
    fn example_2() {
        let (first, second) = read(File::open("example1.txt").unwrap());
        assert_eq!(part_2(&first, &second), 31);
    }
}
