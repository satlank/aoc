// https://adventofcode.com/2024/day/14

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<Robot> {
    let br = BufReader::new(io);
    br.lines()
        .map_while(Result::ok)
        .map(|l| l.parse().unwrap())
        .collect()
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn step(&mut self, steps: i32, width: i32, height: i32) {
        self.x += self.vx * steps;
        self.y += self.vy * steps;
        self.teleport(width, height);
    }

    fn teleport(&mut self, width: i32, height: i32) {
        self.x = self.x.rem_euclid(width);
        self.y = self.y.rem_euclid(height);
    }

    fn quadrant(&self, width: i32, height: i32) -> Option<(i32, i32)> {
        if self.x == width / 2 || self.y == height / 2 {
            return None;
        }
        let x = if self.x < width / 2 { 0 } else { 1 };
        let y = if self.y < height / 2 { 0 } else { 1 };
        Some((x, y))
    }
}

impl std::str::FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        let pparts: Vec<&str> = parts[0][2..].split(",").collect();
        let vparts: Vec<&str> = parts[1][2..].split(",").collect();
        Ok(Robot {
            x: pparts[0].parse().unwrap(),
            y: pparts[1].parse().unwrap(),
            vx: vparts[0].parse().unwrap(),
            vy: vparts[1].parse().unwrap(),
        })
    }
}

fn part_1(input: &[Robot], wdith: i32, height: i32) -> usize {
    input
        .iter()
        .map(|r| {
            let mut r = *r;
            r.step(100, wdith, height);
            r
        })
        .filter_map(|r| r.quadrant(wdith, height))
        .fold(HashMap::new(), |mut acc, q| {
            *acc.entry(q).or_insert(0) += 1;
            acc
        })
        .values()
        .product()
}

struct Grid {
    width: i32,
    height: i32,
    bots: HashMap<(i32, i32), i32>,
}

impl Grid {
    fn new(width: i32, height: i32) -> Self {
        Grid {
            width,
            height,
            bots: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        self.bots.clear();
    }

    fn add(&mut self, r: Robot) {
        *self.bots.entry((r.x, r.y)).or_insert(0) += 1;
    }

    fn all_unique(&self) -> bool {
        self.bots.values().all(|&v| v == 1)
    }
}

impl ::std::fmt::Display for Grid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = match self.bots.get(&(x, y)) {
                    Some(cnt) => {
                        if *cnt < 10 {
                            cnt.to_string().chars().next().unwrap()
                        } else {
                            'X'
                        }
                    }
                    None => '.',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part_2(input: &[Robot], wdith: i32, height: i32) -> usize {
    let mut bots = input.to_vec();
    let mut grid = Grid::new(wdith, height);
    let mut steps = 0;
    loop {
        grid.clear();
        bots.iter_mut().for_each(|r| {
            r.step(1, wdith, height);
            grid.add(*r);
        });
        steps += 1;
        // Guessing that we need all robots to be in a unique location for the tree to appear
        if grid.all_unique() {
            println!("{}", grid);
            println!("Steps so far: {}", steps);
            println!("(s)top, any key to continue > ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "s" {
                break;
            }
        }
    }
    steps
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input, 101, 103);
    println!("Part 1: {}", p1);
    let p2 = part_2(&input, 101, 103);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teleport() {
        let mut r = Robot {
            x: 0,
            y: 0,
            vx: 1,
            vy: 0,
        };
        r.step(9, 10, 10);
        assert_eq!(r.x, 9);
        assert_eq!(r.y, 0);
        r.step(2, 10, 10);
        assert_eq!(r.x, 1);
        assert_eq!(r.y, 0);
    }

    #[test]
    fn teleport_neg() {
        let mut r = Robot {
            x: 0,
            y: 0,
            vx: -1,
            vy: 0,
        };
        r.step(9, 10, 10);
        assert_eq!(r.x, 1);
        assert_eq!(r.y, 0);
        r.step(2, 10, 10);
        assert_eq!(r.x, 9);
        assert_eq!(r.y, 0);
    }

    #[test]
    fn quadrant() {
        let r = Robot {
            x: 3,
            y: 3,
            vx: 1,
            vy: 1,
        };
        assert_eq!(r.quadrant(4, 4), Some((1, 1)));
        assert_eq!(r.quadrant(7, 4), None);
        assert_eq!(r.quadrant(4, 7), None);
        assert_eq!(r.quadrant(7, 7), None);
        assert_eq!(r.quadrant(13, 4), Some((0, 1)));
        assert_eq!(r.quadrant(13, 13), Some((0, 0)));
    }

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input, 11, 7), 12);
    }
}
