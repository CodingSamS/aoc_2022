use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub mod puzzle01 {
    use crate::{map_value, Rucksack};

    pub fn solve(file_path: &str) -> Result<u32, &str> {
        if let Ok(lines) = crate::read_lines(file_path) {
            let mut sum = 0;
            for line in lines {
                if let Ok(l) = line{
                    let mut rucksack = Rucksack::new();
                    for c in l.chars() {
                        rucksack.add_item(c);
                    }
                    for r1_item in rucksack.first_compartment {
                        if rucksack.second_compartment.contains(&r1_item) {
                            sum += map_value(&r1_item).unwrap();
                            break;
                        }
                    }
                }
            }
            Ok(sum)
        } else {
            Err("no valid file")
        }
    }
}

pub mod puzzle02 {
    use crate::map_value;

    pub fn solve(file_path: &str) -> Result<u32, &str> {
        if let Ok(lines) = crate::read_lines(file_path) {
            let mut sum = 0;
            let mut rucksack_id = 0;
            let mut rucksack_vec: Vec<Vec<char>> = vec![];
            for line in lines {
                if let Ok(l) = line{
                    let mut v: Vec<char> = vec![];
                    for c in l.chars() {
                        v.push(c);
                    }
                    rucksack_vec.push(v);
                    if rucksack_id == 2 {
                        for r0_item in rucksack_vec.get(0).unwrap() {
                            if rucksack_vec.get(1).unwrap().contains(r0_item) {
                                if rucksack_vec.get(2).unwrap().contains(r0_item) {
                                    sum += map_value(r0_item).unwrap();
                                    break;
                                }
                            }
                        }
                        rucksack_id = 0;
                        rucksack_vec.clear();
                    } else {
                        rucksack_id += 1;
                    }
                }
            }
            Ok(sum)
        } else {
            Err("no valid file")
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Rucksack {
    first_compartment: Vec<char>,
    second_compartment: Vec<char>,
    shift: bool
}

impl Rucksack {

    pub fn new() -> Rucksack {
        Rucksack {
            first_compartment: vec![],
            second_compartment: vec![],
            shift: false
        }
    }

    pub fn first_compartment(self) -> Vec<char> {
        self.first_compartment
    }

    pub fn second_compartment(self) -> Vec<char> {
        self.second_compartment
    }

    pub fn add_item(&mut self, c: char) {
        if self.shift {
            self.first_compartment.push(self.second_compartment.pop().unwrap());
            self.second_compartment.insert(0, c);
            self.shift = false;
        } else {
            self.second_compartment.insert(0, c);
            self.shift = true;
        }
    }
}

fn map_value(c: &char) -> Option<u32> {
    match c {
        'a' => Some(1),
        'b' => Some(2),
        'c' => Some(3),
        'd' => Some(4),
        'e' => Some(5),
        'f' => Some(6),
        'g' => Some(7),
        'h' => Some(8),
        'i' => Some(9),
        'j' => Some(10),
        'k' => Some(11),
        'l' => Some(12),
        'm' => Some(13),
        'n' => Some(14),
        'o' => Some(15),
        'p' => Some(16),
        'q' => Some(17),
        'r' => Some(18),
        's' => Some(19),
        't' => Some(20),
        'u' => Some(21),
        'v' => Some(22),
        'w' => Some(23),
        'x' => Some(24),
        'y' => Some(25),
        'z' => Some(26),
        'A' => Some(27),
        'B' => Some(28),
        'C' => Some(29),
        'D' => Some(30),
        'E' => Some(31),
        'F' => Some(32),
        'G' => Some(33),
        'H' => Some(34),
        'I' => Some(35),
        'J' => Some(36),
        'K' => Some(37),
        'L' => Some(38),
        'M' => Some(39),
        'N' => Some(40),
        'O' => Some(41),
        'P' => Some(42),
        'Q' => Some(43),
        'R' => Some(44),
        'S' => Some(45),
        'T' => Some(46),
        'U' => Some(47),
        'V' => Some(48),
        'W' => Some(49),
        'X' => Some(50),
        'Y' => Some(51),
        'Z' => Some(52),
        _ => None
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_puzzle_1() {
        assert_eq!(crate::puzzle01::solve("input_test").unwrap(), 157)
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(crate::puzzle02::solve("input_test").unwrap(), 70)
    }

}
