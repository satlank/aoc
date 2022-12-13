// https://adventofcode.com/2022/day/13

use serde_json::Value;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<(ValueOrList, ValueOrList)> {
    let br = BufReader::new(io);
    let mut lines: Vec<String> = br.lines().filter_map(Result::ok).collect();
    lines.push("".into());
    (0..(lines.len() / 3))
        .map(|i| {
            let first = lines[i * 3].as_str();
            let second = lines[i * 3 + 1].as_str();

            (ValueOrList::from(first), ValueOrList::from(second))
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ValueOrList {
    Value(usize),
    List(Vec<ValueOrList>),
}

impl Ord for ValueOrList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for ValueOrList {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            ValueOrList::Value(lhs) => match other {
                ValueOrList::Value(rhs) => Some(lhs.cmp(rhs)),
                ValueOrList::List(_) => {
                    let lhs_as_list = ValueOrList::List(vec![ValueOrList::Value(*lhs)]);
                    lhs_as_list.partial_cmp(other)
                }
            },
            ValueOrList::List(lhs) => match other {
                ValueOrList::Value(rhs) => {
                    let rhs_as_list = ValueOrList::List(vec![ValueOrList::Value(*rhs)]);
                    self.partial_cmp(&rhs_as_list)
                }
                ValueOrList::List(rhs) => {
                    for i in 0..lhs.len() {
                        if i >= rhs.len() {
                            return Some(std::cmp::Ordering::Greater);
                        }
                        if lhs[i] < rhs[i] {
                            return Some(std::cmp::Ordering::Less);
                        }
                        if lhs[i] > rhs[i] {
                            return Some(std::cmp::Ordering::Greater);
                        }
                    }
                    if lhs.len() < rhs.len() {
                        return Some(std::cmp::Ordering::Less);
                    }
                    None
                }
            },
        }
    }
}

impl From<&str> for ValueOrList {
    fn from(txt: &str) -> Self {
        let thing: Value = serde_json::from_str(txt).unwrap();
        (&thing).into()
    }
}

impl From<&Value> for ValueOrList {
    fn from(thing: &Value) -> Self {
        match thing {
            Value::Array(arr) => {
                ValueOrList::List(arr.iter().map(|x| x.into()).collect::<Vec<ValueOrList>>())
            }
            Value::Number(num) => ValueOrList::Value(num.as_u64().unwrap() as usize),
            _ => unreachable!(),
        }
    }
}

fn part_1(input: &[(ValueOrList, ValueOrList)]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, (left, right))| {
            if left < right {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(input: &[(ValueOrList, ValueOrList)]) -> usize {
    let div1 = ValueOrList::List(vec!(ValueOrList::List(vec![ValueOrList::Value(2)])));
    let div2 = ValueOrList::List(vec!(ValueOrList::List(vec![ValueOrList::Value(6)])));
    let mut real = Vec::new();
    for (left, right) in input {
        real.push(left.clone());
        real.push(right.clone());
    }
    real.push(div1.clone());
    real.push(div2.clone());

    real.sort();
    let idx1 = real.iter().position(|x| *x == div1).unwrap() + 1;
    let idx2 = real.iter().position(|x| *x == div2).unwrap() + 1;

    idx1 * idx2
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
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input), 13);
        assert_eq!(part_2(&input), 140);
    }
}
