use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::HashSet;
use super::Day;

#[derive(Clone)]
pub struct ConwaySpace3D {
    space: HashMap<(i32, i32, i32), Cube>
}

#[derive(Clone)]
pub struct ConwaySpace4D {
    space: HashMap<(i32, i32, i32, i32), Cube>
}

#[derive(Clone)]
enum Cube {
    Active,
    Inactive
}

impl ConwaySpace3D {
    fn iterate(&mut self) -> bool {
        let mut next = self.space.clone();
        let mut changed = false;
        let mut affected: HashSet<(i32, i32, i32)> = HashSet::new();
        for coords in self.space.keys() {
            let (x, y, z) = coords;
            for (x, y, z) in self.affected_coords_for(*x, *y, *z) {
                affected.insert((x, y, z));
            }
        }
        for (x, y, z) in affected {
            if self.is_active_at(x, y, z) {
                let active_neighbours = self.active_count_near(x, y, z);
                if active_neighbours >= 2 && active_neighbours <= 3 {
                    // no-op. cube remains active
                } else {
                    match next.entry((x, y, z)) {
                        Entry::Occupied(entry) => { 
                            entry.remove_entry();
                        },
                        Entry::Vacant(_) => ()
                    }
                    changed = true;
                }
            } else {
                let active_neighbours = self.active_count_near(x, y, z);
                if active_neighbours == 3 {
                    match next.entry((x, y, z)) {
                        Entry::Occupied(mut entry) => { 
                            entry.insert(Cube::Active);
                        },
                        Entry::Vacant(entry) => { 
                            entry.insert(Cube::Active);
                        }
                    }
                    changed = true;
                }
            }
        }
        if changed {
            self.space = next;
        }
        changed
    }
    fn affected_coords_for(&self, x: i32, y: i32, z: i32) -> Vec<(i32, i32, i32)> {
        let mut affected = vec![];
        for cx in x-1..=x+1 {
            for cy in y-1..=y+1 {
                for cz in z-1..=z+1 {
                    affected.push((cx, cy, cz));
                }
            }
        }
        affected
    }
    fn active_cube_count(&self) -> u64 {
        self.space.len() as u64
    }
    fn cube_at(&self, x: i32, y: i32, z: i32) -> Option<&Cube> {
        if let Some(cube) = self.space.get(&(x, y, z)) {
            return Some(cube)
        }
        None
    }
    fn is_active_at(&self, x: i32, y: i32, z: i32) -> bool {
        if let Some(cube) = self.cube_at(x, y, z) {
            return *cube == Cube::Active;
        }
        false
    }
    fn active_count_near(&self, x: i32, y: i32, z: i32) -> u32 {
        let mut occupied = 0;
        for (cx, cy, cz) in self.affected_coords_for(x, y, z) {
            if x == cx && y == cy && z == cz {
                continue;
            }
            if self.is_active_at(cx, cy, cz) {
                occupied += 1;
            }
        }
        occupied
    }
}


impl ConwaySpace4D {
    fn from(other: &ConwaySpace3D) -> ConwaySpace4D {
        let mut space: HashMap<(i32, i32, i32, i32), Cube> = HashMap::new();
        for (coords, cube) in &other.space {
            let (x, y, z) = coords;
            space.entry((*x, *y, *z, 0)).or_insert(cube.clone());
        }
        ConwaySpace4D { space: space }
    }
    fn iterate(&mut self) -> bool {
        let mut next = self.space.clone();
        let mut changed = false;
        let mut affected: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        for coords in self.space.keys() {
            let (x, y, z, w) = coords;
            for (x, y, z, w) in self.affected_coords_for(*x, *y, *z, *w) {
                affected.insert((x, y, z, w));
            }
        }
        for (x, y, z, w) in affected {
            if self.is_active_at(x, y, z, w) {
                let active_neighbours = self.active_count_near(x, y, z, w);
                if active_neighbours >= 2 && active_neighbours <= 3 {
                    // no-op. cube remains active
                } else {
                    match next.entry((x, y, z, w)) {
                        Entry::Occupied(entry) => { 
                            entry.remove_entry();
                        },
                        Entry::Vacant(_) => ()
                    }
                    changed = true;
                }
            } else {
                let active_neighbours = self.active_count_near(x, y, z, w);
                if active_neighbours == 3 {
                    match next.entry((x, y, z, w)) {
                        Entry::Occupied(mut entry) => { 
                            entry.insert(Cube::Active);
                        },
                        Entry::Vacant(entry) => { 
                            entry.insert(Cube::Active);
                        }
                    }
                    changed = true;
                }
            }
        }
        if changed {
            self.space = next;
        }
        changed
    }
    fn affected_coords_for(&self, x: i32, y: i32, z: i32, w: i32) -> Vec<(i32, i32, i32, i32)> {
        let mut affected = vec![];
        for cx in x-1..=x+1 {
            for cy in y-1..=y+1 {
                for cz in z-1..=z+1 {
                    for cw in w-1..=w+1 {
                        affected.push((cx, cy, cz, cw));
                    }
                }
            }
        }
        affected
    }
    fn active_cube_count(&self) -> u64 {
        self.space.len() as u64
    }
    fn cube_at(&self, x: i32, y: i32, z: i32, w: i32) -> Option<&Cube> {
        if let Some(cube) = self.space.get(&(x, y, z, w)) {
            return Some(cube)
        }
        None
    }
    fn is_active_at(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        if let Some(cube) = self.cube_at(x, y, z, w) {
            return *cube == Cube::Active;
        }
        false
    }
    fn active_count_near(&self, x: i32, y: i32, z: i32, w: i32) -> u32 {
        let mut occupied = 0;
        for (cx, cy, cz, cw) in self.affected_coords_for(x, y, z, w) {
            if x == cx && y == cy && z == cz && w == cw {
                continue;
            }
            if self.is_active_at(cx, cy, cz, cw) {
                occupied += 1;
            }
        }
        occupied
    }
    
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Cube::Active, Cube::Active) | (Cube::Inactive, Cube::Inactive) => true,
            _ => false
        }
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Cube::Active => '#',
            Cube::Inactive => '.'
        };
        write!(f, "{}", c)
    }
}

pub struct Day17 {}

impl Day for Day17 { 
    type Input = ConwaySpace3D;
    type Output = u64;

    fn read() -> ConwaySpace3D {
        let mut data: HashMap<(i32, i32, i32), Cube> = HashMap::new();
        let file = File::open("./src/day17/input").expect("Input file must exist");
        for (x, line) in BufReader::new(file).lines().enumerate() {
            let line = line.expect("Line must be present");
            for (y, c) in line.trim().chars().enumerate() {
                let cube = match c {
                    '#' => Cube::Active,
                    '.' => Cube::Inactive,
                    _ => panic!("Unexpected cell value: {:?}", c)
                };
                if cube == Cube::Active {
                    data.entry((x as i32, y as i32, 0)).or_insert(cube);
                }
            }
        }
        ConwaySpace3D { space: data }
    }

    fn part1(input: &ConwaySpace3D) -> u64 {
        let mut space = input.clone();
        for _ in 0..6 {
            space.iterate();
        }
        space.active_cube_count()
    }

    fn part2(input: &ConwaySpace3D) -> u64 {
        let mut space = ConwaySpace4D::from(input);
        for _ in 0..6 {
            space.iterate();
        }
        space.active_cube_count()
    }
}