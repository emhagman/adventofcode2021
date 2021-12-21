use crate::utils;

struct Tile(String, bool);

struct Board {
    tiles: Vec<Tile>,
}

impl Board {
    pub fn new() -> Self {
        Board { tiles: Vec::new() }
    }
    pub fn mark(&mut self, callout: &str) {
        let index = self.tiles.iter().position(|f| f.0 == callout);
        if let Some(i) = index {
            self.tiles.get_mut(i).expect("That tile doesn't exist").1 = true;
        }
    }
    fn index_from_row_col(&self, row: i32, col: i32) -> usize {
        ((row * 5) + col) as usize
    }
    fn is_row_win(&self, row: i32) -> bool {
        let mut win = true;
        for col in 0..5 {
            let index = self.index_from_row_col(row, col);
            let marked = self.tiles.get(index).expect("Not a tile").1;
            if !marked {
                win = false;
            }
        }
        win
    }
    fn is_col_win(&self, col: i32) -> bool {
        let mut win = true;
        for row in 0..5 {
            let index = self.index_from_row_col(row, col);
            let marked = self.tiles.get(index).expect("Not a tile").1;
            if !marked {
                win = false;
            }
        }
        win
    }
    pub fn is_winner(&self) -> bool {
        for row in 0..5 {
            if self.is_row_win(row) {
                return true;
            }
        }
        for col in 0..5 {
            if self.is_col_win(col) {
                return true;
            }
        }
        return false;
    }
    pub fn sum_of_unmarked(&self) -> i32 {
        let mut total = 0;
        for c in &self.tiles {
            if !c.1 {
                total += c.0.parse::<i32>().expect("Can't parse number");
            }
        }
        total
    }
}

pub fn part1() -> i32 {
    let mut lines = utils::read_lines("./inputs/day4.txt").unwrap();

    // grab the callouts for each
    let callout_str = lines.next().unwrap().unwrap();
    let callouts: Vec<&str> = callout_str.split(",").collect();

    // create our boards
    let mut boards: Vec<Board> = Vec::new();
    while let Some(line) = lines.next() {
        let value = line.unwrap();
        if value.is_empty() {
            boards.push(Board::new());
        } else {
            let board = boards.last_mut().expect("No boards exist");
            let mut row: Vec<Tile> = value.split_whitespace().map(|f| Tile(f.to_string(), false)).collect();
            board.tiles.append(&mut row);
        }
    }

    // callout the numbers on each board
    for callout in callouts {
        for b in boards.iter_mut() {
            b.mark(callout);
            if b.is_winner() {
                let sum = b.sum_of_unmarked();
                let call = callout.parse::<i32>().expect("Can't parse callout int");
                return sum * call;
            }
        }
    }
    -1
}
