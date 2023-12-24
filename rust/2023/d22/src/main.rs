// https://adventofcode.com/2023/day/22

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick {
    start: [usize; 3],
    end: [usize; 3],
}

impl Brick {
    fn get_blocks(&self) -> Vec<[usize; 3]> {
        let mut blocks = Vec::new();
        for x in self.start[0]..=self.end[0] {
            for y in self.start[1]..=self.end[1] {
                for z in self.start[2]..=self.end[2] {
                    blocks.push([x, y, z]);
                }
            }
        }
        blocks
    }
}

impl From<String> for Brick {
    fn from(s: String) -> Self {
        let mut parts = s.split('~');
        let start: [usize; 3] = parts
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let end: [usize; 3] = parts
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        assert!(start[0] <= end[0]);
        assert!(start[1] <= end[1]);
        assert!(start[2] <= end[2]);
        Self { start, end }
    }
}

fn read<R: Read>(io: R) -> Vec<Brick> {
    let br = BufReader::new(io);
    br.lines().map_while(Result::ok).map(Brick::from).collect()
}

fn fall(input: &[Brick]) -> (Vec<Brick>, HashMap<[usize; 3], usize>) {
    let mut bricks = input.to_vec();
    bricks.sort_by_key(|b| b.start[2]);

    let mut tower = HashMap::new();
    for (id, brick) in bricks.iter_mut().enumerate() {
        let mut blocks = brick.get_blocks();
        let mut can_fall = true;
        let mut fall_by = 0;
        while can_fall {
            for block in &blocks {
                if block[2] == 1 {
                    can_fall = false;
                    break;
                }
                if tower.contains_key(&[block[0], block[1], block[2] - 1]) {
                    can_fall = false;
                    break;
                }
            }
            if can_fall {
                fall_by += 1;
                for block in blocks.iter_mut() {
                    block[2] -= 1;
                }
            }
        }
        brick.start[2] -= fall_by;
        brick.end[2] -= fall_by;
        for block in blocks {
            tower.insert(block, id);
        }
    }
    (bricks, tower)
}

fn build_support_connections(
    bricks: &[Brick],
    tower: &HashMap<[usize; 3], usize>,
) -> (
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
) {
    let max_z = bricks.iter().map(|b| b.end[2]).max().unwrap();

    let min_x = bricks.iter().map(|b| b.start[0]).min().unwrap();
    let min_y = bricks.iter().map(|b| b.start[1]).min().unwrap();
    let max_x = bricks.iter().map(|b| b.start[0]).max().unwrap();
    let max_y = bricks.iter().map(|b| b.start[1]).max().unwrap();

    // Tracks which brick supports which other brick
    let mut supports = HashMap::new();
    // Tracks which brick is supported by which other brick
    let mut supported_by = HashMap::new();

    for dz in 0..max_z {
        let z = max_z - dz;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(supportee_id) = tower.get(&[x, y, z]) {
                    if let Some(supporter_id) = tower.get(&[x, y, z - 1]) {
                        if supportee_id == supporter_id {
                            continue;
                        }
                        supports
                            .entry(*supporter_id)
                            .or_insert_with(HashSet::new)
                            .insert(*supportee_id);
                        supported_by
                            .entry(*supportee_id)
                            .or_insert_with(HashSet::new)
                            .insert(*supporter_id);
                    }
                }
            }
        }
    }
    (supports, supported_by)
}

fn part_1(input: &[Brick]) -> usize {
    let (bricks, tower) = fall(input);
    let (supports, supported_by) = build_support_connections(&bricks, &tower);

    let not_supporting_anything = (0..bricks.len())
        .filter_map(|id| {
            if supports.contains_key(&id) {
                None
            } else {
                Some(id)
            }
        })
        .collect::<Vec<_>>();

    let multi_supporters = supports
        .iter()
        .flat_map(|(id, supportees)| {
            //println!("Checking {}, which supports {:?}", id, supportees);
            let mut has_enough = true;
            for supportee in supportees {
                //println!(
                //    "  {} is supported by {:?}",
                //    supportee, supported_by[supportee]
                //);
                if supported_by[supportee].len() == 1 {
                    has_enough = false;
                    break;
                }
            }
            if has_enough {
                Some(id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    not_supporting_anything.len() + multi_supporters.len()
}

fn part_2(input: &[Brick]) -> usize {
    let (bricks, tower) = fall(input);
    let (supports, supported_by) = build_support_connections(&bricks, &tower);

    let mut total_falling = 0;

    for id in 0..bricks.len() {
        if !supports.contains_key(&id) {
            continue;
        }
        let mut falling = HashSet::new();
        let mut maybe_falling = supports[&id].clone();
        while !maybe_falling.is_empty() {
            let mut new_maybe_falling = HashSet::new();

            for sid in maybe_falling {
                let supporters = &supported_by[&sid];
                if supporters.len() == 1 {
                    falling.insert(sid);
                    if supports.contains_key(&sid) {
                        new_maybe_falling.extend(supports[&sid].clone());
                    }
                } else if supporters
                    .iter()
                    .all(|supporter_id| falling.contains(supporter_id) || supporter_id == &id)
                {
                    falling.insert(sid);
                    if supports.contains_key(&sid) {
                        new_maybe_falling.extend(supports[&sid].clone());
                    }
                }
            }
            maybe_falling = new_maybe_falling;
        }
        total_falling += falling.len();
    }

    total_falling
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
        assert_eq!(part_1(&input), 5);
        assert_eq!(part_2(&input), 7);
    }
}
