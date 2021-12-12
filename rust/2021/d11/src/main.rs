// https://adventofcode.com/2021/day/11

use ndarray::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

type Field = Array2<u8>;

fn read<R: Read>(io: R) -> Field {
    let br = BufReader::new(io);

    let mut n = 0;
    let a = Array::from_iter(
        br.lines()
            .map(|line| {
                n += 1;
                line.unwrap().chars().collect::<Vec<char>>()
            })
            .flatten()
            .map(|c| c.to_digit(10).unwrap() as u8),
    );
    let a = a.into_shape((n, n)).unwrap();
    let mut map = Field::from_elem((n + 2, n + 2), 0);
    map.slice_mut(s![1..-1, 1..-1]).assign(&a);
    map
}

fn step(field: &mut Field) -> usize {
    let mut flashes = 0;
    let n = field.dim().0;
    let mut temp = field.clone();
    loop {
        let new_incr: Array1<u8> = temp
            .windows((3, 3))
            .into_iter()
            .map(|w| {
                (1 + w.iter().filter(|&&x| x > 9).count() - if w[[1, 1]] > 9 { 1 } else { 0 }) as u8
            })
            .collect();
        let new_incr = new_incr.into_shape((n - 2, n - 2)).unwrap();
        let mut internal = temp.slice_mut(s![1..-1, 1..-1]);
        internal.assign(&(&field.slice(s![1..-1, 1..-1]) + &new_incr));
        let new_flashes = internal.iter().filter(|&&x| x > 9).count();
        if new_flashes != flashes {
            flashes = new_flashes;
        } else {
            field.assign(&temp);
            field.mapv_inplace(|x| if x > 9 { 0 } else { x });
            break;
        }
    }
    flashes
}

fn part_1(field: &mut Field) -> usize {
    (0..100).map(|_| step(field)).sum()
}

fn part_2(field: &mut Field) -> usize {
    let mut cnt = 0;
    loop {
        cnt += 1;
        if step(field) == 100 {
            break;
        }
    }
    cnt
}

fn main() {
    let field = read(File::open("input.txt").unwrap());
    let flashes = part_1(&mut field.clone());
    println!("Total flashes after 100 steps: {}", flashes);
    let synchronized_after = part_2(&mut field.clone());
    println!("Synchronized after {} steps", synchronized_after);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading() {
        let field = read(File::open("test1.txt").unwrap());
        assert_eq!(field[[1, 1]], 5);
        assert_eq!(field[[2, 6]], 5);
        assert_eq!(field[[6, 2]], 1);
    }

    #[test]
    fn ndarray_things() {
        let mut field = read(File::open("test1.txt").unwrap());
        let flashes: usize = (0..2).map(|_| step(&mut field)).sum();
        assert_eq!(flashes, 35);
    }

    #[test]
    fn test_part_1() {
        let mut field = read(File::open("test1.txt").unwrap());
        assert_eq!(part_1(&mut field), 1656);
    }

    #[test]
    fn test_part_2() {
        let mut field = read(File::open("test1.txt").unwrap());
        assert_eq!(part_2(&mut field), 195);
    }
}
