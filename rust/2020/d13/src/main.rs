use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<(u64, Vec<Option<u64>>), Error> {
    let mut br = BufReader::new(io);
    let mut line = String::new();
    br.read_line(&mut line)?;
    let time = line[..line.len()-1]
        .parse::<u64>()
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    let mut line = String::new();
    br.read_line(&mut line)?;
    let arg = line[..line.len()-1].split(",")
            .map(|i| i.parse::<u64>().ok())
            .collect();
    Ok((
        time, arg
    ))
}

fn part1(time: u64, vec: &[Option<u64>]) -> u64 {
    let mut depart_time = time;
    loop {
        for id in vec.iter() {
            if id.is_some() && depart_time % id.unwrap() == 0 {
                return id.unwrap() * (depart_time - time)
            }
        }
        depart_time += 1;
    }
}

fn departs_in(minutes: u64, time: u64, freq: Option<u64>) -> bool {
    if freq.is_none() {
        return true;
    }
    let freq = freq.unwrap();
    (freq - time % freq) % freq == minutes % freq
}

fn part2(vec: &[Option<u64>]) -> u64 {
    let first = vec[0].unwrap();
    let mut time = first;
    let mut idx_match = 0;
    let mut period = 1;
    let max_period = vec.iter().filter(|f| f.is_some()).fold(1, |acc, val| acc * val.unwrap());
    loop {
        for i in idx_match..vec.len() {
            if departs_in(i as u64, time, vec[i]) {
                if vec[i].is_some() {
                    period *= vec[i].unwrap();
                }
            } else {
                break;
            }
            if i > idx_match {
            }
            idx_match = i;
        }
        if period == max_period {
            return time;
        }
        time += period;
    }
}

fn main() -> Result<(), Error> {
    let (time, vec) = read(File::open("input.txt")?)?;
    println!("At {} we have {} active ids", time, vec.len());
    println!("Part1 result: {}", part1(time, &vec));
    println!("Part2 result: {}", part2(&vec));
    Ok(())
}
