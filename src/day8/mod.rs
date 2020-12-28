use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use either::*;
use super::Day;

#[derive(Debug, Clone)]
pub enum Instruction {
    NOP(i32),
    ACC(i32),
    JMP(i32)
}

pub struct Day8 {}

impl Day for Day8 { 
    type Input = Vec<Instruction>;
    type Output = i32;

    fn read() -> Vec<Instruction> {
        let mut data: Vec<Instruction> = vec![];
        let file = File::open("./src/day8/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let line = line.expect("Line must be present");
            let parts: Vec<&str> = line.split(' ').collect();
            let opcode = parts.get(0).expect("Opcode must be present");
            let value = parts.get(1).expect("Value must be present").parse::<i32>().expect("Value must be integer");
            let instruction = match *opcode {
                "nop" => Instruction::NOP(value),
                "acc" => Instruction::ACC(value),
                "jmp" => Instruction::JMP(value),
                _ => panic!("Unexpected opcode {:?}", opcode)
            };
            data.push(instruction);
        }
        data
    }

    fn part1(input: &Vec<Instruction>) -> i32 {
        let result = run(input);
        match result {
            Either::Left(acc) => return acc,
            Either::Right(acc) => panic!("Program completed successfully. That shouldn't happen! ACC: {:?}", acc)
        }
    }

    fn part2(input: &Vec<Instruction>) -> i32 {
        let program = input;
        let last_index = input.len() - 1;
        for pc in 0..=last_index {
            match input[pc] {
                Instruction::NOP(value) => {
                    let program = program.clone().into_iter().enumerate().map(|(i, o)| {
                        if i == pc {
                            Instruction::JMP(value)
                        } else {
                            o
                        }
                    }).collect();
                    if let Either::Right(acc) = run(&program) {
                        return acc
                    }
                },
                Instruction::JMP(value) => {
                    let program = program.clone().into_iter().enumerate().map(|(i, o)| {
                        if i == pc {
                            Instruction::NOP(value)
                        } else {
                            o
                        }
                    }).collect();
                    if let Either::Right(acc) = run(&program) {
                        return acc
                    }
                },
                _ => ()
            }
        }
        panic!("No value found!");
    }
}

fn run(program: &Vec<Instruction>) -> Either<i32, i32> {
    let mut pc = 0;
    let mut cache: HashSet<usize> = HashSet::new();
    let mut acc: i32 = 0;
    while pc < program.len() {
        if cache.contains(&pc) {
            return Either::Left(acc)
        }
        cache.insert(pc);
        match program[pc] {
            Instruction::NOP(_) => pc += 1,
            Instruction::ACC(value) => { 
                acc += value;
                pc += 1;
            },
            Instruction::JMP(value) => pc = ((pc as i32) + value) as usize
        }
    }
    Either::Right(acc)
}