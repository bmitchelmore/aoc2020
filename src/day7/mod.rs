use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use regex::Regex;
use super::Day;

#[derive(Debug)]
pub struct BagRule {
    color: String,
    contains: Vec<(u32,String)>
}

#[derive(Debug)]
pub struct Bag {
    color: String,
    contents: Vec<(u32, Bag)>
}

impl Bag {
    fn contains(&self, target: &String) -> bool {
        let mut result = false;
        for (_count, bag) in &self.contents {
            if &bag.color == target {
                result = true;
            } else if bag.contains(target) {
                result = true;
            }
        }
        result
    }
    fn bag_size(&self) -> usize {
        let mut size = 1;
        for (count, bag) in &self.contents {
            size += *count as usize * bag.bag_size();
        }
        size
    }
}

pub struct Day7 {}

impl Day for Day7 { 
    type Input = Vec<BagRule>;
    type Output = usize;

    fn read() -> Vec<BagRule> {
        let mut data: Vec<BagRule> = vec![];
        let file = File::open("./src/day7/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let line = line.expect("Line must be present");
            let mut color: Option<String> = None;
            let mut contains: Vec<(u32,String)> = vec![];
            let regex = Regex::new(r"([a-z ]+) bags contain (no other bags|.*)\.").expect("Regex is invalid");
            if regex.is_match(&line) {
                for m in regex.captures_iter(&line) {
                    color = Some(String::from(m.get(1).expect("Color should be present").as_str()));
                    if let Some(c) = m.get(2) {
                        if c.as_str() != "no other bags" {
                            let contents = String::from(m.get(2).expect("Contents should be present").as_str());
                            let contents: Vec<&str> = contents.split(",").map(|s| s.trim()).collect();
                            for content in contents {
                                let regex = Regex::new(r"(\d+) (.*) bags?").expect("Regex is invalid");
                                for m in regex.captures_iter(content) {
                                    let count = m.get(1).expect("Count should be present").as_str().parse::<u32>().expect("Count should be integer");
                                    let color = String::from(m.get(2).expect("Color should be present").as_str());
                                    contains.push((count, color));
                                }
                            }
                        }
                    } else {
                        panic!("Invalid parse");
                    }
                }
            } else {
                println!("Invalid regex for {:?}", line);
            }
            if let Some(color) = color {
                let rule = BagRule { color: color, contains: contains };
                data.push(rule);
            }
        }
        data
    }

    fn part1(input: &Vec<BagRule>) -> usize {
        let target = String::from("shiny gold");
        let trees = build_trees(input);
        let filtered: Vec<Bag> = trees.into_iter().filter(|b| b.contains(&target)).collect();
        filtered.len()
    }

    fn part2(input: &Vec<BagRule>) -> usize {
        let target = String::from("shiny gold");
        let trees = build_trees(input);
        let filtered: Vec<Bag> = trees.into_iter().filter(|b| b.color == target).collect();
        let tree = filtered.get(0).expect("Bag should be present");
        tree.bag_size() - 1
    }
}

fn build_tree(rule: &BagRule, lookup: &HashMap<String, &BagRule>) -> Bag {
    let mut bags: Vec<(u32, Bag)> = vec![];
    for (count, color) in &rule.contains {
        bags.push((*count, build_tree(lookup.get(color).expect("Bag must exist"), lookup)))
    }
    Bag {
        color: rule.color.clone(),
        contents: bags
    }
}

fn build_trees(rules: &Vec<BagRule>) -> Vec<Bag> {
    let mut lookup: HashMap<String, &BagRule> = HashMap::new();
    for rule in rules {
        lookup.insert(rule.color.clone(), rule);
    }
    let mut trees: Vec<Bag> = Vec::new();
    for rule in rules {
        let bag = build_tree(&rule, &lookup);
        trees.push(bag);
    }
    trees
}