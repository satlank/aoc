use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::HashMap;

#[macro_use] extern crate scan_fmt;

#[derive(Debug, Eq, PartialEq)]
struct BPass {
    code: String,
}

impl BPass {
    fn parse<S: AsRef<str>>(line: S) -> Result<BPass, scan_fmt::parse::ScanError> {
        let code = scan_fmt!(line.as_ref(), "{/[FB]{7}[LR]{3}/}", String)?;
        Ok(BPass{code})
    }

    fn id(&self) -> usize {
        let (row, column) = self.resolve();
        row * 8 + column
    }

    fn resolve(&self) -> (usize, usize) {
        let mut row: usize = 0;
        let mut col: usize = 0;
        for c in self.code[0..7].chars() {
            row <<= 1;
            if c == 'B' {
                row += 1;
            }
        }
        for c in self.code[7..10].chars() {
            col <<= 1;
            if c == 'R' {
                col += 1;
            }
        }
        (row, col)
    }
}

fn read<R: Read>(io: R) -> Result<Vec<BPass>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(
            |line| line.and_then(
                |row| BPass::parse(row).map_err(|e| Error::new(ErrorKind::InvalidData, e))
            )
        )
        .collect()
}

fn part1(vec: &Vec<BPass>) -> usize {
    vec.iter().map(|b| b.id()).max().unwrap_or(0)
}

fn part2(vec: &Vec<BPass>) -> usize {
    let map: HashMap<_, _> = vec.iter().map(|b| (b.id(), b)).collect();
    let min = map.keys().min().unwrap();
    let max = map.keys().max().unwrap();
    let mut id = *min + 1;
    loop {
        if id >= *max {
            break;
        }
        if !map.contains_key(&id) && map.contains_key(&(id + 1)) {
            break;
        }
        id += 2;
    };
    id
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    println!("Largest id: {}", part1(&vec));
    println!("Seat id: {}", part2(&vec));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        assert_eq!((70, 7), BPass::parse("BFFFBBFRRR").unwrap().resolve());
        assert_eq!((14, 7), BPass::parse("FFFBBBFRRR").unwrap().resolve());
        assert_eq!((102, 4), BPass::parse("BBFFBBFRLL").unwrap().resolve());
    }

    #[test]
    fn test_id() {
        assert_eq!(567, BPass::parse("BFFFBBFRRR").unwrap().id());
        assert_eq!(119, BPass::parse("FFFBBBFRRR").unwrap().id());
        assert_eq!(820, BPass::parse("BBFFBBFRLL").unwrap().id());
    }

}
