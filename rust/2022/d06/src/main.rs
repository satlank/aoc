// https://adventofcode.com/2022/day/6

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<char> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .nth(0)
        .unwrap()
        .chars()
        .collect()
}

fn part_1(stream: &[char]) -> usize {
    for i in 3..stream.len() {
        if stream[i] != stream[i - 1]
            && stream[i] != stream[i - 2]
            && stream[i] != stream[i - 3]
            && stream[i - 1] != stream[i - 2]
            && stream[i - 1] != stream[i - 3]
            && stream[i - 2] != stream[i - 3]
        {
            return i + 1;
        }
    }
    unreachable!()
}

fn part_2(stream: &[char]) -> usize {
    let mut pos = 14;
    loop {
        if stream[pos-14..pos].iter().collect::<HashSet<_>>().len() == 14 {
            return pos;
        }
        pos += 1;
    }
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
    fn examples_part_1() {
        assert_eq!(
            part_1(&"bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect::<Vec<_>>()),
            5
        );
        assert_eq!(
            part_1(&"nppdvjthqldpwncqszvftbrmjlhg".chars().collect::<Vec<_>>()),
            6
        );
        assert_eq!(
            part_1(
                &"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
                    .chars()
                    .collect::<Vec<_>>()
            ),
            10
        );
        assert_eq!(
            part_1(
                &"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
                    .chars()
                    .collect::<Vec<_>>()
            ),
            11
        );
    }

    #[test]
    fn examples_part_2() {
        assert_eq!(
            part_2(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect::<Vec<_>>()),
            19
        );
        assert_eq!(
            part_2(&"bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect::<Vec<_>>()),
            23
        );
        assert_eq!(
            part_2(&"nppdvjthqldpwncqszvftbrmjlhg".chars().collect::<Vec<_>>()),
            23
        );
        assert_eq!(
            part_2(
                &"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
                    .chars()
                    .collect::<Vec<_>>()
            ),
            29
        );
        assert_eq!(
            part_2(
                &"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
                    .chars()
                    .collect::<Vec<_>>()
            ),
            26
        );
    }
}
