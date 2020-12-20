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

#[derive(Debug, Clone)]
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

    fn from_tile_map(map: &Vec<Vec<Tile>>) -> Tile {
        let mut data: Vec<char> = Vec::new();
        let inner_rows = map[0][0].rows - 2;
        let inner_cols = map[0][0].cols - 2;
        let rows = inner_rows * 12;
        let cols = inner_cols * 12;
        for row in 0..12 {
            for inner_row in 0..inner_rows {
                for col in 0..12 {
                    for inner_col in 0..inner_cols {
                        data.push(map[row][col].at(inner_col+1, inner_row+1));
                    }
                }
            }
        }
        Tile { id: 0, data, cols, rows }
    }

    fn rotate(&self) -> Tile {
        let mut data = self.data.clone();
        for row in 0..self.rows {
            for col in 0..self.cols {
                data[(row * self.cols) + col] = self.at(row, self.cols - 1 - col);
            }
        }
        Tile {
            id: self.id,
            data,
            cols: self.rows,
            rows: self.cols,
        }
    }

    fn flip(&self) -> Tile {
        let mut data = self.data.clone();
        for row in 0..self.rows {
            for col in 0..self.cols {
                data[(row * self.cols) + col] = self.at(col, self.rows - 1 - row);
            }
        }
        Tile {
            id: self.id,
            data,
            cols: self.cols,
            rows: self.rows,
        }
    }

    fn at(&self, col: usize, row: usize) -> char {
        self.data[row * self.cols + col]
    }

    fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{}", self.at(col, row))
            }
            println!("");
        }
    }

    fn border(&self, which: &Side) -> String {
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
            usize::from_str_radix(&self.border(&Side::Top), 2).unwrap(),
            usize::from_str_radix(&self.border(&Side::Top).chars().rev().collect::<String>(), 2)
                .unwrap(),
            usize::from_str_radix(&self.border(&Side::Bottom), 2).unwrap(),
            usize::from_str_radix(
                &self.border(&Side::Bottom).chars().rev().collect::<String>(),
                2,
            )
            .unwrap(),
            usize::from_str_radix(&self.border(&Side::Left), 2).unwrap(),
            usize::from_str_radix(
                &self.border(&Side::Left).chars().rev().collect::<String>(),
                2,
            )
            .unwrap(),
            usize::from_str_radix(&self.border(&Side::Right), 2).unwrap(),
            usize::from_str_radix(
                &self.border(&Side::Right).chars().rev().collect::<String>(),
                2,
            )
            .unwrap(),
        ]
    }

    fn count_monster_chars(&self) -> usize {
        // Monster:
        //             1111111111
        //   01234567890123456789
        // 0 ..................#.
        // 1 #....##....##....###
        // 2 .#..#..#..#..#..#...
        let monster_offsets = vec![
            (18, 0),
            (0, 1), (5, 1), (6, 1), (11, 1), (12, 1), (17, 1), (18, 1), (19, 1),
            (1, 2), (4, 2), (7, 2), (10, 2), (13, 2), (16, 2)
        ];
        let mut monster_cnt = 0;
        for col in 0..self.cols - 19 {
            for row in 0..self.rows - 3 {
                let mut matching = 0;
                for test_offset in monster_offsets.iter() {
                    if self.at(col + test_offset.0, row + test_offset.1) == '#' {
                        matching += 1;
                    } else {
                        break;
                    }
                }
                if matching == monster_offsets.len() {
                    monster_cnt += 1;
                }
            }
        }
        monster_cnt * monster_offsets.len()
    }

    fn sea_roughness(&self, monsters_chars: usize) -> usize {
        // Assume that monsters don't overlap
        self.data
            .iter()
            .filter(|c| **c == '#')
            .count()
            - monsters_chars
    }
}

