// https://adventofcode.com/2024/day/11

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<usize> {
    let br = BufReader::new(io);
    br.lines()
        .map_while(Result::ok)
        .take(1)
        .flat_map(|l| {
            l.split_whitespace()
                .map_while(|s| s.parse().ok())
                .collect::<Vec<usize>>()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleResult {
    Single(usize),
    Split(usize, usize),
}

fn apply_rules(n: usize) -> RuleResult {
    if n == 0 {
        return RuleResult::Single(1);
    }
    let number_of_digits = n.checked_ilog10().unwrap_or(0) + 1;
    if number_of_digits % 2 == 0 {
        let half = number_of_digits / 2;
        let first = n / 10usize.pow(half);
        let second = n % 10usize.pow(half);
        RuleResult::Split(first, second)
    } else {
        RuleResult::Single(n * 2024)
    }
}

fn run(n: usize, m: usize) -> usize {
    let mut state = vec![n];
    for _ in 0..m {
        let mut next = vec![];
        for stone in state.into_iter() {
            match apply_rules(stone) {
                RuleResult::Single(s) => next.push(s),
                RuleResult::Split(a, b) => {
                    next.push(a);
                    next.push(b);
                }
            }
        }
        state = next;
    }
    state.len()
}

fn part_1(input: &[usize]) -> usize {
    input.iter().map(|i| run(*i, 25)).sum()
}

fn part_2(input: &[usize]) -> usize {
    let mut state: HashMap<usize, usize> = input.iter().copied().map(|i| (i, 1)).collect();
    for _ in 0..75 {
        let mut new_state = HashMap::new();
        for (stone, count) in state.iter() {
            match apply_rules(*stone) {
                RuleResult::Single(s) => {
                    *new_state.entry(s).or_insert(0) += count;
                }
                RuleResult::Split(a, b) => {
                    *new_state.entry(a).or_insert(0) += count;
                    *new_state.entry(b).or_insert(0) += count;
                }
            }
        }
        state = new_state;
    }
    state.values().sum()
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
    fn test_apply_rules() {
        assert_eq!(apply_rules(0), RuleResult::Single(1));
        assert_eq!(apply_rules(1), RuleResult::Single(2024));
        assert_eq!(apply_rules(10), RuleResult::Split(1, 0));
        assert_eq!(apply_rules(11), RuleResult::Split(1, 1));
        assert_eq!(apply_rules(100), RuleResult::Single(100 * 2024));
    }

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input), 55312);
    }
}
