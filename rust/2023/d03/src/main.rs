// https://adventofcode.com/2023/day/3

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Detail {
    Cog,
    Empty,
    Other,
    Number(usize),
}

pub struct Schematics {
    pub width: i32,
    pub height: i32,
    pub content: Vec<Detail>,
    pub numbers: HashMap<usize, usize>,
}

impl Schematics {
    pub fn new(input: &[String]) -> Self {
        let width = input[0].len() as i32;
        let height = input.len() as i32;
        let mut number_id = 0;
        let mut numbers = HashMap::new();

        let content = input
            .iter()
            .flat_map(|line| {
                let mut it = line.chars();
                let mut content = Vec::new();
                let mut is_number = false;
                while let Some(c) = it.next() {
                    let thing = match c {
                        '*' => {
                            if is_number {
                                number_id += 1;
                                is_number = false;
                            }
                            Detail::Cog
                        }
                        '.' => {
                            if is_number {
                                number_id += 1;
                                is_number = false;
                            }
                            Detail::Empty
                        }
                        '0'..='9' => {
                            if !is_number {
                                number_id += 1;
                            }
                            is_number = true;
                            let digit = c.to_digit(10).unwrap() as usize;
                            let number = numbers.entry(number_id).or_insert(0);
                            *number = *number * 10 + digit;
                            Detail::Number(number_id)
                        }
                        _ => {
                            if is_number {
                                number_id += 1;
                                is_number = false;
                            }
                            Detail::Other
                        }
                    };
                    content.push(thing);
                }
                content
            })
            .collect::<Vec<Detail>>();
        assert_eq!(content.len() as i32, width * height, "Invalid input");
        Self {
            width,
            height,
            content,
            numbers,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Detail> {
        if y < 0 || y >= self.height {
            return None;
        }
        if x < 0 || x >= self.width {
            return None;
        }
        let index = y * self.width + x;
        self.content.get(index as usize).copied()
    }

    pub fn get_neighbours(&self, x: i32, y: i32) -> Vec<Detail> {
        [
            // Row above
            (x - 1, y - 1),
            (x + 0, y - 1),
            (x + 1, y - 1),
            // Same row, excluding self
            (x - 1, y + 0),
            (x + 1, y + 0),
            // Row below
            (x - 1, y + 1),
            (x + 0, y + 1),
            (x + 1, y + 1),
        ]
        .into_iter()
        .filter_map(|(x, y)| self.get(x, y))
        .collect()
    }

    pub fn has_symbol_neighbour(&self, x: i32, y: i32) -> bool {
        self.get_neighbours(x, y)
            .iter()
            .any(|c| *c == Detail::Cog || *c == Detail::Other)
    }
}

fn read<R: Read>(io: R) -> Schematics {
    let br = BufReader::new(io);
    let lines = br.lines().filter_map(Result::ok).collect::<Vec<_>>();
    Schematics::new(&lines)
}

fn part_1(input: &Schematics) -> usize {
    let mut part_number_ids = HashSet::new();
    for row in 0..input.height {
        for col in 0..input.width {
            let c = input.get(col, row).unwrap();
            match c {
                Detail::Number(id) => {
                    if input.has_symbol_neighbour(col, row) {
                        part_number_ids.insert(id);
                    }
                }
                _ => {}
            }
        }
    }
    part_number_ids
        .iter()
        .map(|id| input.numbers.get(id).unwrap())
        .sum()
}

fn part_2(input: &Schematics) -> usize {
    let mut ratios = Vec::new();
    for row in 0..input.height {
        for col in 0..input.width {
            let c = input.get(col, row).unwrap();
            match c {
                Detail::Cog => {
                    let number_neighbours = input
                        .get_neighbours(col, row)
                        .into_iter()
                        .filter_map(|detail| {
                            if let Detail::Number(id) = detail {
                                Some(id)
                            } else {
                                None
                            }
                        })
                        .collect::<HashSet<_>>();
                    if number_neighbours.len() == 2 {
                        let ratio = number_neighbours
                            .iter()
                            .map(|id| input.numbers.get(id).unwrap())
                            .product::<usize>();
                        ratios.push(ratio);
                    }
                }
                _ => {}
            }
        }
    }
    ratios.iter().sum()
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
        assert_eq!(part_1(&input), 4361);
        assert_eq!(part_2(&input), 467835);
    }
}
