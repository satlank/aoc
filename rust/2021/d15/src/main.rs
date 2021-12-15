// https://adventofcode.com/2021/day/15

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use ndarray::prelude::*;

type Field = Array2<u32>;
type BoolField = Array2<bool>;

fn read<R: Read>(io: R) -> Field {
    let br = BufReader::new(io);
    let mut dim = (0, 0);
    let cells = Array::from_iter(
        br.lines()
            .enumerate()
            .map(|(y, line)| {
                line.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        dim.1 = usize::max(dim.1, x);
                        dim.0 = usize::max(dim.0, y);
                        c.to_digit(10).unwrap()
                    })
                    .collect::<Vec<u32>>()
            })
            .flatten(),
    );
    cells.into_shape((dim.0 + 1, dim.1 + 1)).unwrap()
}

fn shortest_path(field: &Field) -> u32 {
    let mut distances = Field::from_elem(field.dim(), u32::MAX);
    let mut completed = BoolField::from_elem(field.dim(), false);
    let mut cur_min_dist = 0;
    let dims = field.dim();
    distances[[0, 0]] = cur_min_dist;
    loop {
        for y in 0..dims.0 {
            for x in 0..dims.1 {
                if distances[[y, x]] == cur_min_dist {
                    if x > 0 {
                        let d = distances[[y, x]] + field[[y, x - 1]];
                        distances[[y, x - 1]] = u32::min(d, distances[[y, x - 1]]);
                    }
                    if x + 1 < dims.1 {
                        let d = distances[[y, x]] + field[[y, x + 1]];
                        distances[[y, x + 1]] = u32::min(d, distances[[y, x + 1]]);
                    }
                    if y > 0 {
                        let d = distances[[y, x]] + field[[y - 1, x]];
                        distances[[y - 1, x]] = u32::min(d, distances[[y - 1, x]]);
                    }
                    if y + 1 < dims.0 {
                        let d = distances[[y, x]] + field[[y + 1, x]];
                        distances[[y + 1, x]] = u32::min(d, distances[[y + 1, x]]);
                    }
                    completed[[y, x]] = true;
                }
            }
        }
        if completed[[dims.0 - 1, dims.1 - 1]] {
            break;
        }
        cur_min_dist = u32::MAX;
        for y in 0..dims.0 {
            for x in 0..dims.1 {
                if !completed[[y, x]] && distances[[y, x]] < cur_min_dist {
                    cur_min_dist = distances[[y, x]];
                }
            }
        }
    }
    cur_min_dist
}

fn expand_map(field: &Field) -> Field {
    let tile_dim = field.dim();
    let mut res = Field::from_elem((tile_dim.0 * 5, tile_dim.1 * 5), 0);
    let mut initial = res.slice_mut(s![0..tile_dim.0, 0..tile_dim.1]);
    initial.assign(field);
    for i in 1..5 {
        let prev_tile = res
            .slice(s![0..tile_dim.0, (i - 1) * tile_dim.1..i * tile_dim.1])
            .into_owned();
        let mut this_tile = res.slice_mut(s![0..tile_dim.0, i * tile_dim.1..(i + 1) * tile_dim.1]);
        this_tile.assign(&prev_tile);
        this_tile.mapv_inplace(|x| if x + 1 > 9 { 1 } else { x + 1 });
    }
    for i in 1..5 {
        let prev_tile = res
            .slice(s![(i - 1) * tile_dim.0..i * tile_dim.0, 0..5 * tile_dim.1])
            .into_owned();
        let mut this_tile =
            res.slice_mut(s![i * tile_dim.0..(i + 1) * tile_dim.0, 0..5 * tile_dim.0]);
        this_tile.assign(&prev_tile);
        this_tile.mapv_inplace(|x| if x + 1 > 9 { 1 } else { x + 1 });
    }
    res
}

fn part_1(field: &Field) -> u32 {
    shortest_path(field)
}

fn part_2(field: &Field) -> u32 {
    let big = expand_map(&field);
    shortest_path(&big)
}

fn main() {
    let field = read(File::open("input.txt").unwrap());
    let shortest_path_length = part_1(&field);
    println!("Shortest path length is: {}", shortest_path_length);
    let shortest_path_length_real = part_2(&field);
    println!(
        "Shortest path length is in the real cave is: {}",
        shortest_path_length_real
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_reading() {
        let cells = read(File::open("test1.txt").unwrap());
        assert_eq!(cells.dim(), (10, 10));
        assert_eq!(cells[[0, 0]], 1);
        assert_eq!(cells[[2, 3]], 6);
        assert_eq!(cells[[9, 9]], 1);
    }

    #[test]
    fn test_part_1() {
        let cells = read(File::open("test1.txt").unwrap());
        assert_eq!(part_1(&cells), 40);
    }

    #[test]
    fn test_part_2() {
        let cells = read(File::open("test1.txt").unwrap());
        assert_eq!(part_2(&cells), 315);
    }
}
