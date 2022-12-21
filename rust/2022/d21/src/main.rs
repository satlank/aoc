// https://adventofcode.com/2022/day/21

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

enum Op {
    Value(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn read<R: Read>(io: R) -> HashMap<String, Op> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .map(|line| {
            let (monkey, optext) = line.split_once(": ").unwrap();
            let op = if optext.chars().all(char::is_alphanumeric) {
                Op::Value(optext.parse::<i64>().unwrap())
            } else {
                let terms = optext.split(' ').collect::<Vec<_>>();
                match terms[1] {
                    "+" => Op::Add(terms[0].to_string(), terms[2].to_string()),
                    "-" => Op::Sub(terms[0].to_string(), terms[2].to_string()),
                    "*" => Op::Mul(terms[0].to_string(), terms[2].to_string()),
                    "/" => Op::Div(terms[0].to_string(), terms[2].to_string()),
                    _ => unreachable!(),
                }
            };
            (monkey.to_string(), op)
        })
        .collect()
}

fn part_1(input: &HashMap<String, Op>) -> i64 {
    let mut resolved: HashMap<String, i64> = HashMap::new();
    while !resolved.contains_key("root") {
        for (monkey, op) in input {
            if !resolved.contains_key(monkey) {
                match op {
                    Op::Value(i) => {
                        resolved.insert(monkey.to_string(), *i);
                    }
                    Op::Add(a, b) => {
                        match (resolved.get(a), resolved.get(b)) {
                            (Some(x), Some(y)) => resolved.insert(monkey.to_string(), x + y),
                            _ => None,
                        };
                    }
                    Op::Sub(a, b) => {
                        match (resolved.get(a), resolved.get(b)) {
                            (Some(x), Some(y)) => resolved.insert(monkey.to_string(), x - y),
                            _ => None,
                        };
                    }
                    Op::Mul(a, b) => {
                        match (resolved.get(a), resolved.get(b)) {
                            (Some(x), Some(y)) => resolved.insert(monkey.to_string(), x * y),
                            _ => None,
                        };
                    }
                    Op::Div(a, b) => {
                        match (resolved.get(a), resolved.get(b)) {
                            (Some(x), Some(y)) => resolved.insert(monkey.to_string(), x / y),
                            _ => None,
                        };
                    }
                }
            }
        }
    }
    resolved["root"]
}

fn part_2(input: &HashMap<String, Op>) -> i64 {
    let mut resolved: HashMap<String, i64> = HashMap::new();
    let not_resolved = input
        .keys()
        .filter_map(|k| {
            if k != "root" && k != "humn" {
                Some(k.to_string())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();
    let mut resolved_something = true;
    while resolved_something {
        resolved_something = false;
        for monkey in &not_resolved {
            let op = &input[monkey];
            if !resolved.contains_key(monkey) {
                match op {
                    Op::Value(i) => {
                        resolved_something = true;
                        resolved.insert(monkey.to_string(), *i);
                    }
                    Op::Add(a, b) => {
                        match (resolved.get(a), resolved.get(b)) {
                            (Some(x), Some(y)) => {
                                resolved_something = true;
                                resolved.insert(monkey.to_string(), x + y)
                            }
                            _ => None,
                        };
                    }
                    Op::Sub(a, b) => {
                        match (resolved.get(a), resolved.get(b)) {
                            (Some(x), Some(y)) => {
                                resolved_something = true;
                                resolved.insert(monkey.to_string(), x - y)
                            }
                            _ => None,
                        };
                    }
                    Op::Mul(a, b) => {
                        match (resolved.get(a), resolved.get(b)) {
                            (Some(x), Some(y)) => {
                                resolved_something = true;
                                resolved.insert(monkey.to_string(), x * y)
                            }
                            _ => None,
                        };
                    }
                    Op::Div(a, b) => {
                        match (resolved.get(a), resolved.get(b)) {
                            (Some(x), Some(y)) => {
                                resolved_something = true;
                                resolved.insert(monkey.to_string(), x / y)
                            }
                            _ => None,
                        };
                    }
                }
            }
        }
    }
    let mut parent = "root";
    let (mut left, mut right) = match &input[parent] {
        Op::Add(a, b) | Op::Sub(a, b) | Op::Mul(a, b) | Op::Div(a, b) => (a, b),
        _ => unreachable!(),
    };
    assert!(resolved.contains_key(left) || resolved.contains_key(right));
    let (unknown_monkey, known_monkey) = if resolved.contains_key(left) {
        (right, left)
    } else {
        (left, right)
    };
    resolved.insert(unknown_monkey.to_string(), resolved[known_monkey]);
    parent = unknown_monkey;
    while !resolved.contains_key("humn") {
        (left, right) = match &input[parent] {
            Op::Add(a, b) | Op::Sub(a, b) | Op::Mul(a, b) | Op::Div(a, b) => (a, b),
            _ => unreachable!(),
        };
        let (unknown_monkey, known_monkey) = if resolved.contains_key(left) {
            (right, left)
        } else {
            (left, right)
        };
        let op = &input[parent];
        let unknown_val = match op {
            Op::Add(_, _) => {
                let parent_val = resolved[parent];
                let known_val = resolved[known_monkey];
                parent_val - known_val
            }
            Op::Sub(a, b) => {
                let parent_val = resolved[parent];
                if a == known_monkey {
                    resolved[a] - parent_val
                } else {
                    parent_val + resolved[b]
                }
            }
            Op::Mul(_, _) => {
                let parent_val = resolved[parent];
                let known_val = resolved[known_monkey];
                parent_val / known_val
            }
            Op::Div(a, b) => {
                let parent_val = resolved[parent];
                if a == known_monkey {
                    resolved[a] / parent_val
                } else {
                    parent_val * resolved[b]
                }
            }
            _ => unreachable!(),
        };
        resolved.insert(unknown_monkey.to_string(), unknown_val);
        parent = unknown_monkey;
    }
    resolved["humn"]
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
        assert_eq!(part_1(&input), 152);
        assert_eq!(part_2(&input), 301);
    }
}
