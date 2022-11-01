// https://adventofcode.com/2015/day/2

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn read<R: Read>(io: R) -> Vec<(usize, usize, usize)> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| {
            let line = line.unwrap();
            let mut nums = line
                .split("x")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            nums.sort();
            (nums[0], nums[1], nums[2])
        })
        .collect()
}

fn part_1(dims: &[(usize, usize, usize)]) -> usize {
    dims.iter()
        .map(|d| 3 * d.0 * d.1 + 2 * d.0 * d.2 + 2 * d.1 * d.2)
        .sum()
}

fn part_2(dims: &[(usize, usize, usize)]) -> usize {
    dims.iter()
        .map(|d| 2 * d.0 + 2 * d.1 + d.0 * d.1 * d.2)
        .sum()
}

fn main() {
    let dims = read(File::open("input.txt").unwrap());
    let sq_feet_of_wrapping_paper = part_1(&dims);
    println!("The elves need {} sq feet of wrapping paper", sq_feet_of_wrapping_paper);
    let feet_of_ribbon = part_2(&dims);
    println!("..and {} feet of ribbon", feet_of_ribbon);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let dims = read(File::open("test1.txt").unwrap());
        assert_eq!(dims.len(), 2);
        assert_eq!(dims[0], (2, 3, 4));
        assert_eq!(dims[1], (1, 1, 10));
    }

    #[test]
    fn test_part_1() {
        let dims = read(File::open("test1.txt").unwrap());
        assert_eq!(part_1(&dims), 58 + 43)
    }

    #[test]
    fn test_part_2() {
        let dims = read(File::open("test1.txt").unwrap());
        assert_eq!(part_2(&dims), 34 + 14)
    }
}
