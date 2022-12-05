// https://adventofcode.com/2022/day/5

use scanf::sscanf;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let br = BufReader::new(io);
    let mut istacks: Vec<String> = br.lines().filter_map(Result::ok).collect();
    let iinstructions = istacks.split_off(10);

    let mut stacks = vec![Vec::new(); 9];
    for h in (0..8).rev() {
        for i in 0..9 {
            let c = istacks[h].chars().nth(i * 4 + 1).unwrap();
            if c != ' ' {
                stacks[i].push(c)
            }
        }
    }

    let instructions = iinstructions
        .iter()
        .map(|line| {
            let mut count: usize = 0;
            let mut from: usize = 0;
            let mut to: usize = 0;
            sscanf!(line, "move {} from {} to {}", count, from, to).unwrap();
            Instruction {
                count,
                from: from - 1,
                to: to - 1,
            }
        })
        .collect::<Vec<_>>();

    (stacks, instructions)
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn part_1(mut stacks: Vec<Vec<char>>, instructions: &[Instruction]) -> String {
    for instruction in instructions {
        for _ in 0..instruction.count {
            let c = stacks[instruction.from].pop().unwrap();
            stacks[instruction.to].push(c);
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn part_2(mut stacks: Vec<Vec<char>>, instructions: &[Instruction]) -> String {
    for instruction in instructions {
        let split_pos = stacks[instruction.from].len() - instruction.count;
        let moved = stacks[instruction.from].split_off(split_pos);
        stacks[instruction.to].extend(moved);
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn main() {
    let (stacks, instructions) = read(File::open("input.txt").unwrap());
    let p1 = part_1(stacks.clone(), &instructions);
    println!("Part 1: {}", p1);
    let p2 = part_2(stacks, &instructions);
    println!("Part 2: {}", p2);
}
