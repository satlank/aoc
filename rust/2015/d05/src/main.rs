// https://adventofcode.com/2015/day/5

use regex::RegexSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn read<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines().map(|line| line.unwrap().to_string()).collect()
}

fn part_1(strings: &[String]) -> usize {
    let rules = RegexSet::new(&vec![
        r"^.*[aeiuo].*[aeiou].*[aeiou].*$",
        r"^.*(aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz).*$",
        r"^.*(ab|cd|pq|xy).*$",
    ])
    .unwrap();
    strings
        .iter()
        .filter(|&s| {
            let m = rules.matches(s);
            m.matched(0) && m.matched(1) && !m.matched(2)
        })
        .count()
}

fn part_2(strings: &[String]) -> usize {
    let rules1 = RegexSet::new(
        "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .map(|x| {
                "abcdefghijklmnopqrstuvwxyz"
                    .chars()
                    .map(|y| format!("{}{}.*{}{}", x, y, x, y))
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect::<Vec<String>>(),
    )
    .unwrap();
    let rules2 = RegexSet::new(
        "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .map(|x| format!("{}.{}", x, x))
            .collect::<Vec<String>>(),
    )
    .unwrap();
    strings
        .iter()
        .filter(|&s| {
            let m1 = rules1.matches(s);
            let m2 = rules2.matches(s);
            m1.iter().count() > 0 && m2.iter().count() > 0
        })
        .count()
}

fn main() {
    let strings = read(File::open("input.txt").unwrap());
    println!("There are a total of {} nice strings", part_1(&strings));
    println!("..actually, there are {} nice strings", part_2(&strings));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        assert_eq!(read(File::open("input.txt").unwrap()).len(), 1000);
        let strings = read(File::open("test.txt").unwrap());
        assert_eq!(
            strings,
            vec![
                "ugknbfddgicrmopn".to_string(),
                "aaa".to_string(),
                "jchzalrnumimnmhp".to_string(),
                "haegwjzuvuyypxyu".to_string(),
                "dvszwmarrgswjxmb".to_string(),
            ]
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&read(File::open("test.txt").unwrap())), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&read(File::open("test2.txt").unwrap())), 2);
    }
}
