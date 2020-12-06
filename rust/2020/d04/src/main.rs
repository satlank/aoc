use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[macro_use]
extern crate scan_fmt;

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
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn valid2(&self) -> bool {
        if !self.valid() {
            return false;
        }
        validate_year(self.byr.as_ref().unwrap(), 1920, 2002)
            && validate_year(self.iyr.as_ref().unwrap(), 2010, 2020)
            && validate_year(self.eyr.as_ref().unwrap(), 2020, 2030)
            && validate_height(self.hgt.as_ref().unwrap())
            && validate_hcl(self.hcl.as_ref().unwrap())
            && validate_ecl(self.ecl.as_ref().unwrap())
            && validate_pid(self.pid.as_ref().unwrap())
    }
}

fn validate_year<S: AsRef<str>>(year: S, min: usize, max: usize) -> bool {
    let val = match scan_fmt!(year.as_ref(), "{}", usize) {
        Ok(x) => x,
        Err(_) => return false,
    };
    val >= min && val <= max
}

fn validate_height<S: AsRef<str>>(hgt: S) -> bool {
    let (val, unit) = match scan_fmt!(hgt.as_ref(), "{d}{}", usize, String) {
        Ok(x) => x,
        Err(_) => return false,
    };
    (unit == "cm" && val >= 150 && val <= 193) || (unit == "in" && val >= 59 && val <= 76)
}

fn validate_hcl<S: AsRef<str>>(hcl: S) -> bool {
    let hex = match scan_fmt!(hcl.as_ref(), "#{/[0-9a-f]+/}", String) {
        Ok(x) => x,
        Err(_) => return false,
    };
    hex.len() == 6
}

fn validate_ecl<S: AsRef<str>>(ecl: S) -> bool {
    let ecl = ecl.as_ref();
    ecl == "amb"
        || ecl == "blu"
        || ecl == "brn"
        || ecl == "gry"
        || ecl == "grn"
        || ecl == "hzl"
        || ecl == "oth"
}

fn validate_pid<S: AsRef<str>>(pid: S) -> bool {
    let digits = match scan_fmt!(pid.as_ref(), "{/[0-9]+/}", String) {
        Ok(x) => x,
        Err(_) => return false,
    };
    digits.len() == 9
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

fn part2(vec: &Vec<Passport>) -> usize {
    vec.iter().filter(|&p| p.valid2()).count()
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    println!("Number of valid passports: {}", part1(&vec));
    println!("Number of valid passports part 2: {}", part2(&vec));
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
        assert!(Passport::parse(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
        )
        .unwrap()
        .valid());
        assert!(!Passport::parse(
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929"
        )
        .unwrap()
        .valid());
        assert!(Passport::parse(
            "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm"
        )
        .unwrap()
        .valid());
        assert!(
            !Passport::parse("hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in")
                .unwrap()
                .valid()
        );
    }

    #[test]
    fn test_valid_year() {
        assert!(validate_year("2002", 1920, 2002));
        assert!(!validate_year("2003", 1920, 2002));
    }

    #[test]
    fn test_valid_hgt() {
        assert!(validate_height("60in"));
        assert!(validate_height("190cm"));
        assert!(!validate_height("190in"));
        assert!(!validate_height("190"));
    }

    #[test]
    fn test_valid_hcl() {
        assert!(validate_hcl("#123abc"));
        assert!(!validate_hcl("#123abz"));
        assert!(!validate_hcl("123abc"));
    }

    #[test]
    fn test_valid_ecl() {
        assert!(validate_ecl("brn"));
        assert!(!validate_ecl("wat"));
    }

    #[test]
    fn test_valid_pid() {
        assert!(validate_pid("000000001"));
        assert!(!validate_pid("0123456789"));
    }
}
