// https://adventofcode.com/2021/day/16

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn read<R: Read>(io: R) -> Vec<bool> {
    let br = BufReader::new(io);
    let line = &br.lines().filter_map(Result::ok).collect::<Vec<String>>()[0];
    line.chars()
        .map(|c| match c {
            '0' => vec![false, false, false, false],
            '1' => vec![false, false, false, true],
            '2' => vec![false, false, true, false],
            '3' => vec![false, false, true, true],
            '4' => vec![false, true, false, false],
            '5' => vec![false, true, false, true],
            '6' => vec![false, true, true, false],
            '7' => vec![false, true, true, true],
            '8' => vec![true, false, false, false],
            '9' => vec![true, false, false, true],
            'A' => vec![true, false, true, false],
            'B' => vec![true, false, true, true],
            'C' => vec![true, true, false, false],
            'D' => vec![true, true, false, true],
            'E' => vec![true, true, true, false],
            'F' => vec![true, true, true, true],
            _ => vec![],
        })
        .flatten()
        .collect()
}

fn bool_vec_to_value(bits: &[bool], num_bits: usize) -> usize {
    let mut value = 0;
    let mut base = 1;
    for i in 0..num_bits {
        value += bits[num_bits - 1 - i] as usize * base;
        base *= 2;
    }
    value
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OperatorType {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    Equal,
}

impl OperatorType {
    fn from_id(id: u8) -> Self {
        match id {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Min,
            3 => OperatorType::Max,
            5 => OperatorType::GreaterThan,
            6 => OperatorType::LessThan,
            7 => OperatorType::Equal,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Literal(u8, usize, usize),
    Operator(u8, OperatorType, Vec<Packet>, usize),
    Padding(usize),
}

impl Packet {
    fn from_bits(bits: &[bool]) -> Self {
        if bits.len() < 3 || bits.iter().filter(|&&b| b).count() == 0 {
            return Packet::Padding(bits.len());
        }
        let version = bits[2] as u8 + bits[1] as u8 * 2 + bits[0] as u8 * 4;
        let type_id = bits[5] as u8 + bits[4] as u8 * 2 + bits[3] as u8 * 4;
        let mut size = 6;
        if type_id == 4 {
            let mut value = 0;
            loop {
                let segment = &bits[size..size + 5];
                value = (value << 4) + bool_vec_to_value(&bits[size + 1..size + 5], 4);
                size += 5;
                if !segment[0] {
                    break;
                }
            }
            return Packet::Literal(version, value, size);
        } else {
            let operator_type = OperatorType::from_id(type_id);
            let length_is_packets = bits[6];
            size += 1;
            let packet_size;
            if length_is_packets {
                packet_size = bool_vec_to_value(&bits[size..], 11);
                size += 11;
            } else {
                packet_size = bool_vec_to_value(&bits[size..], 15);
                size += 15;
            }
            let mut sub_packets: Vec<Packet> = Vec::new();
            let mut sub_bits_read = 0;
            loop {
                if (length_is_packets && sub_packets.len() == packet_size)
                    || (!length_is_packets && sub_bits_read == packet_size)
                {
                    break;
                }
                let package = Packet::from_bits(&bits[size + sub_bits_read..]);
                sub_bits_read += package.size();
                sub_packets.push(package);
            }
            size += sub_bits_read;
            return Packet::Operator(version, operator_type, sub_packets, size);
        }

        unreachable!();
    }

    fn size(&self) -> usize {
        match self {
            &Packet::Padding(s) => s,
            &Packet::Literal(_, _, s) => s,
            &Packet::Operator(_, _, _, s) => s,
        }
    }

    fn version_sum(&self) -> usize {
        match self {
            &Packet::Padding(_) => 0,
            &Packet::Literal(v, _, _) => v as usize,
            Packet::Operator(v, _, packets, _) => {
                let v = *v as usize;
                v + packets.iter().map(|p| p.version_sum()).sum::<usize>()
            }
        }
    }

    fn eval(&self) -> usize {
        match self {
            &Packet::Padding(_) => 0,
            &Packet::Literal(_, val, _) => val,
            Packet::Operator(_, t, packets, _) => {
                let vals = packets.iter().map(|p| p.eval()).collect::<Vec<usize>>();
                match t {
                    OperatorType::Sum => vals.iter().sum(),
                    OperatorType::Product => vals.iter().product(),
                    OperatorType::Min => *vals.iter().min().unwrap(),
                    OperatorType::Max => *vals.iter().max().unwrap(),
                    OperatorType::GreaterThan => (vals[0] > vals[1]) as usize,
                    OperatorType::LessThan => (vals[0] < vals[1]) as usize,
                    OperatorType::Equal => (vals[0] == vals[1]) as usize,
                }
            }
        }
    }
}

fn parse(bits: &[bool]) -> Vec<Packet> {
    let mut read = 0;
    let mut res = Vec::new();
    loop {
        let packet = Packet::from_bits(&bits[read..]);
        read += packet.size();
        res.push(packet);
        if read == bits.len() {
            break;
        }
    }
    res
}

fn part_1(bits: &[bool]) -> usize {
    let packets = parse(bits);
    packets.iter().map(|p| p.version_sum()).sum()
}

fn part_2(bits: &[bool]) -> usize {
    let packets = parse(bits);
    packets[0].eval()
}

fn main() {
    let bits = read(File::open("input.txt").unwrap());
    let version_sum = part_1(&bits);
    println!("Sum of all versions is: {}", version_sum);
    let evaluation = part_2(&bits);
    println!("Evaluated message is: {}", evaluation);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_bool_vec<S: AsRef<str>>(text: S) -> Vec<bool> {
        text.as_ref()
            .chars()
            .map(|c| match c {
                '1' => true,
                '0' => false,
                _ => unreachable!(),
            })
            .collect()
    }

    #[test]
    fn test_file_parse() {
        let bits = read(File::open("test1.txt").unwrap());
        assert_eq!(bits, to_bool_vec("110100101111111000101000"));
        let bits = read(File::open("test2.txt").unwrap());
        assert_eq!(
            bits,
            to_bool_vec("00111000000000000110111101000101001010010001001000000000")
        );
        let bits = read(File::open("test3.txt").unwrap());
        assert_eq!(
            bits,
            to_bool_vec("11101110000000001101010000001100100000100011000001100000")
        );
    }

    #[test]
    fn test_packet_from_bits_parse() {
        let packet = Packet::from_bits(&to_bool_vec("000"));
        assert_eq!(packet, Packet::Padding(3));
        let packet = Packet::from_bits(&to_bool_vec("110100101111111000101000"));
        assert_eq!(packet, Packet::Literal(6, 2021, 21));
        let packet = Packet::from_bits(&to_bool_vec(
            "00111000000000000110111101000101001010010001001000000000",
        ));
        assert_eq!(
            packet,
            Packet::Operator(
                1,
                OperatorType::LessThan,
                vec![Packet::Literal(6, 10, 11), Packet::Literal(2, 20, 16)],
                7 + 15 + 11 + 16
            )
        );
        let packet = Packet::from_bits(&to_bool_vec(
            "11101110000000001101010000001100100000100011000001100000",
        ));
        assert_eq!(
            packet,
            Packet::Operator(
                7,
                OperatorType::Max,
                vec![
                    Packet::Literal(2, 1, 11),
                    Packet::Literal(4, 2, 11),
                    Packet::Literal(1, 3, 11)
                ],
                7 + 11 + 11 + 11 + 11
            )
        );
    }

    #[test]
    fn test_part_1() {
        let bits = read(File::open("test4.txt").unwrap());
        let version_sum = part_1(&bits);
        assert_eq!(version_sum, 16);
        let bits = read(File::open("test5.txt").unwrap());
        let version_sum = part_1(&bits);
        assert_eq!(version_sum, 12);
        let bits = read(File::open("test6.txt").unwrap());
        let version_sum = part_1(&bits);
        assert_eq!(version_sum, 23);
        let bits = read(File::open("test7.txt").unwrap());
        let version_sum = part_1(&bits);
        assert_eq!(version_sum, 31);
    }

    #[test]
    fn test_part_2() {
        let bits = read(File::open("test8.txt").unwrap());
        let evaluation = part_2(&bits);
        assert_eq!(evaluation, 3);
        let bits = read(File::open("test9.txt").unwrap());
        let evaluation = part_2(&bits);
        assert_eq!(evaluation, 54);
        let bits = read(File::open("test10.txt").unwrap());
        let evaluation = part_2(&bits);
        assert_eq!(evaluation, 7);
        let bits = read(File::open("test11.txt").unwrap());
        let evaluation = part_2(&bits);
        assert_eq!(evaluation, 9);
        let bits = read(File::open("test12.txt").unwrap());
        let evaluation = part_2(&bits);
        assert_eq!(evaluation, 1);
        let bits = read(File::open("test13.txt").unwrap());
        let evaluation = part_2(&bits);
        assert_eq!(evaluation, 0);
        let bits = read(File::open("test14.txt").unwrap());
        let evaluation = part_2(&bits);
        assert_eq!(evaluation, 0);
        let bits = read(File::open("test15.txt").unwrap());
        let evaluation = part_2(&bits);
        assert_eq!(evaluation, 1);
    }
}
