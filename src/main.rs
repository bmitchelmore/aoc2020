use std::fmt::Debug;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub trait Day {
    type Input;
    type Output: Debug; 

    fn read() -> Self::Input;
    fn part1(input: &Self::Input) -> Self::Output;
    fn part2(input: &Self::Input) -> Self::Output;
}

fn perform<D: Day>(part: u32) {
    let input = D::read();
    match part {
        1 => {
            let result = D::part1(&input);
            println!("{:?}", result);
        },
        2 => {
            let result = D::part2(&input);
            println!("{:?}", result);
        },
        _ => {
            println!("Unknown part!")
        }
    }
}

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("no pattern given")
        .trim()
        .parse::<u32>()
        .expect("Valid integer required");
    let part = std::env::args()
        .nth(2)
        .expect("no path given")
        .trim()
        .parse::<u32>()
        .expect("Valid integer required");
    match day {
        1 => perform::<day1::Day1>(part),
        2 => perform::<day2::Day2>(part),
        3 => perform::<day3::Day3>(part),
        4 => perform::<day4::Day4>(part),
        5 => perform::<day5::Day5>(part),
        6 => perform::<day6::Day6>(part),
        7 => perform::<day7::Day7>(part),
        8 => perform::<day8::Day8>(part),
        9 => perform::<day9::Day9>(part),
        10 => perform::<day10::Day10>(part),
        11 => perform::<day11::Day11>(part),
        12 => perform::<day12::Day12>(part),
        13 => perform::<day13::Day13>(part),
        14 => perform::<day14::Day14>(part),
        15 => perform::<day15::Day15>(part),
        16 => perform::<day16::Day16>(part),
        17 => perform::<day17::Day17>(part),
        18 => perform::<day18::Day18>(part),
        19 => perform::<day19::Day19>(part),
        20 => perform::<day20::Day20>(part),
        21 => perform::<day21::Day21>(part),
        22 => perform::<day22::Day22>(part),
        23 => perform::<day23::Day23>(part),
        24 => perform::<day24::Day24>(part),
        25 => perform::<day25::Day25>(part),
        _ => println!("Unknown day!")
    }
}
