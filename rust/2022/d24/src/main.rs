// https://adventofcode.com/2022/day/24

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Clone)]
struct Valley {
    start: (i16, i16),
    end: (i16, i16),
    blizzards: HashMap<(i16, i16), Vec<Blizzard>>,
}

impl Valley {
    fn possible_steps(&self, (x, y): (i16, i16)) -> HashSet<(i16, i16)> {
        if self.blizzards.contains_key(&(x, y)) {
            // That means we perished in a blizzard because we moved into a spot last round that a
            // blizzard also moved into.  Yuk.
            return HashSet::new();
        }
        [(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter_map(|(x, y)| {
                if self.start == (x, y) || self.end == (x, y) {
                    Some((x, y))
                } else {
                    if x <= 0 || x > self.end.0 || y <= 0 || y >= self.end.1 {
                        None
                    } else {
                        Some((x, y))
                    }
                }
            })
            .collect()
    }

    fn update(&mut self) {
        let blizzard_list = self
            .blizzards
            .drain()
            .flat_map(|((x, y), blizzards)| {
                blizzards
                    .into_iter()
                    .map(|blizzard| {
                        let mut new_x = x + blizzard.movement.0;
                        let mut new_y = y + blizzard.movement.1;
                        //print!("{} from ({}, {}) -> ({},{})", blizzard, x, y, new_x, new_y);
                        if new_x == 0 {
                            new_x = self.end.0;
                        } else if new_x > self.end.0 {
                            new_x = 1;
                        }
                        if new_y == 0 {
                            new_y = self.end.1 - 1;
                        } else if new_y >= self.end.1 {
                            new_y = 1;
                        }
                        //println!(" -> ({},{})", new_x, new_y);
                        ((new_x, new_y), blizzard)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        for ((x, y), blizzard) in blizzard_list {
            let entry = self.blizzards.entry((x, y)).or_insert(Vec::new());
            entry.push(blizzard);
        }
    }
}

impl Display for Valley {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "#.")?;
        for _ in self.start.0 + 1..self.end.0 + 1 {
            write!(f, "#")?;
        }
        writeln!(f, "#")?;
        for y in self.start.1 + 1..self.end.1 {
            write!(f, "#")?;
            for x in self.start.0..=self.end.0 {
                let sym = if let Some(b) = self.blizzards.get(&(x, y)) {
                    if b.len() == 1 {
                        b[0].to_string().chars().next().unwrap()
                    } else {
                        match b.len() {
                            2 => '2',
                            3 => '3',
                            4 => '4',
                            _ => unreachable!(),
                        }
                    }
                } else {
                    '.'
                };
                write!(f, "{}", sym)?
            }
            writeln!(f, "#")?;
        }
        for _ in self.start.0 - 1..self.end.0 {
            write!(f, "#")?;
        }
        writeln!(f, ".#")
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Blizzard {
    movement: (i16, i16),
}

impl From<char> for Blizzard {
    fn from(c: char) -> Self {
        let movement = match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => unreachable!(),
        };
        Self { movement }
    }
}

impl From<&Blizzard> for char {
    fn from(b: &Blizzard) -> Self {
        match b.movement {
            (1, 0) => '>',
            (-1, 0) => '<',
            (0, 1) => 'v',
            (0, -1) => '^',
            _ => unreachable!(),
        }
    }
}

impl Display for Blizzard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", char::from(self))
    }
}

fn read<R: Read>(io: R) -> Valley {
    let br = BufReader::new(io);
    let lines = br.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let start = (1, 0);
    let end = ((lines[0].len() - 2) as i16, (lines.len() - 1) as i16);
    let blizzards = lines
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '.' || c == '#' {
                        None
                    } else {
                        Some(((x as i16, y as i16), vec![Blizzard::from(c)]))
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>();

    Valley {
        start,
        end,
        blizzards,
    }
}

fn part_1(mut valley: Valley) -> usize {
    let mut steps = 0;
    let mut maybe_elves = HashSet::from([valley.start]);
    loop {
        maybe_elves = maybe_elves
            .into_iter()
            .flat_map(|(x, y)| valley.possible_steps((x, y)))
            .collect();
        valley.update();
        steps += 1;
        if maybe_elves.contains(&valley.end) {
            break;
        }
    }

    steps
}

fn part_2(mut valley: Valley) -> usize {
    let mut steps = 0;
    let mut maybe_elves = HashSet::from([valley.start]);
    loop {
        maybe_elves = maybe_elves
            .into_iter()
            .flat_map(|(x, y)| valley.possible_steps((x, y)))
            .collect();
        valley.update();
        steps += 1;
        if maybe_elves.contains(&valley.end) {
            break;
        }
    }
    maybe_elves = HashSet::from([valley.end]);
    loop {
        maybe_elves = maybe_elves
            .into_iter()
            .flat_map(|(x, y)| valley.possible_steps((x, y)))
            .collect();
        valley.update();
        steps += 1;
        if maybe_elves.contains(&valley.start) {
            break;
        }

    }
    println!("..back to start in {} mins", steps);
    let mut maybe_elves = HashSet::from([valley.start]);
    loop {
        maybe_elves = maybe_elves
            .into_iter()
            .flat_map(|(x, y)| valley.possible_steps((x, y)))
            .collect();
        valley.update();
        steps += 1;
        if maybe_elves.contains(&valley.end) {
            break;
        }
    }
    steps
}

fn main() {
    let valley = read(File::open("input.txt").unwrap());
    let p1 = part_1(valley.clone());
    println!("Time to get through the valley: {}", p1);
    let p2 = part_2(valley);
    println!("..and back with the forgotten snacks: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movements() {
        let mut valley = read(File::open("example1.txt").unwrap());
        println!("start: {:?}  end: {:?}", valley.start, valley.end);
        println!("{}", valley);

        for i in 1..=18 {
            println!("\nMinute {}", i);
            valley.update();
            println!("{}", valley);
        }
    }

    #[test]
    fn example_1() {
        let valley = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(valley.clone()), 18);
        assert_eq!(part_2(valley), 54);
    }
}
