// https://adventofcode.com/2022/day/4

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug)]
struct Assignment(u32, u32);

impl Assignment {
    fn fully_embedded_in(&self, other: &Assignment) -> bool {
        self.0 >= other.0 && self.1 <= other.1
    }

    fn overlaps_with(&self, other: &Assignment) -> bool {
        other.fully_embedded_in(self) || (self.0 >= other.0 && self.0 <= other.1) || (self.1 >= other.0 && self.1 <= other.1)
    }
}

fn read<R: Read>(io: R) -> Vec<(Assignment, Assignment)> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .map(|line| {
            let pair = line.split_once(',').unwrap();
            let first = pair.0.split_once('-').unwrap();
            let second = pair.1.split_once('-').unwrap();
            (
                Assignment(
                    first.0.parse::<u32>().unwrap(),
                    first.1.parse::<u32>().unwrap(),
                ),
                Assignment(
                    second.0.parse::<u32>().unwrap(),
                    second.1.parse::<u32>().unwrap(),
                ),
            )
        })
        .collect()
}

fn part_1(input: &[(Assignment, Assignment)]) -> usize {
    input
        .iter()
        .filter_map(|(a1, a2)| {
            if a1.fully_embedded_in(a2) || a2.fully_embedded_in(a1) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

fn part_2(input: &[(Assignment, Assignment)]) -> usize {
    input
        .iter()
        .filter_map(|(a1, a2)| {
            if a1.overlaps_with(a2) {
                Some(())
            } else {
                None
            }
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
