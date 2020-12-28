use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::collections::HashSet;
use super::Day;

fn read_from<R: Read>(reader: &mut BufReader<R>) -> Option<Vec<HashSet<char>>> {
    let mut working = true;
    let mut result: Option<Vec<HashSet<char>>> = None;
    while working {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => working = false,
            Ok(_count) => {
                line = line.trim().to_string();
                if line == "" {
                    working = false;
                } else {
                    let mut collection =  result.unwrap_or(Vec::new());
                    let mut answers = HashSet::new();
                    for c in line.chars() {
                        match c {
                            'a'..='z' => answers.insert(c),
                            _ => panic!("Unexpected char {:?}", c)
                        };
                    }
                    collection.push(answers);
                    result = Some(collection)
                }
            }, 
            _ => panic!("Invalid input!")
        }
    }
    result
}

pub struct Day6 {}

impl Day for Day6 { 
    type Input = Vec<Vec<HashSet<char>>>;
    type Output = usize;

    fn read() -> Vec<Vec<HashSet<char>>> {
        let mut data: Vec<Vec<HashSet<char>>> = vec![];
        let file = File::open("./src/day6/input").expect("Input file must exist");
        let mut reader = BufReader::new(file);
        while let Some(record) = read_from(&mut reader) {
            data.push(record);
        }
        data
    }

    fn part1(input: &Vec<Vec<HashSet<char>>>) -> usize {
        input.into_iter().fold(0, |acc, group| {
            let any_answers = group.into_iter().fold(HashSet::new(), |acc, x| {
                acc.union(x).copied().collect()
            });
            acc + any_answers.len()
        })
    }

    fn part2(input: &Vec<Vec<HashSet<char>>>) -> usize {
        let mut all: HashSet<char> = HashSet::new();
        for a in 'a'..='z' {
            all.insert(a);
        }
        input.into_iter().fold(0, |acc, group| {
            let any_answers = group.into_iter().fold(all.clone(), |acc, x| {
                acc.intersection(x).copied().collect()
            });
            acc + any_answers.len()
        })
    }
}
