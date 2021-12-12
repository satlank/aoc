// https://adventofcode.com/2021/day/12

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[macro_use]
extern crate scan_fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Cave {
    Large(String),
    Small(String),
    Start,
    End,
}

impl Cave {
    fn parse<S: AsRef<str>>(name: S) -> Cave {
        let name = name.as_ref();
        if name == "start" {
            return Cave::Start;
        }
        if name == "end" {
            return Cave::End;
        }
        if name.chars().filter(|x| x.is_ascii_lowercase()).count() == 0 {
            return Cave::Large(name.to_string());
        }
        return Cave::Small(name.to_string());
    }
}

fn read<R: Read>(io: R) -> Vec<(Cave, Cave)> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| {
            let line = line.unwrap();
            let (a, b) = scan_fmt!(&line, "{}-{}", String, String).unwrap();
            (Cave::parse(a), Cave::parse(b))
        })
        .collect()
}

fn build_tree(connections: &[(Cave, Cave)]) -> HashMap<Cave, HashSet<Cave>> {
    let mut res: HashMap<Cave, HashSet<Cave>> = HashMap::new();
    for con in connections {
        res.entry(con.0.clone())
            .or_insert(HashSet::new())
            .insert(con.1.clone());
        if con.1 != Cave::End {
            res.entry(con.1.clone())
                .or_insert(HashSet::new())
                .insert(con.0.clone());
        }
    }
    res.entry(Cave::End).or_insert(HashSet::new());
    res
}

fn can_add(path: &[Cave], cave: &Cave) -> bool {
    if !path.contains(&cave) {
        //println!("{:?} can be added to {:?}", cave, path);
        true
    } else {
        match cave {
            Cave::Large(_) => {
                //println!("{:?} can be added to {:?}", cave, path);
                true
            }
            _ => {
                if path.last().unwrap() == &Cave::Large("HN".to_string()) {
                    println!("{:?} can NOT be added to {:?}", cave, path);
                }
                false
            }
        }
    }
}

fn can_add_small_twice(path: &[Cave], cave: &Cave) -> bool {
    if !path.contains(&cave) {
        true
    } else {
        match cave {
            Cave::Large(_) => true,
            Cave::Small(_) => {
                let mut tmp: HashSet<&Cave> = HashSet::new();
                path.iter()
                    .filter(|x| match x {
                        Cave::Small(_) => true,
                        _ => false,
                    })
                    .filter(|c| {
                        if tmp.contains(c) {
                            true
                        } else {
                            tmp.insert(c);
                            false
                        }
                    })
                    .count()
                    == 0
            }
            _ => false,
        }
    }
}

fn find_paths(
    caves: &HashMap<Cave, HashSet<Cave>>,
    adding: &dyn Fn(&[Cave], &Cave) -> bool,
) -> Vec<Vec<Cave>> {
    let mut paths = vec![vec![Cave::Start]];
    loop {
        let mut all_completed = true;
        for i in 0..paths.len() {
            let last = paths[i].last().unwrap();
            if last == &Cave::End {
            } else {
                let possible_steps: Vec<&Cave> = caves
                    .get(last)
                    .unwrap()
                    .iter()
                    .filter(|c| adding(&paths[i], c))
                    .collect();
                if possible_steps.len() > 0 {
                    all_completed = false;
                    if possible_steps.len() > 1 {
                        let new_paths = possible_steps[1..]
                            .iter()
                            .map(|&c| {
                                let mut np = paths[i].clone();
                                np.push(c.clone());
                                np
                            })
                            .collect::<Vec<Vec<Cave>>>();
                        paths.extend(new_paths);
                    }
                    paths[i].push(possible_steps[0].clone());
                }
            }
        }
        if all_completed {
            break;
        }
    }
    paths
}

fn part_1(caves: &HashMap<Cave, HashSet<Cave>>) -> usize {
    let paths = find_paths(caves, &can_add);
    paths
        .iter()
        .filter(|v| v.last().unwrap() == &Cave::End)
        .count()
}

fn part_2(caves: &HashMap<Cave, HashSet<Cave>>) -> usize {
    let paths = find_paths(caves, &can_add_small_twice);
    paths
        .iter()
        .filter(|v| v.last().unwrap() == &Cave::End)
        .count()
}

