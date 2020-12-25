use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<u64>, Error> {
    let br = BufReader::new(io);
    br
        .lines()
        .map(|l| l.and_then(|v| v.parse::<u64>().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn transform(subject: u64, loop_counter: usize) -> u64 {
    let mut trf = 1;
    for _i in 0..loop_counter {
        trf = (trf * subject) % 20201227;
    }
    trf
}

fn part1(pubkeys: &[u64]) -> u64 {
    let mut cnt = 1;
    let mut key = 7;
    while pubkeys.iter().filter(|&k| *k == key).count() == 0 {
        key = (key * 7) % 20201227;
        cnt += 1;
    }
    if key == pubkeys[0] {
        transform(pubkeys[1], cnt)
    } else {
        transform(pubkeys[0], cnt)
    }
}

fn main() -> Result<(), Error> {
    //let pubkeys = read(File::open("test.txt")?)?;
    let pubkeys = read(File::open("input.txt")?)?;
    assert_eq!(pubkeys.len(), 2);
    println!("Read {} public keys: {:?}", pubkeys.len(), pubkeys);
    println!("Encryption key: {}", part1(&pubkeys));
    Ok(())
}
