use std::fs::File;
use std::io::{BufRead, BufReader};
use super::Day;

#[derive(Debug, Clone)]
pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Paren(Box<Expression>),
    Num(u64)
}

impl Expression {
    fn from(string: &str) -> Expression {
        let mut chunks: Vec<String> = vec![];
        let mut current = String::new();
        let mut paren_depth = 0;
        for c in string.chars() {
            current.push(c);
            if c == '(' {
                paren_depth += 1;
            } else if c == ')' {
                paren_depth -= 1;
                if paren_depth == 0 {
                    chunks.push(current);
                    current = String::new();
                }
            } else if c.is_whitespace() && paren_depth == 0 {
                chunks.push(current);
                current = String::new();
            }
        }
        chunks.push(current);

        let filtered: Vec<&str> = chunks
            .iter()
            .map(|c| c.trim())
            .filter(|c| !c.is_empty())
            .collect();
        if filtered.is_empty() {
            panic!("Invalid sequence: {:?}", string);
        } else if filtered.len() == 1 {
            let chunk = filtered[0];
            if chunk.chars().all(char::is_numeric) {
                let num = chunk.parse::<u64>().expect("Should be valid integer");
                return Expression::Num(num);
            } else if chunk.starts_with("(") && chunk.ends_with(")") {
                return Expression::Paren(Box::new(Expression::from(&chunk[1..chunk.len()-1])))                
            } else {
                panic!("Invalid chunk: {:?}", chunk);
            }
        } else {
            let mut expr: Expression = Expression::from(filtered[0]);
            let mut previous_op: Option<&str> = None;
            for &chunk in filtered.iter().skip(1) {
                match chunk {
                    "*" | "+" => previous_op = Some(chunk),
                    _ => {
                        match previous_op {
                            Some("*") => {
                                expr = Expression::Mul(Box::new(expr), Box::new(Expression::from(chunk)))
                            },
                            Some("+") => {
                                expr = Expression::Add(Box::new(expr), Box::new(Expression::from(chunk)))
                            },
                            _ => panic!("Unexpected sequence: {:?}", chunk)
                        }
                    }
                }
            }
            return expr;
        }
    }
    fn evaluate(&self) -> u64 {
        match self {
            Expression::Paren(expr) => expr.evaluate(),
            Expression::Add(left, right) => left.evaluate() + right.evaluate(),
            Expression::Mul(left, right) => left.evaluate() * right.evaluate(),
            Expression::Num(num) => *num
        }
    }
    fn transformed_to_advanced(&self) -> Expression {
        match self {
            Expression::Paren(expr) => { 
                let expr = expr.transformed_to_advanced();
                Expression::Paren(Box::new(expr))
            },
            Expression::Mul(left, right) => { 
                let left = left.transformed_to_advanced();
                let right = right.transformed_to_advanced();
                Expression::Mul(Box::new(left), Box::new(right))
            },
            Expression::Num(num) => Expression::Num(*num),
            Expression::Add(left, right) => {
                if let Expression::Mul(ml, mr) = &**left {
                    let ml = ml.transformed_to_advanced();
                    let mr = mr.transformed_to_advanced();
                    let right = right.transformed_to_advanced();
                    Expression::Mul(Box::new(ml), Box::new(Expression::Add(Box::new(mr), Box::new(right))))
                } else if let Expression::Mul(ml, mr) = &**right {
                    let ml = ml.transformed_to_advanced();
                    let mr = mr.transformed_to_advanced();
                    let left = left.transformed_to_advanced();
                    Expression::Mul(Box::new(ml), Box::new(Expression::Add(Box::new(mr), Box::new(left))))
                } else {
                    let right = right.transformed_to_advanced();
                    let left = left.transformed_to_advanced();
                    Expression::Add(Box::new(left), Box::new(right))
                }
            }
        }
        //Mul(Add(Add(Add(Mul(Num(4), Num(1)), Num(1)), Num(1)), Num(1)), Num(4))
        //Mul(Mul(Num(4), Add(Num(1), Add(Num(1), Add(Num(1), Num(1))))), Num(4))
    }
    fn evaluate_advanced(&self) -> u64 {
        // TODO: Figure out way to perform transform more efficiently
        // Right now, transformed_to_advanced performs one iteration
        // of a transform that needs to be run multiple times until 
        // the expression is fully transformed, so we cheat by calling
        // it enough times that we'll handle most complex expressions
        let mut transformed = self.transformed_to_advanced();
        for _ in 0..50 {
            transformed = transformed.transformed_to_advanced();
        }
        match transformed {
            Expression::Paren(expr) => expr.evaluate(),
            Expression::Add(left, right) => left.evaluate() + right.evaluate(),
            Expression::Mul(left, right) => left.evaluate() * right.evaluate(),
            Expression::Num(num) => num
        }
    }
}

pub struct Day18 {}

impl Day for Day18 { 
    type Input = Vec<Expression>;
    type Output = u64;

    fn read() -> Vec<Expression> {
        let mut data: Vec<Expression> = vec![];
        let file = File::open("./src/day18/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let line = line.expect("Line must be present");
            let line = line.trim();
            data.push(Expression::from(line));
        }
        data
    }

    fn part1(input: &Vec<Expression>) -> u64 {
        input.iter().fold(0, |acc, i| acc + i.evaluate())
    }

    fn part2(input: &Vec<Expression>) -> u64 {
        input.iter().fold(0, |acc, i| acc + i.evaluate_advanced())
    }
}