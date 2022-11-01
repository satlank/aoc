// https://adventofcode.com/2015/day/1

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn read<R: Read>(io: R) -> Vec<char> {
    let br = BufReader::new(io);
    br.lines().nth(0).unwrap().unwrap().chars().collect()
}

fn part_1(instructions: &[char]) -> i32 {
    instructions
        .iter()
        .map(|&c| if c == '(' { 1 } else { -1 })
        .sum()
}

fn part_2(instructions: &[char]) -> usize {
    let mut pos = 0;
    for (i, &c) in instructions.iter().enumerate() {
        pos += if c == '(' { 1 } else { -1 };
        if pos == -1 {
            return i + 1;
        }
    }
    instructions.len() + 1
}

fn main() {
    let instructions = read(File::open("input.txt").unwrap());
    let floor = part_1(&instructions);
    println!("Go to floor {}", floor);
    let first_basement_pos = part_2(&instructions);
    println!("Arriving in basement with instruction {}", first_basement_pos);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&("(())".chars().collect::<Vec<char>>())), 0);
        assert_eq!(part_1(&("()()".chars().collect::<Vec<char>>())), 0);
        assert_eq!(part_1(&("(((".chars().collect::<Vec<char>>())), 3);
        assert_eq!(part_1(&("(()(()(".chars().collect::<Vec<char>>())), 3);
        assert_eq!(part_1(&("))(((((".chars().collect::<Vec<char>>())), 3);
        assert_eq!(part_1(&("())".chars().collect::<Vec<char>>())), -1);
        assert_eq!(part_1(&("))(".chars().collect::<Vec<char>>())), -1);
        assert_eq!(part_1(&(")))".chars().collect::<Vec<char>>())), -3);
        assert_eq!(part_1(&(")())())".chars().collect::<Vec<char>>())), -3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&(")".chars().collect::<Vec<char>>())), 1);
        assert_eq!(part_2(&("()())".chars().collect::<Vec<char>>())), 5);
    }
}
