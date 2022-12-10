use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

struct Point {
    x: i32,
    y: i32
}

pub struct Move {
    x: i32,
    y: i32
}

pub struct Walker {
    snake: Vec<Point>,
    visited_points: HashMap<(i32, i32), bool>
}

impl Walker {
    pub fn new(snake_len: usize) -> Option<Self> {
        if 1 < snake_len {
            let mut snake = Vec::new();
            for _ in 0..snake_len {
                snake.push(Point {x: 0, y: 0});
            }
            Some(Walker {
                snake,
                visited_points: HashMap::from([
                    ((0,0), true)
                ])
            })
        } else {
            None
        }
    }

    pub fn walk(&mut self, m: Move) {
        self.walk_rec(m, 0);
    }

    fn walk_rec(&mut self, m: Move, head_index: usize) {
        self.apply_move(&m, head_index);
        let tail_index = head_index + 1;
        if tail_index < self.snake.len(){
            if self.need_to_move(tail_index) {
                let (x_diff,y_diff) = get_diffs(&self.snake[head_index], &self.snake[tail_index]);
                let mut x_move = 0;
                let mut y_move = 0;
                if y_diff.abs() < x_diff.abs() {
                    x_move = calc_move_val(x_diff);
                    if 0 < y_diff.abs() {
                        y_move = calc_move_val(y_diff);
                    }
                } else {
                    y_move = calc_move_val(y_diff);
                    if 0 < x_diff.abs() {
                        x_move = calc_move_val(x_diff);
                    }
                }
                self.walk_rec(Move {
                    x: x_move,
                    y: y_move
                }, tail_index)
            }
        }
    }

    fn need_to_move(&self, index: usize) -> bool {
        let head = &self.snake[index - 1];
        let tail = &self.snake[index];
        if 1 < (head.x - tail.x).abs() {
            true
        } else if 1 < (head.y - tail.y).abs() {
            true
        } else {
            false
        }
    }

    fn apply_move(&mut self, m: &Move, index: usize) {
        self.snake[index].x += m.x;
        self.snake[index].y += m.y;
        if index == self.snake.len() - 1 {
            self.visited_points.insert((self.snake[index].x, self.snake[index].y), true);
        }
    }

    pub fn tail_get_number_of_visited(&self) -> usize {
        self.visited_points.keys().len()
    }
}

fn map_to_move(direction_string: &str) -> Option<Move>{
    match direction_string {
        "U" => Some(Move {
            x: 0,
            y: 1
        }),
        "D" => Some(Move {
            x: 0,
            y: -1
        }),
        "L" => Some(Move {
            x: -1,
            y: 0
        }),
        "R" => Some(Move {
            x: 1,
            y: 0
        }),
        _ => None
    }
}

pub fn read_in_file_system(file_path: &str) -> Result<Vec<Move>, &str> {
    if let Ok(lines) = read_lines(file_path) {
        let mut move_vec = Vec::new();
        for line in lines {
            if let Ok(l) = line{
                let line_vec: Vec<&str> = l.split(' ').collect();
                if line_vec.len() == 2 {
                    for _ in 0..line_vec[1].parse().unwrap() {
                        move_vec.push(map_to_move(line_vec[0]).unwrap());
                    }
                }
            }
        }
        Ok(move_vec)
    } else {
        Err("no valid file")
    }
}

fn get_diffs(head: &Point, tail: &Point) -> (i32, i32) {
    (head.x - tail.x, head.y - tail.y)
}

fn calc_move_val(val: i32) -> i32 {
    if 0 < val {
        1
    } else {
        -1
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::{read_in_file_system, Walker};

    #[test]
    fn test_puzzle_01(){
        let move_vec = read_in_file_system("input_test").unwrap();
        let mut walker = Walker::new(2).unwrap();
        for m in move_vec {
            walker.walk(m);
        }
        assert_eq!(walker.tail_get_number_of_visited(), 13)
    }

    #[test]
    fn test_puzzle_02(){
        let move_vec = read_in_file_system("input_test").unwrap();
        let mut walker = Walker::new(10).unwrap();
        for m in move_vec {
            walker.walk(m);
        }
        assert_eq!(walker.tail_get_number_of_visited(), 1)
    }

    #[test]
    fn test_puzzle_02_extended(){
        let move_vec = read_in_file_system("input_test_2").unwrap();
        let mut walker = Walker::new(10).unwrap();
        for m in move_vec {
            walker.walk(m);
        }
        assert_eq!(walker.tail_get_number_of_visited(), 36)
    }
}