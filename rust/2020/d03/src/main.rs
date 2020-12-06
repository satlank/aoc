use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
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

    fn at(&self, row: usize, col: usize) -> char {
        self.data[row * self.cols + (col % self.cols)]
    }

    fn collisions(&self, row_inc: usize, col_inc: usize) -> usize {
        let mut row = 0;
        let mut virt_col = 0;
        let mut collisions = 0;
        while row < self.rows {
            if self.at(row, virt_col) == '#' {
                collisions += 1
            }
            virt_col += col_inc;
            row += row_inc;
        }
        collisions
    }
}

fn part1(field: &Field) -> usize {
    field.collisions(1, 3)
}

fn part2(field: &Field) -> usize {
    let mut cols = 1;
    let options = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    for (row_inc, col_inc) in options.iter() {
        cols *= field.collisions(*row_inc, *col_inc);
    }
    cols
}

fn main() -> Result<(), Error> {
    let field = Field::parse("input.txt")?;
    println!("Collisions in part 1: {}", part1(&field));
    println!("Collisions in part 2: {} ", part2(&field));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
