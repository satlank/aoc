// https://adventofcode.com/2024/day/13

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<ClawMachine> {
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines().map_while(Result::ok).collect();
    let mut machines = Vec::new();
    let mut start = 0;
    while start < lines.len() {
        let end = start + 3;
        let machine = ClawMachine::from(&lines[start..end]);
        machines.push(machine);
        start = end + 1;
    }
    machines
}

#[derive(Debug)]
struct ClawMachine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

impl From<&[String]> for ClawMachine {
    fn from(input: &[String]) -> Self {
        let mut a = input[0][10..].split(", ");
        let a_x = &a.next().unwrap()[1..];
        let a_x = a_x.parse().unwrap();
        let a_y = &a.next().unwrap()[1..];
        let a_y = a_y.parse().unwrap();

        let mut b = input[1][10..].split(", ");
        let b_x = &b.next().unwrap()[1..];
        let b_x = b_x.parse().unwrap();
        let b_y = &b.next().unwrap()[1..];
        let b_y = b_y.parse().unwrap();

        let mut p = input[2][7..].split(", ");
        let p_x = p.next().unwrap()[2..].parse().unwrap();
        let p_y = p.next().unwrap()[2..].parse().unwrap();

        Self {
            button_a: (a_x, a_y),
            button_b: (b_x, b_y),
            prize: (p_x, p_y),
        }
    }
}

impl ClawMachine {
    // This is part of the brute force solution
    #[allow(dead_code)]
    fn calc_b_steps(&self, current: (usize, usize)) -> Option<usize> {
        let remaining = (self.prize.0 - current.0, self.prize.1 - current.1);
        if remaining.0 % self.button_b.0 == 0 && remaining.1 % self.button_b.1 == 0 {
            let x_steps = remaining.0 / self.button_b.0;
            let y_steps = remaining.1 / self.button_b.1;
            if x_steps == y_steps {
                Some(x_steps)
            } else {
                None
            }
        } else {
            None
        }
    }

    // This is part of the brute force solution
    #[allow(dead_code)]
    fn solve(&self, max_steps: usize) -> Option<(usize, usize)> {
        let mut a_steps = 0;
        let mut b_steps = 0;
        let mut pos = (0, 0);

        while a_steps < max_steps {
            if pos == self.prize {
                return Some((a_steps, b_steps));
            }
            if let Some(steps) = self.calc_b_steps(pos) {
                if steps > max_steps {
                    return None;
                }
                b_steps += steps;
                pos.0 += self.button_b.0 * steps;
                pos.1 += self.button_b.1 * steps;
            } else {
                a_steps += 1;
                pos.0 += self.button_a.0;
                pos.1 += self.button_a.1;
            }
        }
        None
    }

    // Better solution
    fn solve2(&self, conversion_error: usize) -> Option<(usize, usize)> {
        // Adjust to the actual prize position
        let target = (
            self.prize.0 + conversion_error,
            self.prize.1 + conversion_error,
        );

        // Simpler naming
        let ax = self.button_a.0 as i64;
        let ay = self.button_a.1 as i64;
        let bx = self.button_b.0 as i64;
        let by = self.button_b.1 as i64;
        let px = target.0 as i64;
        let py = target.1 as i64;

        // Solve the system of equations
        // px = a * ax + b * bx
        // py = a * ay + b * by
        //
        // which is equivalent to
        // b = b1 / b2 = (py * ax - px * ay) / (by * ax - bx * ay)
        // a = a1 / a2 = (px - b * bx) / ax

        // Get the number of b steps first
        let b1 = py * ax - px * ay;
        let b2 = by * ax - bx * ay; // This is the determinant of the matrix
        if b2 == 0 {
            // No solution if the determinant is 0
            return None;
        }
        // Check if the number of b steps is an integer
        if b1 % b2 != 0 {
            return None;
        }
        let b = b1 / b2;

        // Now lets get the number of a steps
        let a1 = px - b * bx;
        let a2 = ax;

        // Check if the number of a steps is an integer
        if a1 % a2 != 0 {
            return None;
        }
        let a = a1 / a2;

        // Must take only positive steps
        if a < 0 || b < 0 {
            return None;
        }

        // Yay!
        Some((a as usize, b as usize))
    }
}

fn part_1(input: &[ClawMachine]) -> usize {
    input
        .iter()
        .filter_map(|m| m.solve2(0))
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn part_2(input: &[ClawMachine]) -> usize {
    let conversion_error = 10_000_000_000_000;
    input
        .iter()
        .filter_map(|m| m.solve2(conversion_error))
        .map(|(a, b)| a * 3 + b)
        .sum()
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
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input), 480);
        assert_eq!(part_2(&input), 875318608908); // NB: This was not given in the example
    }
}
