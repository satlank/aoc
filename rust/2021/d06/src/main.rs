// https://adventofcode.com/2021/day/4

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn read<R: Read>(io: R) -> Vec<usize> {
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines().filter_map(Result::ok).collect();
    let line = lines.iter().next().unwrap();
    line
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect()
}

fn age_population(by_age: &mut [usize], days: usize) {
    for _day in 1..=days {
        let new_fish = by_age[0];
        for i in 1..9 {
            by_age[i-1] = by_age[i]
        }
        by_age[6] += new_fish;
        by_age[8] = new_fish;
    }
}

fn part_1(nums: &[usize]) -> usize {
    let mut by_age = vec!(0, 0, 0, 0, 0, 0, 0, 0, 0);
    for &idx in nums {
        by_age[idx] += 1;
    }
    age_population(by_age.as_mut_slice(), 80);
    by_age.iter().sum()
}

fn part_2(nums: &[usize]) -> usize {
    let mut by_age = vec!(0, 0, 0, 0, 0, 0, 0, 0, 0);
    for &idx in nums {
        by_age[idx] += 1;
    }
    age_population(by_age.as_mut_slice(), 256);
    by_age.iter().sum()
}

fn main() {
    let nums = read(File::open("input.txt").unwrap());
    let fish = part_1(&nums);
    println!("Lanternfish after 80 days: {}", fish);
    let fish = part_2(&nums);
    println!("Lanternfish after 256 days: {}", fish);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let nums = read(File::open("test1.txt").unwrap());
        let fish = part_1(&nums);
        assert_eq!(nums.len(), 5);
        assert_eq!(fish, 5934);
    }

    #[test]
    fn test_part_2() {
        let nums = read(File::open("test1.txt").unwrap());
        let fish = part_2(&nums);
        assert_eq!(nums.len(), 5);
        assert_eq!(fish, 26984457539);
    }
}
