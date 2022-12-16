// https://adventofcode.com/2022/day/16

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Valve {
    flow_rate: usize,
    connections: Vec<String>,
}

fn read<R: Read>(io: R) -> HashMap<String, Valve> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .map(|line| {
            let (front, back) = line.split_once(';').unwrap();
            let valve_name = front[6..8].to_string();
            let flow_rate = front.split_once('=').unwrap().1.parse::<usize>().unwrap();
            let connections = if back.contains("valves ") {
                let (_, list) = back.split_once("valves ").unwrap();
                list.split(", ")
                    .map(|item| item.to_string())
                    .collect::<Vec<_>>()
            } else {
                vec![back[back.len() - 2..].to_string()]
            };
            (
                valve_name,
                Valve {
                    flow_rate,
                    connections,
                },
            )
        })
        .collect()
}

fn distance(map: &HashMap<String, Valve>, start: &str, end: &str) -> usize {
    dist_walk(map, start, end, &HashSet::new()).unwrap().len() - 1
}

fn dist_walk(
    map: &HashMap<String, Valve>,
    next: &str,
    end: &str,
    path: &HashSet<String>,
) -> Option<HashSet<String>> {
    assert!(!path.contains(next));
    let mut res = path.clone();
    res.insert(next.to_string());
    if next == end {
        Some(res)
    } else {
        let mut possible_path = HashSet::new();
        for c in &map[next].connections {
            if !res.contains(c) {
                if let Some(p) = dist_walk(map, c, end, &res) {
                    if possible_path.len() == 0 || p.len() < possible_path.len() {
                        possible_path = p;
                    }
                }
            }
        }
        if possible_path.is_empty() {
            None
        } else {
            Some(possible_path)
        }
    }
}

fn build_distance_graph(
    input: &HashMap<String, Valve>,
    extra_input: &str,
) -> HashMap<String, HashMap<String, usize>> {
    let valves_with_flow = input
        .iter()
        .filter_map(|(k, v)| {
            if v.flow_rate > 0 {
                Some(k.to_string())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();
    valves_with_flow
        .union(&([extra_input.to_string()].into_iter().collect()))
        .map(|start| {
            let dists = valves_with_flow
                .iter()
                .filter_map(|end| {
                    if end == start {
                        None
                    } else {
                        Some((end.to_string(), distance(input, start, end)))
                    }
                })
                .collect::<HashMap<_, _>>();
            (start.to_string(), dists)
        })
        .collect()
}

fn walk(
    step: usize,
    current: &str,
    current_flow: usize,
    map: &HashMap<String, Valve>,
    distance_graph: &HashMap<String, HashMap<String, usize>>,
    mut turned_on: HashSet<String>,
) -> usize {
    //println!(
    //    "Valve {}, Step {}, current_flow: {}, turned_on: {:?}",
    //    current, step, current_flow, turned_on
    //);
    let step_spent = if current != "AA" {
        turned_on.insert(current.to_string());
        1
    } else {
        0
    };

    let local_flow = map[current].flow_rate;
    let steps_left = 31 - (step + step_spent);

    let options = distance_graph[current]
        .keys()
        .filter_map(|v| {
            if turned_on.contains(v) || distance_graph[current][v] >= steps_left {
                None
            } else {
                Some(v.to_string())
            }
        })
        .collect::<HashSet<_>>();
    if options.len() == 0 {
        return step_spent * current_flow + steps_left * (current_flow + local_flow);
    }
    if current != "AA" {
        assert!(turned_on.contains(current));
    }

    options
        .iter()
        .map(|go_to_valve| {
            let distance = distance_graph[current][go_to_valve];
            current_flow + 
            distance * (current_flow + local_flow)
                + walk(
                    step + 1 + distance,
                    go_to_valve,
                    current_flow + local_flow,
                    map,
                    distance_graph,
                    turned_on.clone(),
                )
        })
        .max()
        .unwrap()
}

fn part_1(input: &HashMap<String, Valve>) -> usize {
    let distance_graph = build_distance_graph(input, "AA");

    walk(0, "AA", 0, input, &distance_graph, HashSet::new())
}

fn part_2(input: &HashMap<String, Valve>) -> usize {
    input.len()
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input);
    println!("Part 1: {}", p1);
    let p2 = part_2(&input);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        println!("{:?}", input);
        assert_eq!(part_1(&input), 1651);
        //assert_eq!(part_2(&input), 1);
    }

    #[test]
    fn test_dist_walk() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(distance(&input, "AA", "BB"), 1);
        assert_eq!(distance(&input, "AA", "CC"), 2);
        assert_eq!(distance(&input, "AA", "DD"), 1);
        assert_eq!(distance(&input, "AA", "EE"), 2);
        assert_eq!(distance(&input, "AA", "FF"), 3);
        assert_eq!(distance(&input, "AA", "GG"), 4);
        assert_eq!(distance(&input, "AA", "HH"), 5);
        assert_eq!(distance(&input, "AA", "II"), 1);
        assert_eq!(distance(&input, "AA", "JJ"), 2);

        assert_eq!(distance(&input, "FF", "JJ"), 5);
        assert_eq!(distance(&input, "DD", "BB"), 2);
    }

    #[test]
    fn test_build_distance_graph() {
        let input = read(File::open("example1.txt").unwrap());
        let graph = build_distance_graph(&input, "AA");
        println!("{:?}", graph);
        assert_eq!(graph.len(), 7);
        assert_eq!(graph["AA"].len(), 6);
        assert_eq!(graph["BB"].len(), 5);
        assert_eq!(graph["CC"].len(), 5);
        assert_eq!(graph["DD"].len(), 5);
        assert_eq!(graph["EE"].len(), 5);
        assert_eq!(graph["HH"].len(), 5);
        assert_eq!(graph["JJ"].len(), 5);
    }
}
