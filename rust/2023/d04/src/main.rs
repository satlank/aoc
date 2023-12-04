// https://adventofcode.com/2023/day/4

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug)]
pub struct Card {
    pub id: usize,
    pub winning: HashSet<u16>,
    pub drawn: HashSet<u16>,
}

impl Card {
    pub fn winning_numbers(&self) -> impl Iterator<Item = u16> + '_ {
        self.winning.intersection(&self.drawn).copied()
    }

    pub fn score(&self) -> usize {
        let winners = self.winning_numbers().count();
        if winners > 0 {
            2usize.pow((winners - 1) as u32)
        } else {
            0
        }
    }
}

fn read<R: Read>(io: R) -> Vec<Card> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .map(|line| {
            let mut parts = line.split(':');
            let id = parts
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .trim()
                .parse()
                .unwrap();
            let mut numbers = parts.next().unwrap().split(" | ");
            let winning = numbers
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let drawn = numbers
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            Card { id, winning, drawn }
        })
        .collect()
}

fn part_1(input: &[Card]) -> usize {
    input.iter().map(|c| c.score()).sum()
}

fn part_2(input: &[Card]) -> usize {
    let mut cards = HashMap::<usize, usize>::new();
    let max_id = input.iter().map(|c| c.id).max().unwrap();
    for card in input {
        let mut id = card.id;
        let mut winners = card.winning_numbers().count();
        let this_entry = cards.entry(id).or_insert(0);
        *this_entry += 1;
        let parent_card_count = *this_entry;
        while winners > 0 {
            id += 1;
            if id > max_id {
                break;
            }
            let entry = cards.entry(id).or_insert(0);
            *entry += parent_card_count;
            winners -= 1;
        }
    }

    cards.values().sum()
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
        assert_eq!(part_1(&input), 13);
        assert_eq!(part_2(&input), 30);
    }
}
