// https://adventofcode.com/2024/day/17

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    a: usize,
    b: usize,
    c: usize,
    iptr: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Computer {
    state: State,
    program: Vec<u8>,
}

impl Computer {
    fn run(&self) -> (Vec<u8>, State) {
        let mut output = Vec::new();
        let mut state = self.state;
        loop {
            match state.step(&self.program) {
                StepResult::Update(new_state) => state = new_state,
                StepResult::Output(new_state, val) => {
                    output.push(val);
                    state = new_state
                }
                StepResult::Halt => break,
            }
        }
        (output, state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StepResult {
    Update(State),
    Output(State, u8),
    Halt,
}

impl State {
    fn step(&self, program: &[u8]) -> StepResult {
        if self.iptr >= program.len() {
            return StepResult::Halt;
        }
        assert!(self.iptr + 1 < program.len());
        let op = Op::from((program[self.iptr], program[self.iptr + 1]));
        match op {
            Op::Adv(operand) => StepResult::Update(self.adv(operand)),
            Op::Bxl(operand) => StepResult::Update(self.bxl(operand)),
            Op::Bst(operand) => StepResult::Update(self.bst(operand)),
            Op::Jnz(operand) => StepResult::Update(self.jnz(operand)),
            Op::Bxc => StepResult::Update(self.bxc()),
            Op::Out(operand) => {
                let res = self.out(operand);
                StepResult::Output(res.0, res.1)
            }
            Op::Bdv(operand) => StepResult::Update(self.bdv(operand)),
            Op::Cdv(operand) => StepResult::Update(self.cdv(operand)),
        }
    }

    fn adv(&self, operand: ComboOperand) -> State {
        let mut new_state = *self;
        let numerator = self.a;
        let denominator = 1 << operand.val(self);
        new_state.a = numerator / denominator;
        new_state.iptr += 2;
        new_state
    }

    fn bxl(&self, operand: LiteralOperand) -> State {
        let mut new_state = *self;
        new_state.b = self.b ^ operand.0 as usize;
        new_state.iptr += 2;
        new_state
    }

    fn bst(&self, operand: ComboOperand) -> State {
        let mut new_state = *self;
        new_state.b = operand.val(self) % 8;
        new_state.iptr += 2;
        new_state
    }

    fn jnz(&self, operand: LiteralOperand) -> State {
        let mut new_state = *self;
        if self.a != 0 {
            new_state.iptr = operand.0 as usize;
        } else {
            new_state.iptr += 2;
        }
        new_state
    }

    fn bxc(&self) -> State {
        let mut new_state = *self;
        new_state.b = self.b ^ self.c;
        new_state.iptr += 2;
        new_state
    }

    fn out(&self, operand: ComboOperand) -> (State, u8) {
        let mut new_state = *self;
        let val = (operand.val(self) % 8) as u8;
        new_state.iptr += 2;
        (new_state, val)
    }

    fn bdv(&self, operand: ComboOperand) -> State {
        let mut new_state = *self;
        let numerator = self.a;
        let denominator = 1 << operand.val(self);
        new_state.b = numerator / denominator;
        new_state.iptr += 2;
        new_state
    }

    fn cdv(&self, operand: ComboOperand) -> State {
        let mut new_state = *self;
        let numerator = self.a;
        let denominator = 1 << operand.val(self);
        new_state.c = numerator / denominator;
        new_state.iptr += 2;
        new_state
    }
}

#[derive(Debug, Copy, Clone)]
enum ComboOperand {
    Literal(u8),
    RegA,
    RegB,
    RegC,
}

impl ComboOperand {
    fn val(&self, state: &State) -> usize {
        match self {
            ComboOperand::Literal(v) => *v as usize,
            ComboOperand::RegA => state.a,
            ComboOperand::RegB => state.b,
            ComboOperand::RegC => state.c,
        }
    }
}

impl From<u8> for ComboOperand {
    fn from(v: u8) -> Self {
        match v {
            0..4 => ComboOperand::Literal(v),
            4 => ComboOperand::RegA,
            5 => ComboOperand::RegB,
            6 => ComboOperand::RegC,
            7.. => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct LiteralOperand(u8);

impl From<u8> for LiteralOperand {
    fn from(v: u8) -> Self {
        assert!(v <= 7);
        Self(v)
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Adv(ComboOperand),
    Bxl(LiteralOperand),
    Bst(ComboOperand),
    Jnz(LiteralOperand),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl From<(u8, u8)> for Op {
    fn from((v1, v2): (u8, u8)) -> Self {
        match v1 {
            0 => Op::Adv(v2.into()),
            1 => Op::Bxl(v2.into()),
            2 => Op::Bst(v2.into()),
            3 => Op::Jnz(v2.into()),
            4 => Op::Bxc,
            5 => Op::Out(v2.into()),
            6 => Op::Bdv(v2.into()),
            7 => Op::Cdv(v2.into()),
            8.. => unreachable!(),
        }
    }
}

fn read<R: Read>(io: R) -> Computer {
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines().map_while(Result::ok).collect();
    let state = State {
        a: lines[0][12..].parse().unwrap(),
        b: lines[1][12..].parse().unwrap(),
        c: lines[2][12..].parse().unwrap(),
        iptr: 0,
    };
    let program = lines[4][9..]
        .split(',')
        .map(|e| e.parse().unwrap())
        .collect();

    Computer { state, program }
}

fn part_1(computer: &Computer) -> String {
    computer
        .run()
        .0
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part_2(computer: &Computer) -> usize {
    let mut new_computer = computer.clone();
    let mut cnt = 0;
    let mut len = computer.program.len();

    // We are going to assemble the required state a by matching the output values from the end.
    // This is based on the observation that the program is a series of operations that operate on
    // 3 bit values and shift them by 3 bits on each iteration until the value of register a is
    // zero.  This is from 'decompiling' the program.
    while len > 0 {
        cnt *= 8; // we shift by 3 bits on each iteration
        let mut x = 0;
        loop {
            // Now we need to find the value of register a that produces the correct next output
            // value while preserving the previous output values.  We iterate over the possible
            // in increasing order until we find the correct value, and then move on the the next
            // digit (which as we are building the value from the end, is the previous digit in the
            // input program).
            new_computer.state.a = cnt + x;
            let (output, _) = new_computer.run();
            if output == new_computer.program[(len - 1)..] {
                break;
            }
            x += 1;
        }
        len -= 1;
        cnt += x;
    }
    cnt
}

fn main() {
    let computer = read(File::open("input.txt").unwrap());
    let p1 = part_1(&computer);
    println!("Part 1: {}", p1);
    let p2 = part_2(&computer);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let computer = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&computer), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn example_2() {
        let mut computer = read(File::open("example2.txt").unwrap());
        computer.state.a = 117440;
        assert_eq!(computer.run().0, computer.program);
        assert_eq!(part_2(&computer), 117440);
    }

    #[test]
    fn simple_1() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let computer = Computer {
            state: State {
                a: 0,
                b: 0,
                c: 9,
                iptr: 0,
            },
            program: vec![2, 6],
        };
        let (output, state) = computer.run();
        assert_eq!(output, []);
        assert_eq!(state.b, 1);
    }

    #[test]
    fn simple_2() {
        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let computer = Computer {
            state: State {
                a: 10,
                b: 0,
                c: 0,
                iptr: 0,
            },
            program: vec![5, 0, 5, 1, 5, 4],
        };
        let (output, _) = computer.run();
        assert_eq!(output, [0, 1, 2]);
    }

    #[test]
    fn simple_3() {
        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        let computer = Computer {
            state: State {
                a: 2024,
                b: 0,
                c: 0,
                iptr: 0,
            },
            program: vec![0, 1, 5, 4, 3, 0],
        };
        let (output, state) = computer.run();
        assert_eq!(output, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(state.a, 0);
    }

    #[test]
    fn simple_4() {
        // If register B contains 29, the program 1,7 would set register B to 26.
        let computer = Computer {
            state: State {
                a: 0,
                b: 29,
                c: 0,
                iptr: 0,
            },
            program: vec![1, 7],
        };
        let (output, state) = computer.run();
        assert_eq!(output, []);
        assert_eq!(state.b, 26);
    }

    #[test]
    fn simple_5() {
        // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
        let computer = Computer {
            state: State {
                a: 0,
                b: 2024,
                c: 43690,
                iptr: 0,
            },
            program: vec![4, 0],
        };
        let (output, state) = computer.run();
        assert_eq!(output, []);
        assert_eq!(state.b, 44354);
    }
}
