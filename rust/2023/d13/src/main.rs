// https://adventofcode.com/2023/day/13

use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::{BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<Field> {
    let mut data = String::new();
    BufReader::new(io).read_to_string(&mut data).unwrap();
    data.split("\n\n").map(Field::from).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldType {
    Ash,
    Rock,
}

impl From<char> for FieldType {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Unknown field type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Field {
    width: usize,
    height: usize,
    field: Vec<Vec<FieldType>>,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.field {
            for field in row {
                match field {
                    FieldType::Ash => write!(f, ".")?,
                    FieldType::Rock => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<&str> for Field {
    fn from(s: &str) -> Self {
        let lines = s.lines().collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();
        let field = lines
            .iter()
            .map(|line| line.chars().map(FieldType::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self {
            width,
            height,
            field,
        }
    }
}

impl Field {
    fn is_column_reflect(&self, column: usize) -> bool {
        assert!(column > 0);
        assert!(column < self.width);

        let cols_left = column;
        let cols_right = self.width - column;

        let dx_range = cols_left.min(cols_right);
        assert!(dx_range > 0, "should at least have one column");

        for y in 0..self.height {
            for dx in 1..=dx_range {
                let left = self.field[y][column - dx];
                let right = self.field[y][column + dx - 1];
                if left != right {
                    return false;
                }
            }
        }
        true
    }

    fn is_row_reflect(&self, row: usize) -> bool {
        assert!(row > 0);
        assert!(row < self.height);

        let rows_above = row;
        let rows_below = self.height - row;

        let dy_range = rows_above.min(rows_below);
        assert!(dy_range > 0, "should at least have one row");

        for x in 0..self.width {
            for dy in 1..=dy_range {
                let left = self.field[row - dy][x];
                let right = self.field[row + dy - 1][x];
                if left != right {
                    return false;
                }
            }
        }
        true
    }

    fn fix_smudge(&self, row: usize, col: usize) -> Self {
        let mut new = self.clone();

        if new.field[row][col] == FieldType::Rock {
            new.field[row][col] = FieldType::Ash;
        } else {
            new.field[row][col] = FieldType::Rock;
        }

        new
    }
}

fn find_reflection(
    field: &Field,
    maybe_previous: Option<(usize, usize)>,
) -> Option<(usize, usize)> {
    for col in 1..field.width {
        let tentative = Some((0, col));
        if field.is_column_reflect(col) && tentative != maybe_previous {
            return tentative;
        }
    }
    for row in 1..field.height {
        let tentative = Some((row, 0));
        if field.is_row_reflect(row) && tentative != maybe_previous {
            return tentative;
        }
    }
    None
}

fn part_1(input: &[Field]) -> usize {
    input
        .iter()
        .map(|f| match find_reflection(f, None) {
            Some((row, col)) => row * 100 + col,
            None => 0,
        })
        .sum()
}

fn part_2(input: &[Field]) -> usize {
    input
        .iter()
        .map(|f| {
            let existing = find_reflection(f, None);
            for y in 0..f.height {
                for x in 0..f.width {
                    let new = f.fix_smudge(y, x);
                    if let Some((row, col)) = find_reflection(&new, existing) {
                        return row * 100 + col;
                    }
                }
            }
            panic!("No solution found");
        })
        .sum()
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
        assert_eq!(part_1(&input), 405);
        assert_eq!(part_2(&input), 400);
    }
}
