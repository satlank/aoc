// https://adventofcode.com/2022/day/20

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<i64> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn shuffle(input: Vec<i64>, factor: i64, rounds: usize) -> Vec<i64> {
    let original_len = input.len();
    let mut output = input
        .iter()
        .enumerate()
        .map(|(idx, val)| (idx, *val * factor))
        .collect::<Vec<_>>();
    for _ in 0..rounds {
        for cnt in 0..original_len {
            let cur_index = output.iter().position(|(idx, _)| *idx == cnt).unwrap();
            let (orig_indx, removed) = output.remove(cur_index);
            assert_eq!(orig_indx, cnt);
            assert_eq!(removed, input[cnt] * factor);
            let shifted_index = cur_index as i64 + removed;
            let new_index = shifted_index.rem_euclid(original_len as i64 - 1) as usize;
            output.insert(new_index, (orig_indx, removed));
        }
    }
    output.into_iter().map(|(_, val)| val).collect()
}

fn part_1(input: Vec<i64>) -> i64 {
    let output = shuffle(input, 1, 1);
    let zero = output.iter().position(|v| *v == 0).unwrap();
    let a = output[(zero + 1000) % output.len()];
    let b = output[(zero + 2000) % output.len()];
    let c = output[(zero + 3000) % output.len()];
    println!("{} {} {}", a, b, c);
    a + b + c
}

fn part_2(input: Vec<i64>) -> i64 {
    let output = shuffle(input, 811589153, 10);
    let zero = output.iter().position(|v| *v == 0).unwrap();
    //println!("{:?}", output);
    let a = output[(zero + 1000) % output.len()];
    let b = output[(zero + 2000) % output.len()];
    let c = output[(zero + 3000) % output.len()];
    println!("{} {} {}", a, b, c);
    a + b + c
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(input.clone());
    println!("Part 1: {}", p1);
    let p2 = part_2(input);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(input.clone()), 3);
        assert_eq!(part_2(input), 1623178306);
    }
}
