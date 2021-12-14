// https://adventofcode.com/2021/day/13

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn read<R: Read>(io: R) -> (String, HashMap<(char, char), char>) {
    let lines = BufReader::new(io).lines();
    let mut template: String = "".to_string();
    let mut part2 = false;
    let mut rules = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        if !part2 {
            if line.is_empty() {
                part2 = true;
            } else {
                template = line.clone();
            }
        } else {
            let sp = line.split(" -> ").take(2).collect::<Vec<&str>>();
            let sp0 = sp[0].chars().collect::<Vec<char>>();
            rules.insert(
                (sp0[0], sp0[1]),
                sp[1].chars().take(1).collect::<Vec<char>>()[0],
            );
        }
    }
    (template, rules)
}

fn calc_histo<'cache>(
    level: usize,
    left: char,
    right: char,
    rules: &HashMap<(char, char), char>,
    cache: &'cache mut HashMap<(usize, char, char), HashMap<char, usize>>,
) -> &'cache HashMap<char, usize> {
    let cache_key = (level, left, right);
    if !cache.contains_key(&cache_key) {
        let mut res = HashMap::new();
        if rules.contains_key(&(left, right)) {
            let new_char = *rules.get(&(left, right)).unwrap();
            *res.entry(new_char).or_insert(0) += 1;
            if level > 0 {
                let left_res = calc_histo(level - 1, left, new_char, rules, cache);
                for (k, v) in left_res {
                    *res.entry(*k).or_insert(0) += *v;
                }
                let right_res = calc_histo(level - 1, new_char, right, rules, cache);
                for (k, v) in right_res {
                    *res.entry(*k).or_insert(0) += *v;
                }
            }
        }
        cache.insert(cache_key, res);
    }
    return cache.get(&cache_key).unwrap();
}

fn part_1(template: &str, rules: &HashMap<(char, char), char>) -> usize {
    let mut histo = HashMap::new();
    template.chars().for_each(|c| {
        *histo.entry(c).or_insert(0) += 1;
    });

    let mut cache = HashMap::new();
    for (left, right) in template.chars().tuple_windows() {
        let res = calc_histo(9, left, right, rules, &mut cache);
        for (k, v) in res {
            *histo.entry(*k).or_insert(0) += *v;
        }
    }

    if let MinMax(min, max) = histo.values().minmax() {
        max - min
    } else {
        unreachable!()
    }
}

fn part_2(template: &str, rules: &HashMap<(char, char), char>) -> usize {
    let mut histo = HashMap::new();
    template.chars().for_each(|c| {
        *histo.entry(c).or_insert(0) += 1;
    });

    let mut cache = HashMap::new();
    for (left, right) in template.chars().tuple_windows() {
        let res = calc_histo(39, left, right, rules, &mut cache);
        for (k, v) in res {
            *histo.entry(*k).or_insert(0) += *v;
        }
    }

    if let MinMax(min, max) = histo.values().minmax() {
        max - min
    } else {
        unreachable!()
    }
}

fn main() {
    let (template, rules) = read(File::open("input.txt").unwrap());
    let res = part_1(&template, &rules);
    println!("Result for part 1: {}", res);
    let res = part_2(&template, &rules);
    println!("Result for part 2: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let (template, rules) = read(File::open("test1.txt").unwrap());
        assert_eq!(template, "NNCB");
        assert_eq!(rules.len(), 16);
        assert!(rules.contains_key(&('C', 'N')));
        assert!(rules.contains_key(&('B', 'B')));
        assert!(rules.contains_key(&('C', 'H')));
    }

    #[test]
    fn test_part_1() {
        let (template, rules) = read(File::open("test1.txt").unwrap());
        let res = part_1(&template, &rules);
        assert_eq!(res, 1588);
    }

    #[test]
    fn test_part_2() {
        let (template, rules) = read(File::open("test1.txt").unwrap());
        let res = part_2(&template, &rules);
        assert_eq!(res, 2188189693529);
    }
}
