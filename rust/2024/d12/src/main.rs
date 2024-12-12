// https://adventofcode.com/2024/day/12

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Plot(char);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Side {
    Top(Vec2),
    Bottom(Vec2),
    Left(Vec2),
    Right(Vec2),
}

impl Side {
    fn pos(&self) -> Vec2 {
        match self {
            Side::Top(pos) => *pos,
            Side::Bottom(pos) => *pos,
            Side::Left(pos) => *pos,
            Side::Right(pos) => *pos,
        }
    }

    fn neighbour(&self) -> [Side; 2] {
        match self {
            Side::Top(pos) => [
                Side::Top(Vec2 {
                    x: pos.x - 1,
                    y: pos.y,
                }),
                Side::Top(Vec2 {
                    x: pos.x + 1,
                    y: pos.y,
                }),
            ],
            Side::Bottom(pos) => [
                Side::Bottom(Vec2 {
                    x: pos.x - 1,
                    y: pos.y,
                }),
                Side::Bottom(Vec2 {
                    x: pos.x + 1,
                    y: pos.y,
                }),
            ],
            Side::Left(pos) => [
                Side::Left(Vec2 {
                    x: pos.x,
                    y: pos.y - 1,
                }),
                Side::Left(Vec2 {
                    x: pos.x,
                    y: pos.y + 1,
                }),
            ],
            Side::Right(pos) => [
                Side::Right(Vec2 {
                    x: pos.x,
                    y: pos.y - 1,
                }),
                Side::Right(Vec2 {
                    x: pos.x,
                    y: pos.y + 1,
                }),
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn neighbours(&self) -> Vec<Vec2> {
        vec![
            Vec2 {
                x: self.x - 1,
                y: self.y,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y,
            },
            Vec2 {
                x: self.x,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }

    fn sides(&self) -> Vec<Side> {
        vec![
            Side::Top(Vec2 {
                x: self.x,
                y: self.y - 1,
            }),
            Side::Bottom(Vec2 {
                x: self.x,
                y: self.y + 1,
            }),
            Side::Left(Vec2 {
                x: self.x - 1,
                y: self.y,
            }),
            Side::Right(Vec2 {
                x: self.x + 1,
                y: self.y,
            }),
        ]
    }
}

#[derive(Debug, Clone)]
struct Garden {
    height: usize,
    width: usize,
    plots: HashMap<Vec2, Plot>,
}

#[derive(Debug, Clone)]
struct Region {
    name: Plot,
    fields: HashSet<Vec2>,
}

impl Region {
    fn new(name: Plot) -> Self {
        Self {
            name,
            fields: HashSet::new(),
        }
    }

    fn add(&mut self, pos: Vec2) {
        self.fields.insert(pos);
    }

    fn area(&self) -> usize {
        self.fields.len()
    }

    fn surface(&self) -> usize {
        let mut surface = 0;
        for pos in self.fields.iter() {
            for n in pos.neighbours() {
                if !self.fields.contains(&n) {
                    surface += 1;
                }
            }
        }
        surface
    }

    fn bulk_discount_sides(&self) -> usize {
        let mut all_sides = HashSet::new();
        for pos in self.fields.iter() {
            for side in pos.sides() {
                if !self.fields.contains(&side.pos()) {
                    all_sides.insert(side);
                }
            }
        }

        let first = *all_sides.iter().next().unwrap();
        all_sides.remove(&first);

        let mut working = HashSet::new();
        working.insert(first);

        let mut squashed_sides = vec![working];

        while !all_sides.is_empty() {
            let last = squashed_sides.len() - 1;
            loop {
                let mut to_add = HashSet::new();
                for this_side in squashed_sides[last].iter() {
                    let neighbours = this_side.neighbour();
                    for n in neighbours.iter() {
                        if all_sides.contains(n) {
                            to_add.insert(*n);
                            all_sides.remove(n);
                        }
                    }
                }
                if to_add.is_empty() {
                    break;
                }
                squashed_sides[last].extend(to_add);
            }
            if !all_sides.is_empty() {
                let next = *all_sides.iter().next().unwrap();
                all_sides.remove(&next);
                let mut new_working = HashSet::new();
                new_working.insert(next);
                squashed_sides.push(new_working);
            }
        }

        squashed_sides.len()
    }

    fn price(&self) -> usize {
        self.area() * self.surface()
    }

    fn bulk_discount_price(&self) -> usize {
        self.area() * self.bulk_discount_sides()
    }
}

impl Garden {
    fn neighbours(&self, pos: &Vec2) -> Vec<Vec2> {
        let mut neighbours = Vec::new();
        if pos.x > 0 {
            neighbours.push(Vec2 {
                x: pos.x - 1,
                y: pos.y,
            });
        }
        if pos.x < (self.width - 1) as isize {
            neighbours.push(Vec2 {
                x: pos.x + 1,
                y: pos.y,
            });
        }
        if pos.y > 0 {
            neighbours.push(Vec2 {
                x: pos.x,
                y: pos.y - 1,
            });
        }
        if pos.y < (self.height - 1) as isize {
            neighbours.push(Vec2 {
                x: pos.x,
                y: pos.y + 1,
            });
        }
        neighbours
    }

    fn to_regions(&self) -> Vec<Region> {
        let mut to_group = self.plots.clone();
        let mut regions: Vec<Region> = Vec::new();
        while !to_group.is_empty() {
            let k = *to_group.keys().next().unwrap();
            let trial = to_group.remove(&k).unwrap();
            let mut region = Region::new(trial);
            let mut check = HashSet::new();
            region.add(k);
            check.insert(k);
            while !check.is_empty() {
                let mut next_check = HashSet::new();
                for pos in check.drain() {
                    for n in self.neighbours(&pos) {
                        if let Some(plot) = to_group.get(&n).copied() {
                            if plot == region.name {
                                region.add(n);
                                next_check.insert(n);
                                to_group.remove(&n);
                            }
                        }
                    }
                }
                check = next_check;
            }
            regions.push(region);
        }
        regions
    }
}

fn read<R: Read>(io: R) -> Garden {
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines().map_while(Result::ok).collect();
    let mut map = HashMap::new();
    let width = lines[0].len();
    let height = lines.len();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(
                Vec2 {
                    x: x as isize,
                    y: y as isize,
                },
                Plot(c),
            );
        }
    }
    Garden {
        height,
        width,
        plots: map,
    }
}

fn part_1(input: &Garden) -> usize {
    let regions = input.to_regions();
    regions.iter().map(Region::price).sum()
}

fn part_2(input: &Garden) -> usize {
    let regions = input.to_regions();
    regions.iter().map(Region::bulk_discount_price).sum()
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
        assert_eq!(part_1(&input), 1930);
        assert_eq!(part_2(&input), 1206);
    }

    #[test]
    fn example_2() {
        let input = read(File::open("example2.txt").unwrap());
        assert_eq!(part_2(&input), 4 * 4 + 4 * 4 + 28 * 12);
    }
}
