// https://adventofcode.com/2023/day/1

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines().filter_map(Result::ok).collect()
}

fn part_1(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| {
            let digits = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
            digits.first().unwrap().to_digit(10).unwrap() as usize * 10
                + digits.last().unwrap().to_digit(10).unwrap() as usize
        })
        .sum()
}

fn part_2(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| {
            let digits = line
                .chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if c.is_digit(10) {
                        Some(c.to_digit(10).unwrap())
                    } else {
                        if line[i..].starts_with("one") {
                            Some(1)
                        } else if line[i..].starts_with("two") {
                            Some(2)
                        } else if line[i..].starts_with("thre") {
                            Some(3)
                        } else if line[i..].starts_with("four") {
                            Some(4)
                        } else if line[i..].starts_with("five") {
                            Some(5)
                        } else if line[i..].starts_with("six") {
                            Some(6)
                        } else if line[i..].starts_with("seven") {
                            Some(7)
                        } else if line[i..].starts_with("eight") {
                            Some(8)
                        } else if line[i..].starts_with("nine") {
                            Some(9)
                        } else {
                            None
                        }
                    }
                })
                .collect::<Vec<_>>();
            *digits.first().unwrap() as usize * 10 + *digits.last().unwrap() as usize
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
        assert_eq!(part_1(&input), 142);
    }

    #[test]
    fn example_2() {
        let input = read(File::open("example2.txt").unwrap());
        assert_eq!(part_2(&input), 281);
    }
}
