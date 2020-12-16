use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[macro_use]
extern crate scan_fmt;

#[derive(Debug, Eq, PartialEq)]
struct Range {
    min1: u32,
    max1: u32,
    min2: u32,
    max2: u32,
}

#[derive(Debug, Eq, PartialEq)]
struct TicketRules {
    fields: HashMap<String, Range>,
}

#[derive(Debug, Eq, PartialEq)]
struct Ticket {
    vals: Vec<u32>,
}

impl Range {
    fn parse<S: AsRef<str>>(details: S) -> Result<Range, scan_fmt::parse::ScanError> {
        let (min1, max1, min2, max2) =
            scan_fmt!(details.as_ref(), "{}-{} or {}-{}", u32, u32, u32, u32)?;
        Ok(Range {
            min1,
            max1,
            min2,
            max2,
        })
    }

    fn matches(&self, val: u32) -> bool {
        (self.min1 <= val && val <= self.max1) || (self.min2 <= val && val <= self.max2)
    }
}

impl TicketRules {
    fn parse(details: &[&String]) -> Result<TicketRules, scan_fmt::parse::ScanError> {
        let mut fields = HashMap::new();
        for entry in details.iter() {
            let (key, val) = scan_fmt!(entry, "{/[a-z ]*/}: {/[0-9 or\\-]*/}", String, String)?;
            fields.insert(key, Range::parse(val)?);
        }
        Ok(TicketRules { fields })
    }

    fn matches_any(&self, val: u32) -> bool {
        self.fields
            .values()
            .filter(|range| range.matches(val))
            .count()
            > 0
    }
}

impl Ticket {
    fn parse<S: AsRef<str>>(details: S) -> Result<Ticket, Error> {
        let vals = details
            .as_ref()
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();
        Ok(Ticket { vals })
    }

    fn error_rate(&self, rules: &TicketRules) -> u32 {
        self.vals
            .iter()
            .filter(|val| !rules.matches_any(**val))
            .sum()
    }
}

fn read<R: Read>(io: R) -> (TicketRules, Ticket, Vec<Ticket>) {
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines().filter_map(Result::ok).collect();
    let rule_lines: Vec<&String> = lines.iter().take_while(|line| !line.is_empty()).collect();
    let ticket_rules = TicketRules::parse(&rule_lines)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))
        .unwrap();
    let my_ticket = Ticket::parse(&lines[rule_lines.len() + 2]).unwrap();
    let other_tickets = lines[rule_lines.len() + 5..]
        .iter()
        .map(|line| Ticket::parse(line).unwrap())
        .collect();
    (ticket_rules, my_ticket, other_tickets)
}

fn part1(rules: &TicketRules, tickets: &[Ticket]) -> u32 {
    tickets.iter().map(|t| t.error_rate(rules)).sum()
}

fn part2(rules: &TicketRules, my_ticket: &Ticket, tickets: &[Ticket]) -> u64 {
    let valid_tickets: Vec<&Ticket> = tickets
        .iter()
        .filter(|t| t.error_rate(rules) == 0)
        .collect();
    let mut options = my_ticket
        .vals
        .iter()
        .map(|_| rules.fields.keys().cloned().collect::<HashSet<String>>())
        .collect::<Vec<HashSet<String>>>();
    for ticket in valid_tickets.iter() {
        for (idx, val) in ticket.vals.iter().enumerate() {
            let new_opts = options[idx]
                .iter()
                .filter(|key| {
                    let rule = rules.fields.get(*key).unwrap();
                    rule.matches(*val)
                })
                .cloned()
                .collect();
            options[idx] = new_opts;
        }
    }
    let mut done: HashSet<usize> = HashSet::new();
    loop {
        for i in 0..options.len() {
            if done.contains(&i) {
                continue;
            }
            if options[i].len() == 1 {
                done.insert(i);
                for j in 0..options.len() {
                    if done.contains(&j) {
                        continue;
                    }
                    options[j] = options[j].difference(&options[i]).cloned().collect();
                }
            }
        }
        if done.len() == options.len() {
            break;
        }
    }
    let mut result: u64 = 1;
    for (idx, opt) in options.iter().enumerate() {
        if opt.iter().next().unwrap().starts_with("departure") {
            result *= my_ticket.vals[idx] as u64;
        }
    }
    result
}

fn main() -> Result<(), Error> {
    let (rules, my_ticket, other_tickets) = read(File::open("input.txt")?);
    println!(
        "{} rules, {} nearby tickets",
        rules.fields.len(),
        other_tickets.len()
    );
    println!("Ticket error rate: {}", part1(&rules, &other_tickets));
    println!(
        "Multiplied departure values: {}",
        part2(&rules, &my_ticket, &other_tickets)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
