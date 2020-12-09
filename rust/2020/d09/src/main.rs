use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<usize>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn is_sum(vec: &[usize], val: usize) -> bool {
    for i in 0..vec.len() - 1 {
        for j in i + 1..vec.len() {
            if vec[i] + vec[j] == val {
                return true;
            }
        }
    }
    false
}

fn part1(vec: &[usize]) -> Option<usize> {
    for i in 25..vec.len() {
        if !is_sum(&vec[i - 25..i], vec[i]) {
            return Some(vec[i]);
        }
    }
    None
}

fn part2(vec: &[usize], val: usize) -> Option<usize> {
    for i in 0..vec.len() - 1 {
        let mut sum = vec[i];
        let mut j = i + 1;
        while sum < val {
            sum += vec[j];
            j += 1;
        }
        if sum == val {
            return Some(vec[i..=j].iter().min().unwrap() + vec[i..=j].iter().max().unwrap());
        }
    }
    None
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    println!("Read {} numbers", vec.len());
    let val = part1(&vec).unwrap();
    println!("{} is not a sum of two preamble values", val);
    println!("Encryption weakness is {}", part2(&vec, val).unwrap());
    Ok(())
}
