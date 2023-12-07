// https://adventofcode.com/2023/day/7

use std::fs::File;

mod part1;
mod part2;

fn part_1(input: &[(part1::Hand, usize)]) -> usize {
    let mut ordered: Vec<&(part1::Hand, usize)> = input.iter().collect();
    ordered.sort_by(|a, b| a.0.cmp(&b.0));
    ordered
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum()
}

fn part_2(input: &[(part2::Hand, usize)]) -> usize {
    let mut ordered: Vec<&(part2::Hand, usize)> = input.iter().collect();
    ordered.sort_by(|a, b| a.0.cmp(&b.0));
    ordered
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum()
}

fn main() {
    let input = part1::read(File::open("input.txt").unwrap());
    let p1 = part_1(&input);
    println!("Part 1: {}", p1);
    let input = part2::read(File::open("input.txt").unwrap());
    let p2 = part_2(&input);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = part1::read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input), 6440);
        let input = part2::read(File::open("example1.txt").unwrap());
        assert_eq!(part_2(&input), 5905);
        assert!(false);
    }
}
