use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use super::Day;

#[derive(Debug)]
pub enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct HexCoords {
    x: i64,
    y: i64,
    z: i64
}

impl HexCoords {
    fn translate(&self, dir: &Direction) -> HexCoords {
        let mut updated = self.clone();
        match dir {
            Direction::E => { 
                updated.x += 1;
                updated.y -= 1;
            },
            Direction::W => { 
                updated.x -= 1;
                updated.y += 1;
            },
            Direction::NE => {
                updated.x += 1;
                updated.z -= 1;
            },
            Direction::NW => {
                updated.y += 1;
                updated.z -= 1;
            },
            Direction::SE => {
                updated.y -= 1;
                updated.z += 1;
            },
            Direction::SW => {
                updated.x -= 1;
                updated.z += 1;
            },
        };
        updated
    }
    fn all_neighbours(&self) -> Vec<HexCoords> {
        vec![
            self.clone(),
            self.translate(&Direction::E),
            self.translate(&Direction::SE),
            self.translate(&Direction::SW),
            self.translate(&Direction::W),
            self.translate(&Direction::NW),
            self.translate(&Direction::NE),
        ]
    }
}

struct HexGrid {
    grid: HashSet<HexCoords>
}

impl HexGrid {
    fn affected_positions(&self) -> Vec<HexCoords> {
        self.grid.iter().flat_map(|c| c.all_neighbours()).collect()
    }
    fn iterate(&mut self) {
        let mut updated = self.grid.clone();
        for pos in self.affected_positions() {
            let black_tile_count = self.black_tile_count_near(&pos);
            if self.grid.contains(&pos) {
                if black_tile_count == 0 || black_tile_count > 2 {
                    updated.remove(&pos);
                }
            } else {
                if black_tile_count == 2 {
                    updated.insert(pos);
                }
            }
        }
        self.grid = updated;
    }
    fn black_tile_count_near(&self, coords: &HexCoords) -> u64 {
        let mut count = 0;
        for dir in vec![Direction::E, Direction::NE, Direction::NW, Direction::W, Direction::SW, Direction::SE] {
            let pos = coords.translate(&dir);
            if self.grid.contains(&pos) {
                count += 1;
            }
        }
        count
    }
    fn black_tile_count(&self) -> u64 {
        self.grid.len() as u64
    }
}

pub struct Day24 {}

impl Day for Day24 { 
    type Input = Vec<Vec<Direction>>;
    type Output = u64;

    fn read() -> Vec<Vec<Direction>> {
        let mut all = vec![];
        let file = File::open("./src/day24/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let line = line.expect("Should be line");
            let mut line = line.trim().to_string();
            let mut directions = vec![];
            while !line.is_empty() {
                if line.starts_with("se") {
                    directions.push(Direction::SE);
                    line = line.chars().skip(2).collect();
                } else if line.starts_with("sw") {
                    directions.push(Direction::SW);
                    line = line.chars().skip(2).collect();
                } else if line.starts_with("e") {
                    directions.push(Direction::E);
                    line = line.chars().skip(1).collect();
                } else if line.starts_with("ne") {
                    directions.push(Direction::NE);
                    line = line.chars().skip(2).collect();
                } else if line.starts_with("nw") {
                    directions.push(Direction::NW);
                    line = line.chars().skip(2).collect();
                } else if line.starts_with("w") {
                    directions.push(Direction::W);
                    line = line.chars().skip(1).collect();
                } else {
                    panic!("Invalid direction: {}", line);
                }
            }
            all.push(directions);
        }
        all
    }

    fn part1(input: &Vec<Vec<Direction>>) -> u64 {
        let mut flips = HashSet::new();
        for instructions in input {
            let mut pos = (0, 0, 0);
            for step in instructions {
                match step {
                    Direction::E => { 
                        pos.0 += 1;
                        pos.1 -= 1;
                    },
                    Direction::W => { 
                        pos.0 -= 1;
                        pos.1 += 1;
                    },
                    Direction::NE => {
                        pos.0 += 1;
                        pos.2 -= 1;
                    },
                    Direction::NW => {
                        pos.1 += 1;
                        pos.2 -= 1;
                    },
                    Direction::SE => {
                        pos.1 -= 1;
                        pos.2 += 1;
                    },
                    Direction::SW => {
                        pos.0 -= 1;
                        pos.2 += 1;
                    },
                }
            }
            if flips.contains(&pos) {
                flips.remove(&pos);
            } else {
                flips.insert(pos);
            }
        }
        flips.len() as u64
    }

    fn part2(input: &Vec<Vec<Direction>>) -> u64 {
        let mut flips = HashSet::new();
        for instructions in input {
            let mut pos = HexCoords { x: 0, y: 0, z: 0 };
            for dir in instructions {
                pos = pos.translate(dir);
            }
            if flips.contains(&pos) {
                flips.remove(&pos);
            } else {
                flips.insert(pos);
            }
        }
        let mut grid = HexGrid { grid: flips };
        for _ in 0..100 {
            grid.iterate();
        }
        grid.black_tile_count()
    }
}