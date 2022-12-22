// https://adventofcode.com/2022/day/22

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum CellType {
    Free,
    Wall,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    Walk(usize),
    TurnRight,
    TurnLeft,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Facing {
    Left,
    Down,
    Right,
    Up,
}

impl std::fmt::Display for Facing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Facing::Right => '>',
                Facing::Down => 'v',
                Facing::Left => '<',
                Facing::Up => '^',
            }
        )
    }
}

impl Facing {
    fn value(&self) -> usize {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }
    fn turn_left(&self) -> Self {
        match self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        }
    }
    fn turn_right(&self) -> Self {
        match self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }
    fn turn_around(&self) -> Self {
        match self {
            Facing::Right => Facing::Left,
            Facing::Down => Facing::Up,
            Facing::Left => Facing::Right,
            Facing::Up => Facing::Down,
        }
    }
    fn move_one(&self, (row, col): (usize, usize)) -> (usize, usize) {
        match self {
            Facing::Right => (row, col + 1),
            Facing::Down => (row + 1, col),
            Facing::Left => (row, col - 1),
            Facing::Up => (row - 1, col),
        }
    }
}

fn read<R: Read>(io: R) -> (HashMap<(usize, usize), CellType>, Vec<Instruction>) {
    let br = BufReader::new(io);
    let mut lines = br.lines().filter_map(Result::ok).collect::<Vec<String>>();
    let commands = lines.pop().unwrap();
    lines.pop().unwrap(); // Remove empty line
    let board = lines
        .into_iter()
        .enumerate()
        .flat_map(move |(row, col_line)| {
            col_line
                .chars()
                .enumerate()
                .filter_map(move |(col, c)| match c {
                    '#' => Some(((row + 1, col + 1), CellType::Wall)),
                    '.' => Some(((row + 1, col + 1), CellType::Free)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>();
    let turns = commands
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if !c.is_ascii_digit() { Some(i) } else { None })
        .collect::<Vec<_>>();
    let mut instructions = vec![Instruction::Walk(commands[..turns[0]].parse().unwrap())];
    for i in 0..turns.len() {
        if &commands[turns[i]..=turns[i]] == "R" {
            instructions.push(Instruction::TurnRight);
        } else {
            instructions.push(Instruction::TurnLeft);
        }
        let bound = if i + 1 == turns.len() {
            commands.len()
        } else {
            turns[i + 1]
        };
        instructions.push(Instruction::Walk(
            commands[turns[i] + 1..bound].parse().unwrap(),
        ));
    }

    (board, instructions)
}

fn find_start(board: &HashMap<(usize, usize), CellType>) -> (usize, usize, Facing) {
    let mut current = (1, 0, Facing::Right);
    loop {
        current.1 += 1;
        if let Some(ct) = board.get(&(current.0, current.1)) {
            if ct == &CellType::Free {
                return current;
            }
        }
    }
}

fn walk(
    start: (usize, usize, Facing),
    board: &HashMap<(usize, usize), CellType>,
    mut distance: usize,
) -> (usize, usize, Facing) {
    let mut current = (start.0, start.1);
    while distance > 0 {
        let next = start.2.move_one(current);
        if let Some(ct) = board.get(&next) {
            match ct {
                CellType::Free => {
                    current = next;
                    distance -= 1;
                }
                CellType::Wall => {
                    distance = 0;
                }
            }
        } else {
            // WRAPPING LOGIC!
            let facing_backward = start.2.turn_around();
            let mut backtrack = facing_backward.move_one(current);
            while board.contains_key(&backtrack) {
                backtrack = facing_backward.move_one(backtrack);
            }
            let other_edge = start.2.move_one(backtrack);
            match board[&other_edge] {
                CellType::Free => {
                    current = other_edge;
                    distance -= 1;
                }
                CellType::Wall => {
                    distance = 0;
                }
            }
        }
    }

    (current.0, current.1, start.2)
}

fn part_1(board: &HashMap<(usize, usize), CellType>, instructions: &[Instruction]) -> usize {
    let mut current = find_start(board);
    println!(
        "Starting at row {}, column {} facing {}",
        current.0, current.1, current.2
    );
    for instruction in instructions {
        match instruction {
            Instruction::TurnLeft => {
                current.2 = current.2.turn_left();
            }
            Instruction::TurnRight => {
                current.2 = current.2.turn_right();
            }
            Instruction::Walk(distance) => {
                current = walk(current, board, *distance);
            }
        }
    }
    println!(
        "Final position is row {}, column {}, facing {}",
        current.0, current.1, current.2
    );
    current.0 * 1000 + current.1 * 4 + current.2.value()
}

type Tracking = ((usize, usize), Facing);

fn cube_walk(
    start: (usize, usize, Facing),
    board: &HashMap<(usize, usize), CellType>,
    links: &HashMap<Tracking, Tracking>,
    mut distance: usize,
) -> (usize, usize, Facing) {
    let mut current = ((start.0, start.1), start.2);
    while distance > 0 {
        let next = if let Some(wrapped) = links.get(&current) {
            *wrapped
        } else {
            (current.1.move_one(current.0), current.1)
        };
        if let Some(ct) = board.get(&next.0) {
            match ct {
                CellType::Free => {
                    current = next;
                    distance -= 1;
                }
                CellType::Wall => {
                    distance = 0;
                }
            }
        } else {
            unreachable!();
        }
    }

    (current.0 .0, current.0 .1, current.1)
}

fn build_my_input_links() -> HashMap<Tracking, Tracking> {
    // Ok, my input is structured like
    //
    //    0123456789
    //  0 +---------+
    //  1 |   222111|
    //  2 |   222111|
    //  3 |   222111|
    //  4 |   333   |
    //  5 |   333   |
    //  6 |   333   |
    //  7 |555444   |
    //  8 |555444   |
    //  9 |555444   |
    // 10 |666      |
    // 11 |666      |
    // 12 |666      |
    // 13 +---------+
    //
    // with:
    //   1:   1 <= row <=  50  &&  101 <= col <= 150
    //   2:   1 <= row <=  50  &&   51 <= col <= 100
    //   3:  51 <= row <= 100  &&   51 <= col <= 100
    //   4: 101 <= row <= 150  &&   51 <= col <= 100
    //   5: 101 <= row <= 150  &&    1 <= col <=  50
    //   6: 151 <= row <= 200  &&    1 <= col <=  50

    let mut links = HashMap::new();

    // 1 going right -> 4 going left
    for row in 1..=50 {
        links.insert(
            ((row, 150), Facing::Right),
            ((151 - row, 100), Facing::Left),
        );
    }
    // 1 going up -> 6 going up
    for col in 101..=150 {
        links.insert(((1, col), Facing::Up), ((200, col - 100), Facing::Up));
    }
    // 1 going down -> 3 going left
    for col in 101..=150 {
        links.insert(
            ((50, col), Facing::Down),
            ((50 + col - 100, 100), Facing::Left),
        );
    }

    // 2 going up -> 6 going right
    for col in 51..=100 {
        links.insert(((1, col), Facing::Up), ((150 + col - 50, 1), Facing::Right));
    }
    // 2 going left -> 5 going right
    for row in 1..=50 {
        links.insert(((row, 51), Facing::Left), ((151 - row, 1), Facing::Right));
    }

    // 3 going right --> 1 going up
    for row in 51..=100 {
        links.insert(
            ((row, 100), Facing::Right),
            ((50, 100 + row - 50), Facing::Up),
        );
    }
    // 3 going left --> 5 going down
    for row in 51..=100 {
        links.insert(((row, 51), Facing::Left), ((101, row - 50), Facing::Down));
    }

    // 4 going right --> 1 going left
    for row in 101..=150 {
        links.insert(
            ((row, 100), Facing::Right),
            ((51 - (row - 100), 150), Facing::Left),
        );
    }
    // 4 going down -> 6 going left
    for col in 51..=100 {
        links.insert(
            ((150, col), Facing::Down),
            ((col - 50 + 150, 50), Facing::Left),
        );
    }

    // 5 going up -> 3 going right
    for col in 1..=50 {
        links.insert(((101, col), Facing::Up), ((50 + col, 51), Facing::Right));
    }
    // 5 going left -> 2 going right
    for row in 101..150 {
        links.insert(
            ((row, 1), Facing::Left),
            ((51 - (row - 100), 51), Facing::Right),
        );
    }

    // 6 going right -> 4 going up
    for row in 151..=200 {
        links.insert(
            ((row, 50), Facing::Right),
            ((150, 50 + row - 150), Facing::Up),
        );
    }
    // 6 going down -> 1 going down
    for col in 1..=50 {
        links.insert(((200, col), Facing::Down), ((1, 100 + col), Facing::Down));
    }
    // 6 going left -> 2 going down
    for row in 151..=200 {
        links.insert(
            ((row, 1), Facing::Left),
            ((1, 50 + row - 150), Facing::Down),
        );
    }

    links
}

fn part_2(
    board: &HashMap<(usize, usize), CellType>,
    instructions: &[Instruction],
    links: &HashMap<Tracking, Tracking>,
) -> usize {
    let mut current = find_start(board);
    println!(
        "Starting at row {}, column {} facing {}",
        current.0, current.1, current.2
    );
    for instruction in instructions {
        match instruction {
            Instruction::TurnLeft => {
                current.2 = current.2.turn_left();
            }
            Instruction::TurnRight => {
                current.2 = current.2.turn_right();
            }
            Instruction::Walk(distance) => {
                current = cube_walk(current, board, links, *distance);
            }
        }
    }
    println!(
        "Final position is row {}, column {}, facing {}",
        current.0, current.1, current.2
    );
    current.0 * 1000 + current.1 * 4 + current.2.value()
}

fn main() {
    let (board, instructions) = read(File::open("input.txt").unwrap());
    let p1 = part_1(&board, &instructions);
    println!("Part 1: {}", p1);
    let links = build_my_input_links();
    let p2 = part_2(&board, &instructions, &links);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_links() -> HashMap<Tracking, Tracking> {
        vec![
            // 1 facing up --> 2 facing down
            (((1, 9), Facing::Up), ((5, 4), Facing::Down)),
            (((1, 10), Facing::Up), ((5, 3), Facing::Down)),
            (((1, 11), Facing::Up), ((5, 2), Facing::Down)),
            (((1, 12), Facing::Up), ((5, 1), Facing::Down)),
            // 1 facing right --> 6 facing left
            (((1, 12), Facing::Right), ((12, 16), Facing::Left)),
            (((2, 12), Facing::Right), ((11, 16), Facing::Left)),
            (((3, 12), Facing::Right), ((10, 16), Facing::Left)),
            (((4, 12), Facing::Right), ((9, 16), Facing::Left)),
            // 1 facing left --> 3 facing down
            (((1, 9), Facing::Left), ((5, 5), Facing::Down)),
            (((2, 9), Facing::Left), ((5, 6), Facing::Down)),
            (((3, 9), Facing::Left), ((5, 7), Facing::Down)),
            (((4, 9), Facing::Left), ((5, 8), Facing::Down)),
            // 3 facing up --> 1 facing right
            (((5, 5), Facing::Up), ((1, 9), Facing::Right)),
            (((5, 6), Facing::Up), ((2, 9), Facing::Right)),
            (((5, 7), Facing::Up), ((3, 9), Facing::Right)),
            (((5, 8), Facing::Up), ((4, 9), Facing::Right)),
            // 3 facing down --> 5 facing right
            (((8, 5), Facing::Down), ((12, 9), Facing::Right)),
            (((8, 6), Facing::Down), ((11, 9), Facing::Right)),
            (((8, 7), Facing::Down), ((10, 9), Facing::Right)),
            (((8, 8), Facing::Down), ((9, 9), Facing::Right)),
            // 2 facing up --> 1 facing down
            (((5, 1), Facing::Up), ((1, 12), Facing::Down)),
            (((5, 2), Facing::Up), ((1, 11), Facing::Down)),
            (((5, 3), Facing::Up), ((1, 10), Facing::Down)),
            (((5, 4), Facing::Up), ((1, 9), Facing::Down)),
            // 2 facing left --> 6 facing up
            (((5, 1), Facing::Left), ((12, 16), Facing::Up)),
            (((6, 1), Facing::Left), ((12, 15), Facing::Up)),
            (((7, 1), Facing::Left), ((12, 14), Facing::Up)),
            (((8, 1), Facing::Left), ((12, 13), Facing::Up)),
            // 2 facing down --> 5 facing up
            (((8, 1), Facing::Down), ((12, 12), Facing::Up)),
            (((8, 2), Facing::Down), ((12, 11), Facing::Up)),
            (((8, 3), Facing::Down), ((12, 10), Facing::Up)),
            (((8, 4), Facing::Down), ((12, 9), Facing::Up)),
            // 4 facing right --> 6 facing down
            (((5, 12), Facing::Right), ((9, 16), Facing::Down)),
            (((6, 12), Facing::Right), ((9, 15), Facing::Down)),
            (((7, 12), Facing::Right), ((9, 14), Facing::Down)),
            (((8, 12), Facing::Right), ((9, 13), Facing::Down)),
            // 5 facing left --> 3 facing up
            (((9, 9), Facing::Left), ((8, 8), Facing::Up)),
            (((10, 9), Facing::Left), ((8, 7), Facing::Up)),
            (((11, 9), Facing::Left), ((8, 6), Facing::Up)),
            (((12, 9), Facing::Left), ((8, 5), Facing::Up)),
            // 5 facing down --> 2 facing up
            (((12, 9), Facing::Down), ((8, 4), Facing::Up)),
            (((12, 10), Facing::Down), ((8, 3), Facing::Up)),
            (((12, 11), Facing::Down), ((8, 2), Facing::Up)),
            (((12, 12), Facing::Down), ((8, 1), Facing::Up)),
            // 6 facing down --> 2 facing right
            (((12, 13), Facing::Down), ((8, 1), Facing::Right)),
            (((12, 14), Facing::Down), ((7, 1), Facing::Right)),
            (((12, 15), Facing::Down), ((6, 1), Facing::Right)),
            (((12, 16), Facing::Down), ((5, 1), Facing::Right)),
            // 6 facing up --> 4 facing left
            (((9, 13), Facing::Up), ((8, 12), Facing::Left)),
            (((9, 14), Facing::Up), ((7, 12), Facing::Left)),
            (((9, 15), Facing::Up), ((6, 12), Facing::Left)),
            (((9, 16), Facing::Up), ((5, 12), Facing::Left)),
            // 6 facing right --> 1 facing left
            (((9, 16), Facing::Right), ((4, 12), Facing::Left)),
            (((10, 16), Facing::Right), ((3, 12), Facing::Left)),
            (((11, 16), Facing::Right), ((2, 12), Facing::Left)),
            (((12, 16), Facing::Right), ((1, 12), Facing::Left)),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn example_1() {
        let (board, instructions) = read(File::open("example1.txt").unwrap());
        assert_eq!(instructions.len(), 13);
        assert_eq!(board.len(), 6 * 16); // 6 blocks of 4x4
        assert_eq!(part_1(&board, &instructions), 6032);
        let links = build_test_links();
        assert_eq!(part_2(&board, &instructions, &links), 5031);
    }
}
