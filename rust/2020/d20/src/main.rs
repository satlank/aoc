use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[macro_use]
extern crate scan_fmt;

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    data: Vec<char>,
    cols: usize,
    rows: usize,
}

enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}x{})", self.id, self.cols, self.rows)
    }
}

impl Tile {
    fn parse(lines: &[String]) -> Tile {
        let id = scan_fmt!(&lines[0], "Tile {d}:", usize).unwrap();
        let mut cols = 0;
        let mut rows = 0;
        let mut data: Vec<char> = Vec::new();
        for line in lines[1..].iter() {
            if cols == 0 {
                cols = line.len();
            }
            for c in line.chars() {
                data.push(c)
            }
            rows += 1;
        }
        Tile {
            id,
            data,
            cols,
            rows,
        }
    }

    fn at(&self, col: usize, row: usize) -> char {
        self.data[row * self.cols + col]
    }

    fn border(&self, which: Side) -> String {
        match which {
            Side::Top => (0..self.cols)
                .map(|i| self.at(i, 0))
                .map(|c| if c == '#' { '1' } else { '0' })
                .collect::<String>(),
            Side::Bottom => (0..self.cols)
                .map(|i| self.at(i, self.rows - 1))
                .map(|c| if c == '#' { '1' } else { '0' })
                .collect::<String>(),
            Side::Left => (0..self.rows)
                .map(|i| self.at(0, i))
                .map(|c| if c == '#' { '1' } else { '0' })
                .collect::<String>(),
            Side::Right => (0..self.rows)
                .map(|i| self.at(self.cols - 1, i))
                .map(|c| if c == '#' { '1' } else { '0' })
                .collect::<String>(),
        }
    }

    fn borders(&self) -> Vec<usize> {
        vec![
            usize::from_str_radix(&self.border(Side::Top), 2).unwrap(),
            usize::from_str_radix(&self.border(Side::Top).chars().rev().collect::<String>(), 2)
                .unwrap(),
            usize::from_str_radix(&self.border(Side::Bottom), 2).unwrap(),
            usize::from_str_radix(
                &self.border(Side::Bottom).chars().rev().collect::<String>(),
                2,
            )
            .unwrap(),
            usize::from_str_radix(&self.border(Side::Left), 2).unwrap(),
            usize::from_str_radix(
                &self.border(Side::Left).chars().rev().collect::<String>(),
                2,
            )
            .unwrap(),
            usize::from_str_radix(&self.border(Side::Right), 2).unwrap(),
            usize::from_str_radix(
                &self.border(Side::Right).chars().rev().collect::<String>(),
                2,
            )
            .unwrap(),
        ]
    }
}

fn read<R: Read>(io: R) -> Vec<Tile> {
    let mut res = Vec::new();
    let br = BufReader::new(io);
    let lines = br.lines().filter_map(Result::ok).collect::<Vec<String>>();
    let mut start = 0;
    for i in 0..lines.len() {
        if lines[i].is_empty() && i > start {
            res.push(Tile::parse(&lines[start..i]));
            start = i + 1;
        }
    }
    res.push(Tile::parse(&lines[start..]));
    res
}

fn part1(tiles: &[Tile]) -> usize {
    let mut borders = HashMap::new(); // border int -> tile id
    for tile in tiles.iter() {
        for border in tile.borders() {
            borders
                .entry(border)
                .or_insert([tile.id].iter().cloned().collect::<HashSet<usize>>())
                .insert(tile.id);
        }
    }
    println!(
        "Border pairs: {}",
        borders.iter().filter(|(_, val)| val.len() == 2).count()
    );
    let mut structure: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for (tile_id, link) in borders
        .iter()
        .filter(|(_, val)| val.len() == 2)
        .flat_map(|(border_id, tiles)| {
            vec![
                (
                    *tiles.iter().nth(0).unwrap(),
                    (*tiles.iter().nth(1).unwrap(), *border_id),
                ),
                (
                    *tiles.iter().nth(1).unwrap(),
                    (*tiles.iter().nth(0).unwrap(), *border_id),
                ),
            ]
        })
        .collect::<Vec<_>>()
    {
        if structure.contains_key(&tile_id) {
            structure.get_mut(&tile_id).unwrap().push(link);
        } else {
            structure.insert(tile_id, vec![link]);
        }
    }
    structure
        .iter()
        .filter(|(_, v)| v.len() == 4)
        .map(|(k, _)| k)
        .product()
}

fn main() {
    let tiles = read(File::open("input.txt").unwrap());
    println!("Total of {} tiles", tiles.len());
    println!("Product of corner tile ids: {}", part1(&tiles));
}
