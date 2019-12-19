use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use itertools::EitherOrBoth::Both;

#[macro_use] extern crate scan_fmt;

use std::collections::HashMap;

fn parse_line(line: &String) -> (String, String) {
    let (parent, child) = scan_fmt_some!(line, "{[A-Z0-9]}){[A-Z0-9]}", String, String);
    (parent.unwrap(), child.unwrap())
}

fn parse_input(input: &Vec<String>) -> HashMap<String, String> {
    // Break up the orbital map into a HashMap mapping each object to one it
    // is orbiting, representin the orbital tree
    let mut res = HashMap::new();
    for line in input.iter() {
        let (parent, child) = parse_line(line);
        res.insert(child, parent);  // Each body orbits exactly one thing
    }
    res.insert("COM".to_string(), "COM".to_string());
    return res;
}

fn count_total_orbits(map: &HashMap<String, String>) -> i32 {
    // Walk the tree from each body to COM and count the steps
    let mut len = 0;
    for (key, val) in map {
        let mut check_key = key;
        let mut check_val = val;
        while check_val != check_key {
            len += 1;
            check_key = check_val;
            check_val = &map[check_key];
        }
    }
    return len;
}

fn get_path(object: String, map: &HashMap<String, String>) -> Vec<String> {
    // Return all bodies from COM to the body `object` is in orbit of (does not
    // include object itself!)
    let mut res = Vec::new();

    let mut key = &object;
    let mut val = &map[&object];

    while key != val {
        res.insert(0, val.clone());
        key = val;
        val = &map[key];
    }
    return res;
}

fn get_orbital_transfers(path1: &Vec<String>, path2: &Vec<String>) -> usize {
    // Given two paths (borth starting at COM) find the number of objects that
    // are common, then the orbital transfer from path1 to path2 is simply the
    // sum of their lengths to the common point
    let mut common = 0;
    for it in path1.iter().zip_longest(path2.iter()) {
        match it {
            Both(x, y) => if x == y { common += 1; } else { break },
            _ => break
        }
    }
    (path1.len() - common) + (path2.len() - common)
}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let map = parse_input(&input);
    let len = count_total_orbits(&map);
    println!("Total orbits: {}", len);

    let my_path = get_path("YOU".to_string(), &map);
    let santa_path = get_path("SAN".to_string(), &map);
    let orbital_transfers = get_orbital_transfers(&my_path, &santa_path);
    println!("Orbital Transfers: {}", orbital_transfers);

    println!("{:?}", map);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line(&"ABC)D".to_string()), ("ABC".to_string(), "D".to_string()));
        assert_eq!(parse_line(&"K)L".to_string()), ("K".to_string(), "L".to_string()));
    }

    #[test]
    fn test_parse_input() {
        let input = vec!["A)B".to_string(), "B)D".to_string(), "A)X".to_string()];
        let map = parse_input(&input);
        assert_eq!(map.len(), 4);
    }

    #[test]
    fn test_part1_ex1() {
        let input = vec![
            "COM)B".to_string(),
            "B)C".to_string(),
            "C)D".to_string(),
            "D)E".to_string(),
            "E)F".to_string(),
            "B)G".to_string(),
            "G)H".to_string(),
            "D)I".to_string(),
            "E)J".to_string(),
            "J)K".to_string(),
            "K)L".to_string(),
        ];
        let map = parse_input(&input);
        assert_eq!(count_total_orbits(&map), 42);
    }

    #[test]
    fn test_get_path() {
        let input = vec![
            "COM)A".to_string(),
            "A)B".to_string(),
            "B)D".to_string(),
            "A)X".to_string()
        ];
        let map = parse_input(&input);
        assert_eq!(get_path("A".to_string(), &map), ["COM"]);
        assert_eq!(get_path("X".to_string(), &map), ["COM", "A"]);
        assert_eq!(get_path("D".to_string(), &map), ["COM", "A", "B"]);
    }

    #[test]
    fn test_part2_ex1() {
        let input = vec![
            "COM)B".to_string(),
            "B)C".to_string(),
            "C)D".to_string(),
            "D)E".to_string(),
            "E)F".to_string(),
            "B)G".to_string(),
            "G)H".to_string(),
            "D)I".to_string(),
            "E)J".to_string(),
            "J)K".to_string(),
            "K)L".to_string(),
            "K)YOU".to_string(),
            "I)SAN".to_string(),
        ];
        let map = parse_input(&input);
        let my_path = get_path("YOU".to_string(), &map);
        let santa_path = get_path("SAN".to_string(), &map);
        assert_eq!(get_orbital_transfers(&my_path, &santa_path), 4);
    }
}
