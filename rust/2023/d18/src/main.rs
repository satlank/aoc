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

fn read2<R: Read>(io: R) -> DigPlan {
    let br = BufReader::new(io);
    DigPlan {
        actions: br
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                let parts = line.split_whitespace().collect::<Vec<_>>();
                assert_eq!(parts.len(), 3);
                let encoded = parts[2];
                // (#70c710)
                // 012345678
                let len = u32::from_str_radix(&encoded[2..7], 16).unwrap();
                match &encoded[7..8] {
                    "3" => Action::Up(len),
                    "1" => Action::Down(len),
                    "2" => Action::Left(len),
                    "0" => Action::Right(len),
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
            } else if outside.contains(&(x, y)) {
            } else {
                inside += 1;
            }
        }
    }
    inside
}

fn part_2(dig_plan: &DigPlan) -> usize {
    let mut perimeter = Vec::new();
    let mut current = (0, 0);
    let mut permimeter_len = 0i64;
    perimeter.push(current);
    for a in &dig_plan.actions {
        let (d, vec) = match a {
            Action::Up(d) => (d, (0, 1)),
            Action::Down(d) => (d, (0, -1)),
            Action::Left(d) => (d, (-1, 0)),
            Action::Right(d) => (d, (1, 0)),
        };
        permimeter_len += *d as i64;
        let d = *d as i32;
        current = (current.0 + d * vec.0, current.1 + d * vec.1);
        perimeter.push(current);
    }
    assert_eq!(perimeter.last(), perimeter.first());

    // Using the [trapazoid area formula](https://en.wikipedia.org/wiki/Shoelace_formula#Trapezoid_formula)
    // BUT we need to correct for the 'thickness' of the perimeter.  For corner tiles, we only
    // account for 1/4 or 3/4 of the tile, and for straight-edge tiles, we only account for 1/2 of the tile.
    // There are going to be 4 more 1/4 tiles than 3/4 tiles (because the polygon is closed), so we
    // can just count the corner as 1/2 as well and then finally add the extra 1 on.
    let mut area = 0i64;

    // NB: -1 because the the first and last are the same point
    for i in 0..perimeter.len() - 1 {
        let (x0, y0) = perimeter[i];
        let (x1, y1) = perimeter[i + 1];
        area += ((y0 + y1) as i64) * ((x0 - x1) as i64);
    }

    (area.abs() / 2 + permimeter_len / 2 + 1) as usize
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input);
    println!("Part 1: {}", p1);
    let input = read2(File::open("input.txt").unwrap());
    let p2 = part_2(&input);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input), 62);
    }

    #[test]
    fn example_1_2() {
        let input = read2(File::open("example1.txt").unwrap());
        assert_eq!(part_2(&input), 952408144115);
    }
}
