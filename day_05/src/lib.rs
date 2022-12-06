use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

pub mod puzzle01 {
    pub fn solve(file_path: &str) -> Result<String, &str> {
        crate::solve_routine(file_path, false)
    }
}

pub mod puzzle02 {
    pub fn solve(file_path: &str) -> Result<String, &str> {
        crate::solve_routine(file_path, true)
    }
}

struct Stack {
    stack_vec: Vec<VecDeque<char>>
}

impl Stack {
    fn new() -> Self {
        Stack {
            stack_vec: Vec::new()
        }
    }

    fn insert_at_index(&mut self, index: usize, value: char) {
        while self.stack_vec.len() <= index {
            self.stack_vec.push(VecDeque::new());
        }
        self.stack_vec[index].push_front(value);
    }

    fn move_crate(&mut self, count: usize, from: usize, to: usize, move_all_at_once: bool) {
        if move_all_at_once {
            let mut temp_stack: VecDeque<char> = VecDeque::new();
            for _ in 0..count {
                temp_stack.push_front(self.stack_vec[from].pop_back().unwrap());
            }
            for item in temp_stack {
                self.stack_vec[to].push_back(item);
            }
        } else {
            for _ in 0..count {
                let from_value = self.stack_vec[from].pop_back().unwrap();
                self.stack_vec[to].push_back(from_value);
            }
        }
    }

    fn get_top_string(self) -> String {
        let mut s = String::new();
        for item in self.stack_vec.iter().map(|v| v.iter().last()).collect::<Vec<_>>() {
            if let Some(c) = item {
                s.push(c.clone());
            }
        }
        s
    }
}

fn solve_routine(file_path: &str, move_all_at_once: bool) -> Result<String, &str> {
    if let Ok(lines) = read_lines(file_path) {
        let mut stack = Stack::new();
        for line in lines {
            if let Ok(l) = line{
                if l.contains('[') {
                    // add items to stack
                    for i in l.match_indices("[").map(|(u, _)| u+1) {
                        let value = l.as_bytes()[i] as char;
                        // string index into vec index since on the top lines there might be spaces
                        let mut index = i-1;
                        if 0 < index {
                            index /= 4;
                        }
                        stack.insert_at_index(index, value);
                    }
                } else if l.contains("move") {
                    // move items between stacks
                    let regex = Regex::new(r"\d+").unwrap();
                    let mut index_list: Vec<usize> = Vec::new();
                    for mat in regex.find_iter(l.as_str()) {
                        let index: usize = mat.as_str().parse().unwrap();
                        index_list.push(index);
                    }
                    if index_list.len() == 3 {
                        stack.move_crate(index_list[0], index_list[1]-1, index_list[2]-1, move_all_at_once)
                    }
                }
            }
        }
        Ok(stack.get_top_string())
    } else {
        Err("no valid file")
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_puzzle_1() {
        assert_eq!(crate::puzzle01::solve("input_test").unwrap(), "CMZ")
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(crate::puzzle02::solve("input_test").unwrap(), "MCD")
    }
}
