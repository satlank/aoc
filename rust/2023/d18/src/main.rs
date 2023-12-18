// https://adventofcode.com/2023/day/18

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> DigPlan {
    let br = BufReader::new(io);
    DigPlan {
        actions: br
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                let parts = line.split_whitespace().collect::<Vec<_>>();
                assert_eq!(parts.len(), 3);
                let distance = parts[1].parse::<u32>().unwrap();
                match parts[0] {
                    "U" => Action::Up(distance),
                    "D" => Action::Down(distance),
                    "L" => Action::Left(distance),
                    "R" => Action::Right(distance),
                    _ => panic!("Invalid action: {}", parts[0]),
                }
            })
            .collect(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DigPlan {
    actions: Vec<Action>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

fn part_1(dig_plan: &DigPlan) -> usize {
    let mut perimeter = HashSet::new();
    let mut current = (0, 0);
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    perimeter.insert(current);
    for a in &dig_plan.actions {
        let (d, vec) = match a {
            Action::Up(d) => (d, (0, 1)),
            Action::Down(d) => (d, (0, -1)),
            Action::Left(d) => (d, (-1, 0)),
            Action::Right(d) => (d, (1, 0)),
        };
        for _ in 0..*d {
            current = (current.0 + vec.0, current.1 + vec.1);
            if current.0 < min_x {
                min_x = current.0;
            }
            if current.0 > max_x {
                max_x = current.0;
            }
            if current.1 < min_y {
                min_y = current.1;
            }
            if current.1 > max_y {
                max_y = current.1;
            }
            perimeter.insert(current);
        }
    }

    let mut outside = HashSet::new();
    for x in min_x - 1..=max_x + 1 {
        outside.insert((x, min_y - 1));
        outside.insert((x, max_y + 1));
    }
    for y in min_y..=max_y {
        outside.insert((min_x - 1, y));
        outside.insert((max_x + 1, y));
    }
    let mut new_outside = outside.clone();
    loop {
        new_outside = new_outside
            .iter()
            .flat_map(|(x, y)| {
                vec![
                    (x - 1, *y),
                    (x + 1, *y),
                    (*x, y - 1),
                    (*x, y + 1),
                    (x - 1, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y + 1),
                    (x + 1, y + 1),
                ]
                .into_iter()
                .filter(|(x, y)| x >= &min_x && x <= &max_x && y >= &min_y && y <= &max_y)
                .filter(|(x, y)| !outside.contains(&(*x, *y)))
                .filter(|(x, y)| !perimeter.contains(&(*x, *y)))
            })
            .collect::<HashSet<(i32, i32)>>();
        if new_outside.is_empty() {
            break;
        }
        outside.extend(new_outside.iter());
    }

    let mut inside = 0;
    for y in min_y - 1..=max_y + 1 {
        for x in min_x - 1..=max_x + 1 {
            if perimeter.contains(&(x, y)) {
                inside += 1;
                //print!("*");
            } else if outside.contains(&(x, y)) {
                //print!(".");
            } else {
                inside += 1;
                //print!("#");
            }
        }
        //println!("");
    }
    inside
}

fn part_2(dig_plan: &DigPlan) -> usize {
    0
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
        assert_eq!(part_1(&input), 62);
        //assert_eq!(part_2(&input), 1);
    }
}
