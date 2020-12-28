use std::fs::File;
use std::io::{BufRead, BufReader};
use super::Day;

pub struct Day13 {}

impl Day for Day13 { 
    type Input = (u64, Vec<Option<u64>>);
    type Output = u64;

    fn read() -> (u64, Vec<Option<u64>>) {
        let file = File::open("./src/day13/input").expect("Input file must exist");
        let mut lines = BufReader::new(file).lines();
        let start_time = lines
            .next()
            .expect("Should be line")
            .expect("Should be line")
            .as_str()
            .parse::<u64>()
            .expect("Should be valid integer");
        let buses = lines
            .next()
            .expect("Should be line")
            .expect("Should be line")
            .as_str()
            .split(',')
            .map(|b| b.parse::<u64>().ok())
            .collect();
        (start_time, buses)
    }

    fn part1(input: &(u64, Vec<Option<u64>>)) -> u64 {
        let mut departure = input.0;
        let mut searching = true;
        let mut selected_bus = 0;
        while searching {
            for wrapped in &input.1 {
                if let Some(bus) = wrapped {
                    if departure % bus == 0 {
                        selected_bus = *bus;
                        searching = false;
                    }
                }
            }
            if searching {
                departure += 1;
            }
        }
        (departure - input.0) * selected_bus
    }

    fn part2(input: &(u64, Vec<Option<u64>>)) -> u64 {
        let mut jump = input.1[0].expect("Value");
        let mut departure: u64 = 0;
        let mut searching = true;
        let mut longest_match = 0;
        while searching {
            let mut all_match = true;
            for (index, wrapped) in input.1.iter().enumerate() {
                if let Some(bus) = wrapped {
                    let modded = (departure + index as u64) % bus;
                    match modded {
                        0 => {
                            if index > longest_match {
                                jump = input.1[0..index].iter().filter_map(|o| o.as_ref()).fold(1, |p, i| p * i);
                                longest_match = index;
                            }
                        },
                        _ => {
                            all_match = false;
                            break;
                        }
                    }
                }
            }
            if all_match {
                searching = false;
            } else {
                departure += jump;
            }
        }
        departure
    }
}