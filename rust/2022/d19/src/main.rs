// https://adventofcode.com/2022/day/2

use memoize::memoize;
use scanf::sscanf;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Costs {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blueprint {
    id: usize,
    ore_bot: Costs,
    clay_bot: Costs,
    obsidian_bot: Costs,
    geode_bot: Costs,
}

impl Blueprint {
    fn max_ore_cost(&self) -> usize {
        [
            self.ore_bot.ore,
            self.clay_bot.ore,
            self.obsidian_bot.ore,
            self.geode_bot.ore,
        ]
        .into_iter()
        .max()
        .unwrap()
    }

    fn max_clay_cost(&self) -> usize {
        [self.obsidian_bot.clay, self.geode_bot.clay]
            .into_iter()
            .max()
            .unwrap()
    }

    fn max_obsidian_cost(&self) -> usize {
        self.geode_bot.obsidian
    }
}

fn read<R: Read>(io: R) -> Vec<Blueprint> {
    let br = BufReader::new(io);
    br.lines().filter_map(Result::ok).map(|line| {
        let mut id: usize = 0;
        let mut ore_bot_cost_ore: usize = 0;
        let mut clay_bot_cost_ore: usize = 0;
        let mut obsidian_bot_cost_ore: usize = 0;
        let mut obsidian_bot_cost_clay: usize = 0;
        let mut geode_bot_cost_ore: usize = 0;
        let mut geode_bot_cost_obsidian: usize = 0;
        sscanf!(&line,
            "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            id,
            ore_bot_cost_ore,
            clay_bot_cost_ore,
            obsidian_bot_cost_ore,
            obsidian_bot_cost_clay,
            geode_bot_cost_ore,
            geode_bot_cost_obsidian,
            ).unwrap();
        Blueprint {
            id,
            ore_bot: Costs {
                ore: ore_bot_cost_ore,
                clay: 0,
                obsidian: 0
            },
            clay_bot: Costs {
                ore: clay_bot_cost_ore,
                clay: 0,
                obsidian: 0
            },
            obsidian_bot: Costs {
                ore: obsidian_bot_cost_ore,
                clay: obsidian_bot_cost_clay,
                obsidian: 0
            },
            geode_bot: Costs {
                ore: geode_bot_cost_ore,
                clay: 0,
                obsidian: geode_bot_cost_obsidian
            }
        }
    })
    .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize,
}

fn can_build_ore_bot(state: &State, blueprint: &Blueprint) -> bool {
    state.ore_bots < blueprint.max_ore_cost() && state.ore >= blueprint.ore_bot.ore
}

fn can_build_clay_bot(state: &State, blueprint: &Blueprint) -> bool {
    state.clay_bots < blueprint.max_clay_cost() && state.ore >= blueprint.clay_bot.ore
}

fn can_build_obsidian_bot(state: &State, blueprint: &Blueprint) -> bool {
    state.obsidian_bots < blueprint.max_obsidian_cost()
        && state.ore >= blueprint.obsidian_bot.ore
        && state.clay >= blueprint.obsidian_bot.clay
}

fn can_build_geode_bot(state: &State, blueprint: &Blueprint) -> bool {
    state.ore >= blueprint.geode_bot.ore && state.obsidian >= blueprint.geode_bot.obsidian
}

impl State {
    fn update_resources(&mut self, bp: &Blueprint) {
        // Not really sure about the factor 4 in clamping here, intention is to get more usage of
        // the memoisation cache to roll-up everything above a certain resource level; the 4 is
        // tuned for the example and my input
        self.ore = (self.ore + self.ore_bots).clamp(0, 4 * bp.max_ore_cost() - self.ore_bots);
        self.clay = (self.clay + self.clay_bots).clamp(0, 4 * bp.max_clay_cost() - self.clay_bots);
        self.obsidian = (self.obsidian + self.obsidian_bots).clamp(0, 4 * bp.max_obsidian_cost() - self.obsidian_bots);
        self.geode += self.geode_bots;
    }

    fn all_possible(&self, bp: &Blueprint) -> Vec<State> {
        let mut res = Vec::new();
        if can_build_geode_bot(self, bp) {
            // Always build a geode bot if possible
            let mut new_state = self.clone();
            new_state.update_resources(bp);
            new_state.geode_bots += 1;
            new_state.ore -= bp.geode_bot.ore;
            new_state.obsidian -= bp.geode_bot.obsidian;
            res.push(new_state);
        } else {
            if can_build_obsidian_bot(self, bp) {
                let mut new_state = self.clone();
                new_state.update_resources(bp);
                new_state.obsidian_bots += 1;
                new_state.ore -= bp.obsidian_bot.ore;
                new_state.clay -= bp.obsidian_bot.clay;
                res.push(new_state);
            }
            if can_build_clay_bot(self, bp) {
                let mut new_state = self.clone();
                new_state.update_resources(bp);
                new_state.clay_bots += 1;
                new_state.ore -= bp.clay_bot.ore;
                res.push(new_state);
            }
            if can_build_ore_bot(self, bp) {
                let mut new_state = self.clone();
                new_state.update_resources(bp);
                new_state.ore_bots += 1;
                new_state.ore -= bp.ore_bot.ore;
                res.push(new_state);
            }
            let mut new_state = self.clone();
            new_state.update_resources(bp);
            res.push(new_state)
        }

        res
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
        }
    }
}

#[memoize]
fn step(max_depth: usize, depth: usize, mut state: State, bp: Blueprint) -> State {
    //println!();
    //println!("Step {},  State: {:?}", depth, state);
    if depth == max_depth {
        state.update_resources(&bp);
        return state;
    }

    let possible_states = state.all_possible(&bp);
    //println!("{} -> {} options", depth, possible_states.len());
    //for opt in &possible_states {
    //    println!("   {:?}", opt);
    //}
    let mut results = possible_states
        .into_iter()
        .map(|ns| step(max_depth, depth + 1, ns, bp))
        .collect::<Vec<_>>();
    results.sort_by_key(|s| s.geode);
    results.pop().unwrap()
}

fn run_blueprint(max_depth: usize, blueprint: &Blueprint) -> usize {
    let state = State::default();
    //println!("Running for {:?}", blueprint);
    memoized_flush_step();
    //println!("Running for {:?}", blueprint);
    step(max_depth, 0, state, *blueprint).geode
}

fn part_1(input: &[Blueprint]) -> usize {
    input.iter().map(|bp| bp.id * run_blueprint(23, bp)).sum()
}

fn part_2(input: &[Blueprint]) -> usize {
    let res = input
        .iter()
        .take(3)
        .map(|bp| run_blueprint(31, bp))
        .collect::<Vec<_>>();
    res[0] * res[1] * res[2]
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
        assert_eq!(
            input[1].obsidian_bot,
            Costs {
                ore: 3,
                clay: 8,
                obsidian: 0
            }
        );
        assert_eq!(part_1(&input), 1 * 9 + 2 * 12);
        let a = run_blueprint(31, &input[0]);
        let b = run_blueprint(31, &input[1]);
        assert_eq!(a * b, 56 * 62);
    }
}
