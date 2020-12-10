use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<usize>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn part1(vec: &[usize]) -> usize {
    let mut sorted = vec.iter().cloned().collect::<Vec<_>>();
    sorted.push(0); // add charging outlet
    sorted.sort();
    sorted.push(sorted[sorted.len() - 1] + 3); // add device
    let mut cnt1 = 0;
    let mut cnt3 = 0;
    for i in 1..sorted.len() {
        match sorted[i] - sorted[i-1] {
            1 => cnt1 += 1,
            3 => cnt3 += 1,
            _ => {}
        }
    }
    cnt1 * cnt3
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    println!("Read {} numbers", vec.len());
    println!("Number of 1 jolt differences multiplied by number of 3 jolt differences: {}", part1(&vec));
    Ok(())
}
