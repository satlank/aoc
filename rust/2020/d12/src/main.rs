use std::fs::File;
use std::convert::AsRef;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[macro_use]
extern crate scan_fmt;

#[derive(Clone, PartialEq, Debug)]
enum Op {
    EAST(i16),
    NORTH(i16),
    WEST(i16),
    SOUTH(i16),
    FWD(i16),
    LEFT(i16),
    RIGHT(i16)
}

#[derive(Clone, PartialEq, Debug)]
struct Loc {
    x: i16,
    y: i16,
    bearing: i16,
}

impl Op {
    fn parse<S: AsRef<str>>(line: S) -> Result<Op, scan_fmt::parse::ScanError> {
        let line = line.as_ref();
        let (op_name, value) = scan_fmt!(line, "{/[ENWSFLR]/}{d}", char, i16)?;
        Ok(match op_name {
            'E' => Op::EAST(value),
            'N' => Op::NORTH(value),
            'W' => Op::WEST(value),
            'S' => Op::SOUTH(value),
            'F' => Op::FWD(value),
            'L' => Op::LEFT(value),
            'R' => Op::RIGHT(value),
            _ => panic!("Huh")
        })
    }
}

impl AsRef<Op> for Op {
    fn as_ref(&self) -> &Op {
        &self
    }
}

impl Loc {
    fn new() -> Loc {
        Loc {x: 0, y: 0, bearing: 90}
    }

    fn apply<O: AsRef<Op>>(&mut self, op: O) {
        match op.as_ref() {
            Op::EAST(x) => self.x += x,
            Op::NORTH(x) => self.y += x,
            Op::WEST(x) => self.x -= x,
            Op::SOUTH(x) => self.y -= x,
            Op::RIGHT(x) => self.bearing = (self.bearing + x) % 360,
            Op::LEFT(x) => self.bearing = (self.bearing - x + 360) % 360,
            Op::FWD(x) => {
                match self.bearing {
                    0 => self.y += x,
                    90 => self.x += x,
                    180 => self.y -= x,
                    270 => self.x -= x,
                    _ => {panic!("Waaah!")}
                }
            },
        }
    }

    fn manhattan_distance(&self) -> u16 {
        (self.x.abs() + self.y.abs()) as u16
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Loc2 {
    x: i16,
    y: i16,
    waypoint_x: i16,
    waypoint_y: i16,
}

impl Loc2 {
    fn new() -> Loc2 {
        Loc2 {x: 0, y: 0, waypoint_x: 10, waypoint_y: 1}
    }

    fn apply<O: AsRef<Op>>(&mut self, op: O) {
        match op.as_ref() {
            Op::EAST(x) => self.waypoint_x += x,
            Op::NORTH(x) => self.waypoint_y += x,
            Op::WEST(x) => self.waypoint_x -= x,
            Op::SOUTH(x) => self.waypoint_y -= x,
            Op::RIGHT(x) => {
                match x {
                    90 => {
                        let tmp = self.waypoint_x;
                        self.waypoint_x = self.waypoint_y;
                        self.waypoint_y = -tmp;
                    },
                    180 => {
                        self.waypoint_x = -self.waypoint_x;
                        self.waypoint_y = -self.waypoint_y;
                    },
                    270 => {
                        let tmp = self.waypoint_x;
                        self.waypoint_x = -self.waypoint_y;
                        self.waypoint_y = tmp;
                    },
                    _ => panic!("Waaah!"),
                }
            },
            Op::LEFT(x) => {
                match x {
                    90 => {
                        let tmp = self.waypoint_x;
                        self.waypoint_x = -self.waypoint_y;
                        self.waypoint_y = tmp;
                    },
                    180 => {
                        self.waypoint_x = -self.waypoint_x;
                        self.waypoint_y = -self.waypoint_y;
                    },
                    270 => {
                        let tmp = self.waypoint_x;
                        self.waypoint_x = self.waypoint_y;
                        self.waypoint_y = -tmp;
                    },
                    _ => panic!("Waaah!"),
                }
            },
            Op::FWD(x) => {
                self.x += x * self.waypoint_x;
                self.y += x * self.waypoint_y;
            },
        }
    }

    fn manhattan_distance(&self) -> u16 {
        (self.x.abs() + self.y.abs()) as u16
    }
}

fn read<R: Read>(io: R) -> Result<Vec<Op>, Error> {
    BufReader::new(io)
        .lines()
        .map(|line| {
            line.and_then(|row| Op::parse(row).map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        })
        .collect()
}

fn part1(ops: &[Op]) -> u16 {
    let mut loc = Loc::new();
    for op in ops {
        loc.apply(op);
        println!("{:?}", loc);
    }
    loc.manhattan_distance()
}

fn part2(ops: &[Op]) -> u16 {
    let mut loc = Loc2::new();
    for op in ops {
        loc.apply(op);
        println!("{:?}", loc);
    }
    loc.manhattan_distance()
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    println!("Read {} instructions", vec.len());
    println!("Manhattan distance part 1: {}", part1(&vec));
    println!("Manhattan distance part 2: {}", part2(&vec));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ops: Vec<Op> = vec![
            Op::parse("F10").unwrap(),
            Op::parse("N3").unwrap(),
            Op::parse("F7").unwrap(),
            Op::parse("R90").unwrap(),
            Op::parse("F11").unwrap(),
        ];
        let mut loc = Loc::new();
        for op in ops {
            loc.apply(op);
            println!("{:?}", loc);
        }
        assert_eq!(loc.manhattan_distance(), 25);
    }

    #[test]
    fn test_2() {
        let ops: Vec<Op> = vec![
            Op::parse("F10").unwrap(),
            Op::parse("N3").unwrap(),
            Op::parse("F7").unwrap(),
            Op::parse("R90").unwrap(),
            Op::parse("F11").unwrap(),
        ];
        let mut loc = Loc2::new();
        for op in ops {
            loc.apply(op);
            println!("{:?}", loc);
        }
        assert_eq!(loc.manhattan_distance(), 286);
    }
}
