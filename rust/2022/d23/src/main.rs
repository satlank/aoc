// https://adventofcode.com/2022/day/23

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> HashSet<Pos> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some(Pos {
                            x: x as i32,
                            y: y as i32,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Pos {
    fn go(&self, dir: Direction) -> Self {
        match dir {
            Direction::N => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::S => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::E => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::W => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn get_shell(&self) -> HashSet<Pos> {
        [Direction::N, Direction::E, Direction::S, Direction::W]
            .into_iter()
            .flat_map(|d| self.look(d))
            .collect()
    }

    fn look(&self, dir: Direction) -> Vec<Pos> {
        match dir {
            Direction::N => {
                vec![
                    Pos {
                        x: self.x - 1,
                        y: self.y - 1,
                    },
                    Pos {
                        x: self.x,
                        y: self.y - 1,
                    },
                    Pos {
                        x: self.x + 1,
                        y: self.y - 1,
                    },
                ]
            }
            Direction::E => {
                vec![
                    Pos {
                        x: self.x + 1,
                        y: self.y + 1,
                    },
                    Pos {
                        x: self.x + 1,
                        y: self.y,
                    },
                    Pos {
                        x: self.x + 1,
                        y: self.y - 1,
                    },
                ]
            }
            Direction::S => {
                vec![
                    Pos {
                        x: self.x - 1,
                        y: self.y + 1,
                    },
                    Pos {
                        x: self.x,
                        y: self.y + 1,
                    },
                    Pos {
                        x: self.x + 1,
                        y: self.y + 1,
                    },
                ]
            }
            Direction::W => {
                vec![
                    Pos {
                        x: self.x - 1,
                        y: self.y - 1,
                    },
                    Pos {
                        x: self.x - 1,
                        y: self.y,
                    },
                    Pos {
                        x: self.x - 1,
                        y: self.y + 1,
                    },
                ]
            }
        }
    }
}

fn find_rectangle(elves: &HashSet<Pos>) -> ((i32, i32), (i32, i32)) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for elf in elves {
        min_x = i32::min(min_x, elf.x);
        max_x = i32::max(max_x, elf.x);
        min_y = i32::min(min_y, elf.y);
        max_y = i32::max(max_y, elf.y);
    }

    ((min_x, min_y), (max_x, max_y))
}

fn find_area(elves: &HashSet<Pos>) -> usize {
    let ((min_x, min_y), (max_x, max_y)) = find_rectangle(elves);

    let dx = (max_x - min_x) as usize + 1;
    let dy = (max_y - min_y) as usize + 1;

    dx * dy
}

#[allow(dead_code)]
fn print_board(elves: &HashSet<Pos>) {
    let ((min_x, min_y), (max_x, max_y)) = find_rectangle(elves);

    println!("({}, {}) - ({}, {})", min_x, min_y, max_x, max_y);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!(
                "{}",
                if elves.contains(&Pos { x, y }) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!()
    }
}

fn part_1(input: &HashSet<Pos>) -> usize {
    let mut current = input.clone();
    let mut viewing = vec![Direction::N, Direction::S, Direction::W, Direction::E];

    for _round in 1..=10 {
        assert_eq!(
            viewing.len(),
            4,
            "Should always have four directions to look in"
        );
        let mut potential_moves = HashMap::new();
        let mut new = HashSet::new();
        for e in &current {
            let shell = e.get_shell();
            if current.intersection(&shell).count() == 0 {
                new.insert(*e);
            } else {
                new.insert(*e); // Pessimistically assume we stay put
                for d in 0..4 {
                    let scanning = e.look(viewing[d]).into_iter().collect::<HashSet<_>>();
                    if current.intersection(&scanning).count() == 0 {
                        let new_pos = e.go(viewing[d]);
                        let sources = potential_moves.entry(new_pos).or_insert(Vec::new());
                        new.remove(e); // Oh, look, we actually can move!
                        sources.push(e);
                        break;
                    }
                }
            }
        }

        for (target, sources) in potential_moves {
            if sources.len() == 1 {
                new.insert(target);
            } else {
                new.extend(sources);
            }
        }
        assert_eq!(
            new.len(),
            current.len(),
            "Oh dear, we lost/created some elves"
        );

        let direction = viewing.remove(0);
        viewing.push(direction);
        current = new;
    }

    find_area(&current) - current.len()
}

fn part_2(input: &HashSet<Pos>) -> usize {
    let mut current = input.clone();
    let mut viewing = vec![Direction::N, Direction::S, Direction::W, Direction::E];
    let mut round = 1;

    loop {
        assert_eq!(
            viewing.len(),
            4,
            "Should always have four directions to look in"
        );
        let mut potential_moves = HashMap::new();
        let mut new = HashSet::new();
        for e in &current {
            let shell = e.get_shell();
            if current.intersection(&shell).count() == 0 {
                new.insert(*e);
            } else {
                new.insert(*e); // Pessimistically assume we stay put
                for d in 0..4 {
                    let scanning = e.look(viewing[d]).into_iter().collect::<HashSet<_>>();
                    if current.intersection(&scanning).count() == 0 {
                        let new_pos = e.go(viewing[d]);
                        let sources = potential_moves.entry(new_pos).or_insert(Vec::new());
                        new.remove(e); // Oh, look, we actually can move!
                        sources.push(e);
                        break;
                    }
                }
            }
        }

        if potential_moves.is_empty() {
            break;
        }

        for (target, sources) in potential_moves {
            if sources.len() == 1 {
                new.insert(target);
            } else {
                new.extend(sources);
            }
        }
        assert_eq!(
            new.len(),
            current.len(),
            "Oh dear, we lost/created some elves"
        );

        let direction = viewing.remove(0);
        viewing.push(direction);
        current = new;
        round += 1;
    }

    round
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
        assert_eq!(input.len(), 22);
        assert_eq!(part_1(&input), 110);
        assert_eq!(part_2(&input), 20);
    }

    #[test]
    fn test_look() {
        let zero = Pos { x: 0, y: 0 };
        let shell = [Direction::N, Direction::E, Direction::S, Direction::W]
            .into_iter()
            .flat_map(|d| zero.look(d))
            .collect::<Vec<_>>();
        assert_eq!(shell.iter().collect::<HashSet<_>>().len(), 8);
    }
}
