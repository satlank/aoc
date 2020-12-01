use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}


fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    for i in 0..(vec.len()-1) {
        for j in (i+1)..vec.len() {
            if vec[i] + vec[j] == 2020 {
                println!("{} + {} = 2020, {} * {} = {}", vec[i], vec[j], vec[i], vec[j], vec[i] * vec[j]);
                return Ok(());
            }
        }
    }
    Ok(())
}
