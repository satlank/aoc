// https://adventofcode.com/2022/day/17

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Jet {
    MoveRight,
    MoveLeft,
}

impl Jet {
    fn as_move(&self) -> (i32, i32) {
        match self {
            Jet::MoveLeft => (-1, 0),
            Jet::MoveRight => (1, 0),
        }
    }
}

impl From<char> for Jet {
    fn from(c: char) -> Self {
        if c == '>' {
            Self::MoveRight
        } else if c == '<' {
            Self::MoveLeft
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
struct Shape {
    cells: HashSet<(i32, i32)>,
    origin_shape: ShapeType,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum ShapeType {
    Minus,
    Cross,
    MirrorL,
    VertLine,
    Block,
}

fn print_rocks(rocks: &HashSet<(i32, i32)>) {
    for y in (0..20).rev() {
        for x in 0..7 {
            print!("{}", if rocks.contains(&(x, y)) { '#' } else { '.' });
        }
        println!();
    }
}

impl From<usize> for ShapeType {
    fn from(cnt: usize) -> ShapeType {
        let wrapped = cnt % 5;
        match wrapped {
            0 => ShapeType::Minus,
            1 => ShapeType::Cross,
            2 => ShapeType::MirrorL,
            3 => ShapeType::VertLine,
            4 => ShapeType::Block,
            _ => unreachable!(),
        }
    }
}

impl Shape {
    fn new(shape_type: ShapeType, bottom: i32) -> Shape {
        Shape {
            cells: shape_type.into(),
            origin_shape: shape_type,
        }
        .shifted(2, bottom + 4)
    }

    fn shifted(&self, x: i32, y: i32) -> Self {
        Self {
            cells: self.cells.iter().map(|(cx, cy)| (cx + x, cy + y)).collect(),
            origin_shape: self.origin_shape,
        }
    }

    fn is_valid(&self) -> bool {
        for c in &self.cells {
            if c.0 >= 7 || c.0 < 0 {
                return false;
            }
        }
        true
    }
}

impl From<ShapeType> for HashSet<(i32, i32)> {
    fn from(st: ShapeType) -> Self {
        match st {
            ShapeType::Minus => [(0, 0), (1, 0), (2, 0), (3, 0)].into_iter().collect(),
            ShapeType::Cross => [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]
                .into_iter()
                .collect(),
            ShapeType::MirrorL => [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]
                .into_iter()
                .collect(),
            ShapeType::VertLine => [(0, 0), (0, 1), (0, 2), (0, 3)].into_iter().collect(),
            ShapeType::Block => [(0, 0), (1, 0), (0, 1), (1, 1)].into_iter().collect(),
        }
    }
}

fn read<R: Read>(io: R) -> Vec<Jet> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .take(1)
        .flat_map(|line| line.chars().map(|c| Jet::from(c)).collect::<Vec<_>>())
        .collect()
}

fn part_1(input: &[Jet]) -> usize {
    let mut completed_rocks = 0;
    let mut blocks = [
        (0, -1),
        (1, -1),
        (2, -1),
        (3, -1),
        (4, -1),
        (5, -1),
        (6, -1),
    ]
    .into_iter()
    .collect::<HashSet<(i32, i32)>>();
    let mut highest = -1;
    let mut cnt = 0;
    while completed_rocks < 2022 {
        let mut rock_moving = true;
        let mut shape = Shape::new(completed_rocks.into(), highest);
        while rock_moving {
            let jet_move = input[cnt % input.len()].as_move();
            cnt += 1;
            //println!("jet move: {:?}", jet_move);
            let jet_moved_shape = shape.shifted(jet_move.0, jet_move.1);
            let after_jet = if jet_moved_shape.is_valid()
                && blocks.intersection(&jet_moved_shape.cells).count() == 0
            {
                jet_moved_shape
            } else {
                shape
            };

            let dropped = after_jet.shifted(0, -1);
            if blocks.intersection(&dropped.cells).count() != 0 {
                // collided with things
                highest = i32::max(
                    highest,
                    after_jet.cells.iter().map(|(_, y)| *y).max().unwrap(),
                );
                //println!("{} {:?}", highest, after_jet);
                blocks.extend(after_jet.cells);
                rock_moving = false;
                completed_rocks += 1;
                shape = Shape::new(completed_rocks.into(), highest);
                //println!("({}, {}) New shape: {:?}", completed_rocks, cnt, shape);
                //print_rocks(&blocks);
                //println!();
            } else {
                shape = dropped;
                rock_moving = true;
            }
        }
        if completed_rocks % 10_000 == 0 {
            println!("Done {}", completed_rocks);
        }
    }
    highest as usize + 1
}

fn part_2(input: &[Jet]) -> usize {
    let target_play = 1_000_000_000_000;

    let mut completed_rocks = 0;
    let mut blocks = [
        (0, -1),
        (1, -1),
        (2, -1),
        (3, -1),
        (4, -1),
        (5, -1),
        (6, -1),
    ]
    .into_iter()
    .collect::<HashSet<(i32, i32)>>();
    let mut highest = -1;
    let mut cnt = 0;

    let mut cycle_detector = HashMap::<(ShapeType, usize, usize), (usize, usize)>::new();

    while completed_rocks < target_play {
        let mut rock_moving = true;
        let mut rock_fall_steps = 0;
        let mut shape = Shape::new(completed_rocks.into(), highest);
        let height_at_start = highest as usize;
        while rock_moving {
            let jet_move = input[cnt % input.len()].as_move();
            cnt += 1;
            let jet_moved_shape = shape.shifted(jet_move.0, jet_move.1);
            let after_jet = if jet_moved_shape.is_valid()
                && blocks.intersection(&jet_moved_shape.cells).count() == 0
            {
                jet_moved_shape
            } else {
                shape
            };

            let dropped = after_jet.shifted(0, -1);
            rock_fall_steps += 1;
            if blocks.intersection(&dropped.cells).count() != 0 {
                // collided with things
                rock_fall_steps -= 1; // fix-up counter of how often we moved
                let wind_cycle = (input.len() + cnt - 1 - rock_fall_steps) % input.len();
                let current_shape = after_jet.origin_shape;
                let cycle_key = (current_shape, wind_cycle, rock_fall_steps);
                if cycle_detector.contains_key(&cycle_key) {
                    let val = cycle_detector[&cycle_key];
                    let cycle = completed_rocks - val.0;
                    let added_rows = height_at_start - val.1;
                    //println!("Cycle is {} with {} rows adds", cycle, added_rows);
                    if (target_play - completed_rocks) % cycle == 0 {
                        println!("The cycle length is {} rocks and there are now {} rocks left to drop..", cycle, target_play - completed_rocks);
                        println!(
                            "..which are {} cycles with {} rows each",
                            (target_play - completed_rocks) / cycle,
                            added_rows
                        );
                        println!(
                            "..so take our current height of {} and add the cycles..",
                            height_at_start
                        );
                        println!("..and add 1 because we start counting at 0");
                        return height_at_start
                            + (target_play - completed_rocks) / cycle * added_rows
                            + 1;
                    }
                }
                cycle_detector.insert(cycle_key, (completed_rocks, height_at_start));
                highest = i32::max(
                    highest,
                    after_jet.cells.iter().map(|(_, y)| *y).max().unwrap(),
                );
                //println!("{} {:?}", highest, after_jet);
                blocks.extend(after_jet.cells);
                rock_moving = false;
                completed_rocks += 1;
                shape = Shape::new(completed_rocks.into(), highest);
                //println!("({}, {}) New shape: {:?}", completed_rocks, cnt, shape);
                //print_rocks(&blocks);
                //println!();
            } else {
                shape = dropped;
                rock_moving = true;
            }
        }
        if completed_rocks % 10_000 == 0 {
            println!("Done {}", completed_rocks);
        }
    }
    highest as usize + 1
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
    fn base_shapes() {
        assert_eq!(4, HashSet::<(i32, i32)>::from(ShapeType::Minus).len());
        assert_eq!(5, HashSet::<(i32, i32)>::from(ShapeType::Cross).len());
        assert_eq!(5, HashSet::<(i32, i32)>::from(ShapeType::MirrorL).len());
        assert_eq!(4, HashSet::<(i32, i32)>::from(ShapeType::VertLine).len());
        assert_eq!(4, HashSet::<(i32, i32)>::from(ShapeType::Block).len());
    }

    #[test]
    fn make_shape() {
        let shape = Shape::new(ShapeType::Cross, 4);
        let expected = [(3, 8), (2, 9), (3, 9), (4, 9), (3, 10)]
            .into_iter()
            .collect::<HashSet<(i32, i32)>>();
        assert_eq!(shape.cells, expected);
    }

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(input.len(), 40);
        assert_eq!(input[4], Jet::MoveLeft);
        assert_eq!(input[32], Jet::MoveRight);
        assert_eq!(input[33], Jet::MoveLeft);
        assert_eq!(input[33 % 40], Jet::MoveLeft);
        assert_eq!(part_1(&input), 3068);
        assert_eq!(part_2(&input), 1514285714288);
    }
}
