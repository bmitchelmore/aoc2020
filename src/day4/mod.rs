use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use regex::Regex;
use super::Day;

pub struct PassportRecord {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String> // (Country ID)
}

impl PassportRecord {
    fn is_valid_for_part1(&self) -> bool {
        self.byr.is_some() && self.iyr.is_some() && self.eyr.is_some() && self.hgt.is_some() && self.hcl.is_some() && self.ecl.is_some() && self.pid.is_some()
    }
    fn is_valid_for_part2(&self) -> bool {
        match &self.byr {
            Some(byr) => { 
                match byr.parse::<usize>() {
                    Ok(1920..=2002) => (),
                    _ => return false
                }
            }
            None => { 
                return false;
            }
        };
        match &self.iyr {
            Some(iyr) => { 
                match iyr.parse::<usize>() {
                    Ok(2010..=2020) => (),
                    _ => return false
                }
            }
            None => return false
        };
        match &self.eyr {
            Some(eyr) => { 
                match eyr.parse::<usize>() {
                    Ok(2020..=2030) => (),
                    _ => return false
                }
            }
            None => return false
        };
        match &self.hgt {
            Some(hgt) => { 
                let regex = Regex::new(r"^(\d+)(in|cm)$").expect("Regex Invalid!");
                if !regex.is_match(hgt) {
                    return false
                }
                for cap in regex.captures_iter(hgt) {
                    let value = cap[1].parse::<usize>().expect("Height must be integer");
                    match &cap[2] {
                        "in" => match value {
                            59..=76 => (),
                            _ => return false
                        },
                        "cm" => match value {
                            150..=193 => (),
                            _ => return false
                        },
                        _ => panic!("Unexpected!")
                    }
                }
            }
            None => return false
        };
        match &self.hcl {
            Some(hcl) => {
                let regex = Regex::new(r"^#[0-9a-f]{6}$").expect("Regex Invalid!");
                if !regex.is_match(hcl) {
                    return false;
                }
            },
            None => return false
        };
        match &self.ecl {
            Some(ecl) => {
                match ecl.as_str() {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
                    _ => return false
                };
            },
            None => return false
        };
        match &self.pid {
            Some(pid) => {
                let regex = Regex::new(r"^\d{9}$").expect("Regex Invalid!");
                if !regex.is_match(pid) {
                    return false;
                }
            },
            None => return false
        };
        true
    }
    fn read_from<R: Read>(reader: &mut BufReader<R>) -> Option<PassportRecord> {
        let mut working = true;
        let mut result: Option<PassportRecord> = None;
        while working {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => working = false,
                Ok(_count) => {
                    line = line.trim().to_string();
                    if line == "" {
                        working = false;
                    } else {
                        let mut record =  result.unwrap_or(PassportRecord { byr: None, iyr: None, eyr: None, hgt: None, hcl: None, ecl: None, pid: None, cid: None });
                        let parts: Vec<&str> = line.split(' ').collect();
                        for part in parts {
                            let part: Vec<&str> = part.split(':').collect();
                            let field = part.get(0).expect("Field should exist");
                            let value = part.get(1).expect("Value should exist");
                            match *field {
                                "byr" => record.byr = Some(String::from(*value)),
                                "iyr" => record.iyr = Some(String::from(*value)),
                                "eyr" => record.eyr = Some(String::from(*value)),
                                "hgt" => record.hgt = Some(String::from(*value)),
                                "hcl" => record.hcl = Some(String::from(*value)),
                                "ecl" => record.ecl = Some(String::from(*value)),
                                "pid" => record.pid = Some(String::from(*value)),
                                "cid" => record.cid = Some(String::from(*value)),
                                _ => panic!("Invalid: {:?}", field)
                            };
                        }
                        result = Some(record)
                    }
                }, 
                _ => panic!("Invalid input!")
            }
        }
        result
    }
}

pub struct Day4 {}

impl Day for Day4 { 
    type Input = Vec<PassportRecord>;
    type Output = usize;

    fn read() -> Vec<PassportRecord> {
        let mut data: Vec<PassportRecord> = vec![];
        let file = File::open("./src/day4/input").expect("Input file must exist");
        let mut reader = BufReader::new(file);
        while let Some(record) = PassportRecord::read_from(&mut reader) {
            data.push(record);
        }
        data
    }

    fn part1(input: &Vec<PassportRecord>) -> usize {
        input.into_iter().filter(|p| p.is_valid_for_part1()).count()
    }

    fn part2(input: &Vec<PassportRecord>) -> usize {
        input.into_iter().filter(|p| p.is_valid_for_part2()).count()
    }
}