use std::fs::File;
use std::io::{BufRead, BufReader};
use super::Day;

pub struct Day10 {}

impl Day for Day10 { 
    type Input = Vec<u64>;
    type Output = u64;

    fn read() -> Vec<u64> {
        let mut data: Vec<u64> = vec![];
        let file = File::open("./src/day10/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let line = line.expect("Line must be present");
            let value = line.parse::<u64>().expect("Value must be integer");
            data.push(value);
        }
        data
    }

    fn part1(input: &Vec<u64>) -> u64 {
        let mut ones = 0;
        let mut threes = 0;
        let mut list = input.clone();
        let mut joltage = 0;
        list.sort();
        while !list.is_empty() {
            let diff = list.first().expect("List cannot be empty") - joltage;
            match diff {
                1 => ones += 1,
                2 => (),
                3 => threes += 1,
                _ => panic!("Invalid joltage jump!")
            } 
            joltage += diff;
            list.remove(0);
        }
        threes += 1;
        println!("{:?} {:?}", ones, threes);
        ones * threes
    }

    fn part2(input: &Vec<u64>) -> u64 {
        let run_combinations: Vec<u64> = vec![0, 1, 2, 4, 7, 13, 24];
        let mut list = input.clone();
        list.sort();
        list.insert(0, 0);
        let diffs: Vec<u64> = list.iter().zip(list.iter().skip(1)).map(|(cur, next)| next - cur).collect();
        let mut run_values: Vec<u64> = vec![0, 0, 0, 0, 0, 0, 0];
        let mut ones = 0;
        for (i, diff) in diffs.iter().enumerate() {
            if *diff == 1 {
                ones += 1;
                if i == diffs.len() - 1 {
                    run_values[ones] += 1;
                    ones = 0;
                }
            } else if ones > 0 {
                run_values[ones] += 1;
                ones = 0;
            }
        }
        let combinations: Vec<u64> = run_values.iter().enumerate().map(|(i, s)| run_combinations[i].pow(*s as u32)).collect();
        let combined = combinations.iter().fold(1, |acc, x| acc * x);
        combined
    }
}