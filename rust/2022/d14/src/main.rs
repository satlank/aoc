// https://adventofcode.com/2022/day/14

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl From<&str> for Coord {
    fn from(test: &str) -> Self {
        let (x, y) = test.split_once(',').unwrap();
        Self {
            x: x.parse::<i32>().unwrap(),
            y: y.parse::<i32>().unwrap(),
        }
    }
}

fn read<R: Read>(io: R) -> HashSet<Coord> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .flat_map(|line| {
            let corners = line.split(" -> ").map(Coord::from).collect::<Vec<_>>();
            assert!(corners.len() > 1);
            (1..corners.len()).flat_map(move |i| {
                if corners[i - 1].x == corners[i].x {
                    let low = if corners[i - 1].y < corners[i].y {
                        corners[i - 1].y
                    } else {
                        corners[i].y
                    };
                    let high = if corners[i - 1].y > corners[i].y {
                        corners[i - 1].y
                    } else {
                        corners[i].y
                    };
                    (low..=high)
                        .map(|y| Coord {
                            x: corners[i - 1].x,
                            y,
                        })
                        .collect::<Vec<_>>()
                } else {
                    assert_eq!(corners[i - 1].y, corners[i].y);
                    let left = if corners[i - 1].x < corners[i].x {
                        corners[i - 1].x
                    } else {
                        corners[i].x
                    };
                    let right = if corners[i - 1].x > corners[i].x {
                        corners[i - 1].x
                    } else {
                        corners[i].x
                    };
                    (left..=right)
                        .map(|x| Coord {
                            x,
                            y: corners[i - 1].y,
                        })
                        .collect::<Vec<_>>()
                }
            })
        })
        .collect()
}

fn part_1(rocks: &HashSet<Coord>) -> usize {
    let low_point = rocks.iter().map(|r| r.y).max().unwrap();
    let mut sand = HashSet::new();
    loop {
        let cur_sand = sand.len();
        let mut new_sand = Coord { x: 500, y: 0 };
        loop {
            new_sand.y += 1;
            if new_sand.y > low_point {
                break;
            }
            if sand.contains(&new_sand) || rocks.contains(&new_sand) {
                new_sand.x -= 1;
                if sand.contains(&new_sand) || rocks.contains(&new_sand) {
                    new_sand.x += 2;
                    if sand.contains(&new_sand) || rocks.contains(&new_sand) {
                        new_sand.x -= 1;
                        new_sand.y -= 1;
                        sand.insert(new_sand);
                        break;
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
        if sand.len() == cur_sand {
            break;
        }
    }
    sand.len()
}

fn part_2(rocks: &HashSet<Coord>) -> usize {
    let floor = rocks.iter().map(|r| r.y).max().unwrap() + 2;
    let mut sand = HashSet::new();
    loop {
        let mut new_sand = Coord { x: 500, y: 0 };
        if sand.contains(&new_sand) {
            break;
        }
        loop {
            if new_sand.y == floor - 1 {
                sand.insert(new_sand);
                break;
            }
            new_sand.y += 1;
            if sand.contains(&new_sand) || rocks.contains(&new_sand) {
                new_sand.x -= 1;
                if sand.contains(&new_sand) || rocks.contains(&new_sand) {
                    new_sand.x += 2;
                    if sand.contains(&new_sand) || rocks.contains(&new_sand) {
                        new_sand.x -= 1;
                        new_sand.y -= 1;
                        sand.insert(new_sand);
                        break;
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }
    sand.len()
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
        assert_eq!(part_1(&input), 24);
        assert_eq!(part_2(&input), 93);
    }
}
