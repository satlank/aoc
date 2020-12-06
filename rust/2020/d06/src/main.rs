use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::HashSet;

fn read<R: Read>(io: R) -> Result<Vec<Vec<String>>, Error> {
    let mut res = Vec::new();
    let mut group = Vec::new();
    let br = BufReader::new(io);
    for line in br.lines() {
        let line = line?;
        if line.len() == 0 {
            res.push(group);
            group = Vec::new();
        } else {
            group.push(line);
        }
    }
    Ok(res)
}

fn part1(vec: &Vec<Vec<String>>) -> usize {
    let mut counts = 0;
    for group in vec {
        let mut answers = HashSet::new();
        for person in group {
            for c in person.chars() {
                answers.insert(c);
            }
        }
        counts += answers.len();
    }
    return counts;
}

fn part2(vec: &Vec<Vec<String>>) -> usize {
    let mut counts = 0;
    for group in vec {
        let mut answers: Option<HashSet<char>> = None;
        for person in group {
            let c: HashSet<char> = person.chars().collect::<HashSet<_>>();
            if answers.is_none() {
                answers = Some(c);
            } else {
                answers = Some(answers.unwrap().intersection(&c).cloned().collect());
            }
        }
        counts += answers.unwrap().len();
    }
    return counts;
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    println!("First count: {}", part1(&vec));
    println!("Second count: {}", part2(&vec));
    Ok(())
}
