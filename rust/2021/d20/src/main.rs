// https://adventofcode.com/2021/day/20

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn read<R: Read>(io: R) -> (Vec<bool>, Field) {
    let br = BufReader::new(io);
    let mut lines = br.lines();
    let line = lines.next().unwrap().unwrap();
    let algo = line.chars().map(|c| c == '#').collect::<Vec<bool>>();
    lines.next().unwrap().unwrap();
    let mut pts = HashMap::new();
    for (row, line) in lines.enumerate() {
        let line = line.unwrap();
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                pts.insert((col as i32, row as i32), true);
            } else {
                pts.insert((col as i32, row as i32), false);
            }
        }
    }
    (algo, Field::from_points(pts, false))
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Field {
    pts: HashMap<(i32, i32), bool>,
    top_left: (i32, i32),
    bottom_right: (i32, i32),
    infinity: bool,
}

fn get_block(pt: &(i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (pt.0 - 1, pt.1 - 1),
        (pt.0, pt.1 - 1),
        (pt.0 + 1, pt.1 - 1),
        (pt.0 - 1, pt.1),
        (pt.0, pt.1),
        (pt.0 + 1, pt.1),
        (pt.0 - 1, pt.1 + 1),
        (pt.0, pt.1 + 1),
        (pt.0 + 1, pt.1 + 1),
    ]
}

impl Field {
    fn from_points(pts: HashMap<(i32, i32), bool>, infinity: bool) -> Self {
        let mut top_left = (0i32, 0i32);
        let mut bottom_right = (0i32, 0i32);

        pts.iter().for_each(|(p, _)| {
            top_left.0 = i32::min(top_left.0, p.0);
            top_left.1 = i32::min(top_left.1, p.1);
            bottom_right.0 = i32::max(bottom_right.0, p.0);
            bottom_right.1 = i32::max(bottom_right.1, p.1);
        });

        Self {
            pts,
            top_left,
            bottom_right,
            infinity,
        }
    }

    fn apply_algo(&self, algo: &[bool]) -> Self {
        let mut pts = HashMap::new();
        for row in self.top_left.1 - 1..=self.bottom_right.1 + 1 {
            for col in self.top_left.0 - 1..=self.bottom_right.0 + 1 {
                let candidate = (col, row);
                let mut idx = 0;
                for test in &get_block(&candidate) {
                    idx *= 2;
                    if self.is_set(test) {
                        idx += 1;
                    }
                }
                if algo[idx] {
                    pts.insert(candidate, true);
                } else {
                    pts.insert(candidate, false);
                }
            }
        }
        Field::from_points(pts, if self.infinity { algo[511] } else { algo[0] })
    }

    fn is_set(&self, pt: &(i32, i32)) -> bool {
        *self.pts.get(pt).unwrap_or(&self.infinity)
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in self.top_left.1 - 1..=self.bottom_right.1 + 1 {
            for col in self.top_left.0 - 1..=self.bottom_right.0 + 1 {
                write!(f, "{}", if self.is_set(&(col, row)) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn enhance(algo: &[bool], field: &Field, level: usize) -> Field {
    assert!(level > 0);
    let mut field = field.apply_algo(algo);
    for _ in 1..level {
        field = field.apply_algo(algo);
    }
    field
}

fn part_1(algo: &[bool], field: &Field) -> usize {
    let field = enhance(algo, field, 2);
    field.pts.iter().filter(|(_, &val)| val).count()
}

fn part_2(algo: &[bool], field: &Field) -> usize {
    let field = enhance(algo, field, 50);
    field.pts.iter().filter(|(_, &val)| val).count()
}

fn main() {
    let (algo, field) = read(File::open("input.txt").unwrap());
    println!("Result of part 1: {}", part_1(&algo, &field));
    println!("Result of part 2: {}", part_2(&algo, &field));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let (algo, field) = read(File::open("test1.txt").unwrap());
        assert_eq!(algo.len(), 512);
        assert!(field.is_set(&(0, 0)));
        assert!(field.is_set(&(3, 0)));
        assert!(field.is_set(&(0, 1)));
        assert!(field.is_set(&(0, 2)));
        assert!(field.is_set(&(1, 2)));
        assert!(field.is_set(&(4, 2)));
        assert!(field.is_set(&(2, 3)));
        assert!(field.is_set(&(2, 4)));
        assert!(field.is_set(&(3, 4)));
        assert!(field.is_set(&(4, 4)));
        assert_eq!(field.top_left, (0, 0));
        assert_eq!(field.bottom_right, (4, 4));
    }

    #[test]
    fn test_part_1() {
        let (algo, field) = read(File::open("test1.txt").unwrap());
        assert_eq!(part_1(&algo, &field), 35);
    }

    #[test]
    fn test_part_2() {
        let (algo, field) = read(File::open("test1.txt").unwrap());
        assert_eq!(part_2(&algo, &field), 3351);
    }
}
