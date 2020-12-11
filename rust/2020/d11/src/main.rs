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

    fn at(&self, row: usize, col: usize) -> Option<char> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(self.data[row * self.cols + col])
        }
    }

    fn count_occupancy_at(&self, row: usize, col: usize) -> usize {
        let row_from = if row == 0 { 0 } else { row - 1 };
        let row_to = if row == self.rows - 1 { row } else { row + 1};
        let col_from = if col == 0 { 0 } else { col - 1 };
        let col_to = if col == self.cols - 1 { col } else {col + 1};
        let mut occupancy = 0;
        for r in row_from..=row_to {
            for c in col_from..=col_to {
                if r == row && c == col {
                    continue;
                }
                occupancy += match self.at(r, c) {
                    Some(x) => if x == '#' { 1 } else { 0 },
                    None => 0,
                }
            }
        }
        occupancy
    }

    fn step(&self) -> Option<Field> {
        let mut new = self.data.clone();
        let mut idx = 0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                new[idx] = match self.at(i, j).unwrap() {
                    '#' => if self.count_occupancy_at(i, j) >= 4 { 'L' } else { '#' },
                    'L' => if self.count_occupancy_at(i, j) == 0 { '#' } else { 'L' },
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
    let mut field = field.step().unwrap();
    loop {
        let new_field = field.step();
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
    Ok(())
}
