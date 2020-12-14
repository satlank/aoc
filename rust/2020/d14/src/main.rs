use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::HashMap;

#[macro_use]
extern crate scan_fmt;

#[derive(Debug)]
enum Command {
    Mask(String),
    Mem(u64, u64),
}

impl Command {
    fn parse<S: AsRef<str>>(cmd: S) -> Result<Command, Error> {
        let cmd = cmd.as_ref();
        if cmd[..4] == *"mask" {
            let mask = scan_fmt!(cmd, "mask = {}", String).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            return Ok(Command::Mask(mask));
        } else {
            let (loc, val) = scan_fmt!(cmd, "mem[{d}] = {d}", u64, u64).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            return Ok(Command::Mem(loc, val));
        }
   }
}

struct VirtualMachine {
    mask: String,
    memory: HashMap<u64, u64>,
    version: u8,
}

fn mask_value(mask: &String, val: u64) -> u64 {
    let mask1 = u64::from_str_radix(
        &mask
            .chars()
            .map(|c| if c != '1' { '0' } else { '1' })
            .collect::<String>(),
        2
    ).unwrap();
    let mask2 = u64::from_str_radix(
        &mask
            .chars()
            .map(|c| if c != '0' { '1' } else { '0' })
            .collect::<String>(),
        2
    ).unwrap();
    !(!(val | mask1) | !mask2)
}

fn mask_addr(mask: &String, addr: u64) -> Vec<u64> {
    let mut addrs: Vec<String> = Vec::new();
    addrs.push(String::new());
    let str_addr = format!("{:036b}", addr);
    for i in 0..mask.len() {
        if &mask[i..i+1] == "X" {
            let mut new_addrs: Vec<String> = Vec::new();
            for a in &addrs {
                new_addrs.push(format!("{}0", a));
                new_addrs.push(format!("{}1", a));
            }
            addrs = new_addrs;
        } else {
            for a in &mut addrs {
                a.push(if &mask[i..i+1] == "1" { '1' } else {str_addr.chars().nth(i).unwrap()});
            }
        }
    }
    addrs.iter()
        .map(|a| u64::from_str_radix(a, 2).unwrap())
        .collect()
}

impl VirtualMachine {
    fn new(version: u8) -> VirtualMachine {
        VirtualMachine {
            mask: format!("{:036b}", 0),
            memory: HashMap::new(),
            version: version
        }
    }

    fn apply(&mut self, cmd: &Command) {
        match cmd {
            Command::Mask(str_mask) => {
                self.mask = str_mask.to_string();
            },
            Command::Mem(addr, val) => {
                if self.version == 1 {
                    self.memory.insert(*addr, mask_value(&self.mask, *val));
                } else {
                    let addrs = mask_addr(&self.mask, *addr);
                    for a in addrs {
                        self.memory.insert(a, *val);
                    }
                }
            },
        }
    }
}

fn read<R: Read>(io: R) -> Result<Vec<Command>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| Command::parse(v).map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn part1(cmds: &[Command]) -> u64 {
    let mut machine = VirtualMachine::new(1);
    for cmd in cmds.iter() {
        machine.apply(cmd);
    }
    machine.memory.values().sum()
}

fn part2(cmds: &[Command]) -> u64 {
    let mut machine = VirtualMachine::new(2);
    for cmd in cmds.iter() {
        machine.apply(cmd);
    }
    machine.memory.values().sum()
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    println!("Read {} commands", vec.len());
    println!("{:?}", &vec[..2]);
    println!("Sum of all values in memory after completion: {}", part1(&vec));
    println!("Sum of all values in memory after completion v2: {}", part2(&vec));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ops = vec![
            Command::Mask("000000000000000000000000000000X1001X".to_string()),
            Command::Mem(42, 100),
            Command::Mask("00000000000000000000000000000000X0XX".to_string()),
            Command::Mem(26, 1),
        ];
        assert_eq!(part2(&ops), 208);
    }
}
