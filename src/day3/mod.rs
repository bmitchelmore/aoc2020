use std::fs::File;
use std::io::{BufRead, BufReader};
use super::Day;

pub struct Tile {
    lines: Vec<Vec<bool>> // vec of lines, each line is a row of bools indicating if there is a tree present there
}

#[derive(Clone, Copy)]
struct Pos {
    x: usize,
    y: usize
}

struct Dir {
    x: usize,
    y: usize
}

impl Tile {
    fn has_tree(&self, pos: Pos) -> bool {
        match self.lines.get(pos.y) {
            Some(line) => {
                match line.get(pos.x % line.len()) {
                    Some(val) => *val,
                    None => panic!("Shouldn't happen!")
                }
            },
            None => false
        }
    }
    fn traverse(&self, dir: Dir) -> usize {
        let mut pos = Pos { x: 0, y: 0 };
        let mut hits = 0;
        while pos.y <= self.lines.len() {
            pos.x += dir.x;
            pos.y += dir.y;
            if self.has_tree(pos) {
                hits += 1;
            }
        }
        hits
    }
}

pub struct Day3 {}

impl Day for Day3 {
    type Input = Tile;
    type Output = usize;

    fn read() -> Tile {
        let mut data: Vec<Vec<bool>> = vec![];
        let file = File::open("./src/day3/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let value = line.expect("Line must be present");
            let mut line: Vec<bool> = vec![];
            for c in value.chars() {
                let b = match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Invalid input! {:?}", c)
                };
                line.push(b);
            }
            data.push(line);
        }
        Tile { lines: data }
    }
    
    fn part1(input: &Tile) -> usize {
        input.traverse(Dir { x: 3, y: 1})
    }
    
    fn part2(input: &Tile) -> usize {
        vec![
            input.traverse(Dir { x: 1, y: 1}),
            input.traverse(Dir { x: 3, y: 1}),
            input.traverse(Dir { x: 5, y: 1}),
            input.traverse(Dir { x: 7, y: 1}),
            input.traverse(Dir { x: 1, y: 2})
        ].iter().fold(1, |acc, x| acc * x)
    }
}
