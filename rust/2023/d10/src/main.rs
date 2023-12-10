// https://adventofcode.com/2023/day/10

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::N => write!(f, "N"),
            Direction::E => write!(f, "E"),
            Direction::S => write!(f, "S"),
            Direction::W => write!(f, "W"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    MaybeAny,
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
struct Map {
    start: Position,
    start_pipe: Pipe,
    height: i32,
    width: i32,
    map: HashMap<Position, Pipe>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position { x, y };
                if pos == self.start {
                    s.push('S');
                } else {
                    match self.map.get(&pos) {
                        Some(Pipe::NS) => s.push('|'),
                        Some(Pipe::EW) => s.push('-'),
                        Some(Pipe::NE) => s.push('L'),
                        Some(Pipe::NW) => s.push('J'),
                        Some(Pipe::SW) => s.push('7'),
                        Some(Pipe::SE) => s.push('F'),
                        Some(Pipe::MaybeAny) => s.push('S'),
                        None => s.push('.'),
                    }
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Map {
    fn find_loop(&mut self) -> HashSet<Position> {
        let the_loop = [Direction::N, Direction::S, Direction::E, Direction::W]
            .into_iter()
            .filter_map(|start_direction| {
                let mut direction = start_direction;
                let mut current = self.start;
                let mut steps = 0;
                loop {
                    let next = self.next_from(&current, direction);
                    match next {
                        Some((new_pos, new_direction)) => {
                            current = new_pos;
                            direction = new_direction;
                            steps += 1;
                            if current == self.start {
                                return Some((start_direction, steps));
                            }
                        }
                        None => return None,
                    }
                }
            })
            .inspect(|(start_direction, steps)| {
                println!("loop length going {}: {}", start_direction, steps)
            })
            .collect::<Vec<_>>();
        assert!(the_loop.len() == 2);
        self.start_pipe = match (the_loop[0].0, the_loop[1].0) {
            (Direction::N, Direction::S) | (Direction::S, Direction::N) => Pipe::NS,
            (Direction::E, Direction::W) | (Direction::W, Direction::E) => Pipe::EW,
            (Direction::N, Direction::E) | (Direction::E, Direction::N) => Pipe::NE,
            (Direction::N, Direction::W) | (Direction::W, Direction::N) => Pipe::NW,
            (Direction::S, Direction::W) | (Direction::W, Direction::S) => Pipe::SW,
            (Direction::S, Direction::E) | (Direction::E, Direction::S) => Pipe::SE,
            _ => unreachable!(),
        };

        let mut direction = the_loop[0].0;
        let mut current = self.start;
        let mut loop_tiles = HashSet::new();
        loop {
            let next = self.next_from(&current, direction);
            match next {
                Some((new_pos, new_direction)) => {
                    loop_tiles.insert(current);
                    current = new_pos;
                    direction = new_direction;
                    if current == self.start {
                        break;
                    }
                }
                None => panic!("loop is broken"),
            }
        }
        assert!(loop_tiles.contains(&self.start));

        loop_tiles
    }

    fn next_from(&self, current: &Position, direction: Direction) -> Option<(Position, Direction)> {
        assert!(self.map.contains_key(current));
        let pipe = self.map.get(&current).unwrap();

        match direction {
            Direction::N => match pipe {
                Pipe::NS | Pipe::NE | Pipe::NW | Pipe::MaybeAny => {
                    let new_pos = Position {
                        x: current.x,
                        y: current.y - 1,
                    };
                    if let Some(mut new_pipe) = self.map.get(&new_pos) {
                        if new_pipe == &Pipe::MaybeAny {
                            new_pipe = &self.start_pipe;
                        }
                        match new_pipe {
                            Pipe::NS => Some((new_pos, Direction::N)),
                            Pipe::SW => Some((new_pos, Direction::W)),
                            Pipe::SE => Some((new_pos, Direction::E)),
                            Pipe::MaybeAny => Some((new_pos, direction)),
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Direction::S => match pipe {
                Pipe::NS | Pipe::SW | Pipe::SE | Pipe::MaybeAny => {
                    let new_pos = Position {
                        x: current.x,
                        y: current.y + 1,
                    };
                    if let Some(mut new_pipe) = self.map.get(&new_pos) {
                        if new_pipe == &Pipe::MaybeAny {
                            new_pipe = &self.start_pipe;
                        }
                        match new_pipe {
                            Pipe::NS => Some((new_pos, Direction::S)),
                            Pipe::NW => Some((new_pos, Direction::W)),
                            Pipe::NE => Some((new_pos, Direction::E)),
                            Pipe::MaybeAny => Some((new_pos, direction)),
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Direction::E => match pipe {
                Pipe::EW | Pipe::NE | Pipe::SE | Pipe::MaybeAny => {
                    let new_pos = Position {
                        x: current.x + 1,
                        y: current.y,
                    };
                    if let Some(mut new_pipe) = self.map.get(&new_pos) {
                        if new_pipe == &Pipe::MaybeAny {
                            new_pipe = &self.start_pipe;
                        }
                        match new_pipe {
                            Pipe::EW => Some((new_pos, Direction::E)),
                            Pipe::NW => Some((new_pos, Direction::N)),
                            Pipe::SW => Some((new_pos, Direction::S)),
                            Pipe::MaybeAny => Some((new_pos, direction)),
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Direction::W => match pipe {
                Pipe::EW | Pipe::NW | Pipe::SW | Pipe::MaybeAny => {
                    let new_pos = Position {
                        x: current.x - 1,
                        y: current.y,
                    };
                    if let Some(mut new_pipe) = self.map.get(&new_pos) {
                        if new_pipe == &Pipe::MaybeAny {
                            new_pipe = &self.start_pipe;
                        }
                        match new_pipe {
                            Pipe::EW => Some((new_pos, Direction::W)),
                            Pipe::NE => Some((new_pos, Direction::N)),
                            Pipe::SE => Some((new_pos, Direction::S)),
                            Pipe::MaybeAny => Some((new_pos, direction)),
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            },
        }
    }
}

fn read<R: Read>(io: R) -> Map {
    let mut height = 0;
    let mut width = 0;
    let mut map = HashMap::new();
    let mut start = Position { x: -1, y: -1 };

    let br = BufReader::new(io);
    br.lines()
        .map_while(Result::ok)
        .enumerate()
        .for_each(|(y, line)| {
            height += 1;
            line.chars().enumerate().for_each(|(x, c)| {
                if height == 1 {
                    width += 1;
                }
                match c {
                    '.' => {}
                    'S' => {
                        start = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        map.insert(start, Pipe::MaybeAny);
                    }
                    _ => {
                        let pos = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        let pipe = Pipe::from(c);
                        map.insert(pos, pipe);
                    }
                }
            });
        });

    assert!(start.x != -1 && start.y != -1);

    Map {
        start,
        start_pipe: Pipe::MaybeAny,
        height,
        width,
        map,
    }
}

fn part_1(mut input: Map) -> usize {
    let the_loop = input.find_loop();
    the_loop.len() / 2
}

fn grow_inside(the_loop: &HashSet<Position>, inside: &HashSet<Position>) -> HashSet<Position> {
    let mut new_inside = HashSet::new();
    inside.iter().for_each(|pos| {
        [
            Position {
                x: pos.x - 1,
                y: pos.y,
            },
            Position {
                x: pos.x + 1,
                y: pos.y,
            },
            Position {
                x: pos.x,
                y: pos.y - 1,
            },
            Position {
                x: pos.x,
                y: pos.y + 1,
            },
        ]
        .into_iter()
        .for_each(|new_pos| {
            if !the_loop.contains(&new_pos) && !inside.contains(&new_pos) {
                new_inside.insert(new_pos);
            }
        });
    });
    new_inside
}

fn part_2(mut map: Map) -> usize {
    let the_loop = map.find_loop();
    let mut top_left = Position {
        x: map.width,
        y: map.height,
    };
    for pos in the_loop.iter() {
        if pos.y <= top_left.y && pos.x <= top_left.x {
            top_left = *pos;
        }
    }

    // Since we have the top-left corner, we know that it is a SE corner tile, so we can only move
    // S or E from it.  Also, we know that when moving clock-wise (E), the 'left' will be outside
    // and the 'right' will be inside.
    // We will only be tracking the inside as that means we don't need to deal with out-of-map
    // checks (and we want the inside anyway).  The below adds for direction we are facing all the
    // possible inside tiles based on the tile we are on.  It is going to add more than we need, in
    // particular it may add tiles that belong to loop itself, so after all possible inside tiles
    // are added, we remove the ones that are in the loop.
    // Finally, we grow the inside tiles, until we have no more new tiles to add.  To grow the
    // inside tile set we look at each inside tile and add any adjacent tile that is not in the
    // loop or already in the inside tile set.  For efficiency, in subsequent iterations we only
    // loop over the new inside tiles that were added in the previous iteration.
    let mut current = top_left;
    let mut direction = Direction::E;
    let mut inside = HashSet::new();
    loop {
        (current, direction) = map.next_from(&current, direction).unwrap();
        let mut tile = map.map.get(&current).unwrap();
        if tile == &Pipe::MaybeAny {
            tile = &map.start_pipe;
        }
        match direction {
            Direction::E => match tile {
                Pipe::EW => {
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y + 1,
                    });
                    inside.insert(Position {
                        x: current.x,
                        y: current.y + 1,
                    });
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y + 1,
                    });
                }
                Pipe::NE => {
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y - 1,
                    });
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y,
                    });
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y + 1,
                    });
                    inside.insert(Position {
                        x: current.x,
                        y: current.y + 1,
                    });
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y + 1,
                    });
                }
                Pipe::SE => {
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y + 1,
                    });
                }
                _ => unreachable!(),
            },
            Direction::W => match tile {
                Pipe::EW => {
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y - 1,
                    });
                    inside.insert(Position {
                        x: current.x,
                        y: current.y - 1,
                    });
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y - 1,
                    });
                }
                Pipe::NW => {
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y - 1,
                    });
                }
                Pipe::SW => {
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y - 1,
                    });
                    inside.insert(Position {
                        x: current.x,
                        y: current.y - 1,
                    });
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y - 1,
                    });
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y,
                    });
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y + 1,
                    });
                }
                _ => unreachable!(),
            },
            Direction::N => match tile {
                Pipe::NS => {
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y - 1,
                    });
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y,
                    });
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y + 1,
                    });
                }
                Pipe::NE => {
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y - 1,
                    });
                }
                Pipe::NW => {
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y - 1,
                    });
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y,
                    });
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y + 1,
                    });
                    inside.insert(Position {
                        x: current.x,
                        y: current.y + 1,
                    });
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y + 1,
                    });
                }
                _ => unreachable!(),
            },
            Direction::S => match tile {
                Pipe::NS => {
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y - 1,
                    });
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y,
                    });
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y + 1,
                    });
                }
                Pipe::SW => {
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y + 1,
                    });
                }
                Pipe::SE => {
                    inside.insert(Position {
                        x: current.x + 1,
                        y: current.y - 1,
                    });
                    inside.insert(Position {
                        x: current.x,
                        y: current.y - 1,
                    });
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y - 1,
                    });
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y,
                    });
                    inside.insert(Position {
                        x: current.x - 1,
                        y: current.y + 1,
                    });
                }
                _ => unreachable!(),
            },
        }

        if current == top_left {
            break;
        }
    }
    // Correct for loop tiles
    let mut inside = inside
        .difference(&the_loop)
        .cloned()
        .collect::<HashSet<_>>();

    // Grow the inside
    loop {
        let new_inside = grow_inside(&the_loop, &inside);
        if new_inside.len() == 0 {
            break;
        }
        inside.extend(new_inside)
    }

    inside.len()
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(input.clone());
    println!("Part 1: {}", p1);
    let p2 = part_2(input);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(input), 4);
    }

    #[test]
    fn example_2() {
        let input = read(File::open("example2.txt").unwrap());
        assert_eq!(part_1(input), 8);
    }

    #[test]
    fn example_3() {
        let input = read(File::open("example3.txt").unwrap());
        assert_eq!(part_2(input), 4);
    }

    #[test]
    fn example_4() {
        let input = read(File::open("example4.txt").unwrap());
        assert_eq!(part_2(input), 8);
    }

    #[test]
    fn example_5() {
        let input = read(File::open("example5.txt").unwrap());
        assert_eq!(part_2(input), 10);
    }
}
