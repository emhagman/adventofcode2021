use crate::utils;

enum Direction {
    Forward,
    Down,
    Up,
    Unknown,
}

struct Command(Direction, i32);

pub fn part1() -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let commands = utils::read_lines("./inputs/day2.txt").unwrap();
    for c in commands {
        let command_line = c.unwrap();
        let command_split: Vec<&str> = command_line.split(" ").collect();
        let value: i32 = command_split[1].parse().unwrap();
        let command = Command(direction_from_str(command_split[0]), value);
        match command {
            Command(Direction::Up, d) => depth -= d,
            Command(Direction::Down, d) => depth += d,
            Command(Direction::Forward, p) => position += p,
            _ => panic!("not known"),
        }
    }
    position * depth
}

pub fn part2() -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    let commands = utils::read_lines("./inputs/day2.txt").unwrap();
    for c in commands {
        let command_line = c.unwrap();
        let command_split: Vec<&str> = command_line.split(" ").collect();
        let value: i32 = command_split[1].parse().unwrap();
        let command = Command(direction_from_str(command_split[0]), value);
        match command {
            Command(Direction::Up, d) => aim -= d,
            Command(Direction::Down, d) => aim += d,
            Command(Direction::Forward, p) => {
                position += p;
                depth += aim * p;
            }
            _ => panic!("not known"),
        }
    }
    position * depth
}

fn direction_from_str(dir: &str) -> Direction {
    if dir == "up" {
        Direction::Up
    } else if dir == "down" {
        Direction::Down
    } else if dir == "forward" {
        Direction::Forward
    } else {
        Direction::Unknown
    }
}
