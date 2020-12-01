use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn part_1(vec: &Vec<i64>) -> Option<(i64, i64)> {
    for i in 0..(vec.len()-1) {
        for j in (i+1)..vec.len() {
            if vec[i] + vec[j] == 2020 {
                return Some((vec[i], vec[j]));
            }
        }
    }
    return None;
}

fn part_2(vec: &Vec<i64>) -> Option<(i64, i64, i64)> {
    for i in 0..(vec.len()-2) {
        for j in (i+1)..(vec.len()-1) {
            for k in (j+2)..vec.len() {
                if vec[i] + vec[j] + vec[k] == 2020 {
                    return Some((vec[i], vec[j], vec[k]));
                }
            }
        }
    }
    return None;
}
fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    let (a, b) = part_1(&vec).unwrap();
    println!("{} + {} = 2020, {} * {} = {}", a, b, a, b, a*b);
    let (a, b, c) = part_2(&vec).unwrap();
    println!("{} + {} + {} = 2020, {} * {} * {} = {}", a, b, c, a, b, c, a*b*c);
    Ok(())
}
