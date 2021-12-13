// https://adventofcode.com/2021/day/13

use std::convert::Infallible;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use ndarray::prelude::*;

type Field = Array2<usize>;

#[derive(Debug, Eq, PartialEq)]
enum Op {
    FoldX(usize),
    FoldY(usize),
}

impl FromStr for Op {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let p = s.split("=").collect::<Vec<&str>>();
        let val = p[1].parse::<usize>().unwrap();
        if p[0].ends_with("x") {
            Ok(Op::FoldX(val))
        } else {
            Ok(Op::FoldY(val))
        }
    }
}

fn read<R: Read>(io: R) -> (Vec<(usize, usize)>, Vec<Op>) {
    let lines = BufReader::new(io).lines();
    let mut points = Vec::new();
    let mut ops = Vec::new();
    let mut part2 = false;
    for line in lines {
        let line = line.unwrap();
        if !part2 {
            if line.is_empty() {
                part2 = true;
            } else {
                let sp = line.split(",").collect::<Vec<&str>>();
                points.push((
                    sp[0].parse::<usize>().unwrap(),
                    sp[1].parse::<usize>().unwrap(),
                ));
            }
        } else {
            ops.push(line.parse::<Op>().unwrap());
        }
    }
    (points, ops)
}

fn get_dims(pts: &[(usize, usize)]) -> (usize, usize) {
    let mut dim = (0, 0);
    pts.iter().for_each(|p| {
        if p.0 > dim.0 {
            dim.0 = p.0;
        }
        if p.1 > dim.1 {
            dim.1 = p.1;
        }
    });
    if dim.1 % 2 == 1 {
        dim.1 += 1;
    }
    if dim.0 % 2 == 1 {
        dim.0 += 1;
    }
    (dim.1 + 1, dim.0 + 1) // both dimensions MUST be odd
}

fn to_field(pts: &[(usize, usize)]) -> Field {
    let dims = get_dims(&pts);
    let mut field = Field::from_elem(dims, 0);
    pts.iter().for_each(|p| {
        field[[p.1, p.0]] = 1;
    });
    field
}

fn count_dots(field: &Field) -> usize {
    field.iter().filter(|&&x| x > 0).count()
}

fn fold_left(field: &Field, col: usize) -> Field {
    assert!(field.dim().1 / 2 == col);
    assert_eq!(field.slice(s![.., col]).sum(), 0);
    let new_dim = (field.dim().0, col);
    let mut folded = Field::from_elem(new_dim, 999);
    for x in 0i32..col as i32 {
        folded
            .slice_mut(s![.., x])
            .assign(&(&field.slice(s![.., x]) + &field.slice(s![.., -1 - x])));
    }
    folded
}

fn fold_up(field: &Field, row: usize) -> Field {
    assert!(field.dim().0 / 2 == row);
    assert_eq!(field.slice(s![row, ..]).sum(), 0);
    let new_dim = (row, field.dim().1);
    let mut folded = Field::from_elem(new_dim, 999);
    for y in 0i32..row as i32 {
        folded
            .slice_mut(s![y, ..])
            .assign(&(&field.slice(s![y, ..]) + &field.slice(s![-1 - y, ..])));
    }
    folded
}

fn fold(field: &Field, op: &Op) -> Field {
    match op {
        Op::FoldX(val) => fold_left(field, *val),
        Op::FoldY(val) => fold_up(field, *val),
    }
}

fn part_1(pts: &[(usize, usize)], ops: &[Op]) -> usize {
    let field = to_field(&pts);
    let field = fold(&field, &ops[0]);
    count_dots(&field)
}

fn part_2(pts: &[(usize, usize)], ops: &[Op]) -> Field {
    let mut field = to_field(&pts);
    for op in ops {
        field = fold(&field, op);
    }
    field
}

fn main() {
    let (points, ops) = read(File::open("input.txt").unwrap());
    let dots = part_1(&points, &ops);
    println!("Visible dots after first fold: {}", dots);
    let folded = part_2(&points, &ops);
    for y in 0..folded.dim().0 {
        for x in 0..folded.dim().1 {
            print!("{}", if folded[[y, x]] > 0 { 'X' } else { ' ' });
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let (points, ops) = read(File::open("test1.txt").unwrap());
        assert_eq!(points.len(), 18);
        assert_eq!(ops, vec![Op::FoldY(7), Op::FoldX(5)]);
    }

    #[test]
    fn test_dims() {
        let (points, _) = read(File::open("test1.txt").unwrap());
        let dims = get_dims(&points);
        assert_eq!(dims, (15, 11));
    }

    #[test]
    fn test_field() {
        let (points, _) = read(File::open("test1.txt").unwrap());
        let field = to_field(&points);
        assert_eq!(field.dim(), (15, 11));
        assert_eq!(field[[0, 9]], 1);
        assert_eq!(field[[0, 10]], 0);
        assert_eq!(count_dots(&field), points.len());
    }

    #[test]
    fn test_1() {
        let (points, ops) = read(File::open("test1.txt").unwrap());
        let field = to_field(&points);
        let field = fold(&field, &ops[0]);
        let field = fold(&field, &ops[1]);
        assert_eq!(count_dots(&field), 16);
    }

    #[test]
    fn test_part_1() {
        let (points, ops) = read(File::open("test1.txt").unwrap());
        assert_eq!(part_1(&points, &ops), 17);
    }
}
