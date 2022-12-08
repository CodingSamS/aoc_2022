use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::slice::Iter;

enum Direction {
    North,
    East,
    South,
    West
}

pub struct Grid {
    rows: Vec<Vec<u32>>,
    cols: Vec<Vec<u32>>
}

impl Grid {
    fn new() -> Self {
        Grid {
            rows: Vec::new(),
            cols: Vec::new()
        }
    }

    fn add_element(&mut self, row_id: usize, col_id: usize, value: u32) {
        while self.rows.len() <= row_id {
            self.rows.push(Vec::new());
        }
        while self.cols.len() <= col_id {
            self.cols.push(Vec::new());
        }
        self.rows[row_id].push(value);
        self.cols[col_id].push(value);
    }

    fn is_visible(&self, row_id: usize, col_id: usize) -> bool {
        let rows_len = self.rows.len();
        let cols_len = self.cols.len();
        if row_id == 0 || row_id == (rows_len - 1) || col_id == 0 || col_id == (cols_len - 1) {
            true
        } else {
            let val = self.rows[row_id][col_id];
            if self.rows[row_id][0..col_id].iter().max().unwrap() < &val ||
                self.rows[row_id][col_id+1..].iter().max().unwrap() < &val ||
                self.cols[col_id][0..row_id].iter().max().unwrap() < &val ||
                self.cols[col_id][row_id+1..].iter().max().unwrap() < &val {
                true
            } else {
                false
            }
        }
    }

    pub fn number_of_visible_elements(&self) -> u32 {
        let mut sum = 0;
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].len() {
                if self.is_visible(i, j) {
                    sum += 1
                }
            }
        }
        sum
    }

    fn view_distance(&self, row_id: usize, col_id: usize, value: u32, direction: Direction) -> u32 {
        let mut sum = 0;
        let mut add_routine = |iterator: Iter<u32>, reverse: bool| {
            if reverse {
                for v in iterator.rev() {
                    sum += 1;
                    if &value <= v {
                        break
                    }
                }
            } else {
                for v in iterator {
                    sum += 1;
                    if &value <= v {
                        break
                    }
                }
            }
            sum
        };
        match direction {
            Direction::North => {
                add_routine(self.cols[col_id][..row_id].iter(), true)
            }
            Direction::South => {
                add_routine(self.cols[col_id][row_id+1..].iter(), false)
            }
            Direction::East => {
                add_routine(self.rows[row_id][col_id+1..].iter(), false)
            }
            Direction::West => {
                add_routine(self.rows[row_id][..col_id].iter(), true)
            }
        }
    }

    fn scenic_score(&self, row_id: usize, col_id: usize) -> u32 {
        let value = self.rows[row_id][col_id];
        self.view_distance(row_id, col_id, value, Direction::North) *
            self.view_distance(row_id, col_id, value, Direction::East) *
            self.view_distance(row_id, col_id, value, Direction::South) *
            self.view_distance(row_id, col_id, value, Direction::West)
    }

    pub fn highest_scenic_score(&self) -> u32 {
        let mut best = 0;
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].len() {
                let score = self.scenic_score(i, j);
                if best < score {
                    best = score;
                }
            }
        }
        best
    }
}

pub fn read_in_file_system(file_path: &str) -> Result<Grid, &str> {
    if let Ok(lines) = read_lines(file_path) {
        let mut line_number = 0;
        let mut grid = Grid::new();
        for line in lines {
            if let Ok(l) = line{
                let mut symbol_number = 0;
                for c in l.chars() {
                    match c.to_digit(10) {
                        Some(value) => {
                            grid.add_element(line_number, symbol_number, value);
                            symbol_number += 1;
                        }
                        None => break
                    }
                }
            }
            line_number += 1;
        }
        Ok(grid)
    } else {
        Err("no valid file")
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::read_in_file_system;

    #[test]
    fn test_visible() {
        let grid = read_in_file_system("input_test").unwrap();
        assert_eq!(grid.number_of_visible_elements(), 21);
    }

    #[test]
    fn test_scenic_score() {
        let grid = read_in_file_system("input_test").unwrap();
        assert_eq!(grid.highest_scenic_score(), 8);
    }
}
