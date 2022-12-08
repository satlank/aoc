// https://adventofcode.com/2022/day/8

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<Vec<u32>> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_1(grid: &[Vec<u32>]) -> usize {
    let dim_x = grid[0].len();
    let dim_y = grid.len();
    let mut total_visible = dim_x * 2 + dim_y * 2 - 4;

    for y in 1..dim_y - 1 {
        for x in 1..dim_x - 1 {
            let mut visible_right = true;
            for xx in x + 1..dim_x {
                if grid[y][x] <= grid[y][xx] {
                    visible_right = false;
                    break;
                }
            }
            if visible_right {
                total_visible += 1;
                continue;
            }

            let mut visible_left = true;
            for xx in (0..x).rev() {
                if grid[y][x] <= grid[y][xx] {
                    visible_left = false;
                    break;
                }
            }
            if visible_left {
                total_visible += 1;
                continue;
            }

            let mut visible_bottom = true;
            for yy in y + 1..dim_y {
                if grid[y][x] <= grid[yy][x] {
                    visible_bottom = false;
                    break;
                }
            }
            if visible_bottom {
                total_visible += 1;
                continue;
            }

            let mut visible_top = true;
            for yy in (0..y).rev() {
                if grid[y][x] <= grid[yy][x] {
                    visible_top = false;
                    break;
                }
            }
            if visible_top {
                total_visible += 1;
                continue;
            }
        }
    }
    total_visible
}

fn part_2(grid: &[Vec<u32>]) -> usize {
    let dim_x = grid[0].len();
    let dim_y = grid.len();
    let mut scenic_score = 0;

    for y in 1..dim_y - 1 {
        for x in 1..dim_x - 1 {
            let mut visible_right = 0;
            for xx in x + 1..dim_x {
                visible_right += 1;
                if grid[y][x] <= grid[y][xx] {
                    break;
                }
            }

            let mut visible_left = 0;
            for xx in (0..x).rev() {
                visible_left += 1;
                if grid[y][x] <= grid[y][xx] {
                    break;
                }
            }

            let mut visible_bottom = 0;
            for yy in y + 1..dim_y {
                visible_bottom += 1;
                if grid[y][x] <= grid[yy][x] {
                    break;
                }
            }

            let mut visible_top = 0;
            for yy in (0..y).rev() {
                visible_top += 1;
                if grid[y][x] <= grid[yy][x] {
                    break;
                }
            }

            let tentative_scenic_score =
                visible_top * visible_bottom * visible_left * visible_right;
            if tentative_scenic_score > scenic_score {
                scenic_score = tentative_scenic_score;
            }
        }
    }
    scenic_score
}

fn main() {
    let grid = read(File::open("input.txt").unwrap());
    let p1 = part_1(&grid);
    println!("Part 1: {}", p1);
    let p2 = part_2(&grid);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&grid), 21);
        assert_eq!(part_2(&grid), 8);
    }
}
