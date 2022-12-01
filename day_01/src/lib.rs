mod puzzle_01;
mod puzzle_02;

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn solve_puzzle_1(file_path: &str) -> Result<i32, &str> {
    puzzle_01::solve(file_path)
}

pub fn solve_puzzle_2(file_path: &str) -> Result<i32, &str> {
    puzzle_02::solve(file_path)
}