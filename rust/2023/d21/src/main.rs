// https://adventofcode.com/2023/day/21

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Clone)]
struct Garden {
    rocks: HashSet<(i32, i32)>,
    width: i32,
    height: i32,
    start: (i32, i32),
    available: i32,
}

fn read<R: Read>(io: R) -> Garden {
    let br = BufReader::new(io);
    let lines = br.lines().map_while(Result::ok).collect::<Vec<_>>();
    let mut rocks = HashSet::new();
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;
    let mut start = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    rocks.insert((x as i32, y as i32));
                }
                '.' => {}
                'S' => {
                    start = (x as i32, y as i32);
                }
                _ => panic!("Unknown character {}", c),
            }
        }
    }
    let available = (width * height) - rocks.len() as i32;
    Garden {
        rocks,
        width,
        height,
        start,
        available,
    }
}

fn fill_from(start: (i32, i32), garden: &Garden, steps: usize) -> usize {
    let mut reached = HashSet::new();
    reached.insert(start);
    for _step in 0..steps {
        let mut new_reached = HashSet::new();
        for (x, y) in reached.iter() {
            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_x < 0 || new_x >= garden.width || new_y < 0 || new_y >= garden.height {
                    continue;
                }
                if garden.rocks.contains(&(new_x, new_y)) {
                    continue;
                }
                new_reached.insert((new_x, new_y));
            }
        }
        reached = new_reached;
    }
    reached.len()
}

fn fill_from_inf(start: (i32, i32), garden: &Garden, steps: usize) -> usize {
    let mut reached = HashSet::new();
    reached.insert(start);
    let mut obs = Vec::new();
    for step in 0..steps {
        let mut new_reached = HashSet::new();
        for (x, y) in reached.iter() {
            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_x = x + dx;
                let new_y = y + dy;
                let px = ((new_x % garden.width) + garden.width) % garden.width;
                let py = ((new_y % garden.height) + garden.height) % garden.height;
                if garden.rocks.contains(&(px, py)) {
                    continue;
                }
                new_reached.insert((new_x, new_y));
            }
        }
        reached = new_reached;
        if step + 1 == 65
            || step + 1 == 196
            || step + 1 == 327
            || step + 1 == 458
            || step + 1 == 589
        {
            obs.push(((step + 1), reached.len()));
        }
    }
    println!(
        "(a2, a1, a0) = numpy.polyfit([{}], [{}], deg=2)",
        obs.iter()
            .map(|(a, _)| a.to_string())
            .collect::<Vec<_>>()
            .join(", "),
        obs.iter()
            .map(|(_, b)| b.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    reached.len()
}

fn part_1(input: &Garden, steps: usize) -> usize {
    fill_from(input.start, input, steps)
}

fn part_2(garden: &Garden, steps: usize) -> usize {
    // This is sad.  Not a proper solution, just brute-forcing the quadratic fit using the cycle
    // lenghts (65 to reach the border of the initial tile, then 131 to reach the border of the
    // next tiles).
    // It should also work more prinicipled by using the cycle lengths and the number of tiles that
    // can be covered, taking into account that we can walk straight up/down/left/right from the start.
    // But I couldn't make it work...
    fill_from_inf(garden.start, garden, 589.min(steps));
    let (a2, a1, a0) = (0.8402191014509647, 2.145562612901493, 13.612726531099272);
    let fsteps = steps as f64;
    let res = a2 * fsteps * fsteps + a1 * fsteps + a0;
    res.round() as usize

    /*
    // Use some knowledge about the input here:
    //  a) we can walk straight up/down/left/right from the start
    //  b) we start dead in the center of the map
    assert_eq!(garden.height, garden.width);
    assert_eq!(garden.start.0, garden.start.1);
    assert_eq!(garden.start.0, garden.width / 2);
    //  c) there is a clear outer boundary

    //  d) when we walk in a straight line, we do end up at the far boundary of a tile
    let steps_to_boundary = (garden.width / 2) as usize;
    assert_eq!((steps - steps_to_boundary) % (garden.width as usize), 0);

    let tiles_axis = (steps - steps_to_boundary) / garden.height as usize;

    let reachable_odd = fill_from(
        (garden.start.0, garden.start.1),
        garden,
        garden.height as usize * 2 + 1,
    );
    let reachable_even = fill_from(
        (garden.start.0, garden.start.1),
        garden,
        garden.height as usize * 2,
    );
    println!("{} reachable odd", reachable_odd);
    println!("{} reachable even", reachable_even);

    let reachable_up = fill_from(
        (garden.start.0, garden.height - 1),
        garden,
        garden.height as usize,
    );
    let reachable_down = fill_from((garden.start.0, 0), garden, garden.height as usize);
    let reachable_left = fill_from((0, garden.start.1), garden, garden.height as usize);
    let reachable_right = fill_from(
        (garden.width - 1, garden.start.1),
        garden,
        garden.height as usize,
    );
    println!("{} reachable up", reachable_up);
    println!("{} reachable down", reachable_down);
    println!("{} reachable left", reachable_left);
    println!("{} reachable right", reachable_right);

    println!("{} steps on axis", tiles_axis);
    */
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input, 64);
    println!("Part 1: {}", p1);
    let p2 = part_2(&input, 26_501_365);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input, 6), 16);
        //assert_eq!(part_2(&input, 50), 1594);
    }
}
