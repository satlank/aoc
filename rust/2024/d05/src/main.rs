// https://adventofcode.com/2024/day/5

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let br = BufReader::new(io);
    let mut rules = vec![];
    let mut lines = br.lines();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        let parts = line.split("|").collect::<Vec<_>>();
        assert_eq!(parts.len(), 2);
        rules.push((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
    }
    let mut inputs = vec![];
    while let Some(Ok(line)) = lines.next() {
        let parts = line.split(',').map(|x| x.parse().unwrap()).collect();
        inputs.push(parts);
    }

    (rules, inputs)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct PageNumber(usize);

struct PageOrderingRules {
    rules: HashMap<PageNumber, HashSet<PageNumber>>,
}

impl From<&[(usize, usize)]> for PageOrderingRules {
    fn from(rules: &[(usize, usize)]) -> Self {
        let mut map = HashMap::new();
        for (k, v) in rules {
            map.entry(PageNumber(*k))
                .or_insert(HashSet::new())
                .insert(PageNumber(*v));
        }
        Self { rules: map }
    }
}

fn cmp_page_numbers(a: PageNumber, b: PageNumber, rules: &PageOrderingRules) -> std::cmp::Ordering {
    if a == b {
        return std::cmp::Ordering::Equal;
    }
    if let Some(set) = rules.rules.get(&a) {
        // We check if a is before b, if so, we return less
        if set.contains(&b) {
            return std::cmp::Ordering::Less;
        }
    }
    if let Some(set) = rules.rules.get(&b) {
        // We check if b is before a, if so, we return greater
        if set.contains(&a) {
            return std::cmp::Ordering::Greater;
        }
    }
    panic!("No ordering rules for {:?} and {:?}", a, b);
}

#[derive(Debug)]
struct PageOrder(Vec<PageNumber>);

impl PageOrder {
    fn is_valid(&self, rules: &PageOrderingRules) -> bool {
        self.0.windows(2).all(|x| {
            let a = x[0];
            let b = x[1];
            cmp_page_numbers(a, b, rules) == std::cmp::Ordering::Less
        })
    }

    fn sort(&mut self, rules: &PageOrderingRules) {
        self.0.sort_by(|a, b| cmp_page_numbers(*a, *b, rules));
    }

    fn middle(&self) -> PageNumber {
        assert!(self.0.len() % 2 == 1);
        let middle = self.0.len() / 2;
        self.0[middle]
    }
}

impl From<&Vec<usize>> for PageOrder {
    fn from(order: &Vec<usize>) -> Self {
        Self(order.iter().copied().map(PageNumber).collect())
    }
}

fn part_1(rules: &[(usize, usize)], input: &[Vec<usize>]) -> usize {
    let ordering_rules = PageOrderingRules::from(rules);
    input
        .iter()
        .map(PageOrder::from)
        .filter(|order| order.is_valid(&ordering_rules))
        .map(|order| order.middle().0)
        .sum()
}

fn part_2(rules: &[(usize, usize)], input: &[Vec<usize>]) -> usize {
    let ordering_rules = PageOrderingRules::from(rules);
    input
        .iter()
        .map(PageOrder::from)
        .filter(|order| !order.is_valid(&ordering_rules))
        .map(|mut order| {
            order.sort(&ordering_rules);
            order
        })
        .map(|order| order.middle().0)
        .sum()
}

fn main() {
    let (rules, input) = read(File::open("input.txt").unwrap());
    let p1 = part_1(&rules, &input);
    println!("Part 1: {}", p1);
    let p2 = part_2(&rules, &input);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let (rules, input) = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&rules, &input), 143);
        assert_eq!(part_2(&rules, &input), 123);
    }
}
