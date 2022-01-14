use crate::utils;
use std::collections::HashMap;

#[derive(Debug)]
struct Lanternfish {
    start_age: i32,
    age: i32,
}

impl Lanternfish {
    pub fn from(age: i32) -> Self {
        Lanternfish { age, start_age: age }
    }
    pub fn tick(&mut self) -> Option<Lanternfish> {
        if self.age == 0 {
            self.age = 6;
            Some(Lanternfish {
                age: 8,
                start_age: self.start_age,
            })
        } else {
            self.age -= 1;
            None
        }
    }
}

fn initial_input() -> Vec<i32> {
    let strings = utils::read_lines_to_vec("./inputs/day6test.txt");
    let line = strings.first().expect("file is empty");
    line.split(",").map(|f| f.parse().expect("cant parse")).collect()
}

fn simulate(days: i32) -> i32 {
    let initial = initial_input();
    let mut initial_fish: Vec<Lanternfish> = initial.iter().map(|f| Lanternfish::from(*f)).collect();
    for day in 0..days {
        let mut offspring = Vec::new();
        for fish in initial_fish.iter_mut() {
            if let Some(new_fish) = fish.tick() {
                offspring.push(new_fish);
            }
        }
        initial_fish.append(&mut offspring);
    }
    initial_fish.len() as i32
}

fn simulate_memo(days: i32, initial_fish: &mut Vec<Lanternfish>) -> (i64, &mut Vec<Lanternfish>) {
    let mut count: HashMap<i32, i64> = HashMap::new();
    for fish in initial_fish.iter() {
        let fish_count = count.entry(fish.age).or_insert(0);
        *fish_count += 1;
    }
    initial_fish.sort_by(|a, b| a.age.cmp(&b.age));
    initial_fish.dedup_by(|a, b| a.age == b.age);
    for _ in 0..days {
        let mut offspring = Vec::new();
        for fish in initial_fish.iter_mut() {
            if let Some(new_fish) = fish.tick() {
                offspring.push(new_fish);
            }
        }
        initial_fish.append(&mut offspring);
    }
    let mut initial_count: HashMap<i32, i64> = HashMap::new();
    for fish in initial_fish.iter() {
        let fish_count = initial_count.entry(fish.start_age).or_insert(0);
        *fish_count += 1;
    }
    let mut total = 0;
    for (k, v) in count.iter() {
        total += v * initial_count.get(&k).unwrap_or(&1).clone();
    }
    (total, initial_fish)
}

pub fn part1() -> i32 {
    simulate(80)
}

pub fn part2() -> i64 {
    let initial = initial_input();
    let mut initial_fish: Vec<Lanternfish> = initial.iter().map(|f| Lanternfish::from(*f)).collect();
    let (count_after_128, fish_after_128) = simulate_memo(128, &mut initial_fish);
    let (count_128_to_256, _) = simulate_memo(128, fish_after_128);
    count_after_128 + count_128_to_256
}

pub fn part2_sourabh() -> i32 {
    let initial = initial_input();
    let initial_fish: Vec<Lanternfish> = initial.iter().map(|f| Lanternfish::from(*f)).collect();
    let mut count: HashMap<i32, i32> = HashMap::new();
    for fish in initial_fish.iter() {
        let fish_count = count.entry(fish.age).or_insert(0);
        *fish_count += 1;
    }
    println!("Initial {:?}", count);
    for day in 0..18 {
        for k in 0..8 {
            let key = 8 - k;
            let current_count = count.get(&key).unwrap_or(&0).clone(); // count[4] = 1
            let next_count = count.get(&(key - 1)).unwrap_or(&0).clone(); // count[3] = 2
            count.insert(key, next_count); // count[4] = count[3] = 2
            if key == 0 {
                count.insert(8, current_count);
                let six_count = count.get(&6).unwrap_or(&0).clone();
                count.insert(6, current_count + six_count);
            }
        }
        println!("{:?} end of day {}", count, day);
    }
    count.iter().map(|(_, f)| f).sum()
}
