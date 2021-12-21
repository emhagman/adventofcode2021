use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_to_vec<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    read_lines(filename).unwrap().map(|f| f.unwrap()).collect()
}
