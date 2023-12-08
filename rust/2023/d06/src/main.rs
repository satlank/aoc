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

fn possible_times(t: usize, d: usize) -> usize {
    assert!(t < 2usize.pow(53), "Not enough precision");
    assert!(d < 2usize.pow(53), "Not enough precision");
    let tf = t as f64;
    let df = d as f64;
    let min = (tf / 2.0 - (tf * tf / 4.0 - df).sqrt()).floor() as usize;
    let max = (tf / 2.0 + (tf * tf / 4.0 - df).sqrt()).ceil() as usize;

    // Deal with floating point, we've floor'ed the min and ceil'ed the max, so most
    // likely have to add (subtract) 1 to the min (max) to get the real min (max).
    // However the floor'ed/ceil'ed value might be the real min/max if we happend to land exactly
    // on the min/max.
    let real_min = if min * (t - min) <= d { min + 1 } else { min };
    let real_max = if max * (t - max) <= d { max - 1 } else { max };

    real_max - real_min + 1
}

fn part_1(times: &[usize], distances: &[usize]) -> usize {
    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| possible_times(*t, *d))
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
    possible_times(act_time, act_dist)
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
