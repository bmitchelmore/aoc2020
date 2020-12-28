use std::fs::File;
use std::io::{BufRead, BufReader};
use super::Day;

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_number) % 20201227;
    }
    value
}

fn seek_loop_size(target: u64) -> u64 {
    let subject_number = 7;
    let mut value = 1;
    let mut loop_size = 1;
    loop {
        value = (value * subject_number) % 20201227;
        if value == target {
            return loop_size;
        }
        loop_size += 1;
    }
}

pub struct Day25 {}

impl Day for Day25 { 
    type Input = (u64, u64);
    type Output = u64;

    fn read() -> (u64, u64) {
        let file = File::open("./src/day25/input").expect("Input file must exist");
        let mut lines = BufReader::new(file).lines();
        let first = lines.next().unwrap().unwrap().trim().to_string();
        let second = lines.next().unwrap().unwrap().trim().to_string();
        (first.parse::<u64>().unwrap(), second.parse::<u64>().unwrap())
    }

    fn part1(input: &(u64, u64)) -> u64 {
        let door_public_key = input.0;
        let card_public_key = input.1;
        let _door_loop_size = seek_loop_size(door_public_key);
        let card_loop_size = seek_loop_size(card_public_key);
        let encryption_key = transform(door_public_key, card_loop_size);
        encryption_key
    }

    fn part2(_input: &(u64, u64)) -> u64 {
        0
    }
}