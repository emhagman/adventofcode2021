use crate::utils;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct Lanternfish {
    start_age: u8,
    age: u8,
    parent: bool,
}

impl Lanternfish {
    pub fn from(age: u8) -> Self {
        Lanternfish {
            age,
            start_age: age,
            parent: true,
        }
    }
    pub fn tick(&mut self) -> Option<Lanternfish> {
        if self.age == 0 {
            self.age = 6;
            Some(Lanternfish {
                age: 8,
                start_age: self.start_age,
                parent: false,
            })
        } else {
            self.age -= 1;
            None
        }
    }
}

fn initial_input() -> Vec<u8> {
    let strings = utils::read_lines_to_vec("./inputs/day6.txt");
    let line = strings.first().expect("file is empty");
    line.split(",").map(|f| f.parse().expect("cant parse")).collect()
}

fn simulate(days: i32) -> i32 {
    let initial = initial_input();
    let mut initial_fish: Vec<Lanternfish> = initial.iter().map(|f| Lanternfish::from(*f)).collect();
    for _ in 0..days {
        let mut offspring = initial_fish.iter_mut().filter_map(|f| f.tick()).collect();
        initial_fish.append(&mut offspring);
    }
    initial_fish.len() as i32
}

pub fn part1() -> i32 {
    simulate(80)
}

pub fn part2() -> i64 {
    // simulate_memo(256)
    part2_sourabh(256)
}

fn simulate_memo(days: i32) -> i64 {
    let initial = initial_input();
    let mut initial_fish: Vec<Lanternfish> = initial.iter().map(|f| Lanternfish::from(*f)).collect();
    for _ in 0..days {
        let mut offspring = initial_fish.par_iter_mut().filter_map(|f| f.tick()).collect();
        initial_fish.append(&mut offspring);
    }
    initial_fish.len() as i64
}

pub fn part2_sourabh(days: i32) -> i64 {
    let initial = initial_input();
    let initial_fish: Vec<Lanternfish> = initial.iter().map(|f| Lanternfish::from(*f)).collect();
    let mut count: HashMap<u8, i64> = HashMap::new();
    for fish in initial_fish.iter() {
        let fish_count = count.entry(fish.age).or_insert(0);
        *fish_count += 1;
    }
    let mut new_count = count.clone();
    for _ in 0..days {
        count = new_count.clone();
        new_count = HashMap::new();
        for k in 0..8 {
            if k == 0 {
                let zero_count = count.get(&0).unwrap_or(&0).clone();
                let one_count = count.get(&1).unwrap_or(&0).clone();
                new_count.insert(8, zero_count);
                new_count.insert(6, zero_count);
                new_count.insert(0, one_count);
            } else {
                let next_count = count.get(&(k + 1)).unwrap_or(&0).clone();
                let current_count = new_count.get(&k).unwrap_or(&0).clone();
                new_count.insert(k, next_count + current_count);
            }
        }
    }
    new_count.iter().map(|(_, f)| f).sum()
}
