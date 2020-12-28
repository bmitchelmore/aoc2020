use std::fs::File;
use std::io::{BufRead, BufReader};
use super::Day;

fn rotate(x: i32, y: i32, deg: i32) -> (i32, i32) {
    let frad = f64::to_radians(deg as f64);
    let cos = f64::cos(frad) as i32;
    let sin = f64::sin(frad) as i32;
    ( 
        cos * x - sin * y,
        sin * x + cos * y
    )
}

#[derive(Debug)]
pub enum Instruction {
    N(u32),
    E(u32),
    W(u32),
    S(u32),
    F(u32),
    L(u32),
    R(u32)
}

trait Ship {
    fn create() -> Self;
    fn perform(&mut self, instruction: &Instruction);
    fn distance_from(&self, x: i32, y: i32) -> i32;
}

#[derive(Debug)]
struct WrongShip {
    x: i32,
    y: i32,
    fx: i32,
    fy: i32
}

impl Ship for WrongShip {
    fn create() -> WrongShip {
        WrongShip { x: 0, y: 0, fx: 1, fy: 0 }
    }
    fn perform(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::N(num) => self.y -= *num as i32,
            Instruction::S(num) => self.y += *num as i32,
            Instruction::E(num) => self.x += *num as i32,
            Instruction::W(num) => self.x -= *num as i32,
            Instruction::R(num) => {
                let (nfx, nfy) = rotate(self.fx, self.fy, *num as i32);
                self.fx = nfx;
                self.fy = nfy;
            },
            Instruction::L(num) => {
                let (nfx, nfy) = rotate(self.fx, self.fy, -(*num as i32));
                self.fx = nfx;
                self.fy = nfy;
            },
            Instruction::F(num) => {
                self.x += self.fx * *num as i32;
                self.y += self.fy * *num as i32;
            }
        }
    }
    fn distance_from(&self, x: i32, y: i32) -> i32 {
        (x - self.x).abs() + (y - self.y).abs()
    }
}

#[derive(Debug)]
struct RightShip {
    x: i32,
    y: i32,
    wx: i32,
    wy: i32,
}

impl Ship for RightShip {
    fn create() -> RightShip {
        RightShip { x: 0, y: 0, wx: 10, wy: -1 }
    }
    fn perform(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::N(num) => self.wy -= *num as i32,
            Instruction::S(num) => self.wy += *num as i32,
            Instruction::E(num) => self.wx += *num as i32,
            Instruction::W(num) => self.wx -= *num as i32,
            Instruction::R(num) => {
                let (nwx, nwy) = rotate(self.wx, self.wy, *num as i32);
                self.wx = nwx;
                self.wy = nwy;
            },
            Instruction::L(num) => {
                let (nwx, nwy) = rotate(self.wx, self.wy, -(*num as i32));
                self.wx = nwx;
                self.wy = nwy;
            },
            Instruction::F(num) => {
                self.x += self.wx * *num as i32;
                self.y += self.wy * *num as i32;
            }
        }
    }
    fn distance_from(&self, x: i32, y: i32) -> i32 {
        (x - self.x).abs() + (y - self.y).abs()
    }
}

pub struct Day12 {}

impl Day for Day12 { 
    type Input = Vec<Instruction>;
    type Output = i32;

    fn read() -> Vec<Instruction> {
        let mut data: Vec<Instruction> = vec![];
        let file = File::open("./src/day12/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let line = line.expect("Line must be present");
            let mut chars = line.chars();
            let c = chars.next().expect("First char should be present");
            let num = chars.as_str().parse::<u32>().expect("Should contain integer");
            let instruction = match c {
                'N' => Instruction::N(num),
                'E' => Instruction::E(num),
                'W' => Instruction::W(num),
                'S' => Instruction::S(num),
                'L' => Instruction::L(num),
                'R' => Instruction::R(num),
                'F' => Instruction::F(num),
                _ => panic!("Unknown instruction: {:?}", c)
            };
            data.push(instruction);
        }
        data
    }

    fn part1(input: &Vec<Instruction>) -> i32 {
        let mut ship = WrongShip::create();
        for instruction in input {
            ship.perform(instruction);
        }
        ship.distance_from(0, 0)
    }

    fn part2(input: &Vec<Instruction>) -> i32 {
        let mut ship = RightShip::create();
        for instruction in input {
            ship.perform(instruction);
        }
        ship.distance_from(0, 0)
    }
}