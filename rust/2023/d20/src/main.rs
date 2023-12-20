// https://adventofcode.com/2023/day/20

use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{BufRead, BufReader, Read},
};

use num::Integer;

fn read<R: Read>(io: R) -> Network {
    let br = BufReader::new(io);
    let mut nw = Network {
        modules: br
            .lines()
            .map_while(Result::ok)
            .map(Module::from)
            .map(|m| (m.name().to_string(), m))
            .collect(),
    };

    // Connect the conjuctions - horrendous as can't get a mutable reference to the module while
    // iterating over the collection...
    let names = nw.modules.keys().cloned().collect::<Vec<_>>();
    for name in names {
        let m = nw.modules.get(&name).unwrap();
        let outputs = match m {
            Module::Broadcast(b) => b.output.clone(),
            Module::FlipFlop(f) => f.output.clone(),
            Module::Conjuction(c) => c.output.clone(),
        };
        for o in outputs {
            if let Some(Module::Conjuction(c)) = nw.modules.get_mut(&o) {
                c.input_states.insert(name.clone(), Pulse::Low);
            }
        }
    }

    nw
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Pulse::High => write!(f, "1"),
            Pulse::Low => write!(f, "0"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Network {
    modules: HashMap<String, Module>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Broadcast {
    output: Vec<String>,
}

impl ModuleTrait for Broadcast {
    fn name(&self) -> &str {
        "broadcast"
    }

    fn process_pulse(&mut self, _from: &str, pulse: Pulse) -> Vec<(String, String, Pulse)> {
        self.output
            .iter()
            .map(|o| (self.name().to_string(), o.clone(), pulse))
            .collect()
    }
}

impl Display for Broadcast {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "broadcast -> {}", self.output.join(", "))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct FlipFlop {
    name: String,
    is_on: bool,
    output: Vec<String>,
}

impl ModuleTrait for FlipFlop {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn process_pulse(&mut self, _from: &str, pulse: Pulse) -> Vec<(String, String, Pulse)> {
        if pulse == Pulse::High {
            return vec![];
        }
        // When on, turn off and send low pulse, when off, turn on and send high pulse
        let response = if self.is_on {
            self.is_on = false;
            Pulse::Low
        } else {
            self.is_on = true;
            Pulse::High
        };
        self.output
            .iter()
            .map(|o| (self.name().to_string(), o.clone(), response))
            .collect()
    }
}

impl Display for FlipFlop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} -> {}",
            self.name,
            self.is_on,
            self.output.join(", ")
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Conjuction {
    name: String,
    input_states: HashMap<String, Pulse>,
    output: Vec<String>,
}

impl ModuleTrait for Conjuction {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn process_pulse(&mut self, from: &str, pulse: Pulse) -> Vec<(String, String, Pulse)> {
        // Update the input state
        let c_state = self.input_states.get_mut(from).unwrap();
        *c_state = pulse;

        let all_high = self.input_states.values().all(|v| *v == Pulse::High);
        let response = if all_high { Pulse::Low } else { Pulse::High };

        self.output
            .iter()
            .map(|o| (self.name().to_string(), o.clone(), response))
            .collect()
    }
}

impl Display for Conjuction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let keys = self.input_states.keys().cloned().collect::<Vec<_>>();
        let values = self
            .input_states
            .values()
            .cloned()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>();
        write!(
            f,
            "{} -> {}\n  {}\n  {}",
            self.name,
            self.output.join(", "),
            keys.join(", "),
            values.join(", "),
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Module {
    Broadcast(Broadcast),
    FlipFlop(FlipFlop),
    Conjuction(Conjuction),
}

trait ModuleTrait {
    fn name(&self) -> &str;
    fn process_pulse(&mut self, from: &str, pulse: Pulse) -> Vec<(String, String, Pulse)>;
}

impl ModuleTrait for Module {
    fn name(&self) -> &str {
        match self {
            Module::Broadcast(b) => b.name(),
            Module::FlipFlop(f) => f.name(),
            Module::Conjuction(c) => c.name(),
        }
    }

    fn process_pulse(&mut self, from: &str, pulse: Pulse) -> Vec<(String, String, Pulse)> {
        match self {
            Module::Broadcast(b) => b.process_pulse(from, pulse),
            Module::FlipFlop(f) => f.process_pulse(from, pulse),
            Module::Conjuction(c) => c.process_pulse(from, pulse),
        }
    }
}

impl From<String> for Module {
    fn from(s: String) -> Self {
        let mut parts = s.split(" -> ");
        let typename = parts.next().unwrap();
        let outputs = parts.next().unwrap();
        match typename.chars().next().unwrap() {
            'b' => Module::Broadcast(Broadcast {
                output: outputs.split(", ").map(String::from).collect(),
            }),
            '%' => Module::FlipFlop(FlipFlop {
                name: typename[1..].to_string(),
                is_on: false,
                output: outputs.split(", ").map(String::from).collect(),
            }),
            '&' => Module::Conjuction(Conjuction {
                name: typename[1..].to_string(),
                input_states: HashMap::new(),
                output: outputs.split(", ").map(String::from).collect(),
            }),
            _ => panic!("Unknown module type {}", typename),
        }
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Module::Broadcast(b) => write!(f, "{}", b),
            Module::FlipFlop(fl) => write!(f, "{}", fl),
            Module::Conjuction(c) => write!(f, "{}", c),
        }
    }
}

fn part_1(nw: &Network) -> usize {
    let mut nw = nw.clone();
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        let mut new = nw
            .modules
            .get_mut("broadcast")
            .unwrap()
            .process_pulse("", Pulse::Low);
        low_pulses += 1;
        while !new.is_empty() {
            let mut next = vec![];
            for (from, to, pulse) in new {
                if pulse == Pulse::High {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
                }
                if let Some(m) = nw.modules.get_mut(&to) {
                    let mut new_pulses = m.process_pulse(&from, pulse);
                    next.append(&mut new_pulses);
                }
            }
            new = next;
        }
    }
    low_pulses * high_pulses
}

fn part_2(nw: &Network) -> usize {
    let mut nw = nw.clone();
    let mut button_presses = 0usize;
    let track = "ll"; // NB: This uses knowledge of the network structure ot MY input, it is the
                      // node just before the rx node

    let mut cycles = HashMap::new();
    loop {
        let mut new = nw
            .modules
            .get_mut("broadcast")
            .unwrap()
            .process_pulse("", Pulse::Low);
        button_presses += 1;
        while !new.is_empty() {
            let mut next = vec![];
            for (from, to, pulse) in new {
                if to == track && pulse == Pulse::High {
                    cycles.insert(from.to_string(), button_presses);
                }
                if let Some(m) = nw.modules.get_mut(&to) {
                    let mut new_pulses = m.process_pulse(&from, pulse);
                    next.append(&mut new_pulses);
                }
            }
            new = next;
        }

        // NB: More knowledge of my input network, the node before the rx node has 4 inputs, once a
        //     cycle for each of those has been found, just lcm it
        if cycles.len() == 4 {
            println!("Cycles: {:?}", cycles);
            return cycles.values().copied().fold(1, |tmp, val| tmp.lcm(&val));
        }
    }
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
        assert_eq!(part_1(&input), 32000000);
        //assert_eq!(part_2(&input), 1);
    }

    #[test]
    fn example_2() {
        let input = read(File::open("example2.txt").unwrap());
        assert_eq!(part_1(&input), 11687500);
        //assert_eq!(part_2(&input), 1);
    }
}
