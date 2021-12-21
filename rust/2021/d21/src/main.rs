// https://adventofcode.com/2021/day/21

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn read<R: Read>(io: R) -> Vec<usize> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| {
            line.unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect()
}

struct DeterministicDice {
    faces: usize,
    state: usize,
}

impl DeterministicDice {
    fn new(faces: usize) -> Self {
        Self {
            faces,
            state: faces - 1,
        }
    }

    fn roll(&mut self) -> usize {
        self.state = (self.state + 1) % self.faces;
        self.state + 1
    }
}

fn play_deterministic_game(positions: &mut Vec<usize>) -> (usize, Vec<usize>) {
    let mut dice = DeterministicDice::new(100);
    let mut scores = (0..positions.len()).map(|_| 0).collect::<Vec<usize>>();
    let num_players = positions.len();
    let mut player = 0;
    let mut rolls = 0;
    loop {
        let movement: usize = (0..3).map(|_| dice.roll()).sum();
        rolls += 3;
        positions[player] = (positions[player] + movement) % 10;
        let score = positions[player] + 1;
        scores[player] += score;
        if scores[player] >= 1000 {
            break;
        }
        player = (player + 1) % num_players;
    }
    (rolls, scores)
}

fn part_1(starting_positions: &[usize]) -> usize {
    let mut positions = starting_positions
        .iter()
        .map(|x| x - 1)
        .collect::<Vec<usize>>();
    let (rolls, scores) = play_deterministic_game(&mut positions);
    rolls * scores.iter().min().unwrap()
}

fn make_move(current_score: usize, current_position: usize) -> Vec<(usize, usize, usize)> {
    let multipliers = [1, 3, 6, 7, 6, 3, 1];
    (0..multipliers.len())
        .map(|i| {
            let movement = i + 3;
            let new_position = (current_position + movement) % 10;
            let new_score = current_score + new_position + 1;
            (new_score, new_position, multipliers[i])
        })
        .collect()
}

#[derive(Copy, Clone, Debug)]
struct Game {
    next: usize,
    players: [Player; 2],
}

#[derive(Copy, Clone, Debug)]
struct Player {
    score: usize,
    position: usize,
    multiplier: usize,
}

fn play_dirac_game(starting_positions: &[usize]) -> usize {
    let mut games = vec![Game {
        next: 0,
        players: [
            Player {
                score: 0,
                position: starting_positions[0],
                multiplier: 1,
            },
            Player {
                score: 0,
                position: starting_positions[1],
                multiplier: 1,
            },
        ],
    }];
    let mut wins = [0, 0];
    loop {
        games = games
            .iter()
            .map(|g| {
                let step = make_move(g.players[g.next].score, g.players[g.next].position);
                step.iter()
                    .filter(|s| s.0 >= 21)
                    .for_each(|s| wins[g.next] += g.players[g.next].multiplier * s.2);
                step.iter().filter(|s| s.0 < 21).map(|s| {
                    let mut new_game = Game {
                        next: (g.next + 1) % 2,
                        players: [g.players[0], g.players[1]],
                    };
                    new_game.players[g.next].score = s.0;
                    new_game.players[g.next].position = s.1;
                    new_game.players[0].multiplier *= s.2;
                    new_game.players[1].multiplier *= s.2;
                    new_game
                })
                .collect::<Vec<Game>>()
            })
            .flatten()
            .collect();
        if games.is_empty() {
            break;
        }
    }
    *wins.iter().max().unwrap()
}

fn part_2(starting_positions: &[usize]) -> usize {
    let positions = starting_positions
        .iter()
        .map(|x| x - 1)
        .collect::<Vec<usize>>();
    play_dirac_game(&positions)
}

fn main() {
    let starting_positions = read(File::open("input.txt").unwrap());
    println!("Result of part 1: {}", part_1(&starting_positions));
    println!("Result of part 2: {}", part_2(&starting_positions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let starting = read(File::open("test1.txt").unwrap());
        assert_eq!(starting, vec![4, 8]);
    }

    #[test]
    fn test_deterministic_dice() {
        let mut dd = DeterministicDice::new(100);
        assert_eq!(
            (0..150).map(|_| dd.roll()).collect::<Vec<usize>>(),
            (1..151)
                .map(|i| if i > 100 { i - 100 } else { i })
                .collect::<Vec<usize>>()
        );
    }

    #[test]
    fn test_part_1() {
        let input = read(File::open("test1.txt").unwrap());
        assert_eq!(part_1(&input), 739785);
    }

    #[test]
    fn test_part_2() {
        let input = read(File::open("test1.txt").unwrap());
        assert_eq!(part_2(&input), 444356092776315);
    }
}
