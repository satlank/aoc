use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<usize>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn part1(vec: &[usize]) -> usize {
    let mut cnt1 = 0;
    let mut cnt3 = 0;
    for i in 1..vec.len() {
        match vec[i] - vec[i - 1] {
            1 => cnt1 += 1,
            3 => cnt3 += 1,
            _ => {}
        }
    }
    cnt1 * cnt3
}

fn part2(vec: &[usize]) -> u128 {
    let mut paths = vec![1; vec.len()];
    for i in 0..vec.len() {
        if i + 2 < vec.len() && vec[i + 2] - vec[i] <= 3 {
            for j in i + 2..vec.len() {
                paths[j] += paths[i];
            }
        }
        if i + 3 < vec.len() && vec[i + 3] - vec[i] <= 3 {
            for j in i + 3..vec.len() {
                paths[j] += paths[i];
            }
        }
    }
    paths[paths.len() - 1]
}

fn prep_vec(vec: &mut Vec<usize>) {
    vec.push(0); // add charging outlet
    vec.sort_unstable();
    vec.push(vec[vec.len() - 1] + 3); // add device
}

fn main() -> Result<(), Error> {
    let mut vec = read(File::open("input.txt")?)?;
    println!("Read {} numbers", vec.len());
    prep_vec(&mut vec);
    println!(
        "Number of 1 jolt differences multiplied by number of 3 jolt differences: {}",
        part1(&vec)
    );
    println!("Total number of combincations: {}", part2(&vec));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut vec = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        prep_vec(&mut vec);
        assert_eq!(part2(&vec), 8);
    }

    #[test]
    fn test_2() {
        let mut vec = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        prep_vec(&mut vec);
        assert_eq!(part2(&vec), 19208);
    }
}
