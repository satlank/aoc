// https://adventofcode.com/2024/day/4

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines().map_while(Result::ok).collect()
}

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Grid {
    dims: Vec2,
    grid: Vec<Vec<char>>,
}

const DIRECTIONS: [Vec2; 8] = [
    Vec2 { x: 1, y: 0 },
    Vec2 { x: -1, y: 0 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: 0, y: -1 },
    Vec2 { x: 1, y: 1 },
    Vec2 { x: -1, y: -1 },
    Vec2 { x: 1, y: -1 },
    Vec2 { x: -1, y: 1 },
];

impl Grid {
    fn find_starts(&self) -> Vec<Vec2> {
        let mut starts = Vec::new();
        for y in 0..self.dims.y {
            for x in 0..self.dims.x {
                let p = Vec2 { x, y };
                if self.get_at(&p) == Some('X') {
                    starts.push(p);
                }
            }
        }
        starts
    }

    fn find_starts_2(&self) -> Vec<Vec2> {
        let mut starts = Vec::new();
        for y in 1..self.dims.y - 1 {
            for x in 1..self.dims.x - 1 {
                let p = Vec2 { x, y };
                if self.get_at(&p) == Some('A') {
                    starts.push(p);
                }
            }
        }
        starts
    }

    fn is_valid(&self, p: &Vec2) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.dims.x && p.y < self.dims.y
    }

    fn get_at(&self, p: &Vec2) -> Option<char> {
        if self.is_valid(p) {
            Some(self.grid[p.y as usize][p.x as usize])
        } else {
            None
        }
    }

    fn is_cross_at(&self, p: &Vec2) -> bool {
        let diag1 = (
            self.get_at(&Vec2 {
                x: p.x - 1,
                y: p.y - 1,
            }),
            self.get_at(&Vec2 {
                x: p.x + 1,
                y: p.y + 1,
            }),
        );
        let diag2 = (
            self.get_at(&Vec2 {
                x: p.x + 1,
                y: p.y - 1,
            }),
            self.get_at(&Vec2 {
                x: p.x - 1,
                y: p.y + 1,
            }),
        );
        matches!(
            (diag1, diag2),
            ((Some('M'), Some('S')), (Some('S'), Some('M')))
                | ((Some('S'), Some('M')), (Some('S'), Some('M')))
                | ((Some('M'), Some('S')), (Some('M'), Some('S')))
                | ((Some('S'), Some('M')), (Some('M'), Some('S')))
        )
    }

    fn check_direction(&self, p: &Vec2, dir: &Vec2) -> bool {
        let a = p.add(dir);
        let b = a.add(dir);
        let c = b.add(dir);
        if let (Some(a), Some(b), Some(c)) = (self.get_at(&a), self.get_at(&b), self.get_at(&c)) {
            a == 'M' && b == 'A' && c == 'S'
        } else {
            false
        }
    }
}

impl From<&[String]> for Grid {
    fn from(input: &[String]) -> Self {
        let grid = input
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let dims = Vec2 {
            x: grid[0].len() as isize,
            y: grid.len() as isize,
        };
        Self { dims, grid }
    }
}

impl ::std::fmt::Display for Grid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        for y in 0..self.dims.y {
            for x in 0..self.dims.x {
                write!(f, "{}", self.grid[y as usize][x as usize])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part_1(input: &[String]) -> usize {
    let grid = Grid::from(input);
    let starts = grid.find_starts();
    starts
        .iter()
        .map(|start| {
            DIRECTIONS
                .iter()
                .filter(|dir| grid.check_direction(start, dir))
                .count()
        })
        .sum()
}

fn part_2(input: &[String]) -> usize {
    let grid = Grid::from(input);
    let starts = grid.find_starts_2();
    starts
        .iter()
        .filter(|start| grid.is_cross_at(start))
        .count()
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
        assert_eq!(part_1(&input), 18);
        assert_eq!(part_2(&input), 9);
    }
}
