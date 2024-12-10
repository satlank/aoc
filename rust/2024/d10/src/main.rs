// https://adventofcode.com/2024/day/10

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Map {
    let br = BufReader::new(io);
    let lines: Vec<_> = br.lines().map_while(Result::ok).collect();

    lines.into()
}

struct Map {
    height: usize,
    width: usize,
    values: Vec<u8>,
}

impl Map {
    fn locations_iter(&self, val: u8) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.values
            .iter()
            .enumerate()
            .filter_map(move |(i, &v)| if v == val { Some(i) } else { None })
            .map(|i| (i % self.width, i / self.width))
    }

    fn neighbors_down(&self, (x, y): (usize, usize)) -> HashSet<(usize, usize)> {
        let mut neighbors = HashSet::new();
        let val = self[(x, y)];
        if val == 0 {
            return neighbors;
        }
        if x > 0 && self[(x - 1, y)] == val - 1 {
            neighbors.insert((x - 1, y));
        }
        if y > 0 && self[(x, y - 1)] == val - 1 {
            neighbors.insert((x, y - 1));
        }
        if x < self.width - 1 && self[(x + 1, y)] == val - 1 {
            neighbors.insert((x + 1, y));
        }
        if y < self.height - 1 && self[(x, y + 1)] == val - 1 {
            neighbors.insert((x, y + 1));
        }
        neighbors
    }
}

impl ::std::ops::Index<(usize, usize)> for Map {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.values[y * self.width + x]
    }
}

impl From<Vec<String>> for Map {
    fn from(lines: Vec<String>) -> Self {
        let height = lines.len();
        let width = lines[0].len();
        let values = lines
            .into_iter()
            .flat_map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
            .collect();

        Self {
            height,
            width,
            values,
        }
    }
}

fn part_1(topographic_map: &Map) -> usize {
    topographic_map
        .locations_iter(9)
        .flat_map(|(x, y)| {
            let mut reachable = HashSet::new();
            reachable.insert((x, y));
            loop {
                let new_reachable = reachable
                    .iter()
                    .flat_map(|&pos| topographic_map.neighbors_down(pos))
                    .collect::<HashSet<_>>();
                if new_reachable.is_empty() {
                    break;
                }
                reachable = new_reachable;
            }
            reachable
        })
        .count()
}

fn part_2(topographic_map: &Map) -> usize {
    topographic_map
        .locations_iter(9)
        .flat_map(|(x, y)| {
            let mut reachable = Vec::new();
            reachable.push((x, y));
            loop {
                let new_reachable = reachable
                    .iter()
                    .flat_map(|&pos| topographic_map.neighbors_down(pos))
                    .collect::<Vec<_>>();
                if new_reachable.is_empty() {
                    break;
                }
                reachable = new_reachable;
            }
            reachable
        })
        .count()
}

fn main() {
    let topographic_map = read(File::open("input.txt").unwrap());
    let p1 = part_1(&topographic_map);
    println!("Part 1: {}", p1);
    let p2 = part_2(&topographic_map);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let topographic_map = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&topographic_map), 36);
        assert_eq!(part_2(&topographic_map), 81);
    }
}
