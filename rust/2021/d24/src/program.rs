use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Program {
    pub ops: Vec<Op>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for op in &self.ops {
            writeln!(f, "{}", op)?
        }
        Ok(())
    }
}

impl FromStr for Program {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(Self {
            ops: s.lines().map(|line| line.parse::<Op>().unwrap()).collect(),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Reg {
    X,
    Y,
    Z,
    W,
}

impl Reg {
    pub fn i(&self) -> usize {
        match self {
            Reg::X => 0,
            Reg::Y => 1,
            Reg::Z => 2,
            Reg::W => 3,
        }
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Reg::X => 'x',
                Reg::Y => 'y',
                Reg::Z => 'z',
                Reg::W => 'w',
            }
        )?;
        Ok(())
    }
}

impl FromStr for Reg {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "x" => Ok(Reg::X),
            "y" => Ok(Reg::Y),
            "z" => Ok(Reg::Z),
            "w" => Ok(Reg::W),
            _ => Err("Register must be one of [x, y, z, w]"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RegOrVal {
    Reg(Reg),
    Val(i64),
}

impl Display for RegOrVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            RegOrVal::Val(v) => write!(f, "{}", v)?,
            RegOrVal::Reg(r) => write!(f, "{}", r)?,
        }
        Ok(())
    }
}

impl FromStr for RegOrVal {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s.parse::<i64>() {
            Ok(x) => Ok(RegOrVal::Val(x)),
            _ => Ok(RegOrVal::Reg(s.parse::<Reg>()?)),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Op {
    Inp(Reg),
    Add(Reg, RegOrVal),
    Mul(Reg, RegOrVal),
    Div(Reg, RegOrVal),
    Mod(Reg, RegOrVal),
    Eql(Reg, RegOrVal),
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Op::Inp(r) => write!(f, "inp {}", r)?,
            Op::Add(r, rov) => write!(f, "add {} {}", r, rov)?,
            Op::Mul(r, rov) => write!(f, "mul {} {}", r, rov)?,
            Op::Div(r, rov) => write!(f, "div {} {}", r, rov)?,
            Op::Mod(r, rov) => write!(f, "mod {} {}", r, rov)?,
            Op::Eql(r, rov) => write!(f, "eql {} {}", r, rov)?,
        }
        Ok(())
    }
}

impl FromStr for Op {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let comps = s.split(' ').collect::<Vec<&str>>();
        match comps[0] {
            "inp" => Ok(Op::Inp(comps[1].parse::<Reg>()?)),
            "add" => Ok(Op::Add(comps[1].parse::<Reg>()?, comps[2].parse::<RegOrVal>()?)),
            "mul" => Ok(Op::Mul(comps[1].parse::<Reg>()?, comps[2].parse::<RegOrVal>()?)),
            "div" => Ok(Op::Div(comps[1].parse::<Reg>()?, comps[2].parse::<RegOrVal>()?)),
            "mod" => Ok(Op::Mod(comps[1].parse::<Reg>()?, comps[2].parse::<RegOrVal>()?)),
            "eql" => Ok(Op::Eql(comps[1].parse::<Reg>()?, comps[2].parse::<RegOrVal>()?)),
            _ => Err("Bad operation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let p = "inp x\ninp y\nadd x 12".parse::<Program>().unwrap();
        assert_eq!(p.ops, vec![Op::Inp(Reg::X), Op::Inp(Reg::Y), Op::Add(Reg::X, RegOrVal::Val(12))]);
    }
}
