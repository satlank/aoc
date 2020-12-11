use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Field {
    data: Vec<char>,
    cols: usize,
    rows: usize,
}

impl Field {
    fn parse<P: AsRef<Path>>(filename: P) -> Result<Field, Error> {
        let lines = BufReader::new(File::open(filename)?).lines();
        let mut data: Vec<char> = Vec::new();
        let mut cols = 0;
        let mut rows = 0;
        for line in lines {
            let line = line?;
            if cols == 0 {
                cols = line.len();
            } else if cols != line.len() {
                return Err(Error::new(ErrorKind::InvalidData, ""));
            }
            for c in line.chars() {
                data.push(c);
            }
            rows += 1;
        }
        Ok(Field { data, cols, rows })
    }

    fn total_occupancy(&self) -> usize {
        self.data
            .iter()
            .filter(|&c| *c == '#')
            .count()
    }

    fn at(&self, row: isize, col: isize) -> Option<char> {
        if row < 0 || row >= self.rows as isize || col < 0 || col >= self.cols as isize {
            None
        } else {
            Some(self.data[(row as usize) * self.cols + (col as usize)])
        }
    }

    fn find_first(&self, row: usize, col: usize, row_inc: isize, col_inc: isize, steps: usize) -> Option<char> {
        let mut step: isize = 1;
        loop {
            match self.at(row as isize + step * row_inc, col as isize + step * col_inc) {
                Some(x) => if x != '.' { return Some(x) },
                None => return None,
            };
            step += 1;
            if step as usize > steps {
                return None;
            }
        }
    }

    fn count_occupancy_at(&self, row: usize, col: usize, steps: usize) -> usize {
        let mut occupancy = 0;
        for r in -1..=1 {
            for c in -1..=1 {
                if r == 0 && c == 0 {
                    continue;
                }
                occupancy += match self.find_first(row, col, r, c, steps) {
                    Some(x) => if x == '#' { 1 } else { 0 },
                    None => 0,
                }
            }
        }
        occupancy
    }

    fn step(&self, directional_steps: usize, occupancy_limit: usize) -> Option<Field> {
        let mut new = self.data.clone();
        let mut idx = 0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                new[idx] = match self.at(i as isize, j as isize).unwrap() {
                    '#' => if self.count_occupancy_at(i, j, directional_steps) >= occupancy_limit { 'L' } else { '#' },
                    'L' => if self.count_occupancy_at(i, j, directional_steps) == 0 { '#' } else { 'L' },
                    '.' => '.',
                    _ => panic!("WAAAH!")
                };
                idx += 1;
            }
        }
        if new != self.data {
            Some(Field {
                data: new,
                cols: self.cols,
                rows: self.rows
            })
        } else {
            None
        }
    }
}

fn part1(field: &Field) -> usize {
    let mut field = field.step(1, 4).unwrap();
    loop {
        let new_field = field.step(1, 4);
        if new_field.is_none() {
            return field.total_occupancy();
        }
        field = new_field.unwrap();
    }
}

fn part2(field: &Field) -> usize {
    let mut field = field.step(100, 5).unwrap();
    loop {
        let new_field = field.step(100, 5);
        if new_field.is_none() {
            return field.total_occupancy();
        }
        field = new_field.unwrap();
    }
}

fn main() -> Result<(), Error> {
    let field = Field::parse("input.txt")?;
    println!("Field of {} x {} read", field.cols, field.rows);
    println!("Total occupied seats in part 1: {}", part1(&field));
    println!("Total occupied seats in part 2: {}", part2(&field));
    Ok(())
}
