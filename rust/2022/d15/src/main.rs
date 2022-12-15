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

struct Cover {
    sensor_ranges: Vec<((i64, i64), i64)>,
}

#[inline(always)]
fn manhattan(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

impl From<&[(Sensor, Beacon)]> for Cover {
    fn from(data: &[(Sensor, Beacon)]) -> Self {
        Self {
            sensor_ranges: data
                .iter()
                .map(|(s, b)| ((s.x, s.y), manhattan(s.x, s.y, b.x, b.y)))
                .collect(),
        }
    }
}

impl Cover {
    fn maybe_next_x(&self, x: i64, y: i64) -> Option<i64> {
        for ((bx, by), d) in &self.sensor_ranges {
            let m = manhattan(x, y, *bx, *by);
            if m <= *d {
                let x_dist = x - *bx;
                let new_x = if x_dist >= 0 {
                    Some(x + (*d - m) + 1)
                } else {
                    // we are at left side of exclusion zone, so can move to right without
                    // increasing distance to sensor
                    Some(x + x_dist.abs() * 2 + (*d - m) + 1)
                };
                //println!("{} {} in range of {} {} ({} <= {}, skipping to {:?})", x, y, bx, by, m, d, new_x);
                return new_x;
            }
        }
        None
    }
}

fn part_1(input: &[(Sensor, Beacon)], y: i64) -> usize {
    let cells = input
        .iter()
        .filter_map(|(s, b)| get_cells_in_row(s, b, y))
        .flatten()
        .collect::<HashSet<i64>>();
    cells.len()
}

fn part_2(input: &[(Sensor, Beacon)], min: i64, max: i64) -> usize {
    let cover = Cover::from(input);
    for y in min..=max {
        let mut x = min;
        while x <= max {
            if let Some(new_x) = cover.maybe_next_x(x, y) {
                //println!("New x, y: {}, {}", new_x, y);
                x = new_x;
            } else {
                return (x * 4000000 + y) as usize;
            }
        }
    }
    unreachable!();
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input, 2_000_000);
    println!("Part 1: {}", p1);
    let p2 = part_2(&input, 0, 4_000_000);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input, 10), 26);
        assert_eq!(part_2(&input, 0, 20), 56000011);
    }
}
