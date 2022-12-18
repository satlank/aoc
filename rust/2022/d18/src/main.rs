// https://adventofcode.com/2022/day/18

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> HashSet<(i64, i64, i64)> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .map(|line| {
            let coords = line
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .take(3)
                .collect::<Vec<i64>>();
            (coords[0], coords[1], coords[2])
        })
        .collect()
}

fn get_neighbours((x, y, z): (i64, i64, i64)) -> HashSet<(i64, i64, i64)> {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
    .into_iter()
    .collect()
}

fn get_neighbours_clipped(
    (x, y, z): (i64, i64, i64),
    (min_x, min_y, min_z): (i64, i64, i64),
    (max_x, max_y, max_z): (i64, i64, i64),
) -> HashSet<(i64, i64, i64)> {
    get_neighbours((x, y, z))
        .into_iter()
        .filter_map(|c| {
            if (x < min_x || x > max_x) || (y < min_y || y > max_y) || (z < min_z || z > max_z) {
                None
            } else {
                Some(c)
            }
        })
        .collect()
}

fn part_1(input: &HashSet<(i64, i64, i64)>) -> usize {
    input
        .iter()
        .map(|cell| {
            let n = get_neighbours(*cell);
            6 - input.intersection(&n).count()
        })
        .sum()
}

fn part_2(input: &HashSet<(i64, i64, i64)>) -> usize {
    let enclosing_min = (
        input.iter().map(|c| c.0).min().unwrap() - 1,
        input.iter().map(|c| c.1).min().unwrap() - 1,
        input.iter().map(|c| c.2).min().unwrap() - 1,
    );
    let enclosing_max = (
        input.iter().map(|c| c.0).max().unwrap() + 1,
        input.iter().map(|c| c.1).max().unwrap() + 1,
        input.iter().map(|c| c.2).max().unwrap() + 1,
    );

    let mut outside: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut new_outside: HashSet<(i64, i64, i64)> =
        [enclosing_min, enclosing_max].into_iter().collect();
    loop {
        outside.extend(&new_outside);
        new_outside = new_outside
            .into_iter()
            .flat_map(|c| {
                let n = get_neighbours_clipped(c, enclosing_min, enclosing_max);
                n.into_iter()
                    .filter_map(|no| if input.contains(&no) { None } else { Some(no) })
            })
            .collect();
        if new_outside.difference(&outside).count() == 0 {
            break;
        }
    }
    input
        .iter()
        .map(|c| {
            let n = get_neighbours(*c);
            n.iter()
                .filter_map(|nc| if outside.contains(nc) { Some(()) } else { None })
                .count()
        })
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
        assert_eq!(part_1(&input), 64);
        assert_eq!(part_2(&input), 58);
    }
}
