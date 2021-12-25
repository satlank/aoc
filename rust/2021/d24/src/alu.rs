use crate::program::{Op, Program, Reg, RegOrVal};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Alu {
    reg: [i64; 4],
}

impl Display for Alu {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "x: {:10}, y: {:10}, z: {:10}, w: {:10}",
            self.reg[0], self.reg[1], self.reg[2], self.reg[3]
        )?;
        Ok(())
    }
}

impl Alu {
    pub fn fetch(&self, rov: &RegOrVal) -> i64 {
        match rov {
            RegOrVal::Reg(r) => self.reg[r.i()],
            RegOrVal::Val(v) => *v,
        }
    }

    fn set(&mut self, r: &Reg, val: i64) {
        self.reg[r.i()] = val;
    }

    pub fn step(&mut self, op: &Op, input: &mut Vec<i64>) -> Result<(), String> {
        match op {
            Op::Inp(r) => {
                println!("ALU: {}", self.fetch(&RegOrVal::Reg(Reg::Z)));
                let val = input.pop().unwrap();
                self.set(r, val);
            }
            Op::Add(r, rov) => {
                let left = self.fetch(&RegOrVal::Reg(*r));
                let right = self.fetch(rov);
                let res = left + right;
                self.set(r, res);
            }
            Op::Mul(r, rov) => {
                let left = self.fetch(&RegOrVal::Reg(*r));
                let right = self.fetch(rov);
                let res = left * right;
                self.set(r, res);
            }
            Op::Div(r, rov) => {
                let left = self.fetch(&RegOrVal::Reg(*r));
                let right = self.fetch(rov);
                if right == 0 {
                    return Err("Attempted to divide by 0".to_string());
                }
                let res = left / right;
                self.set(r, res);
            }
            Op::Mod(r, rov) => {
                let left = self.fetch(&RegOrVal::Reg(*r));
                let right = self.fetch(rov);
                if left < 0 || right <= 0 {
                    return Err(format!("Bad modulo operation {} % {}", left, right));
                }
                let res = left % right;
                self.set(r, res);
            }
            Op::Eql(r, rov) => {
                let left = self.fetch(&RegOrVal::Reg(*r));
                let right = self.fetch(rov);
                let res = if left == right { 1 } else { 0 };
                self.set(r, res);
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn run(&mut self, prog: &Program, input: &mut Vec<i64>) -> Result<(), String> {
        for op in &prog.ops {
            self.step(op, input)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        let prog = "inp x\nmul x -1".parse::<Program>().unwrap();
        let mut alu = Alu::default();
        let mut input = vec![12];
        alu.step(&prog.ops[0], &mut input).unwrap();
        assert_eq!(alu.reg[0], 12);
        alu.step(&prog.ops[1], &mut input).unwrap();
        assert_eq!(alu.reg[0], -12);
    }

    #[test]
    fn test_add() {
        let mut alu = Alu::default();
        alu.reg[0] = 1;
        alu.reg[1] = 2;
        let op = "add x y".parse::<Op>().unwrap();
        alu.step(&op, &mut vec![]).unwrap();
        assert_eq!(alu.reg[0], 3);
    }

    #[test]
    fn test_mul() {
        let mut alu = Alu::default();
        alu.reg[0] = 99;
        alu.reg[1] = -123412;
        let op = "mul x y".parse::<Op>().unwrap();
        alu.step(&op, &mut vec![]).unwrap();
        assert_eq!(alu.reg[0], 99 * -123412);
    }

    #[test]
    fn test_div() {
        let mut alu = Alu::default();
        alu.reg[0] = -3;
        alu.reg[1] = 2;
        let op = "div x y".parse::<Op>().unwrap();
        alu.step(&op, &mut vec![]).unwrap();
        assert_eq!(alu.reg[0], -1);
    }

    #[test]
    fn test_mod() {
        let mut alu = Alu::default();
        alu.reg[0] = 3;
        alu.reg[1] = 2;
        let op = "mod x y".parse::<Op>().unwrap();
        alu.step(&op, &mut vec![]).unwrap();
        assert_eq!(alu.reg[0], 1);
    }

    #[test]
    fn test_eql() {
        let mut alu = Alu::default();
        alu.reg[0] = 3;
        alu.reg[1] = 2;
        let op = "eql x y".parse::<Op>().unwrap();
        alu.step(&op, &mut vec![]).unwrap();
        assert_eq!(alu.reg[0], 0);
    }

    #[test]
    fn test_example1() {
        let prog = "inp z\ninp x\nmul z 3\neql z x".parse::<Program>().unwrap();
        let mut alu = Alu::default();
        alu.run(&prog, &mut vec![3, 1]).unwrap();
        assert_eq!(alu.fetch(&RegOrVal::Reg(Reg::Z)), 1);

        let mut alu = Alu::default();
        alu.run(&prog, &mut vec![0, 1]).unwrap();
        assert_eq!(alu.fetch(&RegOrVal::Reg(Reg::Z)), 0);
    }

    #[test]
    fn test_example2() {
        let prog = "inp w\nadd z w\nmod z 2\ndiv w 2\nadd y w\nmod y 2\ndiv w 2\nadd x w\nmod x 2\ndiv w 2\nmod w 2".parse::<Program>().unwrap();
        let mut alu = Alu::default();
        alu.run(&prog, &mut vec![0]).unwrap();
        assert_eq!(alu.reg, [0; 4]);

        let mut alu = Alu::default();
        alu.run(&prog, &mut vec![1]).unwrap();
        assert_eq!(alu.reg, [0, 0, 1, 0]);

        let mut alu = Alu::default();
        alu.run(&prog, &mut vec![2]).unwrap();
        assert_eq!(alu.reg, [0, 1, 0, 0]);

        let mut alu = Alu::default();
        alu.run(&prog, &mut vec![4]).unwrap();
        assert_eq!(alu.reg, [1, 0, 0, 0]);

        let mut alu = Alu::default();
        alu.run(&prog, &mut vec![8]).unwrap();
        assert_eq!(alu.reg, [0, 0, 0, 1]);

        let mut alu = Alu::default();
        alu.run(&prog, &mut vec![15]).unwrap();
        assert_eq!(alu.reg, [1, 1, 1, 1]);
    }
}
