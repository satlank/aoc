// https://adventofcode.com/2021/day/17

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[macro_use]
extern crate scan_fmt;

fn read<R: Read>(io: R) -> ((i32, i32), (i32, i32)) {
    let br = BufReader::new(io);
    let line = br.lines().nth(0).unwrap().unwrap();
    let (x0, x1, y0, y1) = scan_fmt!(
        &line,
        "target area: x={d}..{d}, y={d}..{d}",
        i32,
        i32,
        i32,
        i32
    )
    .unwrap();
    ((x0, y0), (x1, y1))
}

fn part_1(target: &((i32, i32), (i32, i32))) -> i32 {
    let max = i32::max(i32::abs(target.0 .1), i32::abs(target.1 .1)) - 1;
    max * (max + 1) / 2
}

fn hits(speed: (i32, i32), target: &((i32, i32), (i32, i32))) -> bool {
    let mut pos = (0i32, 0i32);
    let mut cur_speed = speed;
    loop {
        pos.0 += cur_speed.0;
        pos.1 += cur_speed.1;
        cur_speed.0 = if cur_speed.0 > 0 { cur_speed.0 - 1 } else { 0 };
        cur_speed.1 -= 1;
        if pos.0 >= target.0 .0
            && pos.0 <= target.1 .0
            && pos.1 >= target.0 .1
            && pos.1 <= target.1 .1
        {
            return true;
        }
        if pos.0 > target.1 .0 || pos.1 < target.0 .1 {
            return false;
        }
    }
}

fn part_2(target: &((i32, i32), (i32, i32))) -> usize {
    let max_y = i32::max(i32::abs(target.0 .1), i32::abs(target.1 .1)) - 1;
    let min_y = -(max_y + 1);
    let max_x = target.1 .0;
    let min_x = 1;

    (min_x..=max_x)
        .map(|x| (min_y..=max_y).filter(|&y| hits((x, y), target)).count())
        .sum()
}

fn main() {
    let target = read(File::open("input.txt").unwrap());
    let max_y_height = part_1(&target);
    println!("Mx y height to reach target: {}", max_y_height);
    let no_speeds = part_2(&target);
    println!("Number of speeds hitting the target: {}", no_speeds);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let ((x0, y0), (x1, y1)) = read(File::open("test1.txt").unwrap());
        assert_eq!((x0, x1), (20, 30));
        assert_eq!((y0, y1), (-10, -5));
    }

    #[test]
    fn test_part_1() {
        let target = read(File::open("test1.txt").unwrap());
        assert_eq!(part_1(&target), 45);
    }

    #[test]
    fn test_part_2() {
        let target = read(File::open("test1.txt").unwrap());
        assert_eq!(part_2(&target), 112);
    }
}
