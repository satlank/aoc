// https://adventofcode.com/2023/day/15

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines()
        .map_while(Result::ok)
        .take(1)
        .next()
        .unwrap()
        .split(',')
        .map(String::from)
        .collect()
}
fn holiday_ascii_string_helper<S: AsRef<str>>(s: S) -> usize {
    let mut state = 0;
    for c in s.as_ref().chars() {
        state += c as usize;
        state *= 17;
        state %= 256;
    }
    state
}

struct Boxes {
    boxes: Vec<Vec<(String, usize)>>,
}

impl Boxes {
    fn new() -> Self {
        Self {
            boxes: vec![Vec::new(); 256],
        }
    }

    fn apply(&mut self, s: &str) {
        if s.contains('=') {
            let mut split = s.split('=');
            let label = split.next().unwrap();
            let value = split.next().unwrap().parse().unwrap();
            self.add(label, value);
        } else {
            assert!(s.contains('-'));
            let mut split = s.split('-');
            let label = split.next().unwrap();
            self.remove(label);
        }
    }

    fn add(&mut self, label: &str, value: usize) {
        let idx = holiday_ascii_string_helper(label);
        let target = self.boxes.get_mut(idx).unwrap();
        if let Some((l, v)) = target.iter_mut().find(|(l, _)| l == label) {
            *v = value;
            *l = label.to_string();
        } else {
            target.push((label.to_string(), value));
        }
    }

    fn remove(&mut self, label: &str) {
        let idx = holiday_ascii_string_helper(label);
        let target = self.boxes.get_mut(idx).unwrap();
        if let Some(maybe_idx) = target.iter().position(|(l, _)| l == label) {
            target.remove(maybe_idx);
        }
    }

    fn focussing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                item.iter()
                    .enumerate()
                    .map(|(idx2, (_, value))| value * (idx2 + 1) * (idx + 1))
                    .sum::<usize>()
            })
            .sum()
    }
}

fn part_1(input: &[String]) -> usize {
    input.iter().map(holiday_ascii_string_helper).sum()
}

fn part_2(input: &[String]) -> usize {
    let mut boxes = Boxes::new();
    input.iter().for_each(|s| boxes.apply(s));
    boxes.focussing_power()
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input);
    println!("Part 1: {}", p1);
    let p2 = part_2(&input);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_works() {
        assert_eq!(holiday_ascii_string_helper("HASH"), 52);
        assert_eq!(holiday_ascii_string_helper("rn"), 0);
        assert_eq!(holiday_ascii_string_helper("qp"), 1);
    }

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input), 1320);
        assert_eq!(part_2(&input), 145);
    }
}
