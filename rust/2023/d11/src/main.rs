// https://adventofcode.com/2023/day/11

use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Observation {
    Original(Vec<Pos>),
    Expanded(Vec<Pos>),
}

impl ToString for Observation {
    fn to_string(&self) -> String {
        match self {
            Observation::Original(inp) | Observation::Expanded(inp) => {
                let mut out = String::new();
                let max_x = inp.iter().map(|p| p.x).max().unwrap();
                let max_y = inp.iter().map(|p| p.y).max().unwrap();
                for y in 0..=max_y {
                    for x in 0..=max_x {
                        if inp.contains(&Pos { x, y }) {
                            out.push('#');
                        } else {
                            out.push('.');
                        }
                    }
                    out.push('\n');
                }
                out.push('\n');
                out
            }
        }
    }
}

impl Observation {
    fn expand(&self, factor: i32) -> Observation {
        match self {
            Observation::Original(inp) | Observation::Expanded(inp) => {
                let mut exp = inp.clone();

                exp.sort_by(|a, b| a.y.cmp(&b.y));
                let mut dy = 0;
                let mut last_y = exp[0].y;
                exp.iter_mut().for_each(|p| {
                    let diff = p.y - last_y;
                    last_y = p.y;
                    if diff > 1 {
                        dy += (diff - 1) * (factor - 1);
                    }
                    p.y += dy;
                });

                exp.sort_by(|a, b| a.x.cmp(&b.x));
                let mut dx = 0;
                let mut last_x = exp[0].x;
                exp.iter_mut().for_each(|p| {
                    let diff = p.x - last_x;
                    last_x = p.x;
                    if diff > 1 {
                        dx += (diff - 1) * (factor - 1);
                    }
                    p.x += dx;
                });

                Observation::Expanded(exp)
            }
        }
    }

    fn pairwise_distances(&self) -> usize {
        match self {
            Observation::Original(obs) | Observation::Expanded(obs) => (0..obs.len() - 1)
                .flat_map(|i| {
                    (i + 1..obs.len())
                        .map(|j| {
                            let dx = (obs[i].x - obs[j].x).abs();
                            let dy = (obs[i].y - obs[j].y).abs();
                            (dx + dy) as usize
                        })
                        .collect::<Vec<_>>()
                })
                .sum::<usize>(),
        }
    }
}

fn read<R: Read>(io: R) -> Observation {
    let br = BufReader::new(io);
    Observation::Original(
        br.lines()
            .map_while(Result::ok)
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        if c == '#' {
                            Some(Pos {
                                x: x as i32,
                                y: y as i32,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
    )
}

fn part_1(input: &Observation) -> usize {
    let obs = input.expand(2);
    obs.pairwise_distances()
}

fn part_2(input: &Observation) -> usize {
    let obs = input.expand(1_000_000);
    obs.pairwise_distances()
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
        assert_eq!(part_1(&input), 374);

        let exp = input.expand(10);
        assert_eq!(exp.pairwise_distances(), 1030);
        let exp = input.expand(100);
        assert_eq!(exp.pairwise_distances(), 8410);
    }
}
