// https://adventofcode.com/2024/day/6

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> (Area, Guard) {
    let br = BufReader::new(io);
    let lines = br.lines().map_while(Result::ok).collect::<Vec<_>>();
    let mut obstacles = HashSet::new();
    let mut guard = None;
    for (i, item) in lines.iter().enumerate() {
        for (j, c) in item.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert(Vec2 {
                        x: j as isize,
                        y: i as isize,
                    });
                }
                '.' => (),
                c if c == '^' || c == 'v' || c == '<' || c == '>' => {
                    guard = Some(Guard {
                        pos: Vec2 {
                            x: j as isize,
                            y: i as isize,
                        },
                        facing: match c {
                            '^' => Vec2 { x: 0, y: -1 },
                            '>' => Vec2 { x: 1, y: 0 },
                            'v' => Vec2 { x: 0, y: 1 },
                            '<' => Vec2 { x: -1, y: 0 },
                            _ => panic!("Invalid guard direction"),
                        },
                    })
                }
                _ => panic!("Invalid character at ({}, {})", j, i),
            }
        }
    }
    (
        Area {
            width: lines[0].len(),
            height: lines.len(),
            obstacles,
        },
        guard.expect("Guard not found"),
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vec2 {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Guard {
    pos: Vec2,
    facing: Vec2,
}

impl Guard {
    fn move_forward(&mut self) {
        self.pos.x += self.facing.x;
        self.pos.y += self.facing.y;
    }

    fn in_front(&self) -> Vec2 {
        Vec2 {
            x: self.pos.x + self.facing.x,
            y: self.pos.y + self.facing.y,
        }
    }

    fn turn_90_degrees_clockwise(&mut self) {
        self.facing = match self.facing {
            Vec2 { x: 0, y: -1 } => Vec2 { x: 1, y: 0 },
            Vec2 { x: 1, y: 0 } => Vec2 { x: 0, y: 1 },
            Vec2 { x: 0, y: 1 } => Vec2 { x: -1, y: 0 },
            Vec2 { x: -1, y: 0 } => Vec2 { x: 0, y: -1 },
            _ => panic!("Invalid facing direction"),
        };
    }
}

#[derive(Debug)]
struct Area {
    width: usize,
    height: usize,
    obstacles: HashSet<Vec2>,
}

impl Area {
    fn contains(&self, pos: &Vec2) -> bool {
        pos.x >= 0 && pos.x < self.width as isize && pos.y >= 0 && pos.y < self.height as isize
    }

    fn is_obstacle(&self, pos: &Vec2) -> bool {
        self.obstacles.contains(pos)
    }

    fn add_obstacle(&self, pos: Vec2) -> Area {
        let mut obstacles = self.obstacles.clone();
        let len = obstacles.len();
        obstacles.insert(pos);
        assert!(obstacles.len() == len + 1);
        Area {
            width: self.width,
            height: self.height,
            obstacles,
        }
    }
}

fn part_1(area: &Area, mut guard: Guard) -> usize {
    let mut visited = HashSet::new();
    while area.contains(&guard.pos) {
        visited.insert(guard.pos.clone());
        while area.is_obstacle(&guard.in_front()) {
            guard.turn_90_degrees_clockwise();
        }
        guard.move_forward();
    }
    visited.len()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum WalkResult {
    Loop,
    Exit,
}

fn walk_to_completion(area: Area, guard: &mut Guard, mut visited: HashSet<Guard>) -> WalkResult {
    while area.contains(&guard.pos) {
        let mut turn_cnt = 0;
        while area.is_obstacle(&guard.in_front()) && turn_cnt < 4 {
            guard.turn_90_degrees_clockwise();
            visited.insert(guard.clone());
            turn_cnt += 1;
        }
        if turn_cnt == 4 {
            return WalkResult::Loop;
        }
        guard.move_forward();
        if visited.contains(guard) {
            return WalkResult::Loop;
        }
        visited.insert(guard.clone());
    }
    WalkResult::Exit
}

fn part_2(area: &Area, mut guard: Guard) -> usize {
    let mut visited = HashSet::new(); // Track the guards positions and facing directions
    let mut visited_fields = HashSet::new(); // Track just the positions
    let mut cycle = 0;
    while area.contains(&guard.pos) {
        visited.insert(guard.clone());
        visited_fields.insert(guard.pos.clone());
        while area.is_obstacle(&guard.in_front()) {
            guard.turn_90_degrees_clockwise();
            if visited.contains(&guard) {
                // Don't expect to enter a loop here
                panic!("Loop detected at {:?}", guard);
            }
            visited.insert(guard.clone());
        }
        // Before moving forward, try placing an obstacle in front of the guard and see if she
        // enters a loop
        let front = guard.in_front();
        // ..but we only place an obstacle if the field is not already occupied by an obstacle
        // (should not happen, as we have already checked for obstacles in front of the guard, but
        // just to be sure), the obstacle is within the area, and the field has not been visited
        // (because we place the obstacle before the guard start moving, apparently, we can't sneak
        // in behind here and alter the field once she started walking).
        if area.contains(&front) && !area.is_obstacle(&front) && !visited_fields.contains(&front) {
            let new_obstacle = front.clone();
            match walk_to_completion(
                area.add_obstacle(new_obstacle),
                &mut guard.clone(),
                visited.clone(),
            ) {
                WalkResult::Loop => {
                    // println!("Loop detected at {:?} with obstacle {:?}", guard, front);
                    cycle += 1;
                }
                WalkResult::Exit => {}
            }
        }
        guard.move_forward();
    }
    cycle
}

fn main() {
    let (area, guard) = read(File::open("input.txt").unwrap());
    let p1 = part_1(&area, guard.clone());
    println!("Part 1: {}", p1);
    let p2 = part_2(&area, guard);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let (area, guard) = read(File::open("example1.txt").unwrap());
        assert_eq!(area.width, 10);
        assert_eq!(area.height, 10);
        assert_eq!(area.obstacles.len(), 8);
        assert_eq!(guard.pos, Vec2 { x: 4, y: 6 });
        assert_eq!(guard.facing, Vec2 { x: 0, y: -1 });
        assert_eq!(part_1(&area, guard.clone()), 41);
        assert_eq!(part_2(&area, guard), 6);
    }
}
