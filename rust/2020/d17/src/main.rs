use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
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
        Point {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        }
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

fn find_min_bound(active: &HashSet<Point>) -> Point {
    active.iter().fold(
        Point {
            x: i32::MAX,
            y: i32::MAX,
            z: i32::MAX,
            w: i32::MAX,
        },
        |curr, pt| Point {
            x: min(curr.x, pt.x),
            y: min(curr.y, pt.y),
            z: min(curr.z, pt.z),
            w: min(curr.w, pt.w),
        },
    )
}

fn find_max_bound(active: &HashSet<Point>) -> Point {
    active.iter().fold(
        Point {
            x: i32::MIN,
            y: i32::MIN,
            z: i32::MIN,
            w: i32::MIN,
        },
        |curr, pt| Point {
            x: max(curr.x, pt.x),
            y: max(curr.y, pt.y),
            z: max(curr.z, pt.z),
            w: max(curr.w, pt.w),
        },
    )
}

impl Field {
    fn print(&self) {
        for w in self.min.w..=self.max.w {
            for z in self.min.z..=self.max.z {
                println!("z = {}, w = {}", z, w);
                for y in self.min.y..=self.max.y {
                    for x in self.min.x..=self.max.x {
                        if self.active.contains(&Point { x, y, z, w }) {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    println!("");
                }
                println!("");
            }
            println!("");
        }
    }

    fn get_neighbour_count_at(&self, pt: &Point) -> usize {
        let mut neighbour = Point::new();
        let mut cnt = 0;
        for k in pt.z - 1..=pt.z + 1 {
            neighbour.z = k;
            for j in pt.y - 1..=pt.y + 1 {
                neighbour.y = j;
                for i in pt.x - 1..=pt.x + 1 {
                    neighbour.x = i;
                    for t in pt.w - 1..=pt.w + 1 {
                        neighbour.w = t;
                        if neighbour == *pt {
                            continue;
                        }
                        if self.active.contains(&neighbour) {
                            cnt += 1;
                        }
                    }
                }
            }
        }
        cnt
    }

    fn step(&self, inc: &Point) -> Field {
        let mut new_active = HashSet::new();
        for w in self.min.w - inc.w..=self.max.w + inc.w {
            for z in self.min.z - inc.z..=self.max.z + inc.z {
                for y in self.min.y - inc.y..=self.max.y + inc.y {
                    for x in self.min.x - inc.x..=self.max.x + inc.x {
                        let point = Point { x, y, z, w };
                        let is_active = self.active.contains(&point);
                        let cnt = self.get_neighbour_count_at(&point);
                        if (is_active && (cnt == 2 || cnt == 3)) || (!is_active && cnt == 3) {
                            new_active.insert(point);
                        }
                    }
                }
            }
        }
        Field {
            min: find_min_bound(&new_active),
            max: find_max_bound(&new_active),
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
                    w: 0i32,
                })
                .collect::<HashSet<_>>()
        })
        .flatten()
        .collect();
    Field {
        min: find_min_bound(&map),
        max: find_max_bound(&map),
        active: map,
    }
}

fn run(field: &Field, inc: &Point) -> usize {
    println!("Step 0: {}", field);
    field.print();
    let mut curr_field = field.step(inc);
    println!("Step 1: {}", curr_field);
    for step in 2..7 {
        curr_field = curr_field.step(inc);
        println!("Step {}: {}", step, curr_field);
    }
    curr_field.active.len()
}

fn main() {
    let field = read(File::open("input.txt").unwrap());
    println!("{}", field);
    println!(
        "Active after 6 steps in 3d: {}",
        run(
            &field,
            &Point {
                x: 1,
                y: 1,
                z: 1,
                w: 0
            }
        )
    );
    println!("");
    println!("");
    println!("");
    println!(
        "Active after 6 steps in 4d: {}",
        run(
            &field,
            &Point {
                x: 1,
                y: 1,
                z: 1,
                w: 1
            }
        )
    );
}
