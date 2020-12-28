use std::fs::File;
use std::io::{BufRead, BufReader};
use super::Day;

pub struct Rule {
    min: usize,
    max: usize,
    char: String
}

impl Rule {
    fn is_valid_for_part1(&self, password: &String) -> bool {
        let mut count = 0;
        for c in password.chars() {
            if self.char.contains(c) {
                count += 1;
            }
        }
        count >= self.min && count <= self.max
    }
    fn is_valid_for_part2(&self, password: &String) -> bool {
        let a = password.char_indices().nth(self.min - 1).expect("Char a").1;
        let b = password.char_indices().nth(self.max - 1).expect("Char b").1;
        self.char.contains(a) ^ self.char.contains(b)
    }
}

pub struct Day2 {}

impl Day for Day2 {
    type Input = Vec<(Rule,String)>;
    type Output = usize;

    fn read() -> Vec<(Rule,String)> {
        let mut data: Vec<(Rule,String)> = vec![];
        let file = File::open("./src/day2/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let value = line.expect("Line must be present");
            let parts: Vec<&str> = value.split(':').collect();
            let password = String::from(parts.get(1).expect("No Password").trim());
            let parts: Vec<&str> = parts.get(0).expect("Couldn't break down rule").split(' ').collect();
            let char = String::from(parts.get(1).expect("Couldn't find char").trim());
            let parts: Vec<&str> = parts.get(0).expect("Couldn't extract range").split('-').collect();
            let min = parts.get(0).expect("Couldn't find min").trim().parse::<usize>().expect("Couldn't parse min");
            let max = parts.get(1).expect("Couldn't find max").trim().parse::<usize>().expect("Couldn't parse max");
            data.push((Rule { min, max, char }, password))
        }
        data
    }
    
    fn part1(input: &Vec<(Rule,String)>) -> usize {
        input
            .into_iter()
            .filter(|p| p.0.is_valid_for_part1(&p.1))
            .count()
    }
    
    fn part2(input: &Vec<(Rule,String)>) -> usize {
        input
            .into_iter()
            .filter(|p| p.0.is_valid_for_part2(&p.1))
            .count()
    }
}

