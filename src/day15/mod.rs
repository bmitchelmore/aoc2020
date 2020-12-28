use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use super::Day;

fn next_num(history: &HashMap<u64, u64>, current_num: u64, current_turn: u64) -> u64 {
    match history.get(&current_num) {
        Some(turn) => current_turn - turn,
        None => 0
    }
}

fn play_game(prelude: &Vec<u64>, turns: u64) -> u64 {
    let mut history: HashMap<u64, u64> = HashMap::new();
    for (turn, num) in prelude.iter().enumerate() {
        if turn == prelude.len() - 1 {
            break;
        }
        match history.entry(*num) {
            Entry::Occupied(mut entry) => { 
                entry.insert(turn as u64);
            },
            Entry::Vacant(entry) => { 
                entry.insert(turn as u64);
            }
        };
    }
    let mut current = prelude[prelude.len() - 1];
    let start = prelude.len() as u64 - 1;
    let end = turns - 1;
    for turn in start..end {
        let next = next_num(&history, current, turn);
        match history.entry(current) {
            Entry::Occupied(mut entry) => { 
                entry.insert(turn);
            },
            Entry::Vacant(entry) => { 
                entry.insert(turn);
            }
        }
        current = next;
    }
    current
}

pub struct Day15 {}

impl Day for Day15 { 
    type Input = Vec<u64>;
    type Output = u64;

    fn read() -> Vec<u64> {
        let file = File::open("./src/day15/input").expect("Input file must exist");
        let line = BufReader::new(file).lines().next().expect("Should be line").expect("Should be line");
        let line = line.trim();
        line.split(',').map(|n| n.parse::<u64>().expect("Should be integer")).collect()
    }

    fn part1(input: &Vec<u64>) -> u64 {
        play_game(input, 2020)
    }

    fn part2(input: &Vec<u64>) -> u64 {
        play_game(input, 30000000)       
    }
}