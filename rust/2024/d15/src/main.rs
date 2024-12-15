// https://adventofcode.com/2024/day/15

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> (Warehouse, Vec<Move>) {
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines().map_while(Result::ok).collect();
    let div = lines.iter().position(|l| l.is_empty()).unwrap();
    let mut fields = lines[..div]
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| {
                let pos = Vec2 {
                    x: x as i32,
                    y: y as i32,
                };
                let field = FieldType::try_from(c).unwrap();
                (pos, field)
            })
        })
        .collect::<HashMap<_, _>>();
    let robot = fields
        .iter()
        .find_map(|(pos, field)| {
            if *field == FieldType::Robot {
                Some(*pos)
            } else {
                None
            }
        })
        .unwrap();
    fields.retain(|_, f| *f != FieldType::Robot && *f != FieldType::Empty);
    let warehouse = Warehouse { robot, fields };

    let moves = lines[div + 1..]
        .iter()
        .flat_map(|l| l.chars())
        .map(Move::try_from)
        .map_while(Result::ok)
        .collect();

    (warehouse, moves)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn neighbor(&self, m: Move) -> Vec2 {
        match m {
            Move::Up => Vec2 {
                x: self.x,
                y: self.y - 1,
            },
            Move::Down => Vec2 {
                x: self.x,
                y: self.y + 1,
            },
            Move::Left => Vec2 {
                x: self.x - 1,
                y: self.y,
            },
            Move::Right => Vec2 {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FieldType {
    Wall,
    Crate,
    Empty,
    Robot,
}

impl TryFrom<char> for FieldType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(FieldType::Wall),
            'O' => Ok(FieldType::Crate),
            '.' => Ok(FieldType::Empty),
            '@' => Ok(FieldType::Robot),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Warehouse {
    robot: Vec2,
    fields: HashMap<Vec2, FieldType>,
}

impl Warehouse {
    fn box_gps(&self) -> impl Iterator<Item = usize> + '_ {
        self.fields.iter().flat_map(|(pos, t)| {
            if *t == FieldType::Crate {
                Some((pos.y * 100 + pos.x) as usize)
            } else {
                None
            }
        })
    }

    fn move_crate(&mut self, pos: Vec2, m: Move) -> bool {
        let new_pos = pos.neighbor(m);
        let can_move = if let Some(obj) = self.fields.get(&new_pos) {
            if *obj == FieldType::Wall {
                false
            } else {
                assert_eq!(*obj, FieldType::Crate);
                self.move_crate(new_pos, m)
            }
        } else {
            true
        };
        if can_move {
            self.fields.insert(new_pos, FieldType::Crate);
            self.fields.remove(&pos);
        }
        can_move
    }

    fn move_robot(&mut self, m: Move) {
        let new_pos = self.robot.neighbor(m);
        if let Some(obj) = self.fields.get(&new_pos) {
            let can_move = match obj {
                FieldType::Wall => false,
                FieldType::Crate => self.move_crate(new_pos, m),
                FieldType::Empty | FieldType::Robot => unreachable!(),
            };
            if can_move {
                self.robot = new_pos;
            }
        } else {
            self.robot = new_pos;
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Move::Up),
            'v' => Ok(Move::Down),
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            _ => Err(()),
        }
    }
}

fn part_1(warehouse: &Warehouse, moves: &[Move]) -> usize {
    let mut warehouse = warehouse.clone();
    moves.iter().for_each(|m| {
        warehouse.move_robot(*m);
    });
    warehouse.box_gps().sum()
}

fn part_2(_warehouse: &Warehouse, _moves: &[Move]) -> usize {
    1
}

fn main() {
    let (warehouse, moves) = read(File::open("input.txt").unwrap());
    let p1 = part_1(&warehouse, &moves);
    println!("Part 1: {}", p1);
    let p2 = part_2(&warehouse, &moves);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let (warehouse, moves) = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&warehouse, &moves), 10092);
        assert_eq!(part_2(&warehouse, &moves), 1);
    }

    #[test]
    fn example_2() {
        let (warehouse, moves) = read(File::open("example2.txt").unwrap());
        assert_eq!(part_1(&warehouse, &moves), 2028);
        assert_eq!(part_2(&warehouse, &moves), 1);
    }
}
