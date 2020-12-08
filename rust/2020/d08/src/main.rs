use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[macro_use]
extern crate scan_fmt;

enum OpName {
    ACC, NOP, JMP
}

struct Operation {
    op: OpName,
    value: i32,
}

impl Operation {
    fn parse<S: AsRef<str>>(line: S) -> Result<Operation, scan_fmt::parse::ScanError> {
        let line = line.as_ref();
        let (op_name, sign, value) = scan_fmt!(line, "{/acc|jmp|nop/} {/[+-]/}{d}", String, char, i32)?;
        Ok(Operation {
            op: match op_name.as_ref() {
                "acc" => OpName::ACC,
                "nop" => OpName::NOP,
                "jmp" => OpName::JMP,
                _ => panic!("Huh")
            },
            value: value * (if sign == '-' { -1 } else {1})
        })
    }
}

fn read<R: Read>(io: R) -> Result<Vec<Operation>, Error> {
    BufReader::new(io)
        .lines()
        .map(|line| {
            line.and_then(|row| Operation::parse(row).map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        })
        .collect()
}

fn run(prog: &[Operation]) -> (bool, i32) {
    let mut acc = 0;
    let mut op_cntr = 0;
    let mut executed = HashSet::new();
    loop {
        if op_cntr == prog.len() {
            return (true, acc);
        }
        if executed.contains(&op_cntr) || op_cntr > prog.len() {
            return (false, acc);
        }
        executed.insert(op_cntr);
        match prog[op_cntr].op {
            OpName::ACC => {
                acc += prog[op_cntr].value;
                op_cntr += 1;
            },
            OpName::JMP => {
                op_cntr = (op_cntr as i32 + prog[op_cntr].value) as usize;
            },
            OpName::NOP => {
                op_cntr += 1;
            },
        }
    }
}

fn part1(prog: &[Operation]) -> i32 {
    let (_, result) = run(prog);
    result
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    println!("Read {} instructions", vec.len());
    println!("Accumulation before infinite loop: {}", part1(&vec));
    Ok(())
}
