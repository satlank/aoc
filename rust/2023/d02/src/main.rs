// https://adventofcode.com/2023/day/2

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Default)]
struct Bag {
    red: u8,
    green: u8,
    blue: u8,
}

impl Bag {
    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(&self) -> u32 {
        (self.red as u32) * (self.green as u32) * (self.blue as u32)
    }
}

impl From<&str> for Bag {
    fn from(s: &str) -> Self {
        let mut bag = Bag::default();
        s.trim()
            .split(',')
            .map(|s| s.trim().split(' ').collect::<Vec<_>>())
            .for_each(|item| match item[1] {
                "red" => bag.red += item[0].parse::<u8>().unwrap(),
                "green" => bag.green += item[0].parse::<u8>().unwrap(),
                "blue" => bag.blue += item[0].parse::<u8>().unwrap(),
                _ => (),
            });
        bag
    }
}

type GameList = Vec<(u32, Vec<Bag>)>;

fn read<R: Read>(io: R) -> GameList {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .map(|line| {
            let mut it = line.split(':');
            let game = it
                .next()
                .unwrap()
                .split(' ')
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let tries = it
                .next()
                .unwrap()
                .split(';')
                .map(|s| Bag::from(s.trim()))
                .collect::<Vec<_>>();
            (game, tries)
        })
        .collect::<Vec<_>>()
}

fn part_1(input: &GameList) -> u32 {
    input
        .iter()
        .filter_map(|(id, tries)| {
            if tries.iter().any(|bag| !bag.is_possible()) {
                return None;
            } else {
                return Some(*id);
            }
        })
        .sum()
}

fn part_2(input: &GameList) -> u32 {
    input
        .iter()
        .map(|(_id, tries)| {
            let mut max = Bag::default();
            tries.iter().for_each(|bag| {
                if bag.red > max.red {
                    max.red = bag.red;
                }
                if bag.green > max.green {
                    max.green = bag.green;
                }
                if bag.blue > max.blue {
                    max.blue = bag.blue;
                }
            });
            max.power()
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
        assert_eq!(part_1(&input), 8);
        assert_eq!(part_2(&input), 2286);
    }
}
