// https://adventofcode.com/2023/day/5

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    ops::Range,
};

#[derive(Debug)]
struct RangedMapping {
    key: Range<usize>,
    value: Range<usize>,
}

impl From<&String> for RangedMapping {
    fn from(s: &String) -> Self {
        let mut parts = s.split_whitespace();
        let destination = parts.next().unwrap().parse::<usize>().unwrap();
        let source = parts.next().unwrap().parse::<usize>().unwrap();
        let size = parts.next().unwrap().parse::<usize>().unwrap();
        Self {
            key: source..source + size,
            value: destination..destination + size,
        }
    }
}

#[derive(Debug)]
struct RangedMap {
    content: Vec<RangedMapping>,
}

impl RangedMap {
    fn get(&self, key: usize) -> usize {
        for mapping in &self.content {
            if mapping.key.contains(&key) {
                let offset = key - mapping.key.start;
                return mapping.value.start + offset;
            }
        }
        key
    }
}

#[derive(Debug)]
struct Maps {
    seed_to_soil: RangedMap,
    soil_to_fertilizer: RangedMap,
    fertilizer_to_water: RangedMap,
    water_to_light: RangedMap,
    light_to_temperature: RangedMap,
    temperature_to_humidity: RangedMap,
    humidity_to_location: RangedMap,
}

impl From<&[String]> for RangedMap {
    fn from(s: &[String]) -> Self {
        let content = s[1..].iter().map(RangedMapping::from).collect();
        Self { content }
    }
}

fn read<R: Read>(io: R) -> (Vec<usize>, Maps) {
    let br = BufReader::new(io);
    let lines = br.lines().filter_map(Result::ok).collect::<Vec<String>>();
    let seeds = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut from = 2;
    let mut to = 2;
    while !lines[to].is_empty() {
        to += 1;
    }
    let seed_to_soil = RangedMap::from(&lines[from..to]);

    from = to + 1;
    to = from;
    while !lines[to].is_empty() {
        to += 1;
    }
    let soil_to_fertilizer = RangedMap::from(&lines[from..to]);

    from = to + 1;
    to = from;
    while !lines[to].is_empty() {
        to += 1;
    }
    let fertilizer_to_water = RangedMap::from(&lines[from..to]);

    from = to + 1;
    to = from;
    while !lines[to].is_empty() {
        to += 1;
    }
    let water_to_light = RangedMap::from(&lines[from..to]);

    from = to + 1;
    to = from;
    while !lines[to].is_empty() {
        to += 1;
    }
    let light_to_temperature = RangedMap::from(&lines[from..to]);

    from = to + 1;
    to = from;
    while !lines[to].is_empty() {
        to += 1;
    }
    let temperature_to_humidity = RangedMap::from(&lines[from..to]);

    from = to + 1;
    to = from;
    while to < lines.len() && !lines[to].is_empty() {
        to += 1;
    }
    let humidity_to_location = RangedMap::from(&lines[from..to]);

    let maps = Maps {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };

    (seeds, maps)
}

fn part_1(seeds: &[usize], maps: &Maps) -> usize {
    seeds
        .iter()
        .copied()
        .map(|s| {
            let soil = maps.seed_to_soil.get(s);
            let fertilizer = maps.soil_to_fertilizer.get(soil);
            let water = maps.fertilizer_to_water.get(fertilizer);
            let light = maps.water_to_light.get(water);
            let temperature = maps.light_to_temperature.get(light);
            let humidity = maps.temperature_to_humidity.get(temperature);
            let location = maps.humidity_to_location.get(humidity);
            location
        })
        .min()
        .unwrap()
}

fn part_2(seeds: &[usize], maps: &Maps) -> usize {
    seeds
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .flat_map(|range| {
            range.map(|s| {
                let soil = maps.seed_to_soil.get(s);
                let fertilizer = maps.soil_to_fertilizer.get(soil);
                let water = maps.fertilizer_to_water.get(fertilizer);
                let light = maps.water_to_light.get(water);
                let temperature = maps.light_to_temperature.get(light);
                let humidity = maps.temperature_to_humidity.get(temperature);
                let location = maps.humidity_to_location.get(humidity);
                location
            })
        })
        .min()
        .unwrap()
}

fn main() {
    let (seeds, maps) = read(File::open("input.txt").unwrap());
    let p1 = part_1(&seeds, &maps);
    println!("Part 1: {}", p1);
    let p2 = part_2(&seeds, &maps);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping() {
        let (_, maps) = read(File::open("example1.txt").unwrap());
        assert_eq!(maps.seed_to_soil.get(98), 50);
        assert_eq!(maps.seed_to_soil.get(99), 51);
        assert_eq!(maps.seed_to_soil.get(100), 100);
        assert_eq!(maps.seed_to_soil.get(49), 49);
        assert_eq!(maps.seed_to_soil.get(50), 52);
        assert_eq!(maps.seed_to_soil.get(51), 53);
        assert_eq!(maps.seed_to_soil.get(52), 54);
        assert_eq!(maps.seed_to_soil.get(97), 99);
    }

    #[test]
    fn example_1() {
        let (seeds, maps) = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&seeds, &maps), 35);
        assert_eq!(part_2(&seeds, &maps), 46);
    }
}
