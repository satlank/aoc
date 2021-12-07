// https://adventofcode.com/2021/day/7

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn read<R: Read>(io: R) -> Vec<i32> {
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines().filter_map(Result::ok).collect();
    let line = lines.iter().next().unwrap();
    let mut nums: Vec<i32> = line.split(",").map(|i| i.parse::<i32>().unwrap()).collect();
    nums.sort();
    nums
}

fn part_1(crabs: &[i32]) -> (i32, i32) {
    // Median minimizes Manhattan distance, which is how the fuel cost is calculated
    // Note: crabs must be sorted
    let median = crabs[crabs.len() / 2];
    (median, crabs.iter().map(|x| i32::abs(x - median)).sum())
}

fn fuel_usage_2(crabs: &[i32], pos: i32) -> i32 {
    crabs.iter().map(|&p| {
        let dist = i32::abs(p - pos);
        dist * (dist + 1) / 2
    }).sum()
}

fn part_2(crabs: &[i32]) -> (i32, i32) {
    // Brute-force this, not sure which 'center' measure minimizes Gauss summation distances
    // Note: crabs must be sorted
    let mut cost = i32::MAX;
    let mut min_idx = 0;
    for pos in crabs[0]..=crabs[crabs.len() - 1] {
        let this_cost = fuel_usage_2(crabs, pos);
        if this_cost < cost {
            cost = this_cost;
            min_idx = pos;
        }
    }
    (min_idx, cost)
}

fn main() {
    let nums = read(File::open("input.txt").unwrap());
    let (pos, fuel) = part_1(&nums);
    println!("Aligning at {}, taking {} fuel", pos, fuel);
    let (pos, fuel) = part_2(&nums);
    println!("Aligning at {}, taking {} fuel", pos, fuel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let nums = read(File::open("test1.txt").unwrap());
        assert_eq!(nums.len(), 10);
        let (pos, fuel) = part_1(&nums);
        assert_eq!(pos, 2);
        assert_eq!(fuel, 37);
    }

    #[test]
    fn test_part_2() {
        let nums = read(File::open("test1.txt").unwrap());
        assert_eq!(nums.len(), 10);
        let (pos, fuel) = part_2(&nums);
        assert_eq!(pos, 5);
        assert_eq!(fuel, 168);
    }
}
