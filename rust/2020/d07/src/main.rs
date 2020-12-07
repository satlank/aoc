use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[macro_use]
extern crate scan_fmt;

struct Rule {
    colour: String,
    count: HashMap<String, usize>,
}

impl Rule {
    fn parse<S: AsRef<str>>(line: S) -> Result<Rule, scan_fmt::parse::ScanError> {
        let line = line.as_ref();
        let phrases: Vec<&str> = line.split("contain").collect();
        let colour = phrases[0].split(' ').take(2).collect::<Vec<_>>();
        let colour = format!("{} {}", colour[0], colour[1]);
        let mut count = HashMap::new();
        for rule in phrases[1].split(',') {
            if rule == " no other bags." { continue }
            let (cnt, colour, _) = scan_fmt!(
                rule,
                " {d} {/[a-z]+ [a-z]+/} {/[a-z,.]+/}",
                usize, String, String
            )?;
            count.insert(colour.to_string(), cnt);
        }
        Ok(Rule { colour, count})
    }
}

fn read<R: Read>(io: R) -> Result<Vec<Rule>, Error> {
    BufReader::new(io)
        .lines()
        .map(|line| {
            line.and_then(|row| {
                Rule::parse(row).map_err(|e| Error::new(ErrorKind::InvalidData, e))
            })
        })
        .collect()
}

fn part_1(vec: &[Rule]) -> usize {
    let rules = vec.iter()
        .map(|e| (&e.colour, e.count.keys().collect::<HashSet<_>>()))
        .collect::<HashMap<&String, HashSet<&String>>>();
    let my_bag = "shiny gold".to_string();
    let mut colours: HashSet<&String> = [ &my_bag ].iter().cloned().collect();
    loop {
        let new_colours: HashSet<&String> = rules.iter()
            .filter(|r| !colours.contains(r.0))
            .filter(|r| {
                r.1.intersection(&colours).count() > 0
            })
            .map(|r| *r.0)
            .collect();
        if new_colours.len() > 0 {
            colours = colours.union(&new_colours).cloned().collect();
        } else {
            break;
        }
    }
    colours.len() - 1
}

fn calc_bags(colour: &String, rules: &HashMap<&String, &HashMap<String, usize>>) -> usize {
    let r = rules[colour];
    if r.is_empty() {
        return 0;
    }
    r.iter()
        .map(|item| {
            *item.1 + *item.1 * calc_bags(item.0, rules)
        })
        .sum()
}

fn part_2(vec: &[Rule]) -> usize {
    let rules = vec.iter()
        .map(|e| (&e.colour, &e.count))
        .collect::<HashMap<&String, &HashMap<String, usize>>>();
    calc_bags(&"shiny gold".to_string(), &rules)
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    println!("Read {} rules", vec.len());
    println!("Coloured bags eventually containing a shiny gold one: {}", part_1(&vec));
    println!("Bags inside the shiny gold bag: {}", part_2(&vec));
    Ok(())
}
