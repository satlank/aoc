// https://adventofcode.com/2024/day/19

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> (Vec<String>, Vec<String>) {
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines().map_while(Result::ok).collect();
    let towels = lines[0].split(", ").map(String::from).collect();
    (towels, lines[2..].to_vec())
}

fn part_1(towels: &[String], stripes: &[String]) -> usize {
    let tmp = towels.join("|");
    let patterns = format!("^({})+$", tmp);
    let re = regex::Regex::new(&patterns).unwrap();
    stripes.iter().filter(|s| re.is_match(s)).count()
}

fn find_all(token: &[String], haystack: &str, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(val) = cache.get(haystack) {
        return *val;
    }
    let res = token
        .iter()
        .filter(|t| haystack.starts_with(t.as_str()))
        .map(|t| {
            if t == haystack {
                1
            } else {
                find_all(token, &haystack[t.len()..], cache)
            }
        })
        .sum::<usize>();
    cache.insert(haystack.to_string(), res);
    res
}

fn part_2(towels: &[String], stripes: &[String]) -> usize {
    let tmp = towels.join("|");
    let patterns = format!("^({})+$", tmp);
    let re = regex::Regex::new(&patterns).unwrap();
    let mut cache = HashMap::new();
    stripes
        .iter()
        .filter(|s| re.is_match(s))
        .map(|s| find_all(towels, s, &mut cache))
        .sum()
}

fn main() {
    let (towels, stripes) = read(File::open("input.txt").unwrap());
    let p1 = part_1(&towels, &stripes);
    println!("Part 1: {}", p1);
    let p2 = part_2(&towels, &stripes);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let (towels, stripes) = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&towels, &stripes), 6);
        assert_eq!(part_2(&towels, &stripes), 16);
    }
}
