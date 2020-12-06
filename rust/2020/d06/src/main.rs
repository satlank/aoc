use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

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
    vec.iter()
        .map(|g| {
            g.iter()
                .map(|l| l.chars().collect::<HashSet<_>>())
                .fold(HashSet::new(), |acc, x| acc.union(&x).cloned().collect())
                .len()
        })
        .sum()
}

fn part2(vec: &Vec<Vec<String>>) -> usize {
    vec.iter()
        .map(|g| {
            g.iter()
                .map(|l| l.chars().collect::<HashSet<_>>())
                .fold(None::<HashSet<char>>, |acc, x| match acc {
                    Some(a) => Some(a.intersection(&x).cloned().collect()),
                    None => Some(x),
                })
                .unwrap()
                .len()
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    println!("First count: {}", part1(&vec));
    println!("Second count: {}", part2(&vec));
    Ok(())
}
