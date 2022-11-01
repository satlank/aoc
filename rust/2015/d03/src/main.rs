// https://adventofcode.com/2015/day/3

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    West,
    East,
    South,
    NotADirection,
}
use Direction::*;

impl Direction {
    fn new_from_char(c: char) -> Self {
        match c {
            '>' => Direction::East,
            '<' => Direction::West,
            'v' => Direction::South,
            '^' => Direction::North,
            _ => Direction::NotADirection,
        }
    }
}

fn read<R: Read>(io: R) -> Vec<Direction> {
    let br = BufReader::new(io);
    br.lines()
        .nth(0)
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| Direction::new_from_char(c))
        .filter(|&d| d != Direction::NotADirection)
        .collect()
}

fn walk(directions: &[Direction], step_size: usize) -> HashSet<(i32, i32)> {
    let mut current_pos = (0i32, 0i32);
    directions
        .iter()
        .step_by(step_size)
        .map(|&d| match d {
            North => {
                current_pos.1 += 1;
                current_pos
            }
            South => {
                current_pos.1 -= 1;
                current_pos
            }
            East => {
                current_pos.0 += 1;
                current_pos
            }
            West => {
                current_pos.0 -= 1;
                current_pos
            }
            NotADirection => current_pos,
        })
        .collect()
}

fn part_1(directions: &[Direction]) -> usize {
    let mut pts = walk(directions, 1);
    pts.insert((0, 0));
    pts.len()
}

fn part_2(directions: &[Direction]) -> usize {
    let mut santa = walk(directions, 2);
    santa.insert((0, 0));
    let robo_santa = walk(&directions[1..], 2);
    santa.union(&robo_santa).count()
}

fn main() {
    let directions = read(File::open("input.txt").unwrap());
    println!(
        "Santa is visting {} houses at least once",
        part_1(&directions)
    );
    println!(
        "Santa and Robo-Santa are visting {} houses at least once",
        part_2(&directions)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        assert_eq!(read(File::open("test1.txt").unwrap()), vec![East]);
        assert_eq!(
            read(File::open("test2.txt").unwrap()),
            vec![North, East, South, West]
        );
        assert_eq!(
            read(File::open("test3.txt").unwrap()),
            vec![North, South, North, South, North, South, North, South, North, South]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&read(File::open("test1.txt").unwrap())), 2);
        assert_eq!(part_1(&read(File::open("test2.txt").unwrap())), 4);
        assert_eq!(part_1(&read(File::open("test3.txt").unwrap())), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&read(File::open("test2.txt").unwrap())), 3);
        assert_eq!(part_2(&read(File::open("test3.txt").unwrap())), 11);
    }
}
