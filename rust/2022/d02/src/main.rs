// https://adventofcode.com/2022/day/2

use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    fn decode_1(c: char) -> Self {
        match c {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissor,
            _ => unreachable!(),
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        }
    }

    fn loses_against(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        }
    }

    fn decode_2(opponent: Shape, c: char) -> Self {
        match c {
            'X' => opponent.wins_against(),
            'Y' => opponent,
            'Z' => opponent.loses_against(),
            _ => unreachable!(),
        }
    }

    fn value(&self) -> i64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissor,
            _ => unreachable!(),
        }
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Shape::Rock => match other {
                Shape::Rock => Ordering::Equal,
                Shape::Paper => Ordering::Less,
                Shape::Scissor => Ordering::Greater,
            },
            Shape::Paper => match other {
                Shape::Rock => Ordering::Greater,
                Shape::Paper => Ordering::Equal,
                Shape::Scissor => Ordering::Less,
            },
            Shape::Scissor => match other {
                Shape::Rock => Ordering::Less,
                Shape::Paper => Ordering::Greater,
                Shape::Scissor => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read<R: Read>(io: R) -> Vec<(char, char)> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .map(|line| (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
        .collect()
}

fn score(opponent: Shape, me: Shape) -> i64 {
    me.value()
        + match me.cmp(&opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
}

fn part_1(cheatsheet: &[(char, char)]) -> i64 {
    cheatsheet
        .iter()
        .map(|g| score(g.0.into(), Shape::decode_1(g.1)))
        .sum()
}

fn part_2(cheatsheet: &[(char, char)]) -> i64 {
    cheatsheet
        .iter()
        .map(|g| score(g.0.into(), Shape::decode_2(g.0.into(), g.1)))
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
    fn correct_rules() {
        assert!(Shape::Rock > Shape::Scissor);
        assert!(Shape::Rock == Shape::Rock);
        assert!(Shape::Rock < Shape::Paper);

        assert!(Shape::Paper < Shape::Scissor);
        assert!(Shape::Paper > Shape::Rock);
        assert!(Shape::Paper == Shape::Paper);

        assert!(Shape::Scissor == Shape::Scissor);
        assert!(Shape::Scissor < Shape::Rock);
        assert!(Shape::Scissor > Shape::Paper);
    }

    #[test]
    fn example_1() {
        let input = read(File::open("test1.txt").unwrap());
        assert_eq!(part_1(&input), 15);
        assert_eq!(part_2(&input), 12);
    }
}
