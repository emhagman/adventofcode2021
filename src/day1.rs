use crate::utils;

pub fn part1() -> i32 {
    let lines = utils::read_lines("./inputs/day1.txt").unwrap();
    let mut larger = -1;
    let mut previous_depth = 0;
    for l in lines {
        let depth: i32 = l.unwrap().parse().unwrap_or_default();
        if depth > previous_depth {
            larger += 1;
        }
        previous_depth = depth;
    }
    larger
}

pub fn part2() -> i32 {
    let lines = utils::read_lines("./inputs/day1.txt").unwrap();
    let mut larger = -1;
    let line_numbers: Vec<i32> = lines
        .map(|f| {
            let depth: i32 = f.unwrap().parse().unwrap_or_default();
            depth
        })
        .collect();
    let mut window_index = 0;
    let mut previous_depth = 0;
    while window_index < line_numbers.len() - 2 {
        let sum: i32 = (0..3).map(|f| line_numbers.get(window_index + f).unwrap()).sum();
        if sum > previous_depth {
            larger += 1;
        }
        previous_depth = sum;
        window_index += 1;
    }
    larger
}
