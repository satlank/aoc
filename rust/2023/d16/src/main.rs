// https://adventofcode.com/2023/day/16

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Contraption {
    let br = BufReader::new(io);
    let lines = br.lines().map_while(Result::ok).collect::<Vec<String>>();

    let size = lines.len();
    let objects = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .filter_map(|(x, y, c)| {
            let o = Object::from(c);
            if o == Object::Empty {
                None
            } else {
                Some((Pos { x: x + 1, y: y + 1 }, o))
            }
        })
        .collect::<HashMap<_, _>>();

    Contraption { size, objects }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Contraption {
    size: usize,
    objects: HashMap<Pos, Object>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Object {
    Empty,
    MirrorRightUp,
    MirrorLeftUp,
    VerticalSplit,
    HorizontalSplit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    pos: Pos,
    dir: Direction,
}

impl From<char> for Object {
    fn from(c: char) -> Self {
        match c {
            '.' => Object::Empty,
            '/' => Object::MirrorRightUp,
            '\\' => Object::MirrorLeftUp,
            '|' => Object::VerticalSplit,
            '-' => Object::HorizontalSplit,
            _ => panic!("Unknown object: {}", c),
        }
    }
}

fn step(con: &Contraption, beam: Beam) -> Option<(Beam, Option<Beam>)> {
    let Beam { pos, dir } = beam;
    let Pos { x, y } = pos;
    let (x, y) = match dir {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    };
    let new_pos = Pos { x, y };
    if new_pos.x > con.size || new_pos.y > con.size || new_pos.x == 0 || new_pos.y == 0 {
        None
    } else if let Some(o) = con.objects.get(&new_pos) {
        match o {
            Object::MirrorRightUp => {
                let dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                Some((Beam { pos: new_pos, dir }, None))
            }
            Object::MirrorLeftUp => {
                let dir = match dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                Some((Beam { pos: new_pos, dir }, None))
            }
            Object::VerticalSplit => match dir {
                Direction::Up | Direction::Down => Some((Beam { pos: new_pos, dir }, None)),
                Direction::Left | Direction::Right => Some((
                    Beam {
                        pos: new_pos,
                        dir: Direction::Up,
                    },
                    Some(Beam {
                        pos: new_pos,
                        dir: Direction::Down,
                    }),
                )),
            },
            Object::HorizontalSplit => match dir {
                Direction::Up | Direction::Down => Some((
                    Beam {
                        pos: new_pos,
                        dir: Direction::Left,
                    },
                    Some(Beam {
                        pos: new_pos,
                        dir: Direction::Right,
                    }),
                )),
                Direction::Left | Direction::Right => Some((Beam { pos: new_pos, dir }, None)),
            },
            _ => unreachable!(),
        }
    } else {
        Some((Beam { pos: new_pos, dir }, None))
    }
}

fn walk(con: &Contraption, beam_head: Beam, beam: &mut HashSet<Beam>) {
    match step(con, beam_head) {
        None => {}
        Some((new_head, None)) => {
            if !beam.contains(&new_head) {
                beam.insert(new_head);
                walk(con, new_head, beam)
            }
        }
        Some((new_head_1, Some(new_head_2))) => {
            if !beam.contains(&new_head_1) {
                beam.insert(new_head_1);
                walk(con, new_head_1, beam)
            }
            if !beam.contains(&new_head_2) {
                beam.insert(new_head_2);
                walk(con, new_head_2, beam)
            }
        }
    }
}

fn part_1(input: &Contraption) -> usize {
    let mut beam = HashSet::new();
    let beam_head = Beam {
        pos: Pos { x: 0, y: 1 },
        dir: Direction::Right,
    };
    walk(input, beam_head, &mut beam);
    let energized = beam.iter().map(|b| b.pos).collect::<HashSet<_>>();

    energized.len()
}

fn part_2(input: &Contraption) -> usize {
    [
        (1..=input.size)
            .map(|y| {
                let mut beam = HashSet::new();
                let beam_head = Beam {
                    pos: Pos { x: 0, y },
                    dir: Direction::Right,
                };
                walk(input, beam_head, &mut beam);
                beam.iter().map(|b| b.pos).collect::<HashSet<_>>().len()
            })
            .max()
            .unwrap(),
        (1..=input.size)
            .map(|y| {
                let mut beam = HashSet::new();
                let beam_head = Beam {
                    pos: Pos {
                        x: input.size + 1,
                        y,
                    },
                    dir: Direction::Left,
                };
                walk(input, beam_head, &mut beam);
                beam.iter().map(|b| b.pos).collect::<HashSet<_>>().len()
            })
            .max()
            .unwrap(),
        (1..=input.size)
            .map(|x| {
                let mut beam = HashSet::new();
                let beam_head = Beam {
                    pos: Pos { x, y: 0 },
                    dir: Direction::Down,
                };
                walk(input, beam_head, &mut beam);
                beam.iter().map(|b| b.pos).collect::<HashSet<_>>().len()
            })
            .max()
            .unwrap(),
        (1..=input.size)
            .map(|x| {
                let mut beam = HashSet::new();
                let beam_head = Beam {
                    pos: Pos {
                        x,
                        y: input.size + 1,
                    },
                    dir: Direction::Up,
                };
                walk(input, beam_head, &mut beam);
                beam.iter().map(|b| b.pos).collect::<HashSet<_>>().len()
            })
            .max()
            .unwrap(),
    ]
    .into_iter()
    .max()
    .unwrap()
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
        assert_eq!(part_1(&input), 46);
        assert_eq!(part_2(&input), 51);
    }
}
