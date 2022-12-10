// https://adventofcode.com/2022/day/10

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<Op> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .map(|line| {
            if line.starts_with("noop") {
                Op::Noop
            } else {
                Op::Addx(line.split_once(' ').unwrap().1.parse::<i32>().unwrap())
            }
        })
        .collect()
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Noop,
    Addx(i32),
}

fn part_1(input: &[Op]) -> i32 {
    let mut x = 1;
    let mut cycle = 0;
    let mut ops = input.iter();
    let mut executing: (Op, i32) = (Op::Noop, 1);
    let output_cycles: HashSet<i32> = vec![20, 60, 100, 140, 180, 220].into_iter().collect();
    let mut res = 0;
    loop {
        cycle += 1;
        match executing {
            (Op::Noop, _) => {
                if let Some(next) = ops.next() {
                    executing = (*next, 1);
                } else {
                    break;
                }
            }
            (Op::Addx(_), 1) => {
                executing.1 += 1;
            }
            (Op::Addx(incr), 2) => {
                x += incr;
                if let Some(next) = ops.next() {
                    executing = (*next, 1);
                } else {
                    break;
                }
            }
            _ => unreachable!(),
        };
        if output_cycles.contains(&cycle) {
            res += cycle * x;
        }
    }
    res
}

fn part_2(input: &[Op]) -> String {
    let mut x = 1;
    let mut cycle = 0;
    let mut ops = input.iter();
    let mut executing: (Op, i32) = (Op::Noop, 1);
    let mut crt = String::with_capacity(40 * 6);
    loop {
        cycle += 1;
        match executing {
            (Op::Noop, _) => {
                if let Some(next) = ops.next() {
                    executing = (*next, 1);
                } else {
                    break;
                }
            }
            (Op::Addx(_), 1) => {
                executing.1 += 1;
            }
            (Op::Addx(incr), 2) => {
                x += incr;
                if let Some(next) = ops.next() {
                    executing = (*next, 1);
                } else {
                    break;
                }
            }
            _ => unreachable!(),
        };
        let dist = ((cycle - 1) % 40) - x;
        if (-1..=1).contains(&dist) {
            crt.push('#');
        } else {
            crt.push('.');
        }
    }
    crt
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input);
    println!("Part 1: {}", p1);
    let p2 = part_2(&input);
    println!(
        "Part 2: \n{}\n{}\n{}\n{}\n{}\n{}",
        &p2[0..40],
        &p2[40..80],
        &p2[80..120],
        &p2[120..160],
        &p2[160..200],
        &p2[200..240]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input), 13140);
        let line1 = "##..##..##..##..##..##..##..##..##..##..";
        let line2 = "###...###...###...###...###...###...###.";
        let line3 = "####....####....####....####....####....";
        let line4 = "#####.....#####.....#####.....#####.....";
        let line5 = "######......######......######......####";
        let line6 = "#######.......#######.......#######.....";
        let res = part_2(&input);
        assert_eq!(&res[0..40], line1);
        assert_eq!(&res[40..80], line2);
        assert_eq!(&res[80..120], line3);
        assert_eq!(&res[120..160], line4);
        assert_eq!(&res[160..200], line5);
        assert_eq!(&res[200..240], line6);
    }
}
