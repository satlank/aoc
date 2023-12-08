// https://adventofcode.com/2023/day/8

use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

use num::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Direction {
    L,
    R,
}

impl From<char> for Direction {
    fn from(s: char) -> Self {
        match s {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    label: [char; 3],
}

impl Node {
    fn new(label: [char; 3]) -> Self {
        Self { label }
    }

    fn is_start_2(&self) -> bool {
        self.label[2] == 'A'
    }

    fn is_end(&self) -> bool {
        self.label == ['Z', 'Z', 'Z']
    }
    fn is_end_2(&self) -> bool {
        self.label[2] == 'Z'
    }
}

type Network = HashMap<Node, (Node, Node)>;

fn read<R: Read>(io: R) -> (Vec<Direction>, Network) {
    let br = BufReader::new(io);
    let mut it = br.lines().map_while(Result::ok);

    let directions = it.next().unwrap().chars().map(Direction::from).collect();
    it.next().unwrap();
    let network = it
        .map(|line| {
            let node = Node {
                label: [
                    line.chars().next().unwrap(),
                    line.chars().nth(1).unwrap(),
                    line.chars().nth(2).unwrap(),
                ],
            };
            let left = Node {
                label: [
                    line.chars().nth(7).unwrap(),
                    line.chars().nth(8).unwrap(),
                    line.chars().nth(9).unwrap(),
                ],
            };
            let right = Node {
                label: [
                    line.chars().nth(12).unwrap(),
                    line.chars().nth(13).unwrap(),
                    line.chars().nth(14).unwrap(),
                ],
            };
            (node, (left, right))
        })
        .collect();

    (directions, network)
}

fn part_1(directions: &[Direction], network: &Network) -> usize {
    let mut steps = 0;
    let cycle = directions.len();
    let mut current = Node::new(['A', 'A', 'A']);
    loop {
        let instruction = directions[steps % cycle];
        steps += 1;

        current = match instruction {
            Direction::L => network[&current].0,
            Direction::R => network[&current].1,
        };

        if current.is_end() {
            return steps;
        }
    }
}

#[derive(Debug)]
struct PathCycle {
    before_cycle_start: Vec<usize>,
    after_cycle_start: Vec<usize>,
    cycle_start: usize,
    length: usize,
}

impl PathCycle {
    pub fn new(end_nodes_at: Vec<usize>, cycle_start: usize, length: usize) -> Self {
        let end_nodes_before_cycle_start = end_nodes_at
            .iter()
            .filter_map(|n| if *n < cycle_start { Some(*n) } else { None })
            .collect::<Vec<_>>();
        let end_nodes_after_cycle_start = end_nodes_at
            .iter()
            .filter_map(|n| if *n >= cycle_start { Some(*n) } else { None })
            .collect::<Vec<_>>();
        Self {
            before_cycle_start: end_nodes_before_cycle_start,
            after_cycle_start: end_nodes_after_cycle_start,
            cycle_start,
            length,
        }
    }

    pub fn iter(&self) -> PathCycleIterator {
        let next = if self.before_cycle_start.is_empty() {
            self.after_cycle_start[0]
        } else {
            self.before_cycle_start[0]
        };
        PathCycleIterator {
            cycle: self,
            next,
            end_nodes_idx: 0,
        }
    }
}

#[derive(Debug)]
struct PathCycleIterator<'a> {
    cycle: &'a PathCycle,
    next: usize,
    end_nodes_idx: usize,
}

impl Iterator for PathCycleIterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.next;

        if res < self.cycle.cycle_start {
            if self.end_nodes_idx + 1 < self.cycle.before_cycle_start.len() {
                self.end_nodes_idx += 1;
                self.next = self.cycle.before_cycle_start[self.end_nodes_idx];
            } else {
                self.next = self.cycle.after_cycle_start[0];
                self.end_nodes_idx = 0;
            }
        } else {
            let pos = self.cycle.after_cycle_start[self.end_nodes_idx];
            self.end_nodes_idx = (self.end_nodes_idx + 1) % self.cycle.after_cycle_start.len();
            let next_pos = self.cycle.after_cycle_start[self.end_nodes_idx];
            let incr = match next_pos.cmp(&pos) {
                Ordering::Less => ((next_pos + self.cycle.length) - pos) % self.cycle.length,
                Ordering::Equal => self.cycle.length,
                Ordering::Greater => next_pos - pos,
            };
            self.next += incr;
        }

