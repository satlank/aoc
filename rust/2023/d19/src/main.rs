// https://adventofcode.com/2023/day/19

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Class {
    Cool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl From<&str> for Class {
    fn from(s: &str) -> Self {
        match s {
            "x" => Self::Cool,
            "m" => Self::Musical,
            "a" => Self::Aerodynamic,
            "s" => Self::Shiny,
            _ => panic!("Invalid class: {}", s),
        }
    }
}

#[derive(Debug)]
enum Condition {
    LessThan(Class, usize, String),
    GreaterThan(Class, usize, String),
    Final(String),
}

impl From<&str> for Condition {
    fn from(s: &str) -> Self {
        let parts = s.split(':').collect::<Vec<_>>();
        if parts.len() == 1 {
            Self::Final(parts[0].to_string())
        } else {
            assert_eq!(parts.len(), 2);
            let class = Class::from(&parts[0][..1]);
            let value = parts[0][2..].parse().unwrap();
            match &parts[0][1..2] {
                "<" => Self::LessThan(class, value, parts[1].to_string()),
                ">" => Self::GreaterThan(class, value, parts[1].to_string()),
                _ => panic!("Invalid condition: {}", s),
            }
        }
    }
}

#[derive(Debug)]
struct WorkFlow {
    conditions: Vec<Condition>,
}

impl WorkFlow {
    fn process(&self, part: &Part) -> &str {
        for con in &self.conditions {
            match con {
                Condition::LessThan(c, val, target) => {
                    if part.values[c] < *val {
                        return target;
                    } else {
                        continue;
                    }
                }
                Condition::GreaterThan(c, val, target) => {
                    if part.values[c] > *val {
                        return target;
                    } else {
                        continue;
                    }
                }
                Condition::Final(target) => return target,
            }
        }
        unreachable!()
    }
}

#[derive(Debug)]
struct Part {
    values: HashMap<Class, usize>,
}

impl Part {
    fn value(&self) -> usize {
        self.values.values().sum()
    }
}

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        let s = &s[1..s.len() - 1];
        let values = s
            .split(',')
            .map(|s| {
                let mut tmp = s.split('=');
                let label = tmp.next().unwrap();
                let value = tmp.next().unwrap();
                (Class::from(label), value.parse::<usize>().unwrap())
            })
            .collect();

        Self { values }
    }
}

type Rules = HashMap<String, WorkFlow>;

fn accepts(rules: &Rules, part: &Part) -> bool {
    let mut wf = rules.get("in").unwrap();
    loop {
        let r = wf.process(part);
        if r == "A" {
            return true;
        } else if r == "R" {
            return false;
        }
        wf = rules.get(r).unwrap();
    }
}

fn read<R: Read>(io: R) -> (Rules, Vec<Part>) {
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines().map_while(Result::ok).collect();
    let mut i = 0;
    let mut rules = HashMap::new();
    while !lines[i].is_empty() {
        let mut it = lines[i].split('{');
        let name = it.next().unwrap().trim().to_string();
        let it = it.next().unwrap().split(',');
        let mut conditions = Vec::new();
        for condition in it {
            let condition = if let Some(condition) = condition.strip_suffix('}') {
                condition
            } else {
                condition
            };
            let condition = Condition::from(condition);
            conditions.push(condition);
        }
        rules.insert(name, WorkFlow { conditions });
        i += 1;
    }
    i += 1;
    let parts = lines[i..]
        .iter()
        .map(|line| Part::from(line.as_str()))
        .collect();
    (rules, parts)
}

fn part_1(rules: &Rules, parts: &[Part]) -> usize {
    parts
        .iter()
        .filter(|&p| accepts(rules, p))
        .map(|p| p.value())
        .sum()
}

fn part_2(_rules: &Rules) -> usize {
    todo!()
}

fn main() {
    let (rules, parts) = read(File::open("input.txt").unwrap());
    let p1 = part_1(&rules, &parts);
    println!("Part 1: {}", p1);
    //let p2 = part_2(&rules);
    //println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let (rules, parts) = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&rules, &parts), 19114);
        //assert_eq!(part_2(&rules), 167409079868000);
    }
}
