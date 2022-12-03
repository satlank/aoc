// https://adventofcode.com/2022/day/2

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines().filter_map(Result::ok).collect()
}

fn find_duplicate(input: &str) -> Vec<char> {
    assert!(input.len() >= 2);
    assert!(input.len() % 2 == 0); // Must be even
    let c1 = input[..input.len() / 2].chars().collect::<HashSet<char>>();
    let c2 = input[input.len() / 2..].chars().collect::<HashSet<char>>();
    c1.intersection(&c2).copied().collect()
}

fn find_badge(group: [&str; 3]) -> char {
    let g1: HashSet<char> = group[0].chars().collect();
    let g2: HashSet<char> = group[1].chars().collect();
    let g3: HashSet<char> = group[2].chars().collect();

    g1.intersection(&g2)
        .copied()
        .collect::<HashSet<char>>()
        .intersection(&g3)
        .copied()
        .nth(0)
        .unwrap()
}

fn into_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        assert!(c.is_uppercase());
        c as u32 - 'A' as u32 + 27
    }
}

fn part_1(items: &[String]) -> u32 {
    items
        .iter()
        .map(|sack| {
            find_duplicate(sack)
                .iter()
                .map(|c| into_priority(*c))
                .sum::<u32>()
        })
        .sum()
}

fn part_2(items: &[String]) -> u32 {
    assert!(items.len() % 3 == 0); // Must be multiple of 3
    (0..items.len() / 3)
        .map(|idx| {
            into_priority(find_badge([
                &items[idx * 3],
                &items[idx * 3 + 1],
                &items[idx * 3 + 2],
            ]))
        })
        .sum()
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input);
    println!("Sum of the priorities of the misplaced items: {}", p1);
    let p2 = part_2(&input);
    println!("Sum of priorities of badges: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicates() {
        assert_eq!(find_duplicate("vJrwpWtwJgWrhcsFMMfFFhFp"), vec!['p']);
        assert_eq!(
            find_duplicate("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            vec!['L']
        );
        assert_eq!(find_duplicate("PmmdzqPrVvPwwTWBwg"), vec!['P']);
        assert_eq!(find_duplicate("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), vec!['v']);
        assert_eq!(find_duplicate("ttgJtRGJQctTZtZT"), vec!['t']);
        assert_eq!(find_duplicate("CrZsJsPPZsGzwwsLwLmpwMDw"), vec!['s']);
    }

    #[test]
    fn example1() {
        let data = read(File::open("test1.txt").unwrap());
        assert_eq!(part_1(&data), 157);
        assert_eq!(part_2(&data), 70);
    }

    #[test]
    fn badge() {
        assert_eq!(
            find_badge([
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
            ]),
            'r'
        );

        assert_eq!(
            find_badge([
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ]),
            'Z'
        );
    }
}
