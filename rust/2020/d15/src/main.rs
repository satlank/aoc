use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<u32>, Error> {
    let br = BufReader::new(io);
    let line = br.lines().next().unwrap()?;
    line.split(",")
        .map(|i| {
            i.parse::<u32>()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))
        })
        .collect()
}

fn find_number(vec: &[u32], turn_limit: u32) -> u32 {
    let mut history = vec[0..vec.len() - 1]
        .iter()
        .enumerate()
        .map(|(idx, val)| (*val, ((idx + 1) as u32, 0u32)))
        .collect::<HashMap<u32, (u32, u32)>>();
    let mut turn = vec.len() as u32 + 1;
    let mut last = vec[vec.len() - 1];
    loop {
        let prev_index = history.get(&last);
        let new_index = match prev_index {
            Some(&x) => (turn - 1, x.0),
            None => (turn - 1, 0),
        };
        let number = match prev_index {
            Some(_) => new_index.0 - new_index.1,
            None => 0,
        };
        history.insert(last, new_index);
        if turn == turn_limit {
            return number;
        }
        turn += 1;
        last = number;
    }
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    println!("Read {} numbers", vec.len());
    println!("The 2020th number is {}", find_number(&vec, 2020));
    println!("The 30000000th number is {}", find_number(&vec, 30000000));
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let input = vec![0, 3, 6];
        assert_eq!(find_number(&input, 10), 0);
    }

    #[test]
    fn test_2() {
        let input = vec![1, 3, 2];
        assert_eq!(find_number(&input, 2020), 1);
    }

    #[test]
    fn test_3() {
        let input = vec![2, 1, 3];
        assert_eq!(find_number(&input, 2020), 10);
    }
}
