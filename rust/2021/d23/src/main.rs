// https://adventofcode.com/2021/day/23

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

mod burrow;

use burrow::{Burrow, Home};

fn read<R: Read>(io: R) -> Burrow<2> {
    let br = BufReader::new(io);
    let mut lines = br.lines();
    lines.nth(1).unwrap().unwrap();
    let top = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .filter(|&c| c != '#' && c != ' ')
        .collect::<Vec<char>>();
    let bottom = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .filter(|&c| c != '#' && c != ' ')
        .collect::<Vec<char>>();
    Burrow::<2> {
        hallway: [' '; 11],
        homes: [
            Home {
                owner: 'A',
                id: 0,
                hallway: 2,
                places: [bottom[0], top[0]],
            },
            Home {
                owner: 'B',
                id: 1,
                hallway: 4,
                places: [bottom[1], top[1]],
            },
            Home {
                owner: 'C',
                id: 2,
                hallway: 6,
                places: [bottom[2], top[2]],
            },
            Home {
                owner: 'D',
                id: 3,
                hallway: 8,
                places: [bottom[3], top[3]],
            },
        ],
    }
}

fn solve<const D: usize>(burrow: &Burrow<D>, current_cost: usize) -> Option<usize> {
    if burrow.is_complete() {
        return Some(current_cost);
    }
    let moves = burrow.valid_moves();
    if moves.is_empty() {
        return None;
    }
    moves
        .iter()
        .map(|m| solve(&m.1, m.0 + current_cost))
        .flatten()
        .min()
}

fn upcast(small: &Burrow<2>) -> Burrow<4> {
    let mut large = Burrow::<4>::default();
    for i in 0..4 {
        large.homes[i].places[0] = small.homes[i].places[0];
        large.homes[i].places[3] = small.homes[i].places[1];
    }
    large.homes[0].places[1] = 'D';
    large.homes[0].places[2] = 'D';
    large.homes[1].places[1] = 'B';
    large.homes[1].places[2] = 'C';
    large.homes[2].places[1] = 'A';
    large.homes[2].places[2] = 'B';
    large.homes[3].places[1] = 'C';
    large.homes[3].places[2] = 'A';

    large
}

fn part_1(burrow: &Burrow<2>) -> usize {
    solve(burrow, 0).unwrap()
}

fn part_2(burrow: &Burrow<2>) -> usize {
    let real_burrow = upcast(burrow);
    solve(&real_burrow, 0).unwrap()
}

fn main() {
    let burrow = read(File::open("input.txt").unwrap());
    println!("Result of part 1: {}", part_1(&burrow));
    println!("Result of part 2: {}", part_2(&burrow));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let burrow = read(File::open("test1.txt").unwrap());
        assert_eq!(burrow.homes[0].places, ['A', 'B']);
        assert_eq!(burrow.homes[1].places, ['D', 'C']);
        assert_eq!(burrow.homes[2].places, ['C', 'B']);
        assert_eq!(burrow.homes[3].places, ['A', 'D']);
    }

    #[test]
    fn test_upcast() {
        let burrow = read(File::open("test1.txt").unwrap());
        let burrow = upcast(&burrow);
        assert_eq!(burrow.homes[0].places, ['A', 'D', 'D', 'B']);
        assert_eq!(burrow.homes[1].places, ['D', 'B', 'C', 'C']);
        assert_eq!(burrow.homes[2].places, ['C', 'A', 'B', 'B']);
        assert_eq!(burrow.homes[3].places, ['A', 'C', 'A', 'D']);
    }

    #[test]
    fn test_part_1() {
        let input = read(File::open("test1.txt").unwrap());
        assert_eq!(part_1(&input), 12521);
    }

    #[test]
    fn test_part_2() {
        let input = read(File::open("test1.txt").unwrap());
        assert_eq!(part_2(&input), 44169);
    }
}
