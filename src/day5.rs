use std::{collections::HashMap, fmt::Debug};

use crate::utils::read_lines_to_vec;

#[derive(Debug)]
struct Map {
    vents: Vec<Line>,
    side_length: i32,
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
        max
    }
    fn generate_points_from_line(line: &Line) -> Vec<Point> {
        let mut points = Vec::new();
        // horizontal
        if line.a.x == line.b.x {
            for y in line.a.y..=line.b.y {
                points.push(Point { x: line.a.x, y })
            }
        } else if line.a.y == line.b.y {
            // vertical
            for x in line.a.x..=line.b.x {
                points.push(Point { x, y: line.a.y })
            }
        } else {
            println!("diagonal lines not supported");
            println!("{:?}", line);
        }
        points
    }
    pub fn determine_overlap(&self) -> HashMap<usize, i32> {
        let mut overlap = HashMap::new();
        for line in self.vents.iter() {
            let points = Map::generate_points_from_line(line);
            for point in points.iter() {
                println!("{:?}", point);
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
        let x: i32 = points.get(0).expect("no x coord").parse().expect("failed to parse");
        let y: i32 = points.get(1).expect("no y coord").parse().expect("failed to parse");
        Point { x, y }
    }
}

pub fn part1() -> i32 {
    let map = Map::from_file("./inputs/day5.txt");
    let overlap = map.determine_overlap();
    println!("{:?}", overlap);
    let mut dangerous = 0;
    for (_k, v) in overlap {
        if v >= 1 {
            dangerous += 1;
        }
    }
    dangerous
}
