use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use super::Day;

pub struct Day9 {}

impl Day for Day9 { 
    type Input = Vec<u64>;
    type Output = u64;

    fn read() -> Vec<u64> {
        let mut data: Vec<u64> = vec![];
        let file = File::open("./src/day9/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let line = line.expect("Line must be present");
            let value = line.parse::<u64>().expect("Value must be integer");
            data.push(value);
        }
        data
    }

    fn part1(input: &Vec<u64>) -> u64 {
        let preamble_size = 25;
        let first_index = preamble_size;
        let last_index = input.len() - 1;
        for i in first_index..=last_index {
            let value = input[i];
            let end = i - 1;
            let start = end - (preamble_size - 1);
            let mut sum_exists = false;
            for pair in input[start..=end].into_iter().combinations(2) {
                if pair.into_iter().fold(0, |acc, x| acc + x) == value {
                    sum_exists = true;
                    break;
                }
            }
            if !sum_exists {
                return value;
            }
        }
        panic!("Entire sequence is valid!");
    }

    fn part2(input: &Vec<u64>) -> u64 {
        let target = Self::part1(input);
        let largest_subset = input.len();
        for i in 2..=largest_subset {
            let last_start = input.len() - 1 - i;
            for j in 0..=last_start {
                let sum = input[j..=j+i].into_iter().fold(0, |acc, x| acc + x);
                if sum == target {
                    let slice: Vec<u64> = input[j..=j+i].into_iter().map(|i| *i).collect();
                    let min = slice.iter().fold(u64::MAX, |m, x| m.min(*x));
                    let max = slice.iter().fold(u64::MIN, |m, x| m.max(*x));
                    return max + min
                }
            }
        }
        panic!("Sum not found!");
    }
}
