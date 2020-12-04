use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::HashMap;

#[macro_use] extern crate scan_fmt;

#[derive(Debug, Eq, PartialEq)]
struct Passport {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

impl Passport {
    fn parse<S: AsRef<str>>(details: S) -> Result<Passport, scan_fmt::parse::ScanError> {
        let mut vals = HashMap::new();
        for entry in details.as_ref().split_whitespace() {
            let (key, val) = scan_fmt!(entry, "{}:{}", String, String)?;
            vals.insert(key, val);
        }
        Ok(Passport {
            byr: vals.remove("byr").or(None),
            iyr: vals.remove("iyr").or(None),
            eyr: vals.remove("eyr").or(None),
            hgt: vals.remove("hgt").or(None),
            hcl: vals.remove("hcl").or(None),
            ecl: vals.remove("ecl").or(None),
            pid: vals.remove("pid").or(None),
            cid: vals.remove("cid").or(None),
        })
    }

    fn valid(&self) -> bool {
        self.byr.is_some() && self.iyr.is_some() && self.eyr.is_some() && self.hgt.is_some() && self.hcl.is_some() && self.ecl.is_some() && self.pid.is_some()
    }
}

fn read<R: Read>(io: R) -> Result<Vec<Passport>, Error> {
    let mut res = Vec::new();
    let br = BufReader::new(io);
    let mut item = String::from("");
    for line in br.lines() {
        let line = line?;
        if line.len() == 0 {
            res.push(Passport::parse(item).map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
            item = String::from("");
        }
        item = format!("{} {}", item, line);
    }
    Ok(res)
}

fn part1(vec: &Vec<Passport>) -> usize {
    vec.iter().filter(|&p| p.valid()).count()
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    println!("Number of valid passports: {}", part1(&vec));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let passport = Passport::parse("byr:123").unwrap();
        assert_eq!(passport.byr, Some("123".to_string()));
    }

    #[test]
    fn test_validation() {
        assert!(
            Passport::parse("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm")
            .unwrap()
            .valid()
        );
        assert!(
            !Passport::parse("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929")
            .unwrap()
            .valid()
        );
        assert!(
            Passport::parse("hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm")
            .unwrap()
            .valid()
        );
        assert!(
            !Passport::parse("hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in")
            .unwrap()
            .valid()
        );
    }
}
