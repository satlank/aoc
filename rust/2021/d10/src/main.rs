// https://adventofcode.com/2021/day/10

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Delim {
    OpenA,   // (
    OpenB,   // [
    OpenC,   // {
    OpenD,   // <
    CloseA,  // )
    CloseB,  // ]
    CloseC,  // }
    CloseD,  // >
    Invalid,
}

impl Delim {
    fn parse(c: char) -> Delim {
        match c {
            '(' => Delim::OpenA,
            '[' => Delim::OpenB,
            '{' => Delim::OpenC,
            '<' => Delim::OpenD,
            ')' => Delim::CloseA,
            ']' => Delim::CloseB,
            '}' => Delim::CloseC,
            '>' => Delim::CloseD,
            _ => Delim::Invalid,
        }
    }

    fn is_open(&self) -> bool {
        match self {
            Delim::OpenA | Delim::OpenB | Delim::OpenC | Delim::OpenD => true,
            _ => false,
        }
    }

    fn is_close(&self) -> bool {
        match self {
            Delim::CloseA | Delim::CloseB | Delim::CloseC | Delim::CloseD => true,
            _ => false,
        }
    }

    fn is_closing(&self, open: &Self) -> bool {
        match self {
            Delim::CloseA => *open == Delim::OpenA,
            Delim::CloseB => *open == Delim::OpenB,
            Delim::CloseC => *open == Delim::OpenC,
            Delim::CloseD => *open == Delim::OpenD,
            _ => false
        }
    }

    fn score(&self) -> usize {
        match self {
            Delim::CloseA => 3,
            Delim::CloseB => 57,
            Delim::CloseC => 1197,
            Delim::CloseD => 25137,
            Delim::OpenA => 1,
            Delim::OpenB => 2,
            Delim::OpenC => 3,
            Delim::OpenD => 4,
            _ => 0
        }
    }
}

fn find_illegal(line: &[Delim]) -> (Option<Delim>, Option<Vec<Delim>>) {
    let mut stack: Vec<Delim> = Vec::new();
    for d in line {
        if d.is_open() {
            stack.push(*d);
        } else if d.is_close() {
            if let Some(last) = stack.pop() {
                if !d.is_closing(&last) {
                    return (Some(*d), None);
                }
            } else {
                return (Some(*d), None);
            }
        }
    }
    if stack.len() == 0 { (None, None) } else { (None, Some(stack)) }
}

fn read<R: Read>(io: R) -> Vec<Vec<Delim>> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.unwrap().chars().map(|c| Delim::parse(c)).collect())
        .collect()
}

fn part_1(lines: &[Vec<Delim>]) -> usize {
    lines.iter().map(|line| {
        if let (Some(d), _) = find_illegal(line) {
            d.score()
        } else {
            0
        }
    }).sum()
}

fn part_2(lines: &[Vec<Delim>]) -> usize {
    let mut scores = lines.iter().filter(|line| {
        let (illegal, unclosed) = find_illegal(&line);
        illegal.is_none() && unclosed.is_some()
    }).map(|line| {
        let (_, unclosed) = find_illegal(&line);
        let mut unclosed = unclosed.unwrap();
        unclosed.reverse();
        let mut res = 0;
        for d in unclosed {
            res *= 5;
            res += d.score()
        }
        res
    }).collect::<Vec<usize>>();
    scores.sort();
    scores[scores.len()/2]
}

fn main() {
    let lines = read(File::open("input.txt").unwrap());
    let corrupt_score = part_1(&lines);
    println!("Total score of syntax errors: {}", corrupt_score);
    let autocomplete_score = part_2(&lines);
    println!("Total score of autocomplete: {}", autocomplete_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Delim::OpenA, Delim::parse('('));
        assert_eq!(Delim::CloseA, Delim::parse(')'));
        assert_eq!(
            vec![Delim::OpenA, Delim::CloseD, Delim::Invalid],
            "(>d"
                .chars()
                .map(|c| Delim::parse(c))
                .collect::<Vec<Delim>>()
        );
    }

    #[test]
    fn test_find_illegal() {
        assert_eq!((None, None), find_illegal(&[Delim::OpenA, Delim::CloseA]));
        let test = "{([(<{}[<>[]}>{[]{[(<()>"
            .chars()
            .map(|c| Delim::parse(c))
            .collect::<Vec<Delim>>();
        assert_eq!((Some(Delim::CloseC), None), find_illegal(&test));
        let test = "[({(<(())[]>[[{[]{<()<>>"
            .chars()
            .map(|c| Delim::parse(c))
            .collect::<Vec<Delim>>();
        assert_eq!((None, Some(vec!(Delim::OpenB, Delim::OpenA, Delim::OpenC, Delim::OpenA, Delim::OpenB, Delim::OpenB, Delim::OpenC, Delim::OpenC))), find_illegal(&test));
    }

    #[test]
    fn test_part_1() {
        let lines = read(File::open("test1.txt").unwrap());
        assert_eq!(26397, part_1(&lines));
    }

    #[test]
    fn test_part_2() {
        let lines = read(File::open("test1.txt").unwrap());
        assert_eq!(288957, part_2(&lines));
    }
}
