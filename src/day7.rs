use crate::utils;

fn read_crabs(filename: &str) -> Vec<i32> {
    utils::read_lines_to_vec(filename)
        .get(0)
        .expect("missing first line")
        .split(",")
        .map(|f| f.parse().expect("cant parse as number"))
        .collect()
}

fn calculate_min_fuel<F>(crabs: &Vec<i32>, fuel_fn: F) -> i64
where
    F: Fn(i32) -> i32,
{
    let min = crabs.iter().min().expect("no min");
    let max = crabs.iter().max().expect("no max");
    let mut min_fuel = i32::MAX;
    for i in *min..=*max {
        let mut fuel = 0;
        for c in crabs.iter() {
            let distance = (*c - i).abs();
            let crab_fuel = fuel_fn(distance);
            fuel += crab_fuel;
        }
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel as i64
}

fn fuel_distance(n: i32) -> i32 {
    n
}

fn fuel_arithmetic_mean(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

pub fn part1() -> i64 {
    let crabs = read_crabs("./inputs/day7.txt");
    calculate_min_fuel(&crabs, fuel_distance)
}

pub fn part2() -> i64 {
    let crabs = read_crabs("./inputs/day7.txt");
    calculate_min_fuel(&crabs, fuel_arithmetic_mean)
}
