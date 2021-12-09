// https://adventofcode.com/2021/day/9

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn read<R: Read>(io: R) -> Vec<Vec<u32>> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn get_neighbours(
    pos: (usize, usize),
    bounds: &((usize, usize), (usize, usize)),
) -> Vec<(usize, usize)> {
    let mut n: Vec<(usize, usize)> = Vec::new();
    if pos.0 > bounds.0 .0 {
        n.push((pos.0 - 1, pos.1));
    }
    if pos.0 < bounds.1 .0 - 1 {
        n.push((pos.0 + 1, pos.1));
    }
    if pos.1 > bounds.0 .1 {
        n.push((pos.0, pos.1 - 1));
    }
    if pos.1 < bounds.1 .1 - 1 {
        n.push((pos.0, pos.1 + 1));
    }
    n
}

fn find_low_points(field: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    let bounds = ((0, 0), (field[0].len(), field.len()));
    for y in 0..field.len() {
        for x in 0..field[y].len() {
            let neighbours = get_neighbours((x, y), &bounds);
            let is_low = neighbours
                .iter()
                .filter(|n| field[n.1][n.0] < field[y][x])
                .count()
                == 0;
            if is_low {
                res.push((x, y));
            }
        }
    }
    res
}

fn find_basin(min: (usize, usize), field: &[Vec<u32>]) -> HashSet<(usize, usize)> {
    let mut basin: HashSet<(usize, usize)> = HashSet::new();
    let bounds = ((0, 0), (field[0].len(), field.len()));
    basin.insert(min);
    loop {
        let new: Vec<(usize, usize)> = basin
            .iter()
            .map(|p| {
                get_neighbours(*p, &bounds)
                    .iter()
                    .filter(|p| !basin.contains(p))
                    .filter(|p| field[p.1][p.0] < 9)
                    .copied()
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect();
        if new.len() == 0 {
            return basin;
        }
        for p in new {
            basin.insert(p);
        }
    }
}

fn part_1(field: &[Vec<u32>]) -> u32 {
    find_low_points(field).iter().map(|p| field[p.1][p.0] + 1).sum()
}

fn part_2(field: &[Vec<u32>]) -> usize {
    let mut bassin_sizes: Vec<usize> = find_low_points(field).iter().map(|p| find_basin(*p, field).len()).collect();
    bassin_sizes.sort();
    let len = bassin_sizes.len() - 1;
    bassin_sizes[len] * bassin_sizes[len - 1] * bassin_sizes[len - 2]
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    assert_eq!(input.len(), 100);
    let risk = part_1(&input);
    println!("The risk is {}", risk);
    let bassin_sizes = part_2(&input);
    println!("Product of bassin_sizes is {}", bassin_sizes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let input = read(File::open("test1.txt").unwrap());
        assert_eq!(input.len(), 5);
        for row in input {
            assert_eq!(row.len(), 10);
        }
    }

    #[test]
    fn test_find_low_points() {
        let input = read(File::open("test1.txt").unwrap());
        let low_points = find_low_points(&input);
        assert_eq!(low_points, vec!((1, 0), (9, 0), (2, 2), (6, 4)));
    }

    #[test]
    fn test_part_1() {
        let input = read(File::open("test1.txt").unwrap());
        let risk = part_1(&input);
        assert_eq!(risk, 15);
    }

    #[test]
    fn test_bassin() {
        let input = read(File::open("test1.txt").unwrap());
        let basin = find_basin((1, 0), &input);
        assert_eq!(basin.len(), 3);
        let basin = find_basin((2, 2), &input);
        assert_eq!(basin.len(), 14);

    }

    #[test]
    fn test_part_2() {
        let input = read(File::open("test1.txt").unwrap());
        let bassin_sizes = part_2(&input);
        assert_eq!(bassin_sizes, 1134);
    }
}
