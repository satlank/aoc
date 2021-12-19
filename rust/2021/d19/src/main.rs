// https://adventofcode.com/2021/day/19

use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::convert::Infallible;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let coords = s
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Ok(Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Scanner {
    id: usize,
    pts: HashSet<Point>,
}

impl Scanner {
    fn from_points(id: usize, pts: HashSet<Point>) -> Self {
        Self { id, pts }
    }

    fn rotations(&self) -> Vec<HashSet<Point>> {
        let t = &self.pts;

        // Based on rotation matrices around x/y/z axes, e.g.
        //
        //             | 1     0       0   |
        // rot_x(a) =  | 0  cos(a) -sin(a) |
        //             | 0  sin(a)  cos(a) |
        //
        //             |  cos(a)  0   sin(a) |
        // rot_y(a) =  |    0     1     0    |
        //             | -sin(a)  0   cos(a) |
        //
        //             | cos(a)  -sin(a)  0 |
        // rot_z(a) =  | sin(a)   cos(a)  0 |
        //             |   0        0     1 |
        //
        // First find 6 choices to point in positive/negative x/y/z axes:
        let mut res: Vec<HashSet<Point>> = vec![
            t.iter()
                .map(|p| Point {
                    x: p.x,
                    y: p.y,
                    z: p.z,
                })
                .collect(), // facing positive x
            t.iter()
                .map(|p| Point {
                    x: -p.x,
                    y: -p.y,
                    z: p.z,
                })
                .collect(), // facing negative x
            t.iter()
                .map(|p| Point {
                    x: p.y,
                    y: -p.x,
                    z: p.z,
                })
                .collect(), // facing positive y
            t.iter()
                .map(|p| Point {
                    x: -p.y,
                    y: p.x,
                    z: p.z,
                })
                .collect(), // facing negative y
            t.iter()
                .map(|p| Point {
                    x: p.z,
                    y: p.y,
                    z: -p.x,
                })
                .collect(), // facing positive z
            t.iter()
                .map(|p| Point {
                    x: -p.z,
                    y: p.y,
                    z: p.x,
                })
                .collect(), // facing negative z
        ];

        // ..then generating the 3 other directions for 'up' by rotating the according vector around the x-axis in 90, 180, 270 degree
        for i in 0..6 {
            let t = &res[i];
            let r90 = t
                .iter()
                .map(|p| Point {
                    x: p.x,
                    y: -p.z,
                    z: p.y,
                })
                .collect();
            let r180 = t
                .iter()
                .map(|p| Point {
                    x: p.x,
                    y: -p.y,
                    z: -p.z,
                })
                .collect();
            let r270 = t
                .iter()
                .map(|p| Point {
                    x: p.x,
                    y: p.z,
                    z: -p.y,
                })
                .collect();
            res.push(r90);
            res.push(r180);
            res.push(r270);
        }

        // Phew
        res
    }
}

fn read<R: Read>(io: R) -> Vec<Scanner> {
    let br = BufReader::new(io);
    let lines = br.lines();
    let mut scanners = Vec::new();
    let mut pts = HashSet::new();
    for line in lines {
        let line = line.unwrap();
        if line.starts_with("--- scanner") {
            continue;
        }
        if line.is_empty() {
            scanners.push(Scanner::from_points(scanners.len(), pts));
            pts = HashSet::new();
        } else {
            pts.insert(line.parse::<Point>().unwrap());
        }
    }
    scanners.push(Scanner::from_points(scanners.len(), pts));
    scanners
}

fn check_addition(base: &HashSet<Point>, trial: &Scanner) -> Option<(Point, HashSet<Point>)> {
    for rot in trial.rotations() {
        for a in base {
            for b in &rot {
                let offset = b.sub(a);
                let tmp = &rot
                    .iter()
                    .map(|x| x.sub(&offset))
                    .collect::<HashSet<Point>>();
                if base.intersection(tmp).count() >= 12 {
                    return Some((offset, tmp.clone()));
                }
            }
        }
    }
    None
}

fn part_1(scnr: &[Scanner]) -> usize {
    let mut final_points = scnr[0].pts.clone();
    let mut working_set = (0..scnr.len()).collect::<VecDeque<usize>>();

    while !working_set.is_empty() {
        let trial = working_set.pop_front().unwrap();
        if let Some((_, addition)) = check_addition(&final_points, &scnr[trial]) {
            final_points = final_points.union(&addition).copied().collect();
        } else {
            working_set.push_back(trial);
        }
    }

    final_points.len()
}

fn part_2(scnr: &[Scanner]) -> usize {
    let mut final_points = scnr[0].pts.clone();
    let mut working_set = (0..scnr.len()).collect::<VecDeque<usize>>();
    let mut offsets = vec![Point::new(0, 0, 0)];

    while !working_set.is_empty() {
        let trial = working_set.pop_front().unwrap();
        if let Some((offset, addition)) = check_addition(&final_points, &scnr[trial]) {
            final_points = final_points.union(&addition).copied().collect();
            offsets.push(offset);
        } else {
            working_set.push_back(trial);
        }
    }

    offsets
        .iter()
        .combinations(2)
        .map(|com| {
            let dist = com[0].sub(com[1]);
            let dist = i32::abs(dist.x) + i32::abs(dist.y) + i32::abs(dist.z);
            dist as usize
        })
        .max()
        .unwrap()
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    println!("Result of part 1: {}", part_1(&input));
    println!("Result of part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let input = read(File::open("test1.txt").unwrap());
        assert_eq!(input.len(), 5);
        assert_eq!(
            input.iter().map(|x| x.pts.len()).collect::<Vec<usize>>(),
            vec![25, 25, 26, 25, 26]
        );
        assert_eq!(
            input.iter().map(|x| x.id).collect::<Vec<usize>>(),
            vec![0, 1, 2, 3, 4]
        );
    }

    #[test]
    fn test_part_1() {
        let input = read(File::open("test1.txt").unwrap());
        assert_eq!(part_1(&input), 79);
    }

    #[test]
    fn test_part_2() {
        let input = read(File::open("test1.txt").unwrap());
        assert_eq!(part_2(&input), 3621);
    }

    #[test]
    fn test_rotation() {
        let mut pt = HashSet::new();
        pt.insert(Point::new(1, 2, 3));
        let scnr = Scanner::from_points(0, pt);
        let rots = scnr.rotations();
        let all_pts = rots
            .iter()
            .map(|hs| hs.iter().cloned())
            .flatten()
            .collect::<HashSet<Point>>();
        assert_eq!(all_pts.len(), 24);
        assert!(rots[0].contains(&Point::new(1, 2, 3)));
        assert!(rots[6].contains(&Point::new(1, -3, 2)));
        assert!(rots[7].contains(&Point::new(1, -2, -3)));
        assert!(rots[8].contains(&Point::new(1, 3, -2)));
        assert!(rots[1].contains(&Point::new(-1, -2, 3)));
    }
}
