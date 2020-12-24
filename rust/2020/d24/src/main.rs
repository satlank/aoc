use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

#[derive(Debug, Clone)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn read<R: Read>(io: R) -> Result<Vec<Vec<Direction>>, Error> {
    let br = BufReader::new(io);
    let mut res = Vec::new();
    for line in br.lines() {
        let mut dir = Vec::new();
        let mut prev = None;
        for c in line?.chars() {
            let tmp = match c {
                's' => {
                    prev = Some('s');
                    None
                }
                'n' => {
                    prev = Some('n');
                    None
                }
                'e' => {
                    let m = prev;
                    prev = None;
                    match m {
                        Some(x) => {
                            if x == 's' {
                                Some(Direction::SouthEast)
                            } else {
                                Some(Direction::NorthEast)
                            }
                        }
                        None => Some(Direction::East),
                    }
                }
                'w' => {
                    let m = prev;
                    prev = None;
                    match m {
                        Some(x) => {
                            if x == 's' {
                                Some(Direction::SouthWest)
                            } else {
                                Some(Direction::NorthWest)
                            }
                        }
                        None => Some(Direction::West),
                    }
                }
                _ => panic!("Unhandled character"),
            };
            if tmp.is_some() {
                dir.push(tmp.unwrap());
            }
        }
        res.push(dir);
    }
    Ok(res)
}

fn step(tile: &(i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::East => (tile.0 + 2, tile.1),
        Direction::SouthEast => (tile.0 + 1, tile.1 - 1),
        Direction::SouthWest => (tile.0 - 1, tile.1 - 1),
        Direction::West => (tile.0 - 2, tile.1),
        Direction::NorthWest => (tile.0 - 1, tile.1 + 1),
        Direction::NorthEast => (tile.0 + 1, tile.1 + 1),
    }
}

fn generate_blacks(dirs: &Vec<Vec<Direction>>) -> HashSet<(i32, i32)> {
    let mut blacks: HashSet<(i32, i32)> = HashSet::new();
    for dir in dirs.iter() {
        let mut pos = (0i32, 0i32);
        for x in dir.iter() {
            pos = step(&pos, x);
        }
        if blacks.contains(&pos) {
            blacks.remove(&pos);
        } else {
            blacks.insert(pos);
        }
    }
    blacks
}

fn get_neighbours(tile: &(i32, i32)) -> Vec<(i32, i32)> {
    vec![
        step(&tile, &Direction::East),
        step(&tile, &Direction::SouthEast),
        step(&tile, &Direction::SouthWest),
        step(&tile, &Direction::West),
        step(&tile, &Direction::NorthWest),
        step(&tile, &Direction::NorthEast),
    ]
}

fn part1(dirs: &Vec<Vec<Direction>>) -> usize {
    generate_blacks(dirs).len()
}

fn part2(dirs: &Vec<Vec<Direction>>) -> usize {
    let mut blacks = generate_blacks(dirs);
    for _day in 1..=100 {
        let mut whites: HashSet<(i32, i32)> = HashSet::new();
        let mut new_blacks: HashSet<(i32, i32)> = HashSet::new();
        for tile in blacks.iter() {
            let neighbours = get_neighbours(tile);
            let mut cnt = 0;
            for n in neighbours.iter() {
                if blacks.contains(n) {
                    cnt += 1;
                } else {
                    whites.insert(*n);
                }
            }
            if cnt > 0 && cnt < 3 {
                new_blacks.insert(tile.clone());
            }
        }
        for tile in whites.iter() {
            let neighbours = get_neighbours(tile);
            let cnt = neighbours.iter().filter(|n| blacks.contains(n)).count();
            if cnt == 2 {
                new_blacks.insert(tile.clone());
            }
        }
        blacks = new_blacks;
    }
    blacks.len()
}

fn main() -> Result<(), Error> {
    let dirs = read(File::open("input.txt")?)?;
    println!("Read {} directions", dirs.len());
    println!("Number of black tiles: {}", part1(&dirs));
    println!("Number of black tiles after 100 days: {}", part2(&dirs));
    Ok(())
}
