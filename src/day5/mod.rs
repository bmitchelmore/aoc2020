use std::fs::File;
use std::io::{BufRead, BufReader};
use super::Day;

pub struct Day5 {}

impl Day for Day5 { 
    type Input = Vec<u32>;
    type Output = u32;

    fn read() -> Vec<u32> {
        let mut data: Vec<u32> = vec![];
        let file = File::open("./src/day5/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            
            let line = line.expect("Line must be present");
            let mut seat: u32 = 0;
            for c in line.chars() {
                let b = match c {
                    'B' | 'R' => 1,
                    'F' | 'L' => 0,
                    _ => panic!("Invalid input! {:?}", c)
                };
                seat |= b;
                seat <<= 1;
            }
            seat >>= 1;
            // let row = seat >> 3;
            // let col = seat & 0x7;
            data.push(seat);
        }
        data
    }

    fn part1(input: &Vec<u32>) -> u32 {
        input.into_iter().fold(0, |acc, x| acc.max(*x))
    }

    fn part2(input: &Vec<u32>) -> u32 {
        let max = input.into_iter().fold(0, |acc, x| acc.max(*x));
        let min = input.into_iter().fold(max, |acc, x| acc.min(*x));
        let sum = sum_to(max) - sum_to(min - 1);
        let total: u32 = input.into_iter().sum();
        sum - total
    }
}

fn sum_to(i: u32) -> u32 {
    (i * (i + 1)) / 2
}