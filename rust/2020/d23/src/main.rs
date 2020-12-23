use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn read<R: Read>(io: R) -> Result<Vec<u32>, Error> {
    let br = BufReader::new(io);
    let line = br.lines().nth(0).unwrap()?;
    Ok(line.chars().map(|c| c.to_digit(10).unwrap()).collect())
}

fn in_removed(val: u32, removed: &[u32]) -> bool {
    val == removed[0] || val == removed[1] || val == removed[2]
}

fn play_game(cups: &Vec<u32>, rounds: usize) -> Vec<u32> {
    let min = cups.iter().min().unwrap().clone();
    let max = cups.iter().max().unwrap().clone();
    assert_eq!(max, cups.len() as u32);
    assert_eq!(min, 1);
    let mut cup_neighbours = cups.clone();
    for i in 0..cups.len() {
        cup_neighbours[(cups[i] - 1) as usize] = cups[(i + 1) % cups.len()];
    }
    let mut current_cup = cups[0];
    let mut removed: [u32; 3] = [0; 3];
    for _round in 1..=rounds {
        //println!("-- move {} --", round);
        //println!("current cup: {}", current_cup);
        removed[0] = cup_neighbours[(current_cup - 1) as usize];
        removed[1] = cup_neighbours[(removed[0] - 1) as usize];
        removed[2] = cup_neighbours[(removed[1] - 1) as usize];
        //println!("removed: {:?}", removed);
        let mut destination_cup = current_cup - 1;
        while in_removed(destination_cup, &removed) || destination_cup < min {
            if destination_cup < min {
                destination_cup = max;
            } else {
                destination_cup -= 1;
            }
        }
        //println!("destination: {}", destination_cup);
        let orig_neighbour = cup_neighbours[(destination_cup - 1) as usize];
        let orig_removed_neighbour = cup_neighbours[(removed[2] - 1) as usize];
        cup_neighbours[(destination_cup - 1) as usize] = removed[0];
        cup_neighbours[(removed[2] - 1) as usize] = orig_neighbour;
        cup_neighbours[(current_cup - 1) as usize] = orig_removed_neighbour;
        current_cup = orig_removed_neighbour;
    }
    cup_neighbours
}

fn part1(mut cups: Vec<u32>) -> String {
    cups = play_game(&cups, 100);
    let mut out: String = String::new();
    let mut current_cup = 1u32;
    let mut cnt = 0;
    while cnt < cups.len() - 1 {
        current_cup = cups[(current_cup - 1) as usize];
        out.push(char::from_digit(current_cup, 10).unwrap());
        cnt += 1;
    }
    out
}

fn part2(mut cups: Vec<u32>) -> u64 {
    let max = cups.iter().max().unwrap().clone();
    cups.reserve(1_000_000);
    for i in max + 1..=1_000_000 {
        cups.push(i as u32);
    }
    cups = play_game(&cups, 10_000_000);
    let first = cups[0];
    let second = cups[(first - 1) as usize];
    first as u64 * second as u64
}

fn main() -> Result<(), Error> {
    let cups = read(File::open("input.txt")?)?;
    println!("Initial cups: {:?}", cups);
    println!("Cups after 100 rounds in part 1: {}", part1(cups.clone()));
    println!("Product of labels after 10M moves: {}", part2(cups.clone()));
    Ok(())
}
