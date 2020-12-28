use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use super::Day;

#[derive(Debug, Clone)]
pub struct Player {
    id: u8,
    hand: Vec<u64>
}

impl Player {
    fn is_empty(&self) -> bool {
        self.hand.is_empty()
    }
    fn score(&self) -> u64 {
        self.hand.iter().rev().enumerate().fold(0, |acc, i| acc + (((i.0 as u64) + 1) * i.1))
    }
}

fn play(one: &mut Player, two: &mut Player) -> Player {
    let mut winner: Option<Player> = None;
    let mut history: HashSet<(Vec<u64>,Vec<u64>)> = HashSet::new();
    while !one.is_empty() && !two.is_empty() {
        let record = (one.hand.clone(), two.hand.clone());
        if history.contains(&record) {
            winner = Some(one.clone());
            break;
        }
        history.insert(record);
        let p1 = *one.hand.first().unwrap();
        let p2 = *two.hand.first().unwrap();
        one.hand.remove(0);
        two.hand.remove(0);
        if p1 <= one.hand.len() as u64 && p2 <= two.hand.len() as u64 {
            let mut s1 = Player { id: one.id, hand: one.hand[0..p1 as usize].iter().map(|c| *c).collect() };
            let mut s2 = Player { id: two.id, hand: two.hand[0..p2 as usize].iter().map(|c| *c).collect() };
            let winner = play(&mut s1, &mut s2);
            if winner.id == 1 {
                one.hand.push(p1);
                one.hand.push(p2);    
            } else if winner.id == 2 {
                two.hand.push(p2);
                two.hand.push(p1);
            } else {
                panic!("I don't know what to do!");
            }
        } else if p1 > p2 {
            one.hand.push(p1);
            one.hand.push(p2);
        } else if p2 > p1 {
            two.hand.push(p2);
            two.hand.push(p1);
        } else {
            panic!("How do ties work?");
        }
    }
    if let Some(winner) = winner {
        winner
    } else if one.is_empty() {
        two.clone()
    } else {
        one.clone()
    }
}

pub struct Input {
    one: Player,
    two: Player,
}

pub struct Day22 {}

impl Day for Day22 { 
    type Input = Input;
    type Output = u64;

    fn read() -> Input {
        let mut one: Option<Player> = None;
        let mut hand = vec![];
        let file = File::open("./src/day22/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let line = line.expect("Line must be present");
            let line = line.trim();
            if line.starts_with("Player") {
                continue;
            } else if line.is_empty() {
                one = Some(Player { id: 1, hand: hand.clone() });
                hand = vec![];
                continue;
            } else {
                let card = line.parse::<u64>().expect("Should be valid integer");
                hand.push(card);
            }
        }
        Input {
            one: one.unwrap(),
            two: Player { id: 2, hand: hand.clone() }
        }
    }

    fn part1(input: &Input) -> u64 {
        let mut one = input.one.clone();
        let mut two = input.two.clone();
        while !one.is_empty() && !two.is_empty() {
            let p1 = *one.hand.first().unwrap();
            let p2 = *two.hand.first().unwrap();
            one.hand.remove(0);
            two.hand.remove(0);
            if p1 > p2 {
                one.hand.push(p1);
                one.hand.push(p2);
            } else if p2 > p1 {
                two.hand.push(p2);
                two.hand.push(p1);
            } else {
                panic!("How do ties work?");
            }
        }
        if one.is_empty() {
            two.score()
        } else {
            one.score()
        }
    }

    fn part2(input: &Input) -> u64 {
        let mut one = input.one.clone();
        let mut two = input.two.clone();
        let winner = play(&mut one, &mut two);
        winner.score()
    }
}