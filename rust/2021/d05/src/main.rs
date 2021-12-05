// https://adventofcode.com/2021/day/5

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[macro_use]
extern crate scan_fmt;

#[derive(Debug, Eq, PartialEq)]
struct Line {
    from_x: i32,
    to_x: i32,
    from_y: i32,
    to_y: i32,
}

impl Line {
    fn parse<S: AsRef<str>>(line: S) -> Result<Line, scan_fmt::parse::ScanError> {
        let (from_x, from_y, to_x, to_y) =
            scan_fmt!(line.as_ref(), "{d},{d} -> {d},{d}", i32, i32, i32, i32)?;
        Ok(Line {
            from_x,
            to_x,
            from_y,
            to_y,
        })
    }

    fn is_vertical(&self) -> bool {
        self.from_x == self.to_x
    }

    fn is_horizontal(&self) -> bool {
        self.from_y == self.to_y
    }

    fn as_points(&self) -> Vec<(i32, i32)> {
        let inc_x = self.to_x - self.from_x;
        let inc_y = self.to_y - self.from_y;
        let max_inc = i32::max(i32::abs(inc_x), i32::abs(inc_y));
        (0..=max_inc)
            .map(|inc| {
                (
                    self.from_x + inc * i32::signum(inc_x),
                    self.from_y + inc * i32::signum(inc_y),
                )
            })
            .collect()
    }
}

fn read<R: Read>(io: R) -> Result<Vec<Line>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| {
            line.and_then(|row| Line::parse(row).map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        })
        .collect()
}

fn part_1(lines: &[Line]) -> i32 {
    let mut histo: HashMap<(i32, i32), i32> = HashMap::new();
    for line in lines {
        if line.is_horizontal() || line.is_vertical() {
            for p in line.as_points() {
                *histo.entry(p).or_insert(0) += 1;
            }
        }
    }
    histo.values().filter(|&x| *x >= 2).count() as i32
}

fn part_2(lines: &[Line]) -> i32 {
    let mut histo: HashMap<(i32, i32), i32> = HashMap::new();
    for line in lines {
        for p in line.as_points() {
            *histo.entry(p).or_insert(0) += 1;
        }
    }
    histo.values().filter(|&x| *x >= 2).count() as i32
}

fn main() -> Result<(), Error> {
    let lines = read(File::open("input.txt")?)?;
    let overlaps = part_1(&lines);
    println!(
        "At least two lines overlap in {} points (only horizontal/vertical lines)",
        overlaps
    );
    let overlaps_2 = part_2(&lines);
    println!(
        "At least two lines overlap in {} points (all lines)",
        overlaps_2
    );
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let lines = read(File::open("test1.txt").unwrap()).unwrap();
        assert_eq!(lines.len(), 10);
        let overlaps = part_1(&lines);
        assert_eq!(overlaps, 5);
    }

    #[test]
    fn test_part_2() {
        let lines = read(File::open("test1.txt").unwrap()).unwrap();
        assert_eq!(lines.len(), 10);
        let overlaps = part_2(&lines);
        assert_eq!(overlaps, 12);
    }
}