fn main() {
    let mappings = read(File::open("input.txt").unwrap());
    let tree = build_tree(&mappings);
    let total_paths = part_1(&tree);
    println!("Total paths through the caves: {}", total_paths);
    let total_paths = part_2(&tree);
    println!(
        "Total paths through the caves when visiting a single small cave twice: {}",
        total_paths
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Cave::End, Cave::parse("end"));
        assert_eq!(Cave::Start, Cave::parse("start"));
        assert_eq!(Cave::Large("AB".to_string()), Cave::parse("AB"));
        assert_eq!(Cave::Small("ab".to_string()), Cave::parse("ab"));
    }

    #[test]
    fn test_reading() {
        let mappings = read(File::open("test1.txt").unwrap());
        assert_eq!(mappings.len(), 7);
        assert_eq!(mappings[0], (Cave::Start, Cave::Large("A".to_string())));
        assert_eq!(mappings[1], (Cave::Start, Cave::Small("b".to_string())));
        assert_eq!(mappings[6], (Cave::Small("b".to_string()), Cave::End));
    }

    #[test]
    fn test_tree_build() {
        let mappings = read(File::open("test1.txt").unwrap());
        let tree = build_tree(&mappings);
        assert_eq!(tree.len(), 6);
        let mut expected = HashSet::new();
        expected.insert(Cave::Start);
        expected.insert(Cave::Small("c".to_string()));
        expected.insert(Cave::Small("b".to_string()));
        expected.insert(Cave::End);
        assert_eq!(tree.get(&Cave::Large("A".to_string())).unwrap(), &expected);
        assert_eq!(tree.get(&Cave::End).unwrap().len(), 0);
    }

    #[test]
    fn check_if_cave_can_be_added_to_path() {
        assert!(can_add(&vec![Cave::Start], &Cave::End));
        assert!(can_add(
            &vec![Cave::Start, Cave::Large("A".to_string())],
            &Cave::Large("A".to_string())
        ));
        assert!(can_add(
            &vec![Cave::Start, Cave::Large("A".to_string())],
            &Cave::Large("B".to_string())
        ));
        assert!(can_add(
            &vec![Cave::Start, Cave::Large("A".to_string())],
            &Cave::Small("a".to_string())
        ));
        assert!(!can_add(
            &vec![Cave::Start, Cave::Small("a".to_string())],
            &Cave::Small("a".to_string())
        ));
        assert!(can_add(
            &vec![Cave::Start, Cave::Small("b".to_string())],
            &Cave::Small("a".to_string())
        ));
    }

    #[test]
    fn check_if_cave_can_be_added_to_path_2() {
        assert!(can_add_small_twice(&vec![Cave::Start], &Cave::End));
        assert!(can_add_small_twice(
            &vec![Cave::Start, Cave::Large("A".to_string())],
            &Cave::Large("A".to_string())
        ));
        assert!(can_add_small_twice(
            &vec![Cave::Start, Cave::Large("A".to_string())],
            &Cave::Large("B".to_string())
        ));
        assert!(can_add_small_twice(
            &vec![Cave::Start, Cave::Large("A".to_string())],
            &Cave::Small("a".to_string())
        ));
        assert!(can_add_small_twice(
            &vec![Cave::Start, Cave::Small("a".to_string())],
            &Cave::Small("a".to_string())
        ));
        assert!(can_add_small_twice(
            &vec![Cave::Start, Cave::Small("b".to_string())],
            &Cave::Small("a".to_string())
        ));
        assert!(can_add_small_twice(
            &vec![
                Cave::Start,
                Cave::Small("b".to_string()),
                Cave::Small("a".to_string())
            ],
            &Cave::Small("b".to_string())
        ));
        assert!(!can_add_small_twice(
            &vec![
                Cave::Start,
                Cave::Small("b".to_string()),
                Cave::Small("a".to_string()),
                Cave::Small("b".to_string())
            ],
            &Cave::Small("a".to_string())
        ));
    }

    #[test]
    fn test_part_1_1() {
        let mappings = read(File::open("test1.txt").unwrap());
        let tree = build_tree(&mappings);
        let paths = part_1(&tree);
        assert_eq!(paths, 10);
    }

    #[test]
    fn test_part_1_2() {
        let mappings = read(File::open("test2.txt").unwrap());
        let tree = build_tree(&mappings);
        let paths = part_1(&tree);
        assert_eq!(paths, 19);
    }

    #[test]
    fn test_part_1_3() {
        let mappings = read(File::open("test3.txt").unwrap());
        let tree = build_tree(&mappings);
        let paths = part_1(&tree);
        assert_eq!(paths, 226);
    }

    #[test]
    fn test_part_2_1() {
        let mappings = read(File::open("test1.txt").unwrap());
        let tree = build_tree(&mappings);
        let paths = part_2(&tree);
        assert_eq!(paths, 36);
    }

    #[test]
    fn test_part_2_2() {
        let mappings = read(File::open("test2.txt").unwrap());
        let tree = build_tree(&mappings);
        let paths = part_2(&tree);
        assert_eq!(paths, 103);
    }

    #[test]
    fn test_part_2_3() {
        let mappings = read(File::open("test3.txt").unwrap());
        let tree = build_tree(&mappings);
        let paths = part_2(&tree);
        assert_eq!(paths, 3509);
    }
}
