use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use super::Day;

#[derive(Debug, Clone)]
pub struct Tile {
    id: u64,
    pixels: Vec<Vec<bool>>
}

#[derive(Debug)]
enum TileConnection {
    Left,
    Right,
    Top,
    Bottom,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tile {}:", self.id)?;
        write!(f, "\n")?;
        for row in &self.pixels {
            for pixel in row {
                write!(f, "{}", if *pixel { "#" } else { "." })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn pluck_top(pixels: &Vec<Vec<bool>>) -> Vec<bool> {
    pixels.first().unwrap().clone()
}

fn pluck_bottom(pixels: &Vec<Vec<bool>>) -> Vec<bool> {
    pixels.last().unwrap().clone()
}

fn pluck_left(pixels: &Vec<Vec<bool>>) -> Vec<bool> {
    pixels.iter().map(|v| *v.first().unwrap()).collect::<Vec<bool>>()
}

fn pluck_right(pixels: &Vec<Vec<bool>>) -> Vec<bool> {
    pixels.iter().map(|v| *v.last().unwrap()).collect::<Vec<bool>>()
}

fn rotated(pixels: &Vec<Vec<bool>>, times: u8) -> Vec<Vec<bool>> {
    match times {
        1 => {
            let mut rotated = vec![];
            for _ in pixels {
                rotated.push(vec![]);
            }
            for row in pixels {
                for (col, pixel) in row.iter().enumerate() {
                    rotated[col].push(*pixel)
                }
            }
            rotated
        },
        2 => {
            rotated(&rotated(pixels, 1), 1)
        },
        3 => {
            rotated(&rotated(&rotated(pixels, 1), 1), 1)
        },
        _ => panic!("Invalid turns: {:?}", times)
    }
}

fn flip_vert(pixels: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    pixels.iter().rev().map(|r| r.clone()).collect::<Vec<Vec<bool>>>()
}

fn flip_horiz(pixels: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    pixels.iter().map(|r| r.iter().rev().map(|p| *p).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>()
}

impl Tile {
    fn rotated(&self, times: u8) -> Tile {
        let rotated = rotated(&self.pixels, times);
        Tile {
            id: self.id,
            pixels: rotated
        }
    }
    fn flipped_vert(&self) -> Tile {
        let flipped = flip_vert(&self.pixels);
        Tile {
            id: self.id,
            pixels: flipped
        }
    }
    fn flipped_horiz(&self) -> Tile {
        let flipped = flip_horiz(&self.pixels);
        Tile {
            id: self.id,
            pixels: flipped
        }
    }
    fn current_arrangement(&self) -> Vec<Tile> {
        vec![
            self.clone(),
        ]
    }
    fn all_arrangements(&self) -> Vec<Tile> {
        vec![
            self.clone(),
            self.flipped_vert(),
            self.flipped_horiz(),
            self.rotated(1),
            self.rotated(1).flipped_vert(),
            self.rotated(1).flipped_horiz(),
            self.rotated(1).flipped_horiz().flipped_vert(),
            self.rotated(2),
            self.rotated(2).flipped_vert(),
            self.rotated(2).flipped_horiz(),
            self.rotated(2).flipped_horiz().flipped_vert(),
            self.rotated(3),
            self.rotated(3).flipped_vert(),
            self.rotated(3).flipped_horiz(),
            self.rotated(3).flipped_horiz().flipped_vert(),
        ]
    }
    fn all_edges(&self) -> Vec<Vec<bool>> {
        vec![
            pluck_top(&self.pixels),
            pluck_bottom(&self.pixels),
            pluck_left(&self.pixels),
            pluck_right(&self.pixels),
            pluck_top(&self.pixels).iter().rev().map(|b| *b).collect(),
            pluck_bottom(&self.pixels).iter().rev().map(|b| *b).collect(),
            pluck_left(&self.pixels).iter().rev().map(|b| *b).collect(),
            pluck_right(&self.pixels).iter().rev().map(|b| *b).collect(),
        ]
    }
    fn aligns_with(&self, other: &Tile) -> bool {
        for se in self.all_edges() {
            for oe in other.all_edges() {
                if se == oe {
                    return true;
                }
            }
        }
        false
    }
    fn get_connection(&self, other: &Tile) -> Option<TileConnection> {
        if pluck_top(&self.pixels) == pluck_bottom(&other.pixels) {
            Some(TileConnection::Top)
        } else if pluck_bottom(&self.pixels) == pluck_top(&other.pixels) {
            Some(TileConnection::Bottom)
        } else if pluck_left(&self.pixels) == pluck_right(&other.pixels) {
            Some(TileConnection::Left)
        } else if pluck_right(&self.pixels) == pluck_left(&other.pixels) {
            Some(TileConnection::Right)
        } else {
            None
        }
    }
}

fn find_corners<'a> (tiles: &'a Vec<Tile>) -> Vec<&'a Tile> {
    let mut available: HashSet<u64> = HashSet::from_iter(tiles.iter().map(|t| t.id));
    let mut corners: Vec<&Tile> = vec![];
    while !available.is_empty() {
        for tile in tiles {
            if !available.contains(&tile.id) {
                continue;
            }
            let mut aligned_edges = 0;
            for opt in tiles {
                if tile.id == opt.id {
                    continue;
                }
                if tile.aligns_with(opt) {
                    aligned_edges += 1;
                }
            }
            match aligned_edges {
                2 => {
                    available.remove(&tile.id);
                    println!("Found corner for Tile {}!", tile.id);
                    corners.push(tile);
                },
                3 => { 
                    available.remove(&tile.id);
                    println!("Found edge for Tile {}!", tile.id);
                },
                4 => { 
                    available.remove(&tile.id);
                    println!("Found inner piece for Tile {}!", tile.id);
                },
                _ => println!("Unexpected number of alignments found for Tile {}: {}", tile.id, aligned_edges)
            }
        }
    }
    corners
}

fn combine_tiles(tiles: &Vec<Tile>) -> Tile {
    let mut all_ids: HashSet<u64> = HashSet::from_iter(tiles.iter().map(|p| p.id));
    let tiles: HashMap<u64,Tile> = HashMap::from_iter(tiles.iter().map(|p| (p.id, p.clone())));
    let mut connections: HashMap<u64,HashSet<u64>> = HashMap::new();
    for ti in &all_ids {
        for oi in &all_ids {
            if ti == oi {
                continue;
            }
            let tile = tiles.get(&ti).unwrap().clone();
            let other = tiles.get(&oi).unwrap().clone();
            let tile_alignments = tile.all_arrangements();
            let other_alignments = other.all_arrangements();
            let mut found = false;
            for tile in tile_alignments {
                if found {
                    break;
                }
                for other in &other_alignments {
                    if found {
                        break;
                    }
                    if let Some(_) = tile.get_connection(&other) {
                        connections.entry(tile.id).or_default().insert(other.id);
                        connections.entry(other.id).or_default().insert(tile.id);
                        found = true;
                    }
                }
            }
        }
    }

    let edge_count = connections.iter().filter(|(_, ns)| ns.len() == 3).collect::<Vec<(&u64,&HashSet<u64>)>>().len();
    let width = (edge_count / 4) + 2;
    let height = width;
    let mut plane = vec![];
    for _ in 0..height {
        let mut row = vec![];
        for _ in 0..width {
            row.push(0);
        }
        plane.push(row);
    }

    let mut oriented: HashMap<u64,Tile> = HashMap::new();
    let mut target = *connections.iter().filter(|(_,ns)| ns.len() == 2).map(|(i,_)| *i).collect::<Vec<u64>>().first().unwrap();
    while !all_ids.is_empty() {
        let nids = connections.get(&target).unwrap();
        let tos: Vec<Tile>;
        if let Some(tile) = oriented.get(&target) {
            tos = tile.current_arrangement();
        } else {
            let tile = tiles.get(&target).unwrap().clone();
            tos = tile.all_arrangements();
        }
        for ta in tos {
            let mut found_count = 0;
            let mut top: Option<u64> = None;
            let mut left: Option<u64> = None;
            let mut right: Option<u64> = None;
            let mut bottom: Option<u64> = None;
            for nid in nids {
                let nos: Vec<Tile>;
                if let Some(neighbour) = oriented.get(&nid) {
                    nos = neighbour.current_arrangement();
                } else {
                    let neighbour = tiles.get(&nid).unwrap();
                    nos = neighbour.all_arrangements();
                }
                for na in nos {
                    if let Some(connection) = ta.get_connection(&na) {
                        match connection {
                            TileConnection::Top => {
                                top = Some(na.id);
                                oriented.entry(na.id).or_insert(na.clone());
                            },
                            TileConnection::Left => {
                                left = Some(na.id);
                                oriented.entry(na.id).or_insert(na.clone());
                            },
                            TileConnection::Bottom => {
                                bottom = Some(na.id);
                                oriented.entry(na.id).or_insert(na.clone());
                            },
                            TileConnection::Right => {
                                right = Some(na.id);
                                oriented.entry(na.id).or_insert(na.clone());
                            }
                        }
                        found_count += 1;
                        break;
                    }
                }
            }
            if found_count == nids.len() {
                oriented.entry(ta.id).or_insert(ta.clone());
                match (top, left, bottom, right) {
                    (None, None, Some(_), Some(right)) => {
                        plane[0][0] = ta.id;
                        all_ids.remove(&ta.id);
                        target = right;
                        // top left corner
                    },
                    (None, Some(_), Some(bottom), None) => {
                        plane[0][width-1] = ta.id;
                        all_ids.remove(&ta.id);
                        target = bottom;
                        // top right corner
                    },
                    (Some(_), Some(left), None, None) => {
                        plane[height-1][width-1] = ta.id;
                        all_ids.remove(&ta.id);
                        target = left;
                        // bottom right corner
                    },
                    (Some(top), None, None, Some(_)) => {
                        plane[height-1][0] = ta.id;
                        all_ids.remove(&ta.id);
                        target = top;
                        // bottom left corner
                    },
                    (Some(top), None, Some(bottom), Some(right)) => {
                        let hi = plane.iter().position(|r| r.contains(&bottom)).unwrap();
                        plane[hi - 1][0] = ta.id;
                        all_ids.remove(&ta.id);
                        if all_ids.contains(&top) {
                            target = top;
                        } else {
                            target = right;
                        }
                        // left side piece
                    },
                    (None, Some(left), Some(bottom), Some(right)) => {
                        let wi = plane[0].iter().position(|&p| p == left).unwrap();
                        plane[0][wi + 1] = ta.id;
                        all_ids.remove(&ta.id);
                        if all_ids.contains(&right) {
                            target = right;
                        } else {
                            target = bottom;
                        }
                        // top side piece
                    },
                    (Some(top), Some(left), Some(bottom), None) => {
                        let hi = plane.iter().position(|r| r.contains(&top)).unwrap();
                        let wi = plane[hi].len() - 1;
                        plane[hi + 1][wi] = ta.id;
                        all_ids.remove(&ta.id);
                        if all_ids.contains(&bottom) {
                            target = bottom;
                        } else {
                            target = left;
                        }
                        // right side piece
                    },
                    (Some(top), Some(left), None, Some(right)) => {
                        let hi = plane.len() - 1;
                        let wi = plane[hi].iter().position(|&p| p == right).unwrap();
                        plane[hi][wi - 1] = ta.id;
                        all_ids.remove(&ta.id);
                        if all_ids.contains(&left) {
                            target = left;
                        } else {
                            target = top;
                        }
                        // bottom side piece
                    },
                    (Some(top), Some(left), Some(bottom), Some(right)) => {
                        let hi = plane.iter().position(|r| r.contains(&right)).unwrap();
                        let wi = plane[hi].iter().position(|&p| p == right).unwrap();
                        plane[hi][wi - 1] = ta.id;
                        all_ids.remove(&ta.id);
                        if all_ids.contains(&top) {
                            target = top;
                        } else if all_ids.contains(&right) {
                            target = right;
                        } else if all_ids.contains(&bottom) {
                            target = bottom;
                        } else if all_ids.contains(&left) {
                            target = left;
                        }
                        // inner piece
                    },
                    _ => {
                        panic!("{:?} {:?} {:?} {:?}", top, left, bottom, right);
                    }
                }
                break;
            }
        }
    }

    let mut data = vec![];
    let height = plane.len() * (tiles.values().next().unwrap().pixels.len() - 2);
    let width = plane[0].len() * (tiles.values().next().unwrap().pixels[0].len() - 2);
    for _ in 0..height {
        let mut row = vec![];
        for _ in 0..width {
            row.push(false);
        }
        data.push(row);
    }
    for (ri, row) in plane.iter().enumerate() {
        for (ci, id) in row.iter().enumerate() {
            let tile = oriented.get(id).unwrap();
            let height = tile.pixels.len() - 2;
            let width = tile.pixels[0].len() - 2;
            for y in 0..height {
                for x in 0..width {
                    data[ri * height + y][ci * width + x] = tile.pixels[1 + y][1 + x];
                }
            }
        }
    }

    Tile {
        id: 0,
        pixels: data
    }
}

fn find_water_roughness(plane: Tile, monster: &Vec<Vec<Option<bool>>>) -> u64 {
    let mh = monster.len();
    let mw = monster[0].len();
    let max_y_offset = plane.pixels.len() - mh;
    let max_x_offset = plane.pixels[0].len() - mw;
    for tile in plane.all_arrangements() {
        let mut found_count = 0;
        for y in 0..max_y_offset {
            for x in 0..max_x_offset {
                let mut monster_found = true;
                for my in 0..mh {
                    for mx in 0..mw {
                        if let Some(mp) = monster[my][mx] {
                            if mp != tile.pixels[y+my][x+mx] {
                                monster_found = false;
                                break;
                            }
                        }
                    }
                }
                if monster_found {
                    found_count += 1;
                }
            }
        }
        if found_count > 0 {
            let all = tile.pixels.iter().fold(0, |acc, row| acc + row.iter().fold(0, |acc, &p| if p { acc + 1 } else { acc }));
            let monster = monster.iter().fold(0, |acc, row| acc + row.iter().fold(0, |acc, &p| if p.is_some() { acc + 1 } else { acc }));
            return all - monster * found_count;
        }
    } 
    0
}

enum ReadState {
    ReadingTileId,
    ReadingPixels
}

pub struct Input {
    tiles: Vec<Tile>,
    monster: Vec<Vec<Option<bool>>>,
}

pub struct Day20 {}

impl Day for Day20 { 
    type Input = Input;
    type Output = u64;

    fn read() -> Input {
        let mut tiles: Vec<Tile> = vec![];
        let file = File::open("./src/day20/input").expect("Input file must exist");
        let mut current_id: u64 = 0;
        let mut current_pixels: Vec<Vec<bool>> = vec![];
        let mut read_state = ReadState::ReadingTileId;
        for line in BufReader::new(file).lines() {
            let line = line.expect("Line must be present");
            let line = line.trim();
            match read_state {
                ReadState::ReadingTileId => {
                    current_id = line
                        .split(' ').skip(1).collect::<String>()
                        .split(':').take(1).collect::<String>()
                        .parse::<u64>()
                        .expect("Should be integer");
                    read_state = ReadState::ReadingPixels;
                },
                ReadState::ReadingPixels => {
                    if line.is_empty() {
                        let tile = Tile {
                            id: current_id,
                            pixels: current_pixels.clone()
                        };
                        tiles.push(tile);
                        current_pixels = vec![];
                        read_state = ReadState::ReadingTileId;
                    } else {
                        current_pixels.push(line.chars().map(|c| {
                            match c {
                                '.' => false,
                                '#' => true,
                                _ => panic!("Invalid char: {:?}", c)
                            }
                        }).collect());
                    }
                }
            }
        }
        let tile = Tile {
            id: current_id,
            pixels: current_pixels
        };
        tiles.push(tile);
        
        let mut monster_pattern: Vec<Vec<Option<bool>>> = vec![];
        for line in BufReader::new(File::open("./src/day20/monster").expect("Input file must exist")).lines() {
            let line = line.expect("Line must be present");
            let values = line.chars().map(|c| {
                match c {
                    '.' => Some(false),
                    '#' => Some(true),
                    ' ' => None,
                    _ => panic!("Invalid char: {:?}", c)
                }
            }).collect();
            monster_pattern.push(values);
        }
        Input {
            tiles: tiles,
            monster: monster_pattern
        }
    }

    fn part1(input: &Input) -> u64 {
        let corners = find_corners(&input.tiles);
        corners.iter().fold(1, |acc, t| acc * t.id)
    }

    fn part2(input: &Input) -> u64 {
        let plane = combine_tiles(&input.tiles);
        find_water_roughness(plane, &input.monster)
    }
}