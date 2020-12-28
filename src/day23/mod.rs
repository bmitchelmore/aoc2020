use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::ops::RangeInclusive;
use super::Day;

struct Game {
    current: u64,
    range: RangeInclusive<u64>,
    cups: HashMap<u64,u64>
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut val = self.cups[&self.current];
        write!(f, "{}", self.current)?;
        loop {
            write!(f, ", {}", val)?;
            val = self.cups[&val];
            if val == self.current {
                write!(f, "\n")?;
                break;
            }
        }
        Ok(())
    }
}

impl Game {
    fn from(vec: &Vec<u64>, range: RangeInclusive<u64>) -> Game {
        let max = vec.iter().fold(u64::MIN, |acc, c| acc.max(*c));
        let current = *vec.first().unwrap();
        let mut cups = HashMap::new();
        for (&cur, &nxt) in vec.iter().zip(vec.iter().skip(1)) {
            cups.entry(cur).or_insert(nxt);
        }
        let mut prev = *vec.last().unwrap();
        for i in max+1..=*range.end() {
            cups.entry(prev).or_insert(i);
            prev = i;
        }
        cups.entry(prev).or_insert(current);
        Game { 
            current: current,
            range: range,
            cups: cups
        }
    }
    fn play(&mut self) {
        let mut extracted = vec![];
        let mut next = self.cups[&self.current];
        for _ in 0..3 {
            extracted.push(next);
            let value = next;
            next = self.cups[&next];
            self.cups.remove(&value);
        }
        self.cups.entry(self.current).and_modify(|e| *e = next);

        let mut dest = self.dec(self.current);
        loop {
            if let Some(&val) = self.cups.get(&dest) {
                let first = *extracted.first().unwrap();
                self.cups.entry(dest).and_modify(|e| *e = first);
                dest = first;
                for c in extracted.iter().skip(1) {
                    self.cups.entry(dest).or_insert(*c);
                    dest = *c;
                }
                self.cups.entry(*extracted.last().unwrap()).or_insert(val);
                break;
            } else {
                dest = self.dec(dest);
            }
        }

        self.current = next;
    }
    fn dec(&self, val: u64) -> u64 {
        if val == *self.range.start() { 
            *self.range.end() 
        } else { 
            val - 1 
        }
    }
}

pub struct Day23 {}

impl Day for Day23 { 
    type Input = Vec<u64>;
    type Output = String;

    fn read() -> Vec<u64> {
        let file = File::open("./src/day23/input").expect("Input file must exist");
        let line = BufReader::new(file).lines().next().expect("Line should be present");
        let line = line.expect("Should be line");
        let line = line.trim();
        line.chars().map(|c| c.to_string().parse::<u64>().expect("Should be integer")).collect()
    }

    fn part1(input: &Vec<u64>) -> String {
        let mut game = Game::from(input, 1..=9);
        for _ in 0..100 {
            game.play();
        }
        let mut cups: Vec<String> = vec![];
        let mut next_idx = game.cups[&1];
        for _ in 0..8 {
            cups.push(next_idx.to_string());
            next_idx = game.cups[&next_idx];
        }
        cups.join("")
    }

    fn part2(input: &Vec<u64>) -> String {
        let mut game = Game::from(input, 1..=1000000);
        for _ in 0..10000000 {
            game.play();
        }
        (game.cups[&1] * game.cups[&game.cups[&1]]).to_string()
    }
}