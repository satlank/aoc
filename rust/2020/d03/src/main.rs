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
                return Err(Error::new(ErrorKind::InvalidData, ""))
            }
            for c in line.chars() {
                data.push(c);
            }
            rows += 1;
        }
        Ok(Field {data, cols, rows})
    }

    fn at(&self, row: usize, col: usize) -> char {
        self.data[row * self.cols + (col % self.cols)]
    }
}

fn part1(field: &Field) -> usize {
    let mut row = 0;
    let mut virt_col = 0;
    let mut collisions = 0;
    while row < field.rows {
        if field.at(row, virt_col) == '#' { collisions += 1 }
        virt_col += 3;
        row += 1;
    }
    return collisions
}

fn main() -> Result<(), Error> {
    let field = Field::parse("input.txt")?;
    println!("Collisions in part 1: {}", part1(&field));
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

}
