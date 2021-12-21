use crate::utils;

#[derive(Clone)]
struct Tile(String, bool);

#[derive(Clone)]
struct Board {
    tiles: Vec<Tile>,
}

impl Board {
    pub fn new() -> Self {
        Board { tiles: Vec::new() }
    }
    // pub fn mark(&self, row: i32, col: i32) {
    //     let index = row * col;
    //     // self.tiles.get_mut(index).unwrap().1 = true;
    // }
}

pub fn part1() -> i32 {
    let mut lines = utils::read_lines("./inputs/day4.txt").unwrap();
    let callout: Vec<&str> = lines.next().unwrap().unwrap().split(",").collect();
    let mut boards: Vec<Board> = Vec::new();
    let mut new_board = false;
    let mut board = Board::new();
    while let Some(line) = lines.next() {
        let value = line.unwrap();
        println!("{} {}", value, value.len());
        if value.is_empty() {
            if !new_board {
                println!("board begins");
                new_board = true;
                board = Board::new();
            } else {
                println!("board ends");
                new_board = false;
                boards.push(board.clone());
            }
        } else {
            let mut row: Vec<Tile> = value.split(" ").map(|f| Tile(f.to_string(), false)).collect();
            board.tiles.append(&mut row);
        }
    }
    for b in boards {
        for b in b.tiles {
            println!("{} {}", b.0, b.1);
        }
    }
    0
}
