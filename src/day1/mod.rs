use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use super::Day;

pub struct Day1 {}

impl Day for Day1 {
    type Input = Vec<u32>;
    type Output = u32;

    fn read() -> Vec<u32> {
        let mut data: Vec<u32> = vec![];
        let file = File::open("./src/day1/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let value = line
                .expect("Line must be present")
                .trim()
                .parse::<u32>()
                .expect("Line must contain integer");
            data.push(value);
        }
        data
    }

    fn part1(input: &Vec<u32>) -> u32 {
        find(input, 2, 2020)
    }
    
    fn part2(input: &Vec<u32>) -> u32 {
        find(input, 3, 2020)
    }
}

fn find(input: &Vec<u32>, k: usize, val: u32) -> u32 {
    input
        .into_iter()
        .combinations(k)
        .find(|x| x.into_iter().fold(0, |acc, x| acc + *x) == val)
        .expect("Sum must be present in dataset")
        .into_iter()
        .fold(1, |acc, x| acc * *x)
}