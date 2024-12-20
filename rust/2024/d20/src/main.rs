// https://adventofcode.com/2024/day/20

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Racetrack {
    let br = BufReader::new(io);
    let mut tracks = HashSet::new();
    let mut height = 0;
    let mut width = 0;
    let mut start = None;
    let mut end = None;
    for (y, line) in br.lines().map_while(Result::ok).enumerate() {
        height = y;
        for (x, c) in line.chars().enumerate() {
            if x > width {
                width = x;
            }
            let p = Vec2 {
                x: x as i32,
                y: y as i32,
            };
            match c {
                '.' => {
                    tracks.insert(p);
                }
                'S' => {
                    tracks.insert(p);
                    start = Some(p);
                }
                'E' => {
                    tracks.insert(p);
                    end = Some(p);
                }
                _ => {}
            }
        }
    }
    Racetrack {
        tracks,
        start: start.unwrap(),
        end: end.unwrap(),
        width,
        height,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn neighbours(&self) -> [Vec2; 4] {
        [
            Vec2 {
                x: self.x - 1,
                y: self.y,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y,
            },
            Vec2 {
                x: self.x,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }

    #[allow(dead_code)]
    fn cheat_neighbours(&self) -> [Vec2; 8] {
        [
            // 4 diagonal
            Vec2 {
                x: self.x - 1,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x - 1,
                y: self.y + 1,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y + 1,
            },
            // 4 straight
            Vec2 {
                x: self.x - 2,
                y: self.y,
            },
            Vec2 {
                x: self.x + 2,
                y: self.y,
            },
            Vec2 {
                x: self.x,
                y: self.y - 2,
            },
            Vec2 {
                x: self.x,
                y: self.y + 2,
            },
        ]
    }

    fn neighbours_up_to_steps(&self, steps: usize) -> HashMap<Vec2, usize> {
        let mut result = HashMap::new();
        result.insert(*self, 0);
        let mut s = 0;
        while s < steps {
            s += 1;
            let current = result.keys().cloned().collect::<Vec<_>>();
            for p in current {
                for n in p.neighbours() {
                    if !result.contains_key(&n) {
                        result.insert(n, s);
                    }
                }
            }
        }
        result
    }
}

#[test]
fn test_neighbours() {
    let p = Vec2 { x: 0, y: 0 };
    let n = p.neighbours();
    assert_eq!(n.len(), 4);
    assert!(n.contains(&Vec2 { x: -1, y: 0 }));
    assert!(n.contains(&Vec2 { x: 1, y: 0 }));
    assert!(n.contains(&Vec2 { x: 0, y: -1 }));
    assert!(n.contains(&Vec2 { x: 0, y: 1 }));
}

#[test]
fn test_neighbours_is_same_as_exact_1step() {
    let p = Vec2 { x: 0, y: 0 };
    let n = p.neighbours();
    let n2 = p.neighbours_up_to_steps(1);
    assert_eq!(n.len(), n2.iter().filter(|(_, v)| **v == 1).count());
    for p in n {
        assert!(n2.contains_key(&p));
    }
}

#[test]
fn test_cheat_is_same_as_exact_2step() {
    let p = Vec2 { x: 0, y: 0 };
    let n = p.cheat_neighbours();
    let n2 = p.neighbours_up_to_steps(2);
    assert_eq!(n.len(), n2.iter().filter(|(_, v)| **v == 2).count());
    for p in n {
        assert!(n2.contains_key(&p));
    }
}

#[derive(Debug)]
struct Racetrack {
    tracks: HashSet<Vec2>,
    start: Vec2,
    end: Vec2,
    width: usize,
    height: usize,
}

impl ::std::fmt::Display for Racetrack {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        for y in 0..=self.height {
            for x in 0..=self.width {
                let p = Vec2 {
                    x: x as i32,
                    y: y as i32,
                };
                if p == self.start {
                    write!(f, "S")?;
                } else if p == self.end {
                    write!(f, "E")?;
                } else if self.tracks.contains(&p) {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

struct Track {
    tiles: HashMap<Vec2, usize>,
}

impl Track {
    fn new(racetrack: &Racetrack) -> Self {
        let mut tiles = HashMap::new();
        let mut cp = racetrack.start;
        tiles.insert(cp, 0);

        let mut ct = 1;
        while cp != racetrack.end {
            let mut next = None;
            for n in cp.neighbours() {
                // There is only one path to the end, so there is only exactly one neighbor we can
                // go to
                if racetrack.tracks.contains(&n) && !tiles.contains_key(&n) {
                    next = Some(n);
                    break;
                }
            }
            if let Some(n) = next {
                cp = n;
            } else {
                panic!("No path found");
            }
            tiles.insert(cp, ct);
            ct += 1;
        }

        Self { tiles }
    }

    fn cheats_at_with_min(&self, p: Vec2, t: usize, min: usize) -> Vec<(Vec2, Vec2, usize)> {
        let ttp = self.tiles.get(&p).unwrap();
        p.neighbours_up_to_steps(t)
            .iter()
            .filter(|(n, steps)| {
                let ttn = self.tiles.get(n);
                ttn.is_some() && ttn.unwrap() >= &(ttp + *steps + min)
            })
            .map(|(n, steps)| {
                let saved = self.tiles.get(n).unwrap() - self.tiles.get(&p).unwrap() - steps;
                (p, *n, saved)
            })
            .collect()
    }

    fn all_cheats_with_min(&self, t: usize, min: usize) -> Vec<(Vec2, Vec2, usize)> {
        let mut result = Vec::new();
        for (p, _) in &self.tiles {
            result.extend(self.cheats_at_with_min(*p, t, min));
        }
        result
    }
}

fn part_1(input: &Racetrack, min: usize) -> usize {
    let track = Track::new(input);
    track.all_cheats_with_min(2, min).len()
}

fn part_2(input: &Racetrack, min: usize) -> usize {
    let track = Track::new(input);
    track.all_cheats_with_min(20, min).len()
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input, 100);
    println!("Part 1: {}", p1);
    let p2 = part_2(&input, 100);
    println!("Part 2: {}", p2);
}

#[test]
fn example_1() {
    let racetrack = read(File::open("example1.txt").unwrap());
    assert_eq!(
        part_1(&racetrack, 2),
        14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1
    );
    assert_eq!(
        part_2(&racetrack, 50),
        32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
    );
}
