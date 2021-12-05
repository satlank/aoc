// https://adventofcode.com/2021/day/3

use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn read<R: Read>(io: R) -> Vec<Vec<bool>> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.unwrap().chars().map(|c| c == '1').collect())
        .collect()
}

fn part_1(vals: &[Vec<bool>]) -> (i64, i64) {
    let mut histo = Vec::new();
    let mut place_value = Vec::new();
    for i in 0..vals[0].len() {
        histo.push(0);
        place_value.push(2_i64.pow(i as u32));
    }
    let place_value: Vec<i64> = place_value.into_iter().rev().collect();
    for val in vals {
        for i in 0..val.len() {
            if val[i] {
                histo[i] += 1;
            }
        }
    }
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    let half = vals.len() / 2;
    for i in 0..histo.len() {
        if histo[i] > half {
            gamma_rate += place_value[i];
        } else {
            epsilon_rate += place_value[i];
        }
    }
    (gamma_rate, epsilon_rate)
}

fn mostcommon_at_pos_inclusive(vals: &[&Vec<bool>], idx: usize) -> bool {
    let mut cnt = 0;
    for val in vals {
        if val[idx] {
            cnt += 1
        }
    }
    cnt >= vals.len() - cnt
}

fn find_o2_gen_rating(vals: &[Vec<bool>]) -> Vec<bool> {
    let mut tmp: Vec<&Vec<bool>> = vals.iter().collect();
    let mut idx = 0;
    while tmp.len() > 1 {
        let mc = mostcommon_at_pos_inclusive(&tmp, idx);
        tmp = tmp.iter().filter(|x| x[idx] == mc).copied().collect();
        idx += 1;
    }
    tmp[0].to_vec()
}

fn find_co2_scrub_rating(vals: &[Vec<bool>]) -> Vec<bool> {
    let mut tmp: Vec<&Vec<bool>> = vals.iter().collect();
    let mut idx = 0;
    while tmp.len() > 1 {
        let mc = mostcommon_at_pos_inclusive(&tmp, idx);
        tmp = tmp.iter().filter(|x| x[idx] != mc).copied().collect();
        idx += 1;
    }
    tmp[0].to_vec()
}

fn to_value(vec: &[bool], place_value: &[i64]) -> i64 {
    let mut res = 0;
    for i in 0..vec.len() {
        if vec[i] {
            res += place_value[i];
        }
    }
    res
}

fn part_2(vals: &[Vec<bool>]) -> (i64, i64) {
    let mut place_value = Vec::new();
    for i in 0..vals[0].len() {
        place_value.push(2_i64.pow(i as u32));
    }
    let place_value: Vec<i64> = place_value.into_iter().rev().collect();

    (
        to_value(&find_o2_gen_rating(vals), &place_value),
        to_value(&find_co2_scrub_rating(vals), &place_value),
    )
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?);
    let (gamma_rate, epsilon_rate) = part_1(&vec);
    println!(
        "gamma rate: {},  epsilon rate: {}  -->  {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );
    let (o2_rate, co2_rate) = part_2(&vec);
    println!(
        "o2 generator rate: {},  co2 scrubbing rate: {}  -->  {}",
        o2_rate,
        co2_rate,
        o2_rate * co2_rate
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let vals = read(File::open("test1.txt").unwrap());
        assert_eq!(vals.len(), 12);
        assert_eq!(vals[0].len(), 5);
        let (g_rate, e_rate) = part_1(&vals);
        assert_eq!(g_rate, 22);
        assert_eq!(e_rate, 9);
    }

    #[test]
    fn test_part_2() {
        let vals = read(File::open("test1.txt").unwrap());
        assert_eq!(vals.len(), 12);
        assert_eq!(vals[0].len(), 5);
        let (o2_rate, co2_rate) = part_2(&vals);
        assert_eq!(o2_rate, 23);
        assert_eq!(co2_rate, 10);
    }
}
