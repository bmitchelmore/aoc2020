use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use super::Day;

#[derive(Debug, Clone)]
pub struct Food {
    ingredients: HashSet<String>,
    known_allergens: HashSet<String>,
}

pub struct Input {
    foods: Vec<Food>,
}

pub struct Day21 {}

impl Day for Day21 { 
    type Input = Input;
    type Output = u64;

    fn read() -> Input {
        let mut foods: Vec<Food> = vec![];
        let file = File::open("./src/day21/input").expect("Input file must exist");
        for line in BufReader::new(file).lines() {
            let line = line.expect("Line must be present");
            let line = line.trim();
            let food: Food;
            if line.ends_with(")") {
                let parts: Vec<&str> = line.split(" (").collect();
                let ingredients: Vec<String> = parts[0].split(" ").map(|s| s.to_string()).collect();
                let allergens: Vec<String> = parts[1].chars().skip(9).collect::<Vec<char>>().iter().rev().skip(1).rev().collect::<String>().split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
                food = Food {
                    ingredients: HashSet::from_iter(ingredients),
                    known_allergens: HashSet::from_iter(allergens),
                }
            } else {
                let ingredients: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
                food = Food {
                    ingredients: HashSet::from_iter(ingredients),
                    known_allergens: HashSet::new(),
                }
            }
            foods.push(food)
        }
        Input {
            foods: foods,
        }
    }

    fn part1(input: &Input) -> u64 {
        let allergens = input.foods
            .iter()
            .flat_map(|f| f
                .known_allergens
                .iter()
                .collect::<Vec<&String>>())
            .collect::<HashSet<&String>>();
        let mut dangerous = HashSet::new();
        for allergen in allergens {
            let mut candidates = HashSet::new();
            for f in &input.foods {
                if f.known_allergens.contains(allergen) {
                    if candidates.is_empty() {
                        candidates = candidates.union(&f.ingredients).map(|s| s.clone()).collect();
                    } else {
                        candidates = candidates.intersection(&f.ingredients).map(|s| s.clone()).collect();
                    }
                }
            }
            dangerous = dangerous.union(&candidates).map(|s| s.clone()).collect();
        }
        let mut safe = input.foods
            .iter()
            .flat_map(|f| f.ingredients.clone())
            .collect::<HashSet<String>>();
        for i in &dangerous {
            safe.remove(i);
        }
        let mut appearances = 0;
        for f in &input.foods {
            for i in &f.ingredients {
                if safe.contains(i) {
                    appearances += 1;
                }
            }
        }
        appearances
    }

    fn part2(input: &Input) -> u64 {
        let allergens = input.foods
            .iter()
            .flat_map(|f| f
                .known_allergens
                .iter()
                .collect::<Vec<&String>>())
            .collect::<HashSet<&String>>();
        let mut dangerous = HashMap::new();
        for allergen in allergens {
            let mut candidates = HashSet::new();
            for f in &input.foods {
                if f.known_allergens.contains(allergen) {
                    if candidates.is_empty() {
                        candidates = candidates.union(&f.ingredients).map(|s| s.clone()).collect();
                    } else {
                        candidates = candidates.intersection(&f.ingredients).map(|s| s.clone()).collect();
                    }
                }
            }
            dangerous.entry(allergen.clone()).or_insert(candidates);
        }
        let mut allergens: HashMap<String,String> = HashMap::new();
        while !dangerous.is_empty() {
            let mut found = vec![];
            for (allergen, ingredients) in &dangerous {
                if ingredients.len() == 1 {
                    let ingredient = ingredients.iter().collect::<Vec<&String>>().first().unwrap().to_string();
                    found.push(ingredient.clone());
                    allergens.entry(allergen.clone()).or_insert(ingredient);
                    break;
                }
            }
            for i in found {
                let mut empty = vec![];
                for (allergen, ingredients) in dangerous.iter_mut() {
                    ingredients.remove(&i);
                    if ingredients.is_empty() {
                        empty.push(allergen.clone());
                    }
                }
                for a in empty {
                    dangerous.remove(&a);
                }
            }
        }
        let mut sorted = allergens.iter().collect::<Vec<(&String,&String)>>();
        sorted.sort_by(|a,b| a.0.partial_cmp(b.0).unwrap());
        println!("{}", sorted.iter().map(|(_a,i)| i.to_string()).collect::<Vec<String>>().join(","));
        0
    }
}