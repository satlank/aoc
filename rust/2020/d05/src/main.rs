use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[macro_use]
extern crate scan_fmt;

#[derive(Debug, Eq, PartialEq)]
struct BPass {
    code: String,
}

impl BPass {
    fn parse<S: AsRef<str>>(line: S) -> Result<BPass, scan_fmt::parse::ScanError> {
        let code = scan_fmt!(line.as_ref(), "{/[FB]{7}[LR]{3}/}", String)?;
        Ok(BPass { code })
    }

    fn id(&self) -> usize {
        let bin = self
            .code
            .chars()
            .map(|c| if c == 'B' || c == 'R' { '1' } else { '0' })
            .collect::<String>();
        usize::from_str_radix(&bin, 2).unwrap()
    }

    #[cfg(test)]
    fn resolve(&self) -> (usize, usize) {
        let id = self.id();
        ((id & 1017) >> 3, id & 7)
    }
}

fn read<R: Read>(io: R) -> Result<Vec<BPass>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| {
            line.and_then(|row| {
                BPass::parse(row).map_err(|e| Error::new(ErrorKind::InvalidData, e))
            })
        })
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
    }
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
