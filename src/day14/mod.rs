use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use super::Day;

#[derive(Debug)]
pub enum Instruction {
    UpdateMask(Vec<Option<bool>>),
    AssignMemory(u64, u64),
}

impl Instruction {
    fn from(str: &str) -> Instruction {
        let parts: Vec<&str> = str.split(" = ").collect();
        if parts[0] == "mask" {
            let mask: Vec<Option<bool>> = parts[1].chars().map(|c| {
                match c {
                    'X' => None,
                    '0' => Some(false),
                    '1' => Some(true),
                    _ => panic!("Invalid value in bitmask: {:?}", c)
                }
            }).collect();
            return Instruction::UpdateMask(mask);
        } else {
            let subparts: Vec<&str> = parts[0].split("[").collect();
            if subparts[0] == "mem" {
                let location = subparts[1].trim_end_matches(']').parse::<u64>().expect("Should be valid integer");
                let value = parts[1].parse::<u64>().expect("Should be valid integer");
                return Instruction::AssignMemory(location, value);
            } else {
                panic!("Invalid instruction: {:?}", str);
            }
        }
    }
}

fn apply_bitmask(value: &u64, mask: &Vec<Option<bool>>) -> u64 {
    let mut updated = 0;
    for (i, mask) in mask.iter().rev().enumerate() {
        let mut bit: bool = (value >> i) & 0b1 == 0b1;
        if let Some(mask) = mask {
            bit = *mask;
        }
        if bit {
            updated |= ((bit as u64) << i) as u64;
        }
    }
    updated
}

fn values_from_masking_bit(bit: &Option<bool>) -> Vec<bool> {
    if let Some(value) = bit {
        vec![*value]
    } else {
        vec![false, true]
    }
}

fn generate_all_locations_from_masked_location(location: &Vec<Option<bool>>) -> Vec<Vec<bool>> {
    if location.is_empty() {
        vec![]
    } else if location.len() == 1 {
        values_from_masking_bit(&location[0]).iter().map(|v| vec![*v]).collect()
    } else {
        values_from_masking_bit(&location[0]).iter().flat_map(|v| {
            let mut locations = vec![];
            let remainder = location[1..location.len()].to_vec().clone();
            let suffixes = generate_all_locations_from_masked_location(&remainder);
            for suffix in suffixes {
                let mut value = vec![*v];
                for bit in suffix {
                    value.push(bit);
                }
                locations.push(value);
            }
            locations
        }).collect()
    }
}

fn generate_locations_from_bitmask(value: &u64, mask: &Vec<Option<bool>>) -> Vec<u64> {
    let masked: Vec<Option<bool>> = mask.iter().rev().enumerate().map(|(i,mask)| {
        let bit: bool = (value >> i) & 0b1 == 0b1;
        if let Some(mask) = mask {
            if *mask {
                Some(true)
            } else {
                Some(bit)
            }
        } else {
            None
        }
    }).rev().collect();
    let locations = generate_all_locations_from_masked_location(&masked);
    locations.iter().map(|l| convert_location_bits_to_location(l)).collect()
}

fn convert_location_bits_to_location(bits: &Vec<bool>) -> u64 {
    let mut updated = 0;
    for (i, bit) in bits.iter().rev().enumerate() {
        if *bit {
            updated |= ((*bit as u64) << i) as u64;
        }
    }
    updated
}

fn _format_bitmask(mask: &Vec<Option<bool>>) -> String {
    mask.iter().map(|o| {
        if let Some(true) = o {
            '1'
        } else if let Some(false) = o {
            '0'
        } else {
            'X'
        }
    }).collect()
}

fn _format_bits(mask: &Vec<bool>) -> String {
    mask.iter().map(|o| {
        if *o {
            '1'
        } else {
            '0'
        }
    }).collect()
}

pub struct Day14 {}

impl Day for Day14 { 
    type Input = Vec<Instruction>;
    type Output = u64;

    fn read() -> Vec<Instruction> {
        let mut data = vec![];
        let file = File::open("./src/day14/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let line = line.expect("Should be line");
            let line = line.trim();
            data.push(Instruction::from(line));
        }
        data
    }

    fn part1(input: &Vec<Instruction>) -> u64 {
        let mut memory: HashMap<u64,u64> = HashMap::new();
        let mut current_mask: Vec<Option<bool>> = [None; 36].iter().map(|o| *o).collect();
        for instruction in input {
            match instruction {
                Instruction::AssignMemory(location, value) => {
                    let value = apply_bitmask(&value, &current_mask);
                    match memory.entry(*location) {
                        Entry::Occupied(mut entry) => {
                            if value == 0 {
                                entry.remove_entry();
                            } else {
                                entry.insert(value);
                            }
                        },
                        Entry::Vacant(entry) => {
                            if value != 0 {
                                entry.insert(value);
                            }
                        }
                    }
                },
                Instruction::UpdateMask(mask) => {
                    current_mask = mask.clone();
                }
            }
        }
        memory.values().fold(0, |acc, i| acc + i)
    }

    fn part2(input: &Vec<Instruction>) -> u64 {
        let mut memory: HashMap<u64,u64> = HashMap::new();
        let mut current_mask: Vec<Option<bool>> = [None; 36].iter().map(|o| *o).collect();
        for instruction in input {
            match instruction {
                Instruction::AssignMemory(location, value) => {
                    let locations = generate_locations_from_bitmask(&location, &current_mask);
                    for location in locations {
                        match memory.entry(location) {
                            Entry::Occupied(mut entry) => {
                                if *value == 0 {
                                    entry.remove_entry();
                                } else {
                                    entry.insert(*value);
                                }
                            },
                            Entry::Vacant(entry) => {
                                if *value != 0 {
                                    entry.insert(*value);
                                }
                            }
                        }
                    }
                },
                Instruction::UpdateMask(mask) => {
                    current_mask = mask.clone();
                }
            }
        }
        memory.values().fold(0, |acc, i| acc + i)
    }
}