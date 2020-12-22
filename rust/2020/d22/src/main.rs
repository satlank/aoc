use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::VecDeque;

fn read<R: Read>(io: R) -> Result<(VecDeque<usize>, VecDeque<usize>), Error> {
    let mut p1 = VecDeque::new();
    let mut p2 = VecDeque::new();
    let mut act = &mut p1;

    let br = BufReader::new(io);
    for line in br.lines() {
        let line = line?;
        if line.starts_with("Player") { continue }
        if line.len() == 0 {
            act = &mut p2;
        } else {
            act.push_back(line.parse::<usize>().map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
        }
    }
    Ok((p1, p2))
}

fn calc_score(cards: &VecDeque<usize>) -> usize  {
    cards
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, val)| acc + (idx + 1) * val)
}

fn part1(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> usize {
    //let mut round = 1;
    while p1.len() > 0 && p2.len() > 0 {
        //println!("-- Round {} --", round);
        //println!("Player 1's deck: {:?}", p1);
        //println!("Player 2's deck: {:?}", p2);
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        //println!("Player 1 plays: {}", c1);
        //println!("Player 2 plays: {}", c2);
        if c1 > c2 {
            //println!("Player 1 wins the round!");
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            //println!("Player 2 wins the round!");
            p2.push_back(c2);
            p2.push_back(c1);
        }
        //round += 1;
    }
    println!("  Post-game player 1: {:?}", p1);
    println!("  Post-game player 2: {:?}", p2);
    if p1.is_empty() { calc_score(&p2) } else { calc_score(&p1) }
}

fn same_before(history: &[VecDeque<usize>], hand: &VecDeque<usize>) -> bool {
    for prev in history.iter() {
        if prev == hand { return true; }
    }
    false
}

fn game(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> (bool, VecDeque<usize>, VecDeque<usize>) {
    let mut history1: Vec<VecDeque<usize>> = Vec::new();
    let mut history2: Vec<VecDeque<usize>> = Vec::new();
    loop {
        let player1_wins: bool;
        if same_before(&history1, &p1) || same_before(&history2, &p2) {
            // Player 1 wins!
            //println!("Player 1 wins because a same combination of cards happened before!");
            return (true, p1, p2);
        }
        history1.push(p1.clone());
        history2.push(p2.clone());
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1 <= p1.len() && c2 <= p2.len() {
            //println!("Recursing into new game");
            let recurse_p1: VecDeque<usize> = p1.iter().take(c1).cloned().collect();
            let recurse_p2: VecDeque<usize> = p2.iter().take(c2).cloned().collect();
            //println!("  Player 1: {:?}", recurse_p1);
            //println!("  Player 2: {:?}", recurse_p2);
            let result = game(recurse_p1, recurse_p2);
            player1_wins = result.0;
        } else {
            // Normal round
            if c1 > c2 {
                player1_wins = true;
            } else {
                player1_wins = false;
            }
        }
        if player1_wins {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
        if p1.is_empty() {
            // player 2 won
            return (false, p1, p2);
        } else if p2.is_empty() {
            // player 1 won
            return (true, p1, p2);
        }
    }
}

fn part2(p1: VecDeque<usize>, p2: VecDeque<usize>) -> usize {
    let (player1_wins, p1, p2) = game(p1, p2);
    println!("  Post-game player 1: {:?}", p1);
    println!("  Post-game player 2: {:?}", p2);
    if player1_wins { calc_score(&p1) } else { calc_score(&p2) }
}

fn main() -> Result<(), Error> {
    let (p1, p2) = read(File::open("input.txt")?)?;
    println!("Players have their cards: p1: {}, p2: {}", p1.len(), p2.len());
    println!("Score: {}", part1(p1.clone(), p2.clone()));
    println!("Recursive Game Score: {}", part2(p1.clone(), p2.clone()));
    Ok(())
}
