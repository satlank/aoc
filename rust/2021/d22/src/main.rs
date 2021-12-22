// https://adventofcode.com/2021/day/22

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[macro_use]
extern crate scan_fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cuboid {
    from: (i32, i32, i32),
    to: (i32, i32, i32),
}

impl Cuboid {
    fn new(from: (i32, i32, i32), to: (i32, i32, i32)) -> Self {
        Self { from, to }
    }

    fn size(&self) -> usize {
        (i32::abs(self.to.0 - self.from.0) as usize + 1)
            * (i32::abs(self.to.1 - self.from.1) as usize + 1)
            * (i32::abs(self.to.2 - self.from.2) as usize + 1)
    }

    fn is_valid(&self) -> bool {
        self.from.0 <= self.to.0 && self.from.1 <= self.to.1 && self.from.2 <= self.to.2
    }

    fn overlap(&self, other: &Cuboid) -> Option<Cuboid> {
        if other.to.0 < self.from.0
            || other.from.0 > self.to.0
            || other.to.1 < self.from.1
            || other.from.1 > self.to.1
            || other.to.2 < self.from.2
            || other.from.2 > self.to.2
        {
            return None;
        }
        Some(Cuboid::new(
            (
                i32::max(self.from.0, other.from.0),
                i32::max(self.from.1, other.from.1),
                i32::max(self.from.2, other.from.2),
            ),
            (
                i32::min(self.to.0, other.to.0),
                i32::min(self.to.1, other.to.1),
                i32::min(self.to.2, other.to.2),
            ),
        ))
    }

    fn cut(&self, other: &Cuboid) -> Vec<Cuboid> {
        if let Some(overlap) = self.overlap(other) {
            let xs = (self.from.0, overlap.from.0, overlap.to.0, self.to.0);
            let ys = (self.from.1, overlap.from.1, overlap.to.1, self.to.1);
            let zs = (self.from.2, overlap.from.2, overlap.to.2, self.to.2);

            let res = vec![
                // x=0, y=0, z=0
                Cuboid::new((xs.0, ys.0, zs.0), (xs.1 - 1, ys.1 - 1, zs.1 - 1)),
                // x=1, y=0, z=0
                Cuboid::new((xs.1, ys.0, zs.0), (xs.2, ys.1 - 1, zs.1 - 1)),
                // x=2, y=0, z=0
                Cuboid::new((xs.2 + 1, ys.0, zs.0), (xs.3, ys.1 - 1, zs.1 - 1)),
                //
                // x=0, y=1, z=0
                Cuboid::new((xs.0, ys.1, zs.0), (xs.1 - 1, ys.2, zs.1 - 1)),
                // x=1, y=1, z=0
                Cuboid::new((xs.1, ys.1, zs.0), (xs.2, ys.2, zs.1 - 1)),
                // x=2, y=1, z=0
                Cuboid::new((xs.2 + 1, ys.1, zs.0), (xs.3, ys.2, zs.1 - 1)),
                //
                // x=0, y=2, z=0
                Cuboid::new((xs.0, ys.2 + 1, zs.0), (xs.1 - 1, ys.3, zs.1 - 1)),
                // x=1, y=2, z=0
                Cuboid::new((xs.1, ys.2 + 1, zs.0), (xs.2, ys.3, zs.1 - 1)),
                // x=2, y=2, z=0
                Cuboid::new((xs.2 + 1, ys.2 + 1, zs.0), (xs.3, ys.3, zs.1 - 1)),
                //
                //
                // x=0, y=0, z=1
                Cuboid::new((xs.0, ys.0, zs.1), (xs.1 - 1, ys.1 - 1, zs.2)),
                // x=1, y=0, z=1
                Cuboid::new((xs.1, ys.0, zs.1), (xs.2, ys.1 - 1, zs.2)),
                // x=2, y=0, z=1
                Cuboid::new((xs.2 + 1, ys.0, zs.1), (xs.3, ys.1 - 1, zs.2)),
                //
                // x=0, y=1, z=1
                Cuboid::new((xs.0, ys.1, zs.1), (xs.1 - 1, ys.2, zs.2)),
                // x=1, y=1, z=1
                // Cuboid::new((xs.1, ys.1, zs.1), (xs.2, ys.2, zs.2));
                // x=2, y=1, z=1
                Cuboid::new((xs.2 + 1, ys.1, zs.1), (xs.3, ys.2, zs.2)),
                //
                // x=0, y=2, z=1
                Cuboid::new((xs.0, ys.2 + 1, zs.1), (xs.1 - 1, ys.3, zs.2)),
                // x=1, y=2, z=1
                Cuboid::new((xs.1, ys.2 + 1, zs.1), (xs.2, ys.3, zs.2)),
                // x=2, y=2, z=1
                Cuboid::new((xs.2 + 1, ys.2 + 1, zs.1), (xs.3, ys.3, zs.2)),
                //
                //
                // x=0, y=0, z=2
                Cuboid::new((xs.0, ys.0, zs.2 + 1), (xs.1 - 1, ys.1 - 1, zs.3)),
                // x=1, y=0, z=2
                Cuboid::new((xs.1, ys.0, zs.2 + 1), (xs.2, ys.1 - 1, zs.3)),
                // x=2, y=0, z=2
                Cuboid::new((xs.2 + 1, ys.0, zs.2 + 1), (xs.3, ys.1 - 1, zs.3)),
                //
                // x=0, y=1, z=2
                Cuboid::new((xs.0, ys.1, zs.2 + 1), (xs.1 - 1, ys.2, zs.3)),
                // x=1, y=1, z=2
                Cuboid::new((xs.1, ys.1, zs.2 + 1), (xs.2, ys.2, zs.3)),
                // x=2, y=1, z=2
                Cuboid::new((xs.2 + 1, ys.1, zs.2 + 1), (xs.3, ys.2, zs.3)),
                //
                // x=0, y=2, z=2
                Cuboid::new((xs.0, ys.2 + 1, zs.2 + 1), (xs.1 - 1, ys.3, zs.3)),
                // x=1, y=2, z=2
                Cuboid::new((xs.1, ys.2 + 1, zs.2 + 1), (xs.2, ys.3, zs.3)),
                // x=2, y=2, z=2
                Cuboid::new((xs.2 + 1, ys.2 + 1, zs.2 + 1), (xs.3, ys.3, zs.3)),
            ];
            res.iter().copied().filter(|c| c.is_valid()).collect()
        } else {
            vec![self.clone()]
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    On(Cuboid),
    Off(Cuboid),
}

fn read<R: Read>(io: R) -> Vec<Operation> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| {
            let line = line.unwrap();
            let (cmd, xmin, xmax, ymin, ymax, zmin, zmax) = scan_fmt!(
                line.as_ref(),
                "{} x={d}..{d},y={d}..{d},z={d}..{d}",
                String,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32
            )
            .unwrap();
            let c = Cuboid {
                from: (xmin, ymin, zmin),
                to: (xmax, ymax, zmax),
            };
            if cmd == "on" {
                Operation::On(c)
            } else {
                Operation::Off(c)
            }
        })
        .collect()
}