fn border_id<S: AsRef<str>>(border: S) -> usize {
    usize::from_str_radix(border.as_ref(), 2).unwrap()
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

fn get_borders(tiles: &[Tile]) -> HashMap<usize, HashSet<usize>> {
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
    borders
}

// Return map: tile_id -> [(tile_id, border_id)]
fn get_structure(borders: &HashMap<usize, HashSet<usize>>) -> HashMap<usize, Vec<(usize, usize)>> {
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
}

fn part1(tiles: &[Tile]) -> usize {
    let borders = get_borders(tiles);
    let structure = get_structure(&borders);
    structure
        .iter()
        .filter(|(_, v)| v.len() == 4)
        .map(|(k, _)| k)
        .product()
}

fn match_top_left(tile: &Tile, links: &Vec<(usize, usize)>) -> Tile {
    let mut matching_tile = tile.clone();
    let mut flip = true;
    let mut cnt = 0;
    let border_ids = links
        .iter()
        .map(|(_, border_id)| border_id)
        .collect::<HashSet<_>>();
    //println!("Border ids: {:?}", border_ids);
    loop {
        //println!("");
        //matching_tile.print();
        let right_border = border_id(matching_tile.border(&Side::Right));
        let bottom_border = border_id(matching_tile.border(&Side::Bottom));
        //println!("right border: {},  bottom border: {}", right_border, bottom_border);
        if border_ids.contains(&right_border) && border_ids.contains(&bottom_border) {
            println!("Top left corner found after {} re-orientations", cnt);
            return matching_tile;
        }
        if flip {
            //println!("Flipping");
            matching_tile = matching_tile.flip();
            flip = false;
        } else {
            //println!("Un-flipping and rotating");
            matching_tile = matching_tile.flip().rotate();
            flip = true;
        }
        cnt += 1;
        if cnt > 9 {
            panic!("WWWAAAAAH");
        }
    }
}

fn match_tile(required_border_id: usize, right_tile: &Tile, side: Side) -> Tile {
    let mut matching_tile = right_tile.clone();
    let mut flip = true;
    let mut cnt = 0;
    loop {
        let match_border = border_id(matching_tile.border(&side));
        if match_border == required_border_id {
            return matching_tile;
        }
        if flip {
            //println!("Flipping");
            matching_tile = matching_tile.flip();
            flip = false;
        } else {
            //println!("Un-flipping and rotating");
            matching_tile = matching_tile.flip().rotate();
            flip = true;
        }
        cnt += 1;
        if cnt > 9 {
            panic!("WWWAAAAAH");
        }
    }
}

fn match_right_tile(required_border_id: usize, right_tile: &Tile) -> Tile {
    let mut matching_tile = right_tile.clone();
    let mut flip = true;
    let mut cnt = 0;
    loop {
        let left_border = border_id(matching_tile.border(&Side::Left));
        if left_border == required_border_id {
            return matching_tile;
        }
        if flip {
            //println!("Flipping");
            matching_tile = matching_tile.flip();
            flip = false;
        } else {
            //println!("Un-flipping and rotating");
            matching_tile = matching_tile.flip().rotate();
            flip = true;
        }
        cnt += 1;
        if cnt > 9 {
            panic!("WWWAAAAAH");
        }
    }
}

fn part2(tiles: &[Tile]) -> usize {
    assert_eq!(tiles.len(), 144);
    let borders = get_borders(tiles);
    let structure = get_structure(&borders);
    let tile_dict = tiles
        .iter()
        .map(|t| (t.id, t))
        .collect::<HashMap<usize, &Tile>>();
    let mut tile_map: Vec<Vec<Tile>> = Vec::new();
    let corners = structure
        .iter()
        .filter(|(_, v)| v.len() == 4)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    tile_map.push(Vec::new());
    tile_map[0].push(match_top_left(
        tile_dict[corners[0]],
        structure.get(corners[0]).unwrap(),
    ));
    for row in 0..12 {
        if row > 0 {
            tile_map.push(Vec::new())
        }
        for col in 0..12 {
            if row == 0 && col == 0 {
                // initialised (0,0) with one of the identified corners already
                continue;
            }
            if col == 0 {
                // get it from the top
                let required_border_id = border_id(tile_map[row-1][0].border(&Side::Bottom));
                //println!(
                //    "Now need to find tile with border_id {}",
                //    required_border_id
                //);
                let top_tile = &tile_map[row - 1][col];
                let bottom_tile_id = structure
                    .get(&top_tile.id)
                    .unwrap()
                    .iter()
                    .filter(|(_, bid)| required_border_id == *bid)
                    .map(|(tid, _)| tid)
                    .nth(0)
                    .unwrap();
                //println!("Found bottom tile id {}", bottom_tile_id);
                let new_tile =
                    match_tile(required_border_id, tile_dict.get(&bottom_tile_id).unwrap(), Side::Top);
                tile_map[row].push(new_tile);
            } else {
                // get it from the left
                let required_border_id = border_id(tile_map[row][col - 1].border(&Side::Right));
                //println!(
                //    "Now need to find tile with border_id {}",
                //    required_border_id
                //);
                let left_tile = &tile_map[row][col - 1];
                let right_tile_id = structure
                    .get(&left_tile.id)
                    .unwrap()
                    .iter()
                    .filter(|(_, bid)| required_border_id == *bid)
                    .map(|(tid, _)| tid)
                    .nth(0)
                    .unwrap();
                //println!("Found right tile id {}", right_tile_id);
                let new_tile =
                    match_right_tile(required_border_id, tile_dict.get(&right_tile_id).unwrap());
                tile_map[row].push(new_tile);
            }
        }
    }
    //for row in 0..12 {
    //    for col in 0..12 {
    //        print!("{} ", tile_map[row][col].id);
    //    }
    //    println!("");
    //}
    let mut final_tile = Tile::from_tile_map(&tile_map);
    let mut flip = true;
    let mut monsters;
    loop {
        monsters = final_tile.count_monster_chars();
        if monsters > 0 {
            println!("Found {} monsters", monsters);
            break;
        }
        if flip {
            final_tile = final_tile.flip();
            flip = false;
        } else {
            final_tile = final_tile.flip().rotate();
            flip = true;
        }
    }
    final_tile.print();
    final_tile.sea_roughness(monsters)
}

fn main() {
    let tiles = read(File::open("input.txt").unwrap());
    println!("Total of {} tiles", tiles.len());
    println!("Product of corner tile ids: {}", part1(&tiles));
    println!("Sea Roughness: {}", part2(&tiles));
}
