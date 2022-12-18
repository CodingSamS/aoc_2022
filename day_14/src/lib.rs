use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;


pub fn read_in_file_system(file_path: &str) -> Result<HashMap<(i32, i32), bool>, io::Error> {
    let lines = read_lines(file_path)?;
    let mut cave = HashMap::new();
    for line in lines {
        if let Ok(l) = line{
            let tuple_vec: Vec<&str> = l.split(" -> ").collect();
            for i in 0..tuple_vec.len()-1 {
                let start_point: Vec<i32> = tuple_vec[i].split(",").map(|x| x.parse().unwrap()).collect();
                let end_point: Vec<i32> = tuple_vec[i+1].split(",").map(|x| x.parse().unwrap()).collect();
                for x in min(start_point[0], end_point[0])..=max(start_point[0], end_point[0]) {
                    for y in min(start_point[1], end_point[1])..=max(start_point[1], end_point[1]) {
                        cave.insert((x, y), true);
                    }
                }
            }
        }
    }
    Ok(cave)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub mod cave1 {
    use std::collections::HashMap;

    pub struct Cave {
        cave: HashMap<(i32, i32), bool>,
        bottom: i32,
        sand_start: (i32, i32)
    }

    impl Cave {
        pub fn new(cave: HashMap<(i32, i32), bool>) -> Self {
            let mut bottom = 0;
            for (_, y) in cave.keys() {
                if &bottom < y {
                    bottom = *y;
                }
            }
            Cave {
                cave,
                bottom,
                sand_start: (500, 0)
            }
        }

        fn fall_down(&self, x: i32, y: i32) -> Option<(i32, i32)> {
            if !self.cave.contains_key(&(x, y+1)) {
                Some((x, y+1))
            } else if !self.cave.contains_key(&(x-1, y+1)) {
                Some((x-1, y+1))
            } else if !self.cave.contains_key(&(x+1, y+1)) {
                Some((x+1, y+1))
            } else {
                None
            }
        }

        pub fn pour_in_sand(&mut self) -> bool {
            let mut sand_pos = self.sand_start;
            loop {
                if let Some(point) = self.fall_down(sand_pos.0, sand_pos.1) {
                    sand_pos = point;
                } else {
                    // sand has come to rest
                    self.cave.insert(sand_pos, true);
                    return true
                }
                if self.bottom < sand_pos.1 {
                    // sand flows into the abyss
                    return false
                }
            }
        }
    }
}

pub mod cave2 {
    use std::collections::HashMap;

    pub struct Cave {
        cave: HashMap<(i32, i32), bool>,
        bottom: i32,
        sand_start: (i32, i32)
    }

    impl Cave {
        pub fn new(cave: HashMap<(i32, i32), bool>) -> Self {
            let mut bottom = 0;
            for (_, y) in cave.keys() {
                if &bottom < y {
                    bottom = *y;
                }
            }
            bottom += 2;
            Cave {
                cave,
                bottom,
                sand_start: (500, 0)
            }
        }

        fn fall_down(&self, x: i32, y: i32) -> Option<(i32, i32)> {
            if y == self.bottom - 1 {
                None
            } else if !self.cave.contains_key(&(x, y+1)) {
                Some((x, y+1))
            } else if !self.cave.contains_key(&(x-1, y+1)) {
                Some((x-1, y+1))
            } else if !self.cave.contains_key(&(x+1, y+1)) {
                Some((x+1, y+1))
            } else {
                None
            }
        }

        pub fn pour_in_sand(&mut self) -> bool {
            if self.cave.contains_key(&self.sand_start) {
                return false
            }
            let mut sand_pos = self.sand_start;
            loop {
                if let Some(point) = self.fall_down(sand_pos.0, sand_pos.1) {
                    sand_pos = point;
                } else {
                    // sand has come to rest
                    self.cave.insert(sand_pos, true);
                    return true
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::read_in_file_system;
    use crate::cave1;
    use crate::cave2;

    #[test]
    fn test_read() {
        assert!(read_in_file_system("input_test").is_ok())
    }

    #[test]
    fn test_puzzle_1() {
        let map = read_in_file_system("input_test").unwrap();
        let mut cave = cave1::Cave::new(map);
        let mut sand_counter = 0;
        while cave.pour_in_sand() {
            sand_counter += 1;
        }
        assert_eq!(sand_counter, 24)
    }

    #[test]
    fn test_puzzle_2() {
        let map = read_in_file_system("input_test").unwrap();
        let mut cave = cave2::Cave::new(map);
        let mut sand_counter = 0;
        while cave.pour_in_sand() {
            sand_counter += 1;
        }
        assert_eq!(sand_counter, 93)
    }
}