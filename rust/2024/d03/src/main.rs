// https://adventofcode.com/2024/day/3

use std::fs::read_to_string;

use regex::Regex;

fn read(p: &str) -> String {
    read_to_string(p).unwrap()
}

fn get_regex() -> Regex {
    Regex::new(r"mul\(([\d]{1,3}),([\d]{1,3})\)").unwrap()
}

fn part_1(input: &str) -> usize {
    let regex = get_regex();
    regex
        .captures_iter(input)
        .map(|c| {
            let (_, [x, y]) = c.extract();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            x * y
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let regex = get_regex();
    let mut pos = 0;
    let mut res = 0;
    loop {
        let end = input[pos..].find("don't()").unwrap_or(input[pos..].len());
        let sub = &input[pos..pos + end];
        res += regex
            .captures_iter(sub)
            .map(|c| {
                let (_, [x, y]) = c.extract();
                let x: usize = x.parse().unwrap();
                let y: usize = y.parse().unwrap();
                x * y
            })
            .sum::<usize>();
        pos += end + 7;
        if pos >= input.len() {
            break;
        }

        // Find next do() block
        let end = input[pos..].find("do()").unwrap_or(input[pos..].len());
        if end == input.len() {
            break;
        }
        pos += end + 4;
    }
    res
}

fn main() {
    let input = read("input.txt");
    let p1 = part_1(&input);
    println!("Part 1: {}", p1);
    let p2 = part_2(&input);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_mul_works() {
        let r = get_regex();
        let c = r.captures("asdsadmul(1,45)mul(12,mul(non").unwrap();
        let (_, [x, y]) = c.extract();
        assert_eq!(x, "1");
        assert_eq!(y, "45");
    }

    #[test]
    fn example_1() {
        let input = read("example1.txt");
        assert_eq!(part_1(&input), 161);
    }

    #[test]
    fn example_2() {
        let input = read("example2.txt");
        assert_eq!(part_2(&input), 48);
    }
}
