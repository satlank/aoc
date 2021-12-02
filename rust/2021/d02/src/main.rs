use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[macro_use]
extern crate scan_fmt;

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
    Invalid,
}

impl Command {
    fn parse<S: AsRef<str>>(line: S) -> Result<Command, scan_fmt::parse::ScanError> {
        let (cmd, value) = scan_fmt!(line.as_ref(), "{} {d}", String, i32)?;
        match cmd.as_str() {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Ok(Command::Invalid),
        }
    }
}

fn read<R: Read>(io: R) -> Result<Vec<Command>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| {
            line.and_then(|row| {
                Command::parse(row).map_err(|e| Error::new(ErrorKind::InvalidData, e))
            })
        })
        .collect()
}

fn part_1(vec: &[Command]) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;
    for cmd in vec {
        match cmd {
            &Command::Forward(val) => horizontal += val,
            &Command::Down(val) => depth += val,
            &Command::Up(val) => depth -= val,
            &Command::Invalid => {}
        }
    }
    (horizontal, depth)
}

fn part_2(vec: &[Command]) -> (i32, i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for cmd in vec {
        match cmd {
            &Command::Forward(val) => {
                horizontal += val;
                depth += aim * val;
            },
            &Command::Down(val) => aim += val,
            &Command::Up(val) => aim -= val,
            &Command::Invalid => {}
        }
    }
    (horizontal, depth, aim)
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    let (horizontal, depth) = part_1(&vec);
    println!("Horizontal: {},  Depth: {}  --> {}", horizontal, depth, horizontal * depth);
    let (horizontal, depth, aim) = part_2(&vec);
    println!("Horizontal: {},  Depth: {},  Aim: {} --> {}", horizontal, depth, aim, horizontal * depth);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        assert_eq!(Command::parse("forward 1"), Ok(Command::Forward(1)));
        assert_eq!(Command::parse("forward -1"), Ok(Command::Forward(-1)));
        assert_eq!(Command::parse("down 1"), Ok(Command::Down(1)));
        assert_eq!(Command::parse("down -1"), Ok(Command::Down(-1)));
        assert_eq!(Command::parse("up 1"), Ok(Command::Up(1)));
        assert_eq!(Command::parse("up -1"), Ok(Command::Up(-1)));

        assert_eq!(Command::parse("whatever -1"), Ok(Command::Invalid));
        assert!(Command::parse("a b d e").is_err());
    }

    #[test]
    fn test_part_1() {
        let cmds = read(File::open("test1.txt").unwrap()).unwrap();
        let (h, d) = part_1(&cmds);
        assert_eq!(h * d, 150);
    }

    #[test]
    fn test_part_2() {
        let cmds = read(File::open("test1.txt").unwrap()).unwrap();
        let (h, d, a) = part_2(&cmds);
        assert_eq!(a, 10);
        assert_eq!(h * d, 900);
    }
}
