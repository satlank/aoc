// https://adventofcode.com/2022/day/11

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, Read},
    sync::Arc,
};

fn read<R: Read>(io: R) -> Vec<Monkey> {
    let br = BufReader::new(io);
    let mut lines: Vec<String> = br.lines().filter_map(Result::ok).collect();
    lines.push("".to_string());
    let n_monkeys = lines.len() / 7;
    (0..n_monkeys)
        .map(|i| Monkey::from(&lines[i * 7..(i + 1) * 7]))
        .collect()
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<usize>,
    op: Arc<dyn Fn(usize) -> usize>,
    div: usize,
    if_true: usize,
    if_false: usize,
}

impl From<&[String]> for Monkey {
    fn from(lines: &[String]) -> Monkey {
        let items: VecDeque<usize> = lines[1]
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(',')
            .map(|n| n.trim())
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        let op = lines[2].split_once("= ").unwrap().1;
        let op = match op.chars().nth(4).unwrap() {
            '+' => {
                let other: usize = op.chars().skip(6).collect::<String>().parse().unwrap();
                Arc::new(move |old: usize| old + other) as Arc<dyn Fn(usize) -> usize>
            }
            '*' => match op.chars().nth(6).unwrap() {
                'o' => Arc::new(|old: usize| old * old) as Arc<dyn Fn(usize) -> usize>,
                _ => {
                    let other: usize = op.chars().skip(6).collect::<String>().parse().unwrap();
                    Arc::new(move |old: usize| old * other) as Arc<dyn Fn(usize) -> usize>
                }
            },
            _ => unreachable!(),
        };
        let div = lines[3]
            .split_once("by ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let if_true = lines[4]
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let if_false = lines[5]
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();

        Monkey {
            items,
            op,
            div,
            if_true,
            if_false,
        }
    }
}

fn part_1(mut monkeys: Vec<Monkey>) -> usize {
    let mut count = vec![0; monkeys.len()];
    for _ in 1..21 {
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys[m].items.pop_front() {
                count[m] += 1;
                let new_worry = (monkeys[m].op)(item) / 3;
                let throw_to = if new_worry % monkeys[m].div == 0 {
                    monkeys[m].if_true
                } else {
                    monkeys[m].if_false
                };
                monkeys[throw_to].items.push_back(new_worry);
            }
        }
    }

    count.sort();
    count[count.len() - 1] * count[count.len() - 2]
}

fn part_2(mut monkeys: Vec<Monkey>) -> usize {
    let mut count = vec![0; monkeys.len()];
    let mut cycle = 1;
    for m in &monkeys {
        cycle *= m.div;
    }
    for _ in 1..10001 {
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys[m].items.pop_front() {
                count[m] += 1;
                let new_worry = (monkeys[m].op)(item);
                let throw_to = if new_worry % monkeys[m].div == 0 {
                    monkeys[m].if_true
                } else {
                    monkeys[m].if_false
                };
                monkeys[throw_to].items.push_back(new_worry % cycle);
            }
        }
    }

    count.sort();
    count[count.len() - 1] * count[count.len() - 2]
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(input.clone());
    println!("Part 1: {}", p1);
    let p2 = part_2(input);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monkey_parse() {
        let input = vec![
            "Monkey 0:".to_string(),
            "  Starting items: 79, 98".to_string(),
            "  Operation: new = old * 19".to_string(),
            "  Test: divisible by 23".to_string(),
            "    If true: throw to monkey 2".to_string(),
            "    If false: throw to monkey 3".to_string(),
            "".to_string(),
        ];
        let monkey = Monkey::from(&input[0..6]);
        println!("{:?}", monkey.items);
        assert_eq!(
            monkey.items,
            [79, 98].into_iter().collect::<VecDeque<usize>>()
        );
        assert_eq!((monkey.op)(4), 4 * 19);
        assert_eq!(monkey.div, 23);
        assert_eq!(monkey.if_true, 2);
        assert_eq!(monkey.if_false, 3);
    }

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(input.clone()), 10605);
        assert_eq!(part_2(input), 2713310158);
    }
}
