// https://adventofcode.com/2021/day/24

mod alu;
mod program;

/// This is the de-compiled NOMAD with some additional tracking of values to aid in finding the
/// correct min/max model numbers.  The function is tested against the actual NOMAD ALU
/// implementation in the test section.
fn validate_model_number(model_number: &[i64]) -> ([i64; 14], [Option<i64>; 14]) {
    let divs = [1, 1, 1, 26, 26, 1, 26, 26, 1, 1, 26, 1, 26, 26];
    let add1 = [12, 13, 13, -2, -10, 13, -14, -5, 15, 15, -14, 10, -14, -5];
    let add2 = [7, 8, 10, 4, 4, 6, 11, 13, 1, 8, 4, 13, 4, 14];
    let mut z = 0;
    let mut zs = [0; 14];
    let mut target = [None; 14];
    for (i, &w) in model_number.iter().enumerate() {
        // This is the key-insight to make this tractable: Whenever we see a divide-by-26 we do not
        // want to enter the branch where we scale up z by 26.  Given the residual we get from the
        // current z, we know what digit the model number should have so that we enter the correct
        // branch.  Depending on what this ideal value is (it can be outside the range of allowed
        // digits) we can then update other digitis accordingly through trial-and-error.
        if divs[i] == 26 {
            target[i] = Some(z % 26 + add1[i]);
        }
        if z % 26 + add1[i] == w {
            // NB: We need the last digit to get here with z < 26 (and hope that divs[13] is
            //     always 26), the 'rounding-to-zero' here is what is setting the final z to 0.
            z /= divs[i];
        } else {
            z = (z / divs[i]) * 26 + (w + add2[i])
        }
        zs[i] = z;
    }
    (zs, target)
}

fn part_1() -> i64 {
    //let model_number = [7, 3, 1, 9, 1, 9, 1, 9, 9, 9, 3, 9, 8, 5];
    //let model_number = [7, 4, 1, 9, 2, 9, 1, 9, 9, 9, 3, 9, 8, 5];
    let model_number = [7, 9, 1, 9, 7, 9, 1, 9, 9, 9, 3, 9, 8, 5];
    let (zs, trgtz) = validate_model_number(&model_number);
    println!("{:?}", zs);
    println!("{:?}", trgtz);
    println!("{:?}", zs.iter().map(|x| x % 26).collect::<Vec<i64>>());
    let mut res = model_number[0];
    model_number[1..14].iter().for_each(|n| res = res * 10 + n);
    res
}

fn part_2() -> i64 {
    //let model_number = [1, 8, 1, 9, 6, 9, 1, 3, 5, 7, 1, 2, 1, 1];
    //let model_number = [1, 7, 1, 9, 5, 9, 1, 3, 5, 7, 1, 2, 1, 1];
    //                  ?  ?  ?  !  !  ?  !  !  ?  ?  !  ?  !  !
    let model_number = [1, 3, 1, 9, 1, 9, 1, 3, 5, 7, 1, 2, 1, 1];
    let (zs, trgtz) = validate_model_number(&model_number);
    println!("{:?}", zs);
    println!("{:?}", trgtz);
    println!("{:?}", zs.iter().map(|x| x % 26).collect::<Vec<i64>>());
    let mut res = model_number[0];
    model_number[1..14].iter().for_each(|n| res = res * 10 + n);
    res
}

fn main() {
    println!("Result of part 1: {}", part_1());
    println!("Result of part 2: {}", part_2());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alu::Alu;
    use crate::program::{Program, Reg, RegOrVal};
    use std::fs::File;
    use std::io::Read;

    fn read<R: Read>(mut io: R) -> Program {
        let mut s = String::new();
        io.read_to_string(&mut s).unwrap();
        s.parse::<Program>().unwrap()
    }

    #[test]
    fn test_decompiled() {
        // All this nice implementation work to get the ALU working for nought. But at least it
        // serves as a test that I have 'decompiled' the function correctly (and I had indeed
        // messed up copying out the various values correctly..).
        let nomad = read(File::open("input.txt").unwrap());

        let mut alu = Alu::default();
        let model_number = [1, 3, 1, 9, 1, 9, 1, 3, 5, 7, 1, 2, 1, 1];
        let mut input = model_number.into_iter().rev().collect();
        alu.run(&nomad, &mut input).unwrap();
        let (zs, _) = validate_model_number(&model_number);
        assert_eq!(zs[13], alu.fetch(&RegOrVal::Reg(Reg::Z)));

        let mut alu = Alu::default();
        let model_number = [7, 9, 1, 9, 7, 9, 1, 9, 9, 9, 3, 9, 8, 5];
        let mut input = model_number.into_iter().rev().collect();
        alu.run(&nomad, &mut input).unwrap();
        let (zs, _) = validate_model_number(&model_number);
        assert_eq!(zs[13], alu.fetch(&RegOrVal::Reg(Reg::Z)));
    }
}
