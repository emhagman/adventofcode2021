use crate::utils;
use std::collections::HashMap;

struct Count(i32, i32);

#[derive(PartialEq)]
enum BitMode {
    MostCommon,
    LeastCommon,
}

impl Default for Count {
    fn default() -> Self {
        Count(0, 0)
    }
}

impl Count {
    fn max(&self) -> i32 {
        if self.0 > self.1 {
            return 0;
        } else {
            return 1;
        }
    }
    fn min(&self) -> i32 {
        if self.0 >= self.1 {
            return 0;
        } else {
            return 1;
        }
    }
}

fn count_rate(lines: &Vec<String>, mode: BitMode) -> String {
    let mut binary_count: HashMap<usize, Count> = HashMap::new();
    for l in lines {
        for (i, c) in l.chars().enumerate() {
            let count = binary_count.entry(i).or_default();
            if c == '0' {
                if mode == BitMode::MostCommon {
                    count.0 += 1;
                } else {
                    count.1 += 1;
                }
            } else {
                if mode == BitMode::MostCommon {
                    count.1 += 1;
                } else {
                    count.0 += 1;
                }
            }
        }
    }
    let mut new_binary_string = String::from("");
    let mut sorted_binary_count: Vec<(&usize, &Count)> = binary_count.iter().collect();
    sorted_binary_count.sort_by_key(|v| v.0);
    for c in sorted_binary_count {
        if mode == BitMode::MostCommon {
            new_binary_string += &c.1.max().to_string();
        } else {
            new_binary_string += &c.1.min().to_string();
        }
    }
    new_binary_string
}

fn rate_from_bit_string(lines: &Vec<String>, mode: BitMode) -> i32 {
    let rate = count_rate(lines, mode);
    bitstring_to_integer(&rate)
}

fn bitstring_to_integer(bitstring: &str) -> i32 {
    let b = isize::from_str_radix(bitstring, 2).expect("Invalid binary string");
    b as i32
}

fn collect_by_nth_bit(lines: &Vec<String>, n: usize, bit: char) -> Vec<String> {
    let mut bit_strings: Vec<String> = Vec::new();
    for l in lines {
        let c: char = l.chars().nth(n).unwrap();
        if c == bit {
            bit_strings.push(l.clone());
        }
    }
    bit_strings
}

fn char_at_nth_bit<'a>(line: &'a str, n: usize) -> char {
    line.chars().nth(n).unwrap()
}

fn oxygen(lines: &Vec<String>) -> i32 {
    let mut lines = lines.clone();
    let bits = lines.get(0).expect("No input").len();
    for i in 0..bits {
        let common_bits = count_rate(&lines, BitMode::MostCommon);
        let common_bit = char_at_nth_bit(&common_bits, i);
        lines = collect_by_nth_bit(&lines, i, common_bit);
        if lines.len() == 1 {
            break;
        }
    }
    let final_line = lines.get(0).unwrap();
    bitstring_to_integer(final_line)
}

fn co2_scruber(lines: &Vec<String>) -> i32 {
    let mut lines = lines.clone();
    let bits = lines.get(0).expect("No input").len();
    for i in 0..bits {
        let common_bits = count_rate(&lines, BitMode::LeastCommon);
        let common_bit = char_at_nth_bit(&common_bits, i);
        lines = collect_by_nth_bit(&lines, i, common_bit);
        if lines.len() == 1 {
            break;
        }
    }
    let final_line = lines.get(0).unwrap();
    bitstring_to_integer(final_line)
}

pub fn part1() -> i32 {
    let lines = utils::read_lines_to_vec("./inputs/day3.txt");
    let gamma = rate_from_bit_string(&lines, BitMode::MostCommon);
    let epsilon = rate_from_bit_string(&lines, BitMode::LeastCommon);
    gamma * epsilon
}

pub fn part2() -> i32 {
    let lines = utils::read_lines_to_vec("./inputs/day3.txt");
    let oxy = oxygen(&lines);
    let co2 = co2_scruber(&lines);
    oxy * co2
}
