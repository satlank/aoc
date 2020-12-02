use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[macro_use] extern crate scan_fmt;

#[derive(Debug, Eq, PartialEq)]
struct PasswordRow {
    min: i32,
    max: i32,
    letter: char,
    password: String
}

impl PasswordRow {
    fn parse<S: AsRef<str>>(line: S) -> Result<PasswordRow, scan_fmt::parse::ScanError> {
        let (min, max, letter, password) = scan_fmt!(
            line.as_ref(),
            "{d}-{d} {[a-z]}: {}",
            i32, i32, char, String
        )?;
        Ok(PasswordRow{min, max, letter, password})
    }

    fn valid(&self) -> bool {
        let cnt = self.password.chars().filter(|&c| c == self.letter).count() as i32;
        self.min <= cnt && cnt <= self.max
    }

    fn valid2(&self) -> bool {
        let a = self.password.chars().nth((self.min - 1) as usize);
        let b = self.password.chars().nth((self.max - 1) as usize);
        let c = Some(self.letter);
        (a == c && b != c) || (a != c && b == c)
    }
}

fn read<R: Read>(io: R) -> Result<Vec<PasswordRow>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(
            |line| line.and_then(
                |row| PasswordRow::parse(row).map_err(|e| Error::new(ErrorKind::InvalidData, e))
            )
        )
        .collect()
}

fn part1(vec: &Vec<PasswordRow>) -> i32 {
    vec.iter().filter(|&r| r.valid()).count() as i32
}

fn part2(vec: &Vec<PasswordRow>) -> i32 {
    vec.iter().filter(|&r| r.valid2()).count() as i32
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    println!("Number of valid passwords: {}", part1(&vec));
    println!("Number of valid passwords part2: {}", part2(&vec));
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            PasswordRow::parse("1-3 a: asd").unwrap(),
            PasswordRow {min: 1, max: 3, letter: 'a', password: "asd".to_string()});
    }

    #[test]
    fn test_valid() {
        assert!(
            PasswordRow { min: 1, max: 3, letter: 'a', password: "abc".to_string() }.valid()
        );
    }

    #[test]
    fn test_valid2() {
        assert!(
            PasswordRow { min: 1, max: 3, letter: 'a', password: "abcde".to_string() }.valid2()
        );
        assert!(
            !PasswordRow { min: 1, max: 3, letter: 'b', password: "abcde".to_string() }.valid2()
        );
        assert!(
            !PasswordRow { min: 2, max: 9, letter: 'c', password: "ccccccccc".to_string() }.valid2()
        );
    }
}