fn build(ops: &[Operation]) -> Reactor {
    let mut reactor = Reactor::default();
    for op in ops {
        match op {
            Operation::On(c) => reactor.set_on(*c),
            Operation::Off(c) => reactor.set_off(*c),
        }
    }
    reactor
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct Reactor {
    on: Vec<Cuboid>,
}

impl Reactor {
    fn set_on(&mut self, c: Cuboid) {
        self.set_off(c);
        self.on.push(c);
    }

    fn set_off(&mut self, c: Cuboid) {
        self.on = self.on.iter().map(|on| on.cut(&c)).flatten().collect();
    }

    fn number_of_on_cells(&self) -> usize {
        self.on.iter().map(|c| c.size()).sum()
    }

    fn number_of_on_cells_in_area(&self, c: &Cuboid) -> usize {
        self.on
            .iter()
            .map(|on| {
                if let Some(overlap) = c.overlap(on) {
                    overlap.size()
                } else {
                    0
                }
            })
            .sum()
    }
}

fn part_1(ops: &[Operation]) -> usize {
    let reactor = build(ops);
    reactor.number_of_on_cells_in_area(&Cuboid::new((-50, -50, -50), (50, 50, 50)))
    //reactor.iter().filter(|&p| in_range(p)).count()
}

fn part_2(ops: &[Operation]) -> usize {
    let reactor = build(ops);
    reactor.number_of_on_cells()
}

fn main() {
    let ops = read(File::open("input.txt").unwrap());
    println!("Result of part 1: {}", part_1(&ops));
    println!("Result of part 2: {}", part_2(&ops));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let ops = read(File::open("test1.txt").unwrap());
        assert_eq!(ops.len(), 22);
    }

    #[test]
    fn test_part_1() {
        let ops = read(File::open("test1.txt").unwrap());
        assert_eq!(part_1(&ops), 590784);
    }

    #[test]
    fn test_part_2() {
        let ops = read(File::open("test2.txt").unwrap());
        assert_eq!(part_2(&ops), 2758514936282235);
    }

    #[test]
    fn test_overlap() {
        let base = Cuboid::new((-10, -10, -10), (10, 10, 10));
        assert_eq!(
            base.overlap(&Cuboid::new((-20, -20, -20), (-15, -15, -15))),
            None
        );
        assert_eq!(
            base.overlap(&Cuboid::new((-20, -20, -20), (-10, -10, -10))),
            Some(Cuboid::new((-10, -10, -10), (-10, -10, -10)))
        );
        assert_eq!(
            base.overlap(&Cuboid::new((-20, -20, -20), (20, 10, 10))),
            Some(Cuboid::new((-10, -10, -10), (10, 10, 10)))
        );
    }

    #[test]
    fn test_build() {
        let mut reactor = Reactor::default();
        reactor.set_on(Cuboid::new((-10, 0, 0), (10, 10, 10)));
        assert_eq!(reactor.number_of_on_cells(), 21 * 11 * 11);
        reactor.set_off(Cuboid::new((0, 0, 0), (10, 10, 10)));
        assert_eq!(reactor.number_of_on_cells(), 10 * 11 * 11);
        reactor.set_on(Cuboid::new((0, 0, 0), (10, 10, 10)));
        assert_eq!(reactor.number_of_on_cells(), 21 * 11 * 11);
        reactor.set_on(Cuboid::new((0, 0, 0), (10, 10, 10)));
        assert_eq!(reactor.number_of_on_cells(), 21 * 11 * 11);
    }

    #[test]
    fn test_cut() {
        let base = Cuboid::new((-3, -3, -3), (3, 3, 3));
        let slice = Cuboid::new((-3, -3, -3), (-1, -1, -1));
        let cut = base.cut(&slice);
        assert_eq!(cut.len(), 7);
        assert_eq!(
            cut.iter().map(|x| x.size()).sum::<usize>(),
            base.size() - slice.size()
        );
    }
}
