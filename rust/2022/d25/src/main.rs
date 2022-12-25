// https://adventofcode.com/2022/day/25

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn read<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines().filter_map(Result::ok).collect()
}

fn part_1(input: &[String]) -> String {
    let sum = input.iter().map(|s| snafu_to_decimal(s)).sum::<i64>();
    println!("{}", sum);
    decimal_to_snafu(sum)
}

fn resovle_digit(c: char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => unreachable!(),
    }
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(place, c)| 5i64.pow(place as u32) * resovle_digit(c))
        .sum()
}

fn max_possible(num_digits: u32) -> i64 {
    (0..num_digits).map(|i| 2 * 5i64.pow(i)).sum()
}

fn find_leading(target: i64, digit: u32) -> (i64, char) {
    let max_next_level = max_possible(digit);
    let val_2 = target - 2 * 5i64.pow(digit);
    let val_1 = target - 1 * 5i64.pow(digit);
    let val_0 = target - 0 * 5i64.pow(digit);
    let val_m1 = target + 1 * 5i64.pow(digit);
    let val_m2 = target + 2 * 5i64.pow(digit);

    if -max_next_level <= val_2 && val_2 <= max_next_level {
        return (val_2, '2');
    } else if -max_next_level <= val_1 && val_1 <= max_next_level {
        return (val_1, '1');
    } else if -max_next_level <= val_0 && val_0 <= max_next_level {
        return (val_0, '0');
    } else if -max_next_level <= val_m1 && val_m1 <= max_next_level {
        return (val_m1, '-');
    } else if -max_next_level <= val_m2 && val_m2 <= max_next_level {
        return (val_m2, '=');
    }

    unreachable!()
}

fn decimal_to_snafu(mut val: i64) -> String {
    assert!(val >= 0);
    let mut snafu = String::new();
    let mut digits = 1;
    while val / max_possible(digits) != 0 {
        digits += 1;
    }
    digits -= 1;
    let mut leading_zero = true;
    while digits > 0 {
        let (remainder, c) = find_leading(val, digits);
        val = remainder;
        if leading_zero && c != '0' {
            leading_zero = false;
        }
        if !leading_zero {
            snafu.push(c);
        }
        digits -= 1;
    }
    assert!(val >= -2 && val <= 2, "{} is not expected", val);
    if val == 2 {
        snafu.push('2');
    } else if val == 1 {
        snafu.push('1');
    } else if val == 0 {
        snafu.push('0');
    } else if val == -1 {
        snafu.push('-');
    } else if val == -2 {
        snafu.push('=');
    }

    snafu
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input);
    println!("Part 1: {}", p1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s2d() {
        assert_eq!(snafu_to_decimal("1"), 1);
        assert_eq!(snafu_to_decimal("2"), 2);
        assert_eq!(snafu_to_decimal("1="), 3);
        assert_eq!(snafu_to_decimal("1-"), 4);
        assert_eq!(snafu_to_decimal("10"), 5);
        assert_eq!(snafu_to_decimal("11"), 6);
        assert_eq!(snafu_to_decimal("12"), 7);
        assert_eq!(snafu_to_decimal("2="), 8);
        assert_eq!(snafu_to_decimal("2-"), 9);
        assert_eq!(snafu_to_decimal("20"), 10);
        assert_eq!(snafu_to_decimal("1=0"), 15);
        assert_eq!(snafu_to_decimal("1-0"), 20);
        assert_eq!(snafu_to_decimal("1=11-2"), 2022);
        assert_eq!(snafu_to_decimal("1-0---0"), 12345);
        assert_eq!(snafu_to_decimal("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn test_d2s() {
        assert_eq!("1", decimal_to_snafu(1));
        assert_eq!("2", decimal_to_snafu(2));
        assert_eq!("1=", decimal_to_snafu(3));
        assert_eq!("1-", decimal_to_snafu(4));
        assert_eq!("10", decimal_to_snafu(5));
        assert_eq!("11", decimal_to_snafu(6));
        assert_eq!("12", decimal_to_snafu(7));
        assert_eq!("2=", decimal_to_snafu(8));
        assert_eq!("2-", decimal_to_snafu(9));
        assert_eq!("20", decimal_to_snafu(10));
        assert_eq!("1=0", decimal_to_snafu(15));
        assert_eq!("1-0", decimal_to_snafu(20));
        assert_eq!("1=11-2", decimal_to_snafu(2022));
        assert_eq!("1-0---0", decimal_to_snafu(12345));
        assert_eq!("1121-1110-1=0", decimal_to_snafu(314159265));
    }

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input), "2=-1=0");
    }
}
