use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Clone)]
struct Board {
    size: usize,
    entries: Vec<usize>,
    mask: Vec<bool>,
}

impl Board {
    fn new(size: usize, entries: Vec<usize>) -> Board {
        assert_eq!(size * size, entries.len());
        Board {
            size,
            entries,
            mask: vec![false; size * size],
        }
    }

    fn parse<S: AsRef<str>>(board: &[S]) -> Board {
        let size = board.len();
        let entries: Vec<usize> = board
            .iter()
            .map(|row| {
                row.as_ref()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .flatten()
            .collect();
        Board::new(size, entries)
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for i in 0..self.size * self.size {
            if !self.mask[i] {
                score += self.entries[i];
            }
        }
        score
    }

    fn check_for_win(&self, idx: usize) -> bool {
        let x = idx % self.size;
        let y = (idx - x) / self.size;
        let mut row_won = true;
        for row in 0..self.size {
            if !self.mask[row * self.size + x] {
                row_won = false;
            }
        }
        let mut col_won = true;
        for col in 0..self.size {
            if !self.mask[y * self.size + col] {
                col_won = false;
            }
        }
        row_won || col_won
    }

    fn mark(&mut self, number: usize) -> bool {
        for i in 0..self.size * self.size {
            if self.entries[i] == number {
                self.mask[i] = true;
                return self.check_for_win(i);
            }
        }
        false
    }
}

fn read<R: Read>(io: R) -> (Vec<usize>, Vec<Board>) {
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines().filter_map(Result::ok).collect();
    let line = lines.iter().next().unwrap();
    let numbers = line
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    let mut last = 2;
    let mut boards = Vec::new();
    for idx in 2..lines.len() {
        if lines[idx].is_empty() {
            boards.push(Board::parse(&lines[last..idx]));
            last = idx + 1;
        }
    }
    boards.push(Board::parse(&lines[last..lines.len()]));
    (numbers, boards)
}

fn part_1(numbers: &[usize], boards: &mut [Board]) -> (usize, usize) {
    for number in numbers {
        for i in 0..boards.len() {
            if boards[i].mark(*number) {
                return (*number, i);
            }
        }
    }
    unreachable!();
}

fn part_2(numbers: &[usize], boards: &mut [Board]) -> (usize, usize) {
    let mut boards_won: HashSet<usize> = HashSet::new();
    for number in numbers {
        for i in 0..boards.len() {
            if !boards_won.contains(&i) {
                if boards[i].mark(*number) {
                    boards_won.insert(i);
                    if boards_won.len() == boards.len() {
                        return (*number, i);
                    }
                }
            }
        }
    }
    unreachable!();
}

fn main() {
    let (numbers, boards) = read(File::open("input.txt").unwrap());
    let mut part1_boards = boards.clone();
    let (winning_number, winning_board) = part_1(&numbers, &mut part1_boards);
    println!(
        "Board {} won at number {}, scoring {}",
        winning_board + 1,
        winning_number,
        part1_boards[winning_board].score() * winning_number
    );
    let mut part2_boards = boards.clone();
    let (last_winning_number, last_winning_board) = part_2(&numbers, &mut part2_boards);
    println!(
        "Last board to win is {} at number {}, scoring {}",
        last_winning_board + 1,
        last_winning_number,
        part2_boards[last_winning_board].score() * last_winning_number
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (numbers, mut boards) = read(File::open("test1.txt").unwrap());
        let (winning_number, winning_board) = part_1(&numbers, &mut boards);
        assert_eq!(winning_number, 24);
        assert_eq!(winning_board + 1, 3);
        assert_eq!(boards[winning_board].score(), 188);
    }

    #[test]
    fn test_part2() {
        let (numbers, mut boards) = read(File::open("test1.txt").unwrap());
        let (winning_number, winning_board) = part_2(&numbers, &mut boards);
        assert_eq!(winning_number, 13);
        assert_eq!(winning_board + 1, 2);
        assert_eq!(boards[winning_board].score(), 148);
    }
}
