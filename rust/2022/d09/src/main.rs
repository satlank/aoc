// https://adventofcode.com/2022/day/9

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<Movement> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .map(|line| {
            let cmd = line.split_once(' ').unwrap();
            let (dir, steps) = (cmd.0, cmd.1.parse::<usize>().unwrap());
            match dir {
                "L" => Movement::Left(steps),
                "R" => Movement::Right(steps),
                "U" => Movement::Up(steps),
                "D" => Movement::Down(steps),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Movement {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

impl Movement {
    fn vector(&self) -> (usize, (i32, i32)) {
        match self {
            Movement::Right(steps) => (*steps, (1, 0)),
            Movement::Left(steps) => (*steps, (-1, 0)),
            Movement::Up(steps) => (*steps, (0, 1)),
            Movement::Down(steps) => (*steps, (0, -1)),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position(i32, i32);

impl Position {
    fn is_adjacent(&self, other: &Self) -> bool {
        self == other || ((self.0 - other.0).abs() <= 1 && (self.1 - other.1).abs() <= 1)
    }

    fn unconstrained_move(&self, direction: (i32, i32)) -> Position {
        Position(self.0 + direction.0, self.1 + direction.1)
    }

    fn contrained_move(&self, other: &Position) -> Position {
        let mut tmp = *self;
        if !tmp.is_adjacent(other) {
            tmp.0 += (other.0 - tmp.0).clamp(-1, 1);
            tmp.1 += (other.1 - tmp.1).clamp(-1, 1);
            assert!(tmp.is_adjacent(other));
        }
        tmp
    }
}

fn part_1(commands: &[Movement]) -> usize {
    let mut snake = [Position(0, 0); 2];
    commands
        .iter()
        .flat_map(|mv| {
            let (steps, dir) = mv.vector();
            (0..steps)
                .map(|_| {
                    snake[0] = snake[0].unconstrained_move(dir);
                    snake[1] = snake[1].contrained_move(&snake[0]);
                    snake[1]
                })
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>()
        .len()
}

fn part_2(commands: &[Movement]) -> usize {
    let mut snake = [Position(0, 0); 10];
    commands
        .iter()
        .flat_map(|mv| {
            let (steps, dir) = mv.vector();
            (0..steps)
                .map(|_| {
                    snake[0] = snake[0].unconstrained_move(dir);
                    for i in 1..10 {
                        snake[i] = snake[i].contrained_move(&snake[i - 1]);
                    }
                    snake[9]
                })
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>()
        .len()
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
    fn adjacent() {
        assert!(Position(0, 0).is_adjacent(&Position(-1, -1)));
        assert!(Position(0, 0).is_adjacent(&Position(-1, 0)));
        assert!(Position(0, 0).is_adjacent(&Position(-1, 1)));
        assert!(Position(0, 0).is_adjacent(&Position(0, -1)));
        assert!(Position(0, 0).is_adjacent(&Position(0, 0)));
        assert!(Position(0, 0).is_adjacent(&Position(0, 1)));
        assert!(Position(0, 0).is_adjacent(&Position(1, -1)));
        assert!(Position(0, 0).is_adjacent(&Position(1, 0)));
        assert!(Position(0, 0).is_adjacent(&Position(1, 1)));

        assert!(!Position(0, 0).is_adjacent(&Position(2, 0)));
    }

    #[test]
    fn exmaple_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input), 13);
        assert_eq!(part_2(&input), 1);
    }

    #[test]
    fn example_2() {
        let input = read(File::open("example2.txt").unwrap());
        assert_eq!(part_2(&input), 36);
    }
}
