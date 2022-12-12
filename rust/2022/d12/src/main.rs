// https://adventofcode.com/2022/day/12

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Grid<char> {
    let br = BufReader::new(io);
    br.lines().filter_map(Result::ok).collect::<Vec<_>>().into()
}

struct Grid<T: Copy> {
    cols: usize,
    rows: usize,
    data: Vec<T>,
}

impl<T: Copy + From<char>> From<Vec<String>> for Grid<T> {
    fn from(data: Vec<String>) -> Self {
        let rows = data.len();
        let cols = data[0].chars().count();
        let data = data
            .into_iter()
            .flat_map(|s| s.chars().map(|c| c.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(data.len(), cols * rows);
        Grid { rows, cols, data }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    fn new(cols: usize, rows: usize, initial: T) -> Self {
        Grid {
            cols,
            rows,
            data: vec![initial; cols * rows],
        }
    }

    fn find_first(&self, thing: T) -> Option<(usize, usize)> {
        for i in 0..self.data.len() {
            if self.data[i] == thing {
                return Some((i % self.cols, i / self.cols));
            }
        }
        None
    }

    fn find_all(&self, thing: T) -> HashSet<(usize, usize)> {
        let mut res = HashSet::new();
        for i in 0..self.data.len() {
            if self.data[i] == thing {
                res.insert((i % self.cols, i / self.cols));
            }
        }
        res
    }

    fn idx(&self, pos: (usize, usize)) -> usize {
        pos.0 + pos.1 * self.cols
    }
}

fn valid(from: char, to: char) -> bool {
    let from_val = if from == 'S' {
        'a' as usize
    } else {
        from as usize
    };
    let to_val = if to == 'E' { 'z' as usize } else { to as usize };
    from_val + 1 >= to_val
}

fn get_neighbours(map: &Grid<char>, cell: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut ret = HashSet::new();
    let cell_idx = map.idx(cell);
    if cell.0 > 0 {
        let n = (cell.0 - 1, cell.1);
        if valid(map.data[map.idx(n)], map.data[cell_idx]) {
            ret.insert(n);
        }
    }
    if cell.0 < map.cols - 1 {
        let n = (cell.0 + 1, cell.1);
        if valid(map.data[map.idx(n)], map.data[cell_idx]) {
            ret.insert(n);
        }
    }
    if cell.1 > 0 {
        let n = (cell.0, cell.1 - 1);
        if valid(map.data[map.idx(n)], map.data[cell_idx]) {
            ret.insert(n);
        }
    }
    if cell.1 < map.rows - 1 {
        let n = (cell.0, cell.1 + 1);
        if valid(map.data[map.idx(n)], map.data[cell_idx]) {
            ret.insert(n);
        }
    }
    ret
}

fn set_neighbours(
    distance_grid: &mut Grid<usize>,
    map: &Grid<char>,
    cells: HashSet<(usize, usize)>,
    value: usize,
) -> HashSet<(usize, usize)> {
    let mut set = HashSet::<(usize, usize)>::new();
    for cell in cells {
        let neighbours = get_neighbours(map, cell);
        for n in neighbours {
            let n_idx = map.idx(n);
            if distance_grid.data[n_idx] > value {
                distance_grid.data[n_idx] = value;
                set.insert(n);
            }
        }
    }
    set
}

fn part_1(input: &Grid<char>) -> usize {
    let start = input.find_first('S').unwrap();
    let start_idx = input.idx(start);
    let end = input.find_first('E').unwrap();
    let end_idx = input.idx(end);
    let mut distance_grid = Grid::<usize>::new(input.cols, input.rows, usize::MAX);
    distance_grid.data[end_idx] = 0;
    let mut active = vec![end].into_iter().collect::<HashSet<_>>();
    let mut distance = 0;
    loop {
        distance += 1;
        active = set_neighbours(&mut distance_grid, input, active, distance);
        if active.contains(&start) || active.is_empty() {
            break;
        }
    }
    distance_grid.data[start_idx]
}

fn part_2(input: &Grid<char>) -> usize {
    let mut starts = input.find_all('a');
    starts.insert(input.find_first('S').unwrap());
    let end = input.find_first('E').unwrap();
    let end_idx = input.idx(end);
    let mut distance_grid = Grid::<usize>::new(input.cols, input.rows, usize::MAX);
    distance_grid.data[end_idx] = 0;
    let mut active = vec![end].into_iter().collect::<HashSet<_>>();
    let mut distance = 0;
    loop {
        distance += 1;
        active = set_neighbours(&mut distance_grid, input, active, distance);
        if active.is_empty() {
            break;
        }
    }
    let mut paths = starts
        .into_iter()
        .map(|s| distance_grid.data[input.idx(s)])
        .collect::<Vec<_>>();
    paths.sort();
    paths[0]
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
        assert_eq!(part_1(&input), 31);
        assert_eq!(part_2(&input), 29);
    }
}
