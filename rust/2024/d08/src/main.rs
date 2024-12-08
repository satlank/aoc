// https://adventofcode.com/2024/day/8

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

use itertools::Itertools;

fn read<R: Read>(io: R) -> Grid {
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines().map_while(Result::ok).collect();
    let height = lines.len() as isize;
    let width = lines[0].len() as isize;
    let mut antennae = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let vec = Vec2 {
                x: x as isize,
                y: y as isize,
            };
            let freq = Frequency(c);
            let entry = antennae.entry(freq).or_insert(HashSet::new());
            entry.insert(vec);
        }
    }
    Grid {
        height,
        width,
        antennae,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vec2 {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Frequency(char);

struct Grid {
    height: isize,
    width: isize,
    antennae: HashMap<Frequency, HashSet<Vec2>>,
}

impl Grid {
    fn in_grid(&self, vec: &Vec2) -> bool {
        vec.x >= 0 && vec.x < self.width && vec.y >= 0 && vec.y < self.height
    }

    fn get_antinodes(&self) -> HashSet<Vec2> {
        self.antennae
            .keys()
            .flat_map(|freq| self.antinodes_for_freq(*freq).into_iter())
            .collect()
    }

    fn get_resonant_antinodes(&self) -> HashSet<Vec2> {
        self.antennae
            .keys()
            .flat_map(|freq| self.resonant_antinodes_for_freq(*freq).into_iter())
            .collect()
    }

    fn antinodes_for_freq(&self, freq: Frequency) -> HashSet<Vec2> {
        let a = self.antennae.get(&freq).unwrap();
        a.iter()
            .combinations(2)
            .flat_map(|pair| {
                let a = pair[0];
                let b = pair[1];
                let dx = b.x - a.x;
                let dy = b.y - a.y;

                let c = Vec2 {
                    x: b.x + dx,
                    y: b.y + dy,
                };
                let d = Vec2 {
                    x: a.x - dx,
                    y: a.y - dy,
                };
                [
                    if self.in_grid(&c) { Some(c) } else { None },
                    if self.in_grid(&d) { Some(d) } else { None },
                ]
            })
            .flatten()
            .collect()
    }

    fn resonant_antinodes_for_freq(&self, freq: Frequency) -> HashSet<Vec2> {
        let a = self.antennae.get(&freq).unwrap();
        a.iter()
            .combinations(2)
            .flat_map(|pair| {
                let a = pair[0];
                let b = pair[1];
                let dx = b.x - a.x;
                let dy = b.y - a.y;

                let mut anti = HashSet::new();
                let mut peak = *b;
                loop {
                    anti.insert(peak);
                    peak = Vec2 {
                        x: peak.x + dx,
                        y: peak.y + dy,
                    };
                    if !self.in_grid(&peak) {
                        break;
                    }
                }
                peak = *a;
                loop {
                    anti.insert(peak);
                    peak = Vec2 {
                        x: peak.x - dx,
                        y: peak.y - dy,
                    };
                    if !self.in_grid(&peak) {
                        break;
                    }
                }
                anti
            })
            .collect()
    }
}

fn part_1(grid: &Grid) -> usize {
    let antinodes = grid.get_antinodes();
    antinodes.len()
}

fn part_2(grid: &Grid) -> usize {
    let resonant_antinodes = grid.get_resonant_antinodes();
    resonant_antinodes.len()
}

fn main() {
    let grid = read(File::open("input.txt").unwrap());
    let p1 = part_1(&grid);
    println!("Part 1: {}", p1);
    let p2 = part_2(&grid);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&grid), 14);
        assert_eq!(part_2(&grid), 34);
    }
}
