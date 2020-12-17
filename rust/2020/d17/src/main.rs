use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Field {
    min: Point,
    max: Point,
    active: HashSet<Point>,
}

impl Point {
    fn new() -> Point {
        Point {x: 0, y: 0, z:0 }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "min: {} max: {} active: {}",
            self.min,
            self.max,
            self.active.len()
        )
    }
}

impl Field {
    fn print(&self) {
        for z in self.min.z..=self.max.z {
            for y in self.min.y..=self.max.y {
                for x in self.min.x..=self.max.x {
                    if self.active.contains(&Point{x, y, z}) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
            println!("");
        }
    }

    fn step(&self) -> Field {
        let mut new_active = HashSet::new();
        for z in self.min.z-1..=self.max.z+1 {
            for y in self.min.y-1..=self.max.y+1 {
                for x in self.min.x-1..=self.max.x+1 {
                    let point = Point {x, y, z};
                    let is_active = self.active.contains(&point);
                    let mut neighbour = Point::new();
                    let mut cnt = 0;
                    for k in z-1..=z+1 {
                        neighbour.z = k;
                        for j in y-1..=y+1 {
                            neighbour.y = j;
                            for i in x-1..=x+1 {
                                neighbour.x = i;
                                if i == x && j == y && k == z {
                                    continue;
                                }
                                if self.active.contains(&neighbour) {
                                    cnt += 1;
                                }
                            }
                        }
                    }
                    if (is_active && (cnt == 2 || cnt == 3)) || (!is_active && cnt == 3) {
                        new_active.insert(point);
                    }
                }
            }
        }
        Field {
            min: Point {x: self.min.x-1, y: self.min.y-1, z: self.min.z-1},
            max: Point {x: self.max.x+1, y: self.max.y+1, z: self.max.z+1},
            active: new_active,
        }
    }
}

fn read<R: Read>(io: R) -> Field {
    let br = BufReader::new(io);
    let map = br
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| Point {
                    x: x as i32,
                    y: y as i32,
                    z: 0i32,
                })
                .collect::<HashSet<_>>()
        })
        .flatten()
        .collect();
    Field {
        min: Point { x: 0, y: 0, z: 0 },
        max: Point { x: 7, y: 7, z: 0 },
        active: map,
    }
}

fn part1(field: &Field) -> usize {
    println!("Step 0: {}", field);
    field.print();
    let mut curr_field = field.step();
    println!("Step 1: {}", curr_field);
    for step in 2..7 {
        curr_field = curr_field.step();
        println!("Step {}: {}", step, curr_field);
    }
    curr_field.active.len()
}

fn main() {
    let field = read(File::open("input.txt").unwrap());
    println!("{}", field);
    println!("Active after 6 steps: {}", part1(&field));
}
