// https://adventofcode.com/2022/day/16
// This is awful, clearly didn't get the trick today.  Couldn't come up with a clever algorithm, so
// it is brute-forced (though not the totally naïve way of running over all permutations of how to
// visit the valves).  Plenty of scope to reduce duplication and almost-but-not-quite-the-same
// code, but don't have any energy left today.

use itertools::Itertools;
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

fn get_options(
    current: &str,
    remaining: &HashSet<String>,
    distance_graph: &HashMap<String, HashMap<String, usize>>,
    steps_left: usize,
) -> HashSet<String> {
    distance_graph[current]
        .keys()
        .filter_map(|v| {
            if remaining.contains(v) && distance_graph[current][v] + 1 < steps_left {
                Some(v.to_string())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
}

fn walking_to_1(
    walker: Option<(&str, usize)>,
    remaining: HashSet<String>,
    step: usize,
    mut current_flow: usize,
    map: &HashMap<String, Valve>,
    distance_graph: &HashMap<String, HashMap<String, usize>>,
) -> usize {
    let steps_left = 29 - step;
    match walker {
        Some(w1) => {
            let new_w1 = if w1.1 > 0 {
                Some((w1.0, w1.1 - 1))
            } else {
                current_flow += map[w1.0].flow_rate;
                None
            };
            if new_w1.is_none() {
                let options = get_options(w1.0, &remaining, distance_graph, steps_left);
                return current_flow
                    + options
                        .iter()
                        .map(|v| {
                            let d = distance_graph[w1.0][v];
                            let new_remaining = remaining
                                .clone()
                                .into_iter()
                                .filter_map(|tmp| if &tmp != v { Some(tmp) } else { None })
                                .collect::<HashSet<_>>();

                            walking_to_1(
                                Some((v, d)),
                                new_remaining,
                                step + 1,
                                current_flow,
                                map,
                                distance_graph,
                            )
                        })
                        .max()
                        .unwrap_or(steps_left * current_flow);
            } else {
                return current_flow
                    + walking_to_1(
                        new_w1,
                        remaining,
                        step + 1,
                        current_flow,
                        map,
                        distance_graph,
                    );
            }
        }
        None => {
            if remaining.is_empty() {
                return current_flow * steps_left;
            } else {
                // Initialisation, we are in AA
                let options = get_options("AA", &remaining, distance_graph, steps_left);
                return current_flow
                    + options
                        .iter()
                        .map(|v| {
                            let d = distance_graph["AA"][v];
                            let new_remaining = remaining
                                .clone()
                                .into_iter()
                                .filter_map(|tmp| if &tmp != v { Some(tmp) } else { None })
                                .collect::<HashSet<_>>();
                            walking_to_1(
                                Some((v, d)),
                                new_remaining,
                                step + 1,
                                current_flow,
                                map,
                                distance_graph,
                            )
                        })
                        .max()
                        .unwrap();
            }
        }
    }
}

fn combine_options(
    opt1: HashSet<String>,
    opt2: HashSet<String>,
) -> Vec<(Option<String>, Option<String>)> {
    if opt1.is_empty() && opt1.is_empty() {
        return Vec::new();
    }
    if opt1.is_empty() && !opt2.is_empty() {
        return opt2.into_iter().map(|o2| (None, Some(o2))).collect();
    }
    if !opt1.is_empty() && opt2.is_empty() {
        return opt1.into_iter().map(|o1| (Some(o1), None)).collect();
    }
    opt1.iter()
        .flat_map(|o1| opt2.iter().map(|o2| (Some(o1.clone()), Some(o2.clone()))))
        .filter_map(|(x, y)| if x != y { Some((x, y)) } else { None })
        .collect()
}

fn walking_to(
    walker1: Option<(&str, usize)>,
    walker2: Option<(&str, usize)>,
    remaining: HashSet<String>,
    step: usize,
    mut current_flow: usize,
    map: &HashMap<String, Valve>,
    distance_graph: &HashMap<String, HashMap<String, usize>>,
) -> usize {
    let steps_left = 25 - step;
    match (walker1, walker2) {
        (Some(w1), Some(w2)) => {
            let new_w1 = if w1.1 > 0 {
                Some((w1.0, w1.1 - 1))
            } else {
                current_flow += map[w1.0].flow_rate;
                None
            };
            let new_w2 = if w2.1 > 0 {
                Some((w2.0, w2.1 - 1))
            } else {
                current_flow += map[w2.0].flow_rate;
                None
            };
            if new_w1.is_none() && new_w2.is_none() {
                let options1 = get_options(w1.0, &remaining, distance_graph, steps_left);
                let options2 = get_options(w2.0, &remaining, distance_graph, steps_left);
                let comb_opts = combine_options(options1, options2);

                let recursion = comb_opts
                    .into_iter()
                    .map(|(mo1, mo2)| {
                        let w1_tmp = if let Some(o1) = &mo1 {
                            Some((o1.as_ref(), distance_graph[w1.0][o1]))
                        } else {
                            None
                        };
                        let w2_tmp = if let Some(o2) = &mo2 {
                            Some((o2.as_ref(), distance_graph[w2.0][o2]))
                        } else {
                            None
                        };
                        let new_remaining = remaining
                            .clone()
                            .into_iter()
                            .filter_map(|tmp| {
                                if tmp != mo1.clone().unwrap_or("".to_string())
                                    && tmp != mo2.clone().unwrap_or("".to_string())
                                {
                                    Some(tmp)
                                } else {
                                    None
                                }
                            })
                            .collect::<HashSet<_>>();
                        walking_to(
                            w1_tmp,
                            w2_tmp,
                            new_remaining,
                            step + 1,
                            current_flow,
                            map,
                            distance_graph,
                        )
                    })
                    .max()
                    .unwrap_or(steps_left * current_flow);
                return recursion + current_flow;
            } else if new_w1.is_none() && new_w2.is_some() {
                let options1 = get_options(w1.0, &remaining, distance_graph, steps_left);
                if options1.is_empty() {
                    return current_flow
                        + walking_to(
                            None,
                            new_w2,
                            remaining,
                            step + 1,
                            current_flow,
                            map,
                            distance_graph,
                        );
                } else {
                    let recursion = options1
                        .iter()
                        .map(|o1| {
                            let d = distance_graph[w1.0][o1];
                            let new_remaining = remaining
                                .clone()
                                .into_iter()
                                .filter_map(|tmp| if &tmp != o1 { Some(tmp) } else { None })
                                .collect::<HashSet<_>>();
                            walking_to(
                                Some((o1, d)),
                                new_w2,
                                new_remaining,
                                step + 1,
                                current_flow,
                                map,
                                distance_graph,
                            )
                        })
                        .max()
                        .unwrap();
                    return current_flow + recursion;
                }
            } else if new_w1.is_some() && new_w2.is_none() {
                let options2 = get_options(w2.0, &remaining, distance_graph, steps_left);
                if options2.is_empty() {
                    return current_flow
                        + walking_to(
                            new_w1,
                            None,
                            remaining,
                            step + 1,
                            current_flow,
                            map,
                            distance_graph,
                        );
                } else {
                    let recursion = options2
                        .iter()
                        .map(|o2| {
                            let d = distance_graph[w2.0][o2];
                            let new_remaining = remaining
                                .clone()
                                .into_iter()
                                .filter_map(|tmp| if &tmp != o2 { Some(tmp) } else { None })
                                .collect::<HashSet<_>>();
                            walking_to(
                                new_w1,
                                Some((o2, d)),
                                new_remaining,
                                step + 1,
                                current_flow,
                                map,
                                distance_graph,
                            )
                        })
                        .max()
                        .unwrap();
                    return current_flow + recursion;
                }
            } else {
                return current_flow
                    + walking_to(
                        new_w1,
                        new_w2,
                        remaining,
                        step + 1,
                        current_flow,
                        map,
                        distance_graph,
                    );
            }
        }
        (Some(w1), None) => {
            let new_w1 = if w1.1 > 0 {
                Some((w1.0, w1.1 - 1))
            } else {
                current_flow += map[w1.0].flow_rate;
                None
            };
            if new_w1.is_none() {
                let options = get_options(w1.0, &remaining, distance_graph, steps_left);
                let recursion = options
                    .iter()
                    .map(|v| {
                        let d = distance_graph[w1.0][v];
                        let new_remaining = remaining
                            .clone()
                            .into_iter()
                            .filter_map(|tmp| if &tmp != v { Some(tmp) } else { None })
                            .collect::<HashSet<_>>();

                        walking_to(
                            Some((v, d)),
                            None,
                            new_remaining,
                            step + 1,
                            current_flow,
                            map,
                            distance_graph,
                        )
                    })
                    .max()
                    .unwrap_or(steps_left * current_flow);
                return current_flow + recursion;
            } else {
                return current_flow
                    + walking_to(
                        new_w1,
                        None,
                        remaining,
                        step + 1,
                        current_flow,
                        map,
                        distance_graph,
                    );
            }
        }
        (None, Some(_)) => {
            // Flip around so that we end up in Some, None case
            walking_to(
                walker2,
                None,
                remaining,
                step,
                current_flow,
                map,
                distance_graph,
            )
        }
        (None, None) => {
            if remaining.is_empty() {
                return current_flow * steps_left;
            } else {
                // Initialisation, we are in AA
                let options = get_options("AA", &remaining, distance_graph, steps_left);
                let recursion = options
                    .iter()
                    .permutations(2)
                    .map(|p| {
                        let d0 = distance_graph["AA"][p[0]];
                        let d1 = distance_graph["AA"][p[1]];
                        let new_remaining = remaining
                            .clone()
                            .into_iter()
                            .filter_map(|tmp| {
                                if &tmp != p[0] && &tmp != p[1] {
                                    Some(tmp)
                                } else {
                                    None
                                }
                            })
                            .collect::<HashSet<_>>();
                        let rs = walking_to(
                            Some((p[0], d0)),
                            Some((p[1], d1)),
                            new_remaining,
                            step + 1,
                            current_flow,
                            map,
                            distance_graph,
                        );
                        rs
                    })
                    .max()
                    .unwrap();
                return current_flow + recursion;
            }
        }
    }
}

fn part_1(input: &HashMap<String, Valve>) -> usize {
    let distance_graph = build_distance_graph(input, "AA");
    let remaining = input
        .iter()
        .filter_map(|(name, v)| {
            if v.flow_rate > 0 {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    walking_to_1(None, remaining, 0, 0, input, &distance_graph)
}

fn part_2(input: &HashMap<String, Valve>) -> usize {
    let distance_graph = build_distance_graph(input, "AA");
    let remaining = input
        .iter()
        .filter_map(|(name, v)| {
            if v.flow_rate > 0 {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    walking_to(None, None, remaining, 0, 0, input, &distance_graph)
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
        assert_eq!(part_1(&input), 1651);
        assert_eq!(part_2(&input), 1707);
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
