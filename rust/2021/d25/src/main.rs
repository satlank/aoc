// https://adventofcode.com/2021/day/25

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

type Point = (i32, i32);

fn read<R: Read>(io: R) -> (HashSet<Point>, HashSet<Point>, Point) {
    let br = BufReader::new(io);
    let mut east = HashSet::new();
    let mut south = HashSet::new();
    let mut end = (0i32, 0i32);
    for (row, line) in br.lines().enumerate() {
        end.0 = row as i32;
        for (col, c) in line.unwrap().chars().enumerate() {
            end.1 = i32::max(end.1, col as i32);
            if c == '>' {
                east.insert((row as i32, col as i32));
            } else if c == 'v' {
                south.insert((row as i32, col as i32));
            }
        }
    }
    (east, south, (end.0 + 1, end.1 + 1))
}

fn step(
    east: &HashSet<Point>,
    south: &HashSet<Point>,
    max: &Point,
) -> Option<(HashSet<Point>, HashSet<Point>)> {
    let new_east = east
        .iter()
        .map(|sc| {
            let new_pos = (sc.0, (sc.1 + 1) % max.1);
            if east.contains(&new_pos) || south.contains(&new_pos) {
                *sc
            } else {
                new_pos
            }
        })
        .collect::<HashSet<Point>>();

    let new_south = south
        .iter()
        .map(|sc| {
            let new_pos = ((sc.0 + 1) % max.0, sc.1);
            if new_east.contains(&new_pos) || south.contains(&new_pos) {
                *sc
            } else {
                new_pos
            }
        })
        .collect();

    if east == &new_east && south == &new_south {
        None
    } else {
        Some((new_east, new_south))
    }
}

fn print_map(east: &HashSet<Point>, south: &HashSet<Point>, max: &Point) {
    for row in 0..max.0 {
        for col in 0..max.1 {
            let p = (row, col);
            let c = if east.contains(&p) {
                '>'
            } else if south.contains(&p) {
                'v'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn part_1(east: &HashSet<Point>, south: &HashSet<Point>, max: &Point) -> usize {
    let mut cnt = 0;
    let mut east = east.clone();
    let mut south = south.clone();
    loop {
        cnt += 1;
        if let Some(res) = step(&east, &south, max) {
            east = res.0;
            south = res.1;
        } else {
            print_map(&east, &south, max);
            break;
        }
    }
    cnt
}

fn main() {
    let (east, south, max) = read(File::open("input.txt").unwrap());
    println!("Sea cucumbers stop moving with step: {}", part_1(&east, &south, &max));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let (east, south, max) = read(File::open("test1.txt").unwrap());
        assert_eq!(east.len(), 4);
        assert_eq!(south.len(), 4);
        assert_eq!(max, (7, 7));
    }

    #[test]
    fn test_part_1() {
        let (east, south, max) = read(File::open("test2.txt").unwrap());
        assert_eq!(part_1(&east, &south, &max), 58);
    }
}
