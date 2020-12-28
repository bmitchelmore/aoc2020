use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use super::Day;

#[derive(Debug, Clone)]
pub struct Rule {
    id: u32,
    cond: RuleCondition
}

#[derive(Debug, Clone)]
enum RuleCondition {
    Exact(String),
    Any(Vec<Vec<u32>>),
}

#[derive(Debug, Clone)]
pub struct Input {
    rules: HashMap<u32, Rule>,
    messages: Vec<String>
}

enum ReadState {
    ReadingRules,
    ReadingMessages
}

impl Rule {
    fn rule_matches(&self, message: &String, rules: &HashMap<u32, Rule>) -> Vec<String> {
        match &self.cond {
            RuleCondition::Exact(e) => if message.starts_with(e) { vec![ message.chars().skip(e.len()).collect() ] } else { vec![] },
            RuleCondition::Any(s) => {
                s
                    .iter()
                    .flat_map(|s| {
                        let mut all_messages = vec![ message.clone() ];
                        let all: Vec<&Rule> = s
                            .iter()
                            .map(|i| rules.get(i).expect("Rule should exist"))
                            .collect();
                        for rule in &all {
                            if all_messages.iter().fold(true, |acc, m| acc && m.is_empty()) {
                                all_messages.drain(..);
                                break;
                            }
                            if all_messages.is_empty() {
                                break;
                            }
                            let mut updated_messages = vec![];
                            for m in &all_messages {
                                let mut matches = rule.rule_matches(&m, rules);
                                updated_messages.append(&mut matches);
                            }
                            all_messages = updated_messages;
                        }
                        all_messages
                    })
                    .collect()
            }
        }
    }
    fn is_message_valid(&self, message: &String, rules: &HashMap<u32, Rule>) -> bool {
        let matches = self.rule_matches(message, rules);
        if matches.is_empty() {
            false
        } else if !matches.iter().filter(|m| m.len() == 0).collect::<Vec<&String>>().is_empty() {
            true
        } else {
            false
        }
    }
}

pub struct Day19 {}

impl Day for Day19 { 
    type Input = Input;
    type Output = u64;

    fn read() -> Input {
        let mut rules: HashMap<u32, Rule> = HashMap::new();
        let mut messages: Vec<String> = vec![];
        let mut read_state = ReadState::ReadingRules;
        let file = File::open("./src/day19/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let line = line.expect("Line must be present");
            let line = line.trim();
            match read_state {
                ReadState::ReadingRules => {
                    if line == "" {
                        read_state = ReadState::ReadingMessages;
                    } else {
                        let parts: Vec<&str> = line.split(": ").collect();
                        let id = parts[0].parse::<u32>().expect("Should be integer");
                        if parts[1].starts_with('"') {
                            let string = parts[1].trim_matches('"');
                            let rule = Rule { 
                                id: id, 
                                cond: RuleCondition::Exact(string.to_string()) 
                            };
                            rules.entry(id).or_insert(rule);
                        } else {
                            let parts: Vec<Vec<u32>> = parts[1]
                                .split(" | ")
                                .map(|part| {
                                    part
                                        .split(' ')
                                        .map(|p| p.parse::<u32>().expect("Should be integer"))
                                        .collect()
                                }).collect();
                            let rule = Rule {
                                id: id,
                                cond: RuleCondition::Any(parts)
                            };
                            rules.entry(id).or_insert(rule);
                        }
                    }
                },
                ReadState::ReadingMessages => {
                    messages.push(line.to_string());
                }
            }
        }
        Input { rules: rules, messages: messages }
    }

    fn part1(input: &Input) -> u64 {
        let rule = input.rules.get(&0).expect("Should have rule 0");
        input.messages
            .iter()
            .filter(|m| rule.is_message_valid(m, &input.rules))
            .collect::<Vec<&String>>()
            .len() as u64
    }

    fn part2(input: &Input) -> u64 {
        let mut updated = Input {
            rules: input.rules.clone(),
            messages: input.messages.clone()
        };
        updated.rules.entry(8).and_modify(|e| *e = Rule { id: 8, cond: RuleCondition::Any(vec![vec![42], vec![42, 8]]) });
        updated.rules.entry(11).and_modify(|e| *e = Rule { id: 11, cond: RuleCondition::Any(vec![vec![42, 31], vec![42, 11, 31]]) });
        // println!("Rules: {:?}", updated.rules);
        let rule = input.rules.get(&0).expect("Should have rule 0");
        input.messages
            .iter()
            .filter(|m| rule.is_message_valid(m, &updated.rules))
            .collect::<Vec<&String>>()
            .len() as u64
    }
}