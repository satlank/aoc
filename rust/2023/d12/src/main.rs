// https://adventofcode.com/2023/day/12

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

use itertools::repeat_n;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
enum SpringHealth {
    Working,
    Broken,
    Unknown,
}

impl From<char> for SpringHealth {
    fn from(c: char) -> Self {
        match c {
            '.' => SpringHealth::Working,
            '#' => SpringHealth::Broken,
            '?' => SpringHealth::Unknown,
            _ => panic!("Invalid char: {}", c),
        }
    }
}

#[derive(Debug, Clone)]
struct Line {
    obs: Vec<SpringHealth>,
    groups: Vec<usize>,
}

impl From<String> for Line {
    fn from(s: String) -> Self {
        let mut parts = s.split_whitespace();
        let obs = parts
            .next()
            .unwrap()
            .chars()
            .map(SpringHealth::from)
            .collect();
        let groups = parts
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        Line { obs, groups }
    }
}

impl Line {
    fn len(&self) -> usize {
        self.obs.len()
    }

    fn num_broken_total(&self) -> usize {
        self.groups.iter().sum()
    }

    fn num_working_total(&self) -> usize {
        self.len() - self.num_broken_total()
    }

    fn is_valid_extension(&self, prefix_len: usize, ext: &[SpringHealth]) -> bool {
        for (i, &item) in ext.iter().enumerate() {
            if self.obs[prefix_len + i] == SpringHealth::Unknown {
                continue;
            }
            if self.obs[prefix_len + i] != item {
                return false;
            }
        }
        true
    }

    fn unfold(&self) -> Line {
        let new_obs = (0..5)
            .flat_map(|_| self.obs.iter().copied().chain([SpringHealth::Unknown]))
            .collect::<Vec<_>>();

        let new_groups = (0..5).flat_map(|_| self.groups.iter().copied()).collect();

        Line {
            obs: new_obs[..new_obs.len() - 1].into(),
            groups: new_groups,
        }
    }
}

fn find_matches(
    line: &Line,
    prefix_len: usize,
    broken: &[Vec<SpringHealth>],
    working: &[SpringHealth],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&cnt) = cache.get(&(prefix_len, broken.len())) {
        return cnt;
    }
    let maybe_broken_pattern = broken.first();
    let maybe_working = working.first();

    let cnt = match (maybe_broken_pattern, maybe_working) {
        (Some(broken_pattern), Some(_)) => {
            let a = if line.is_valid_extension(prefix_len, broken_pattern) {
                find_matches(
                    line,
                    prefix_len + broken_pattern.len(),
                    &broken[1..],
                    working,
                    cache,
                )
            } else {
                0
            };
            let mut offset = 1;
            while offset < working.len() - 1
                && line.obs[prefix_len + offset] == SpringHealth::Working
                && line.obs[prefix_len + offset + 1] == SpringHealth::Working
            {
                offset += 1;
            }
            let b = if line.is_valid_extension(prefix_len, &working[..offset]) {
                find_matches(line, prefix_len + offset, broken, &working[offset..], cache)
            } else {
                0
            };
            a + b
        }
        (Some(broken_pattern), None) => {
            if line.is_valid_extension(prefix_len, broken_pattern) {
                find_matches(
                    line,
                    prefix_len + broken_pattern.len(),
                    &broken[1..],
                    working,
                    cache,
                )
            } else {
                0
            }
        }
        (None, Some(_)) => {
            if line.is_valid_extension(prefix_len, working) {
                1
            } else {
                0
            }
        }
        (None, None) => return 1,
    };
    cache.insert((prefix_len, broken.len()), cnt);

    cnt
}

fn read<R: Read>(io: R) -> Vec<Line> {
    let br = BufReader::new(io);
    br.lines().map_while(Result::ok).map(Line::from).collect()
}

fn part_1(input: &[Line]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(_, line)| {
            let mut broken_patterns =
                vec![repeat_n(SpringHealth::Broken, line.groups[0]).collect::<Vec<_>>()];
            broken_patterns.extend(line.groups.iter().skip(1).map(|&n| {
                [SpringHealth::Working]
                    .into_iter()
                    .chain(repeat_n(SpringHealth::Broken, n))
                    .collect()
            }));

            let working_patterns = repeat_n(
                SpringHealth::Working,
                line.num_working_total() - (line.groups.len() - 1),
            )
            .collect::<Vec<_>>();
            let mut cache = HashMap::new();
            find_matches(line, 0, &broken_patterns, &working_patterns, &mut cache)
        })
        .sum()
}

fn part_2(input: &[Line]) -> usize {
    let input2 = input.iter().map(Line::unfold).collect::<Vec<_>>();
    part_1(&input2)
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
    fn example_1_part1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input[0..1]), 1);
        assert_eq!(part_1(&input[1..2]), 4);
        assert_eq!(part_1(&input[2..3]), 1);
        assert_eq!(part_1(&input[3..4]), 1);
        assert_eq!(part_1(&input[4..5]), 4);
        assert_eq!(part_1(&input[5..6]), 10);
        assert_eq!(part_1(&input), 21);
    }

    #[test]
    fn example_1_part2() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_2(&input[0..1]), 1);
        assert_eq!(part_2(&input[1..2]), 16384);
        assert_eq!(part_2(&input[2..3]), 1);
        assert_eq!(part_2(&input[3..4]), 16);
        assert_eq!(part_2(&input[4..5]), 2500);
        assert_eq!(part_2(&input[5..6]), 506250);
        assert_eq!(part_2(&input), 525152);
    }

    #[test]
    fn unfold() {
        let line = Line {
            obs: vec![SpringHealth::Working, SpringHealth::Broken],
            groups: vec![1],
        };
        let unfolded = line.unfold();
        assert_eq!(unfolded.groups, [1, 1, 1, 1, 1]);
        assert_eq!(
            unfolded.obs,
            [
                SpringHealth::Working,
                SpringHealth::Broken,
                SpringHealth::Unknown,
                SpringHealth::Working,
                SpringHealth::Broken,
                SpringHealth::Unknown,
                SpringHealth::Working,
                SpringHealth::Broken,
                SpringHealth::Unknown,
                SpringHealth::Working,
                SpringHealth::Broken,
                SpringHealth::Unknown,
                SpringHealth::Working,
                SpringHealth::Broken,
            ]
        );
    }
}
