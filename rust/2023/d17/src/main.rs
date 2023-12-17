// https://adventofcode.com/2023/day/17

use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{BufRead, BufReader, Read},
};

struct Grid {
    heat_loss: Vec<i64>,
    width: i64,
    height: i64,
}

impl Grid {
    fn get(&self, x: i64, y: i64) -> i64 {
        self.heat_loss[(y * self.width + x) as usize]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn read<R: Read>(io: R) -> Grid {
    let br = BufReader::new(io);
    let lines = br.lines().map_while(Result::ok).collect::<Vec<_>>();
    let width = lines[0].len() as i64;
    let height = lines.len() as i64;
    let heat_loss = lines
        .iter()
        .flat_map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<_>>()
        })
        .collect();
    Grid {
        heat_loss,
        width,
        height,
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn veer(&self) -> [Self; 2] {
        match self {
            Dir::Up | Dir::Down => [Dir::Left, Dir::Right],
            Dir::Left | Dir::Right => [Dir::Up, Dir::Down],
        }
    }
}

fn possible_next(grid: &Grid, pos: Pos, dir: Dir) -> Vec<(Pos, i64)> {
    let vec = match dir {
        Dir::Up => Pos { x: 0, y: -1 },
        Dir::Down => Pos { x: 0, y: 1 },
        Dir::Left => Pos { x: -1, y: 0 },
        Dir::Right => Pos { x: 1, y: 0 },
    };

    let mut possible = Vec::with_capacity(3);
    let one = Pos::new(pos.x + vec.x, pos.y + vec.y);
    let two = Pos::new(pos.x + 2 * vec.x, pos.y + 2 * vec.y);
    let three = Pos::new(pos.x + 3 * vec.x, pos.y + 3 * vec.y);
    if one.x >= 0 && one.x < grid.width && one.y >= 0 && one.y < grid.height {
        possible.push((one, grid.get(one.x, one.y)));
    }
    if two.x >= 0 && two.x < grid.width && two.y >= 0 && two.y < grid.height {
        possible.push((two, grid.get(one.x, one.y) + grid.get(two.x, two.y)));
    }
    if three.x >= 0 && three.x < grid.width && three.y >= 0 && three.y < grid.height {
        possible.push((
            three,
            grid.get(one.x, one.y) + grid.get(two.x, two.y) + grid.get(three.x, three.y),
        ));
    }

    possible
}

fn part_1(grid: &Grid) -> i64 {
    let end = Pos::new(grid.width - 1, grid.height - 1);
    let mut min = i64::MAX;
    let mut active = HashMap::new();
    let mut observed = HashMap::new();
    active.insert((Pos::new(0, 0), Dir::Right), 0);
    active.insert((Pos::new(0, 0), Dir::Down), 0);
    while !active.is_empty() {
        let mut new_active = HashMap::new();
        for ((pos, dir), heat_loss) in active.into_iter() {
            let possible = possible_next(grid, pos, dir);
            if possible.is_empty() {
                continue;
            }
            let dirs = dir.veer();
            let next_steps = possible
                .into_iter()
                .flat_map(|(pos, loss)| {
                    dirs.into_iter()
                        .map(|d| ((pos, d), loss + heat_loss))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            for next in next_steps.into_iter() {
                let next_pos = next.0 .0;
                let next_dir = next.0 .1;
                let next_loss = next.1;
                if let Some(previous) = observed.get(&(next_pos, next_dir)) {
                    if next_loss > *previous {
                        continue;
                    }
                }
                observed.insert((next_pos, next_dir), next_loss);
                if next_pos == end {
                    if next_loss < min {
                        min = next_loss;
                    }
                    continue;
                }
                new_active.insert((next_pos, next_dir), next_loss);
            }
        }
        active = new_active;
    }

    min
}

fn possible_next2(grid: &Grid, pos: Pos, dir: Dir) -> Vec<(Pos, i64)> {
    let vec = match dir {
        Dir::Up => Pos { x: 0, y: -1 },
        Dir::Down => Pos { x: 0, y: 1 },
        Dir::Left => Pos { x: -1, y: 0 },
        Dir::Right => Pos { x: 1, y: 0 },
    };

    let vecs = (1i64..=10)
        .filter_map(|i| {
            let x = pos.x + i * vec.x;
            let y = pos.y + i * vec.y;
            if x >= 0 && x < grid.width && y >= 0 && y < grid.height {
                Some(Pos::new(x, y))
            } else {
                None
            }
        })
        .map(|pos| (pos, grid.get(pos.x, pos.y)))
        .collect::<Vec<_>>();
    if vecs.len() < 4 {
        return vec![];
    }
    vecs[3..]
        .iter()
        .enumerate()
        .map(|(i, &(pos, _))| {
            let loss = vecs[..=3 + i].iter().map(|(_, loss)| loss).sum::<i64>();
            (pos, loss)
        })
        .collect::<Vec<_>>()
}

fn part_2(grid: &Grid) -> i64 {
    let end = Pos::new(grid.width - 1, grid.height - 1);
    let mut min = i64::MAX;
    let mut active = HashMap::new();
    let mut observed = HashMap::new();
    active.insert((Pos::new(0, 0), Dir::Right), 0);
    active.insert((Pos::new(0, 0), Dir::Down), 0);
    while !active.is_empty() {
        let mut new_active = HashMap::new();
        for ((pos, dir), heat_loss) in active.into_iter() {
            let possible = possible_next2(grid, pos, dir);
            if possible.is_empty() {
                continue;
            }
            let dirs = dir.veer();
            let next_steps = possible
                .into_iter()
                .flat_map(|(pos, loss)| {
                    dirs.into_iter()
                        .map(|d| ((pos, d), loss + heat_loss))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            for next in next_steps.into_iter() {
                let next_pos = next.0 .0;
                let next_dir = next.0 .1;
                let next_loss = next.1;
                if let Some(previous) = observed.get(&(next_pos, next_dir)) {
                    if next_loss > *previous {
                        continue;
                    }
                }
                observed.insert((next_pos, next_dir), next_loss);
                if next_pos == end {
                    if next_loss < min {
                        min = next_loss;
                    }
                    continue;
                }
                new_active.insert((next_pos, next_dir), next_loss);
            }
        }
        active = new_active;
    }

    min
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
    fn check_possible_next2() {
        let grid = read(File::open("example1.txt").unwrap());
        let pos = Pos::new(0, 0);
        let dir = Dir::Right;
        let expected = vec![
            (Pos::new(4, 0), 4 + 1 + 3 + 4),
            (Pos::new(5, 0), 4 + 1 + 3 + 4 + 3),
            (Pos::new(6, 0), 4 + 1 + 3 + 4 + 3 + 2),
            (Pos::new(7, 0), 4 + 1 + 3 + 4 + 3 + 2 + 3),
            (Pos::new(8, 0), 4 + 1 + 3 + 4 + 3 + 2 + 3 + 1),
            (Pos::new(9, 0), 4 + 1 + 3 + 4 + 3 + 2 + 3 + 1 + 1),
            (Pos::new(10, 0), 4 + 1 + 3 + 4 + 3 + 2 + 3 + 1 + 1 + 3),
        ];
        let actual = possible_next2(&grid, pos, dir);
        assert_eq!(actual.len(), 7);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input), 102);
        assert_eq!(part_2(&input), 94);
    }

    #[test]
    fn example_2() {
        let input = read(File::open("example2.txt").unwrap());
        assert_eq!(part_2(&input), 71);
    }
}
