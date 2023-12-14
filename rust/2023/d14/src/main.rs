// https://adventofcode.com/2023/day/14

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

enum Field {
    Roller,
    Rock,
    Free,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Position {
    row: i32,
    col: i32,
}

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            '.' => Field::Free,
            '#' => Field::Rock,
            'O' => Field::Roller,
            _ => panic!("Unknown field type"),
        }
    }
}

#[derive(Debug, Clone)]
struct Platform {
    rocks: HashSet<Position>,
    rollers: Vec<Position>,
    width: i32,
    height: i32,
}

impl Platform {
    fn calculate_load(&self) -> i32 {
        self.rollers.iter().map(|r| self.height - r.row).sum()
    }

    fn tilt_north(&mut self) {
        let mut new_rollers = HashSet::new();

        // Sorted by ascending row, then ascending column
        self.rollers.sort_by_key(|p| (p.row, p.col));
        for mut roller in self.rollers.drain(..) {
            loop {
                if roller.row == 0 {
                    new_rollers.insert(roller);
                    break;
                }
                let tentative = Position {
                    row: roller.row - 1,
                    col: roller.col,
                };
                if self.rocks.contains(&tentative) || new_rollers.contains(&tentative) {
                    new_rollers.insert(roller);
                    break;
                }
                roller = tentative;
            }
        }
        self.rollers = new_rollers.into_iter().collect::<Vec<_>>();
    }

    fn tilt_south(&mut self) {
        let mut new_rollers = HashSet::new();

        // Sorted by descending row, then ascending column
        self.rollers.sort_by_key(|p| (-p.row, p.col));
        for mut roller in self.rollers.drain(..) {
            loop {
                if roller.row == self.height - 1 {
                    new_rollers.insert(roller);
                    break;
                }
                let tentative = Position {
                    row: roller.row + 1,
                    col: roller.col,
                };
                if self.rocks.contains(&tentative) || new_rollers.contains(&tentative) {
                    new_rollers.insert(roller);
                    break;
                }
                roller = tentative;
            }
        }
        self.rollers = new_rollers.into_iter().collect::<Vec<_>>();
    }

    fn tilt_west(&mut self) {
        let mut new_rollers = HashSet::new();

        // Sorted by ascending column, then ascending row
        self.rollers.sort_by_key(|p| (p.col, p.row));
        for mut roller in self.rollers.drain(..) {
            loop {
                if roller.col == 0 {
                    new_rollers.insert(roller);
                    break;
                }
                let tentative = Position {
                    row: roller.row,
                    col: roller.col - 1,
                };
                if self.rocks.contains(&tentative) || new_rollers.contains(&tentative) {
                    new_rollers.insert(roller);
                    break;
                }
                roller = tentative;
            }
        }
        self.rollers = new_rollers.into_iter().collect::<Vec<_>>();
    }

    fn tilt_east(&mut self) {
        let mut new_rollers = HashSet::new();

        // Sorted by descending column, then ascending row
        self.rollers.sort_by_key(|p| (-p.col, p.row));
        for mut roller in self.rollers.drain(..) {
            loop {
                if roller.col == self.width - 1 {
                    new_rollers.insert(roller);
                    break;
                }
                let tentative = Position {
                    row: roller.row,
                    col: roller.col + 1,
                };
                if self.rocks.contains(&tentative) || new_rollers.contains(&tentative) {
                    new_rollers.insert(roller);
                    break;
                }
                roller = tentative;
            }
        }
        self.rollers = new_rollers.into_iter().collect::<Vec<_>>();
    }
}

fn read<R: Read>(io: R) -> Platform {
    let br = BufReader::new(io);
    let content = br.lines().map_while(Result::ok).collect::<Vec<_>>();
    let width = content[0].len();
    let height = content.len();

    let mut rollers = Vec::new();
    let mut rocks = HashSet::new();

    for (row, item) in content.iter().enumerate().take(height) {
        item.chars().enumerate().for_each(|(col, c)| match c {
            'O' => {
                rollers.push(Position {
                    row: row as i32,
                    col: col as i32,
                });
            }
            '#' => {
                rocks.insert(Position {
                    row: row as i32,
                    col: col as i32,
                });
            }
            _ => (),
        });
    }

    Platform {
        rollers,
        rocks,
        width: width as i32,
        height: height as i32,
    }
}

fn part_1(input: &Platform) -> i32 {
    let mut platform = input.clone();
    platform.tilt_north();
    platform.calculate_load()
}

fn part_2(input: &Platform) -> i32 {
    let mut platform = input.clone();
    let mut seen = HashMap::new();
    let mut cycle = 0;

    // Warmup
    for _ in 0..1_000 {
        platform.tilt_north();
        platform.tilt_west();
        platform.tilt_south();
        platform.tilt_east();
        cycle += 1;
    }

    // Detect cycle
    loop {
        platform.tilt_north();
        platform.tilt_west();
        platform.tilt_south();
        platform.tilt_east();
        cycle += 1;

        let load = platform.calculate_load();
        let mut state_rollers = platform.rollers.clone();
        state_rollers.sort_by_key(|p| (p.row, p.col));
        let key = (load, state_rollers);
        if seen.contains_key(&key) {
            let prev_cycle = seen.get(&key).unwrap();
            let cycle_len = cycle - prev_cycle;
            let remaining = (1_000_000_000 - cycle) % cycle_len;
            for _ in 0..remaining {
                platform.tilt_north();
                platform.tilt_west();
                platform.tilt_south();
                platform.tilt_east();
            }
            return platform.calculate_load();
        }
        seen.insert(key, cycle);
    }
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
        assert_eq!(part_1(&input), 136);
        assert_eq!(part_2(&input), 64);
    }
}
