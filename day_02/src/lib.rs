use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn score(symbol: &char) -> Option<u32> {
    match symbol {
        'W' => Some(6),
        'D' => Some(3),
        'L' => Some(0),
        'R' => Some(1), // Rock
        'P' => Some(2), // Paper
        'S' => Some(3), // Scissors
        _ => None
    }
}

fn puzzle1_selection(symbol: &char) -> Option<char> {
    match symbol {
        'X' => Some('R'),
        'Y' => Some('P'),
        'Z' => Some('S'),
        _ => None
    }
}

fn puzzle1_winner(first_symbol: &char, second_symbol: &char) -> Option<char> {
    match (first_symbol, second_symbol) {
        ('A', 'X') => Some('D'),
        ('A', 'Y') => Some('W'),
        ('A', 'Z') => Some('L'),
        ('B', 'X') => Some('L'),
        ('B', 'Y') => Some('D'),
        ('B', 'Z') => Some('W'),
        ('C', 'X') => Some('W'),
        ('C', 'Y') => Some('L'),
        ('C', 'Z') => Some('D'),
        _ => None
    }
}

// Z = need to win, Y = need to draw, X = need to loose
fn puzzle2_selection(first_symbol: &char, second_symbol: &char) -> Option<char> {
    match (first_symbol, second_symbol) {
        ('A', 'X') => Some('S'),
        ('A', 'Y') => Some('R'),
        ('A', 'Z') => Some('P'),
        ('B', 'X') => Some('R'),
        ('B', 'Y') => Some('P'),
        ('B', 'Z') => Some('S'),
        ('C', 'X') => Some('P'),
        ('C', 'Y') => Some('S'),
        ('C', 'Z') => Some('R'),
        _ => None
    }
}

fn puzzle2_winner(symbol: &char) -> Option<char> {
    match symbol {
        'X' => Some('L'),
        'Y' => Some('D'),
        'Z' => Some('W'),
        _ => None
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn solve_puzzle_1(file_path: &str) -> Result<u32, &str> {
    if let Ok(lines) = read_lines(file_path) {
        let mut scores: u32 = 0;
        for line in lines {
            if let Ok(l) = line{
                if 3 == l.len() {
                    let first_char = l.as_bytes()[0] as char;
                    let second_char = l.as_bytes()[2] as char;
                    // points through selection
                    if let Some(c) = puzzle1_selection(&second_char) {
                        if let Some(s) = score(&c) {
                            scores += s;
                        }
                    }
                    // points through winning
                    if let Some(c) = puzzle1_winner(&first_char, &second_char) {
                        if let Some(s) = score(&c) {
                            scores += s;
                        }
                    }
                }
            }
        }
        Ok(scores)
    } else {
        Err("no valid file")
    }
}
pub fn solve_puzzle_2(file_path: &str) -> Result<u32, &str> {
    if let Ok(lines) = read_lines(file_path) {
        let mut scores: u32 = 0;
        for line in lines {
            if let Ok(l) = line{
                if 3 == l.len() {
                    let first_char = l.as_bytes()[0] as char;
                    let second_char = l.as_bytes()[2] as char;
                    // points through selection
                    if let Some(c) = puzzle2_selection(&first_char, &second_char) {
                        if let Some(s) = score(&c) {
                            scores += s;
                        }
                    }
                    // points through winning
                    if let Some(c) = puzzle2_winner(&second_char) {
                        if let Some(s) = score(&c) {
                            scores += s;
                        }
                    }
                }
            }
        }
        Ok(scores)
    } else {
        Err("no valid file")
    }
}


#[cfg(test)]
mod tests {
    use crate::{solve_puzzle_1, solve_puzzle_2};

    #[test]
    fn test_puzzle_1() {
        assert_eq!(solve_puzzle_1("input_test").unwrap(), 15)
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(solve_puzzle_2("input_test").unwrap(), 12)
    }
}
