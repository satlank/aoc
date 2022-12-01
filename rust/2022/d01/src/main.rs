// https://adventofcode.com/2022/day/1

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<Vec<i64>> {
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines().filter_map(Result::ok).collect();
    let mut res = Vec::new();
    let mut elves = 0;
    res.push(Vec::new());
    for idx in 0..lines.len() {
        if lines[idx].is_empty() {
            elves += 1;
            res.push(Vec::new());
        } else {
            res[elves].push(lines[idx].parse::<i64>().unwrap());
        }
    }
    res
}

fn part_1(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

fn part_2(input: &[Vec<i64>]) -> i64 {
    let mut summed: Vec<i64> = input.iter().map(|elf| elf.iter().sum()).collect();
    summed.sort();
    summed[summed.len()-3..].iter().sum()
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let most_calories = part_1(&input);
    println!("Most calories: {}", most_calories);
    let top3_sum_calories = part_2(&input);
    println!("Top 3 calories: {}", top3_sum_calories);
}
