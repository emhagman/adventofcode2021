use std::{collections::HashMap, fmt::Debug};

use crate::utils::read_lines_to_vec;

#[derive(Debug)]
struct Map {
    vents: Vec<Line>,
    side_length: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CalculationType {
    HorizontalAndVerticalOnly,
    All,
}

impl Map {
    pub fn from_file(filename: &str) -> Self {
        let lines = read_lines_to_vec(filename);
        let mut vents = Vec::new();
        for l in lines {
            let line: Vec<&str> = l.split(" -> ").collect();
            let left_side = line.get(0).expect("left point");
            let right_side = line.get(1).expect("right point");
            let left_point = Point::from(*left_side);
            let right_point = Point::from(*right_side);
            let vent = Line {
                a: left_point,
                b: right_point,
            };
            vents.push(vent);
        }
        let side_length = Map::determine_side_length(&vents);
        Map { vents, side_length }
    }
    fn determine_side_length(vents: &Vec<Line>) -> i32 {
        let mut max = 0;
        for p in vents {
            max = p.a.x.max(max);
            max = p.a.y.max(max);
            max = p.b.x.max(max);
            max = p.b.y.max(max);
        }
        max + 1
    }
    fn generate_points_from_line(line: &Line, calculation_type: CalculationType) -> Vec<Point> {
        let mut points = Vec::new();
        if line.a.x == line.b.x {
            // horizontal
            for y in line.a.y.min(line.b.y)..=line.b.y.max(line.a.y) {
                points.push(Point { x: line.a.x, y })
            }
        } else if line.a.y == line.b.y {
            // vertical
            for x in line.a.x.min(line.b.x)..=line.b.x.max(line.a.x) {
                points.push(Point { x, y: line.a.y })
            }
        } else if calculation_type == CalculationType::All {
            // diagonal
            let min_x = line.a.x.min(line.b.x);
            let max_x = line.a.x.max(line.b.x);
            let min_y = line.a.y.min(line.b.y);
            let max_y = line.a.y.max(line.b.y);
            let mut start_x = if line.a.x < line.b.x { min_x - 1 } else { max_x + 1 };
            let mut start_y = if line.a.y < line.b.y { min_y - 1 } else { max_y + 1 };
            for _i in min_x - 1..max_x {
                let delta_x = if line.a.x < line.b.x { 1 } else { -1 };
                let delta_y = if line.a.y < line.b.y { 1 } else { -1 };
                points.push(Point {
                    x: start_x + delta_x,
                    y: start_y + delta_y,
                });
                start_x += delta_x;
                start_y += delta_y;
            }
        }
        points
    }
    pub fn determine_overlap(&self, calculation_type: CalculationType) -> HashMap<usize, i32> {
        let mut overlap = HashMap::new();
        for line in self.vents.iter() {
            let points = Map::generate_points_from_line(line, calculation_type);
            for point in points.iter() {
                let index = self.index_from_row_col(point.x, point.y);
                if let Some(count) = overlap.get(&index) {
                    let new_count = count + 1;
                    overlap.insert(index, new_count);
                } else {
                    overlap.insert(index, 1);
                }
            }
        }
        overlap
    }
    fn index_from_row_col(&self, row: i32, col: i32) -> usize {
        ((row * self.side_length) + col) as usize
    }
}

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn from(a: &str) -> Self {
        let points: Vec<&str> = a.split(",").collect();
        let x = points.get(0).expect("no x coord").parse().expect("failed to parse");
        let y = points.get(1).expect("no y coord").parse().expect("failed to parse");
        Point { x, y }
    }
}

fn unique_overlap(count: &HashMap<usize, i32>) -> i32 {
    let mut dangerous = 0;
    for (_k, v) in count {
        if v > &1 {
            dangerous += 1;
        }
    }
    dangerous
}

pub fn part1() -> i32 {
    let map = Map::from_file("./inputs/day5.txt");
    let overlap = map.determine_overlap(CalculationType::HorizontalAndVerticalOnly);
    unique_overlap(&overlap)
}

pub fn part2() -> i32 {
    let map = Map::from_file("./inputs/day5.txt");
    let overlap = map.determine_overlap(CalculationType::All);
    unique_overlap(&overlap)
}
