// https://adventofcode.com/2023/day/25

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read, Write},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Machine {
    connections: HashMap<String, HashSet<String>>,
}

fn read<R: Read>(io: R) -> Machine {
    let br = BufReader::new(io);
    let lines = br.lines().map_while(Result::ok).collect::<Vec<_>>();
    let mut machine = Machine::default();
    for line in &lines {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap().trim().to_string();
        let targets = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.trim().to_string())
            .collect::<HashSet<_>>();
        machine.connections.entry(name).or_default().extend(targets);
    }
    machine
}

fn part_1(input: &Machine) -> usize {
    let mut fc = String::new();

    let mut output = File::create("graph.dot").unwrap();
    writeln!(output, "{}", r#"strict graph {"#).unwrap();
    fc.push('\n');
    for (name, targets) in &input.connections {
        for target in targets {
            writeln!(output, "  {} -- {}", name, target).unwrap();
        }
    }
    writeln!(output, "{}", r#"}"#).unwrap();
    // Generate SVG: :dot -Tsvg graph.dot > graph.svg
    // Inspect the SVG (hovering over the connections shows the name) and find that we need to make
    // these cuts:
    //  vzb -- tnr
    //  tqn -- tvf
    //  lmg -- krx

    let mut new_connections = input.connections.clone();
    if new_connections.contains_key("vzb") {
        if new_connections.get_mut("vzb").unwrap().remove("tnr") {
            println!("Cut vzb -- tnr");
        }
    }
    if new_connections.contains_key("tnr") {
        if new_connections.get_mut("tnr").unwrap().remove("vzb") {
            println!("Cut tnr -- vzb");
        }
    }
    if new_connections.contains_key("tqn") {
        if new_connections.get_mut("tqn").unwrap().remove("tvf") {
            println!("Cut tqn -- tvf");
        }
    }
    if new_connections.contains_key("tvf") {
        if new_connections.get_mut("tvf").unwrap().remove("tqn") {
            println!("Cut tvf -- tqn");
        }
    }
    if new_connections.contains_key("lmg") {
        if new_connections.get_mut("lmg").unwrap().remove("krx") {
            println!("Cut lmg -- krx");
        }
    }
    if new_connections.contains_key("krx") {
        if new_connections.get_mut("krx").unwrap().remove("lmg") {
            println!("Cut krx -- lmg");
        }
    }

    let mut bidrectional = HashMap::new();
    for (k, v) in &new_connections {
        bidrectional
            .entry(k.to_string())
            .or_insert_with(HashSet::new)
            .extend(v.iter().cloned());
        for v in v {
            bidrectional
                .entry(v.to_string())
                .or_insert_with(HashSet::new)
                .insert(k.clone());
        }
    }

    count(&bidrectional, "vzb") * count(&bidrectional, "tnr")
}

fn count(connections: &HashMap<String, HashSet<String>>, start: &str) -> usize {
    let mut visited = HashSet::new();
    let mut queue = vec![start.to_string()];
    while let Some(name) = queue.pop() {
        if visited.contains(&name) {
            continue;
        }
        visited.insert(name.clone());
        if let Some(targets) = connections.get(&name) {
            queue.extend(targets.iter().cloned());
        }
    }
    visited.len()
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input);
    println!("Part 1: {}", p1);
}
