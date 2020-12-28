use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::collections::{HashMap,HashSet};
use super::Day;

#[derive(Debug, Clone)]
pub struct TicketRule {
    name: String,
    valid_ranges: Vec<RangeInclusive<u64>>
}

#[derive(Debug)]
pub struct TicketInfo {
    rules: Vec<TicketRule>,
    mine: Vec<u64>,
    theirs: Vec<Vec<u64>>
}

pub struct Day16 {}

impl Day for Day16 { 
    type Input = TicketInfo;
    type Output = u64;

    fn read() -> TicketInfo {
        let file = File::open("./src/day16/input").expect("Input file must exist");
        let mut lines = BufReader::new(file).lines();
        let mut reading_rules = true;
        let mut reading_mine = false;
        let mut reading_theirs = false;
        let mut rules: Vec<TicketRule> = vec![];
        let mut mine: Vec<u64> = vec![];
        let mut theirs: Vec<Vec<u64>> = vec![];
        while let Some(line) = lines.next() {
            let line = line.expect("Should be line");
            let line = line.trim();
            if line.is_empty() {
                if reading_rules {
                    reading_rules = false;
                    reading_mine = true;
                    lines.next();
                } else if reading_mine {
                    reading_mine = false;
                    reading_theirs = true;
                    lines.next();
                } else {
                    break;
                }
            } else if reading_rules {
                let parts: Vec<&str> = line.split(": ").collect();
                let name = parts[0];
                let ranges: Vec<RangeInclusive<u64>> = parts[1].split(" or ").collect::<Vec<&str>>().iter().map(|r| {
                    let parts: Vec<&str> = r.split('-').collect();
                    let start = parts[0].parse::<u64>().expect("Should be integer");
                    let end = parts[1].parse::<u64>().expect("Should be integer");
                    RangeInclusive::new(start, end)
                }).collect();
                rules.push(TicketRule { name: name.to_string(), valid_ranges: ranges });
            } else if reading_mine {
                let fields = line
                    .split(',')
                    .map(|p| p.parse::<u64>().expect("Should be integer"))
                    .collect();
                mine = fields;
            } else if reading_theirs {
                let fields = line
                    .split(',')
                    .map(|p| p.parse::<u64>().expect("Should be integer"))
                    .collect();
                theirs.push(fields);
            }
        }
        TicketInfo { rules: rules, mine: mine, theirs: theirs }
    }

    fn part1(input: &TicketInfo) -> u64 {
        let mut invalid_values = vec![];
        for ticket in &input.theirs {
            for field in ticket {
                let mut valid = false;
                for rule in &input.rules {
                    for range in &rule.valid_ranges {
                        if range.contains(field) {
                            valid = true;
                        }
                    }
                }
                if valid == false {
                    invalid_values.push(*field);
                }
            }
        }
        invalid_values.iter().fold(0, |acc, i| acc + i)
    }

    fn part2(input: &TicketInfo) -> u64 {
        let valid_theirs: Vec<&Vec<u64>> = input.theirs.iter().filter(|ticket| {
            let mut ticket_valid = true;
            for field in *ticket {
                let mut field_valid = false;
                for rule in &input.rules {
                    for range in &rule.valid_ranges {
                        if range.contains(field) {
                            field_valid = true;
                        }
                    }
                }
                if !field_valid {
                    ticket_valid = false;
                }
            }
            ticket_valid
        }).collect();
        let mut mapping: HashMap<String,usize> = HashMap::new();
        let mut available: HashSet<usize> = HashSet::new();
        for idx in 0..input.theirs[0].len() {
            available.insert(idx);
        }

        let mut fields = input.rules.clone();
        while !fields.is_empty() {
            for (i, field) in fields.clone().iter().enumerate() {
                let mut valid_columns: Vec<(String, usize, usize)> = vec![];
                for idx in &available {
                    let mut all_valid = true;
                    for ticket in &valid_theirs {
                        let mut in_valid_range = false;
                        for range in &field.valid_ranges {
                            if range.contains(&ticket[*idx]) {
                                in_valid_range = true;
                            }
                        }
                        if !in_valid_range {
                            all_valid = false;
                        }
                    }
                    if all_valid {
                        valid_columns.push((field.name.clone(), *idx, i));
                    }
                }
                if valid_columns.len() == 1 {
                    let (name, idx, i) = &valid_columns[0];
                    mapping.entry(name.to_string()).or_insert(*idx);
                    available.remove(idx);
                    fields.remove(*i);
                    break;
                }
            }
        }
        let mut departure_values = 1;
        for (k, v) in mapping {
            if k.starts_with("departure") {
                departure_values *= input.mine[v as usize];
            }
        }
        departure_values
    }
}