// https://adventofcode.com/2023/day/23

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug)]
enum PathTile {
    Up,
    Down,
    Left,
    Right,
    Flat,
}

#[derive(Debug)]
struct Trail {
    path: HashMap<(i64, i64), PathTile>,
    start: (i64, i64),
    end: (i64, i64),
}

fn read<R: Read>(io: R) -> Trail {
    let br = BufReader::new(io);
    let path = br
        .lines()
        .map_while(Result::ok)
        .enumerate()
        .flat_map(|(y, sk)| {
            sk.chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    let tile = match c {
                        '^' => Some(PathTile::Up),
                        'v' => Some(PathTile::Down),
                        '<' => Some(PathTile::Left),
                        '>' => Some(PathTile::Right),
                        '.' => Some(PathTile::Flat),
                        _ => None,
                    };
                    if let Some(t) = tile {
                        Some(((x as i64, y as i64), t))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>();
    let start = (1, 0);
    let max_y = path.keys().map(|(_, y)| y).max().unwrap();
    let end = path
        .keys()
        .filter(|(_, y)| y == max_y)
        .copied()
        .collect::<Vec<_>>()[0];
    Trail { path, start, end }
}

#[derive(Debug, Clone, Default)]
struct Path {
    pos: (i64, i64),
    history: HashSet<(i64, i64)>,
}

impl Path {
    fn next_steps(&self, trail: &Trail) -> Vec<(i64, i64)> {
        let current = self.pos;
        let tile = trail.path.get(&current).unwrap();
        let next = match tile {
            PathTile::Up => vec![(current.0, current.1 - 1)],
            PathTile::Down => vec![(current.0, current.1 + 1)],
            PathTile::Left => vec![(current.0 - 1, current.1)],
            PathTile::Right => vec![(current.0 + 1, current.1)],
            PathTile::Flat => vec![
                (current.0 - 1, current.1),
                (current.0 + 1, current.1),
                (current.0, current.1 - 1),
                (current.0, current.1 + 1),
            ],
        };
        next.into_iter()
            .filter(|p| trail.path.contains_key(p) && !self.history.contains(p))
            .collect()
    }

    fn step_to_next_split(&mut self, trail: &Trail) -> Option<(i64, i64)> {
        let mut current = self.pos;
        loop {
            let next = vec![
                (current.0 - 1, current.1),
                (current.0 + 1, current.1),
                (current.0, current.1 - 1),
                (current.0, current.1 + 1),
            ];
            let mut next = next
                .into_iter()
                .filter(|p| (trail.path.contains_key(p) && !self.history.contains(p)))
                .collect::<Vec<_>>();
            match next.len() {
                0 => return None,
                1 => {
                    current = next.pop().unwrap();
                    self.history.insert(current);
                    if current == trail.end || current == trail.start {
                        return Some(current);
                    }
                }
                _ => return Some(current),
            }
        }
    }
}

fn next_steps(pos: (i64, i64), trail: &Trail) -> Vec<(i64, i64)> {
    let next = vec![
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ];

    next.into_iter()
        .filter(|p| trail.path.contains_key(p))
        .collect()
}

fn build_nodes(trail: &Trail) -> HashMap<(i64, i64), Vec<((i64, i64), usize)>> {
    let mut nodes = HashMap::new();
    let mut new_nodes = Vec::new();

    new_nodes.push(trail.start);

    while !new_nodes.is_empty() {
        let pos = new_nodes.pop().unwrap();
        if nodes.contains_key(&pos) {
            continue;
        }
        let next = next_steps(pos, trail);
        let mut paths = next
            .into_iter()
            .map(|p| {
                let mut path = Path::default();
                path.pos = p;
                path.history.insert(pos);
                path.history.insert(p);
                path
            })
            .collect::<Vec<_>>();
        let mut children = Vec::new();
        paths.iter_mut().for_each(|p| {
            if let Some(next_node) = p.step_to_next_split(trail) {
                children.push((next_node, p.history.len() - 1));
                new_nodes.push(next_node);
            }
        });
        if children.is_empty() {
            continue;
        }
        nodes.insert(pos, children);
    }

    nodes
}

fn part_1(input: &Trail) -> usize {
    let mut paths = Vec::new();
    let mut path = Path::default();
    path.pos = input.start;
    path.history.insert(input.start);
    paths.push(path);

    while paths.iter().any(|p| p.pos != input.end) {
        let mut new_paths = Vec::new();
        for mut p in paths.into_iter() {
            if p.pos == input.end {
                continue;
            }
            let next = p.next_steps(input);
            if next.is_empty() {
                continue;
            }
            if next.len() == 1 {
                p.history.insert(next[0]);
                p.pos = next[0];
                new_paths.push(p);
            } else {
                for i in 0..next.len() {
                    let mut new_path = p.clone();
                    new_path.history.insert(next[i]);
                    new_path.pos = next[i];
                    new_paths.push(new_path);
                }
            }
        }
        paths = new_paths;
    }

    paths.iter().map(|p| p.history.len()).max().unwrap() - 1
}

#[derive(Debug, Clone)]
struct NodePath {
    pos: (i64, i64),
    history: HashSet<(i64, i64)>,
    len: usize,
}

fn node_walk(
    nodes: &HashMap<(i64, i64), Vec<((i64, i64), usize)>>,
    start: (i64, i64),
    end: (i64, i64),
) -> usize {
    let mut paths = Vec::new();
    let mut path = NodePath {
        pos: start,
        history: HashSet::new(),
        len: 0,
    };
    path.history.insert(start);
    paths.push(path);

    while paths.iter().any(|p| p.pos != end) {
        let mut new_paths = Vec::new();
        for mut p in paths.into_iter() {
            if p.pos == end {
                new_paths.push(p);
                continue;
            }
            let next = nodes
                .get(&p.pos)
                .unwrap()
                .iter()
                .copied()
                .filter(|(pos, _d)| !p.history.contains(pos))
                .collect::<Vec<_>>();
            if next.is_empty() {
                // Dead end
                continue;
            }
            if next.len() == 1 {
                p.history.insert(next[0].0);
                p.len += next[0].1;
                p.pos = next[0].0;
                new_paths.push(p);
            } else {
                for i in 0..next.len() {
                    let mut new_path = p.clone();
                    new_path.history.insert(next[i].0);
                    new_path.pos = next[i].0;
                    new_path.len += next[i].1;
                    new_paths.push(new_path);
                }
            }
        }
        paths = new_paths;
    }

    paths.iter().map(|p| p.len).max().unwrap()
}

fn part_2(input: &Trail) -> usize {
    let nodes = build_nodes(input);
    node_walk(&nodes, input.start, input.end)
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
        assert_eq!(part_1(&input), 94);
        assert_eq!(part_2(&input), 154);
    }
}
