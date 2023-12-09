// https://adventofcode.com/2023/day/9

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<Vec<i32>> {
    let br = BufReader::new(io);
    br.lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn forward(values: &[i32]) -> i32 {
    let diffs: Vec<i32> = values.windows(2).map(|w| w[1] - w[0]).collect();
    if diffs.iter().all(|&d| d == 0) {
        *values.last().unwrap()
    } else {
        let a = forward(&diffs);
        values.last().unwrap() + a
    }
}

fn backward(values: &[i32]) -> i32 {
    let reversed = values.iter().copied().rev().collect::<Vec<_>>();
    forward(&reversed)
}

fn part_1(input: &[Vec<i32>]) -> usize {
    input.iter().map(|v| forward(v)).sum::<i32>() as usize
}

fn part_2(input: &[Vec<i32>]) -> usize {
    input.iter().map(|v| backward(v)).sum::<i32>() as usize
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
        assert_eq!(forward(&input[0]), 18);
        assert_eq!(forward(&input[1]), 28);
        assert_eq!(forward(&input[2]), 68);
        assert_eq!(part_1(&input), 114);

        assert_eq!(backward(&input[0]), -3);
        assert_eq!(backward(&input[1]), 0);
        assert_eq!(backward(&input[2]), 5);
        assert_eq!(part_2(&input), 2);
    }
}