        Some(res)
    }
}

#[test]
fn iterator() {
    let cycle = PathCycle::new(vec![1, 2, 4, 5], 3, 4);
    let mut it = cycle.iter();
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(4));
    assert_eq!(it.next(), Some(5));
    assert_eq!(it.next(), Some(8));
    assert_eq!(it.next(), Some(9));
    assert_eq!(it.next(), Some(12));
    assert_eq!(it.next(), Some(13));
    assert_eq!(it.next(), Some(16));
    assert_eq!(it.next(), Some(17));
}

fn part_2(directions: &[Direction], network: &Network) -> usize {
    let cycle = directions.len();
    let current = network
        .keys()
        .filter_map(|n| if n.is_start_2() { Some(*n) } else { None })
        .collect::<Vec<_>>();

    let walk_cycles = current
        .into_iter()
        .map(|mut current| {
            let mut steps = 0;
            let mut cycles = Vec::new();
            let mut visited = HashMap::new();
            let steps_to_cycle;
            let cycle_length;
            loop {
                let instruction = directions[steps % cycle];
                if current.is_end_2() {
                    cycles.push(steps);
                }
                let state = (current, steps % cycle);
                if let Some(prev_step) = visited.get(&state) {
                    cycle_length = steps - prev_step;
                    steps_to_cycle = *prev_step;
                    break;
                } else {
                    visited.insert(state, steps);
                }

                steps += 1;

                current = match instruction {
                    Direction::L => network[&current].0,
                    Direction::R => network[&current].1,
                };
            }
            PathCycle::new(cycles, steps_to_cycle, cycle_length)
        })
        .collect::<Vec<PathCycle>>();

    // The general solution is far too slow for the real input, luckily it is not needed when the
    // input is exactly one cycle per path with the cycle length being the same as the number of
    // steps to the end node.
    let run_fast = walk_cycles.iter().all(|pc| {
        pc.after_cycle_start.len() == 1
            && pc.before_cycle_start.is_empty()
            && pc.length == pc.after_cycle_start[0]
    });

    if !run_fast {
        // NB This is hit in the test case, but the test case would also work with the fast path
        // because the cycle length is the same as the number of steps to an end node and the
        // combined steps does not hit the intermediate end node.
        let mut its = walk_cycles.iter().map(|pc| pc.iter()).collect::<Vec<_>>();
        let mut res = its
            .iter_mut()
            .map(|it| it.next().unwrap())
            .collect::<Vec<_>>();
        loop {
            if res.iter().all(|r| *r == res[0]) {
                return res[0];
            }
            let min = res.iter().min().unwrap();
            let min_idx = res.iter().position(|r| *r == *min).unwrap();
            res[min_idx] = its[min_idx].next().unwrap();
        }
    } else {
        walk_cycles
            .iter()
            .map(|pc| pc.length)
            .fold(1, |tmp, cl| tmp.lcm(&cl))
    }
}

fn main() {
    let (directions, network) = read(File::open("input.txt").unwrap());
    let p1 = part_1(&directions, &network);
    println!("Part 1: {}", p1);
    let p2 = part_2(&directions, &network);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let (directions, network) = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&directions, &network), 2);
    }

    #[test]
    fn example_2() {
        let (directions, network) = read(File::open("example2.txt").unwrap());
        assert_eq!(part_1(&directions, &network), 6);
    }

    #[test]
    fn example_3() {
        let (directions, network) = read(File::open("example3.txt").unwrap());
        assert_eq!(part_2(&directions, &network), 6);
    }
}
