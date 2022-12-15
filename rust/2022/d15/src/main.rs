// https://adventofcode.com/2022/day/15

use scanf::sscanf;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Sensor {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Beacon {
    x: i64,
    y: i64,
}

fn read<R: Read>(io: R) -> Vec<(Sensor, Beacon)> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .map(|line| {
            let mut sens_x = 0;
            let mut sens_y = 0;
            let mut beac_x = 0;
            let mut beac_y = 0;
            sscanf!(
                &line,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                sens_x,
                sens_y,
                beac_x,
                beac_y
            )
            .unwrap();

            (
                Sensor {
                    x: sens_x,
                    y: sens_y,
                },
                Beacon {
                    x: beac_x,
                    y: beac_y,
                },
            )
        })
        .collect()
}

fn get_cells_in_row(sensor: &Sensor, beacon: &Beacon, y: i64) -> Option<HashSet<i64>> {
    let beacon_distance = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
    if (sensor.y - y).abs() > beacon_distance {
        None
    } else {
        let flex = beacon_distance - (sensor.y - y).abs();
        Some(
            (-flex..=flex)
                .filter_map(|i| {
                    let x = sensor.x - i;
                    if (sensor.x == x && sensor.y == y) || (beacon.x == x && beacon.y == y) {
                        None
                    } else {
                        Some(x)
                    }
                })
                .collect(),
        )
    }
}

fn part_1(input: &[(Sensor, Beacon)], y: i64) -> usize {
    let cells = input
        .iter()
        .filter_map(|(s, b)| {
            let cells = get_cells_in_row(s, b, y);
            cells
        })
        .flatten()
        .collect::<HashSet<i64>>();
    cells.len()
}

fn part_2(input: &[(Sensor, Beacon)]) -> usize {
    input.len()
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input, 2_000_000);
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
        assert_eq!(part_1(&input, 10), 26);
        //assert_eq!(part_2(&input), 1);
    }
}
