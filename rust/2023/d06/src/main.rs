// https://adventofcode.com/2023/day/6

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> (Vec<usize>, Vec<usize>) {
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines().filter_map(Result::ok).collect();
    let times = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(times.len(), distances.len());
    (times, distances)
}

fn part_1(times: &[usize], distances: &[usize]) -> usize {
    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| (0..*t).map(|x| x * (*t - x)).filter(|x| x > d).count())
        .product()
}

fn part_2(times: &[usize], distances: &[usize]) -> usize {
    let act_time = times
        .iter()
        .fold(String::new(), |mut acc, x| {
            acc.push_str(&format!("{}", x));
            acc
        })
        .parse::<usize>()
        .unwrap();
    let act_dist = distances
        .iter()
        .fold(String::new(), |mut acc, x| {
            acc.push_str(&format!("{}", x));
            acc
        })
        .parse::<usize>()
        .unwrap();
    (0..act_time)
        .map(|x| x * (act_time - x))
        .filter(|x| x > &act_dist)
        .count()
}

fn main() {
    let (times, distances) = read(File::open("input.txt").unwrap());
    let p1 = part_1(&times, &distances);
    println!("Part 1: {}", p1);
    let p2 = part_2(&times, &distances);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let (times, distances) = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&times, &distances), 288);
        assert_eq!(part_2(&times, &distances), 71503);
    }
}
