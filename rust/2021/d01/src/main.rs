// https://adventofcode.com/2021/day/1

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn part_1(vec: &[i64]) -> i64 {
    let mut inc = 0;
    for i in 1..vec.len() {
        if vec[i-1] < vec[i] {
            inc += 1;
        }
    }
    inc
}

fn part_2(vec: &[i64]) -> i64 {
    let mut inc = 0;
    for i in 3..vec.len() {
        let prev = vec[i-3] + vec[i-2] + vec[i-1];
        let cur = vec[i] + vec[i-1] + vec[i-2];
        if prev < cur {
            inc += 1;
        }
    }
    inc
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    let res1 = part_1(&vec);
    println!("Times depth increases: {}", res1);
    let res2 = part_2(&vec);
    println!("Times depth increases in sliding window: {}", res2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let d: Vec<i64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, part_1(&d));
    }

    #[test]
    fn test_2() {
        let d: Vec<i64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, part_2(&d));
    }
}
