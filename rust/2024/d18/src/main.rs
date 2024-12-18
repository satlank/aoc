// https://adventofcode.com/2024/day/18

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl ::std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Vec2 {
    fn neighbors(&self, width: i32, height: i32) -> Vec<Vec2> {
        let mut res = Vec::with_capacity(4);
        if self.x > 0 {
            res.push(Vec2 {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            res.push(Vec2 {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.x < width - 1 {
            res.push(Vec2 {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y < height - 1 {
            res.push(Vec2 {
                x: self.x,
                y: self.y + 1,
            });
        }
        res
    }
}

impl ::std::str::FromStr for Vec2 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .ok_or("missing x")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        let y = parts
            .next()
            .ok_or("missing y")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        Ok(Vec2 { x, y })
    }
}

fn read<R: Read>(io: R) -> Vec<Vec2> {
    let br = BufReader::new(io);
    br.lines()
        .map_while(Result::ok)
        .map(|l| l.parse().unwrap())
        .collect()
}

fn walk(bad: &HashSet<Vec2>, exit: Vec2) -> Option<usize> {
    let mut current = [Vec2 { x: 0, y: 0 }].into_iter().collect::<HashSet<_>>();
    let mut cnt = 0;

    loop {
        let new = current
            .iter()
            .copied()
            .flat_map(|v| v.neighbors(exit.x + 1, exit.y + 1))
            .filter(|v| !bad.contains(v) && !current.contains(v))
            .collect::<HashSet<_>>();
        if new.is_empty() {
            return None;
        }
        current.extend(new);
        cnt += 1;
        if current.contains(&exit) {
            break;
        }
    }
    Some(cnt)
}

fn part_1(input: &[Vec2], n_fallen: usize, exit: Vec2) -> usize {
    walk(&input.iter().take(n_fallen).copied().collect(), exit).unwrap()
}

fn part_2(input: &[Vec2], exit: Vec2) -> Vec2 {
    let mut min = 0;
    let mut max = input.len() - 1;
    let mut trial = (max - min) / 2;

    loop {
        let bad = input.iter().take(trial).copied().collect::<HashSet<_>>();
        let res = walk(&bad, exit);
        let can_exit = match res {
            Some(_) => {
                min = trial + 1;
                true
            }
            None => {
                max = trial - 1;
                false
            }
        };
        if max == min {
            trial = if can_exit { max + 1 } else { max - 1 };
            break;
        }
        trial = min + (max - min) / 2;
    }
    input[trial]
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let exit = Vec2 { x: 70, y: 70 };
    let p1 = part_1(&input, 1024, exit);
    println!("Part 1: {}", p1);
    let p2 = part_2(&input, exit);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        let exit = Vec2 { x: 6, y: 6 };
        println!("{:?}", input);
        assert_eq!(part_1(&input, 12, exit), 22);
        assert_eq!(part_2(&input, exit), Vec2 { x: 6, y: 1 });
    }
}
