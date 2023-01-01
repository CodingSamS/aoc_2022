use std::cmp::max;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::iter::{Cycle};
use std::path::Path;
use std::vec::IntoIter;

pub enum Direction {
    Left,
    Right
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        match self {
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right
        }
    }
}

enum StoneType {
    Minus,
    /*
        ####
    */
    Plus,
    /*
         #
        ###
         #
     */
    ReverseL,
    /*
          #
          #
        ###
     */
    Tower,
    /*
        #
        #
        #
        #
     */
    Cube
    /*
        ##
        ##
     */
}

impl Iterator for StoneType {
    type Item = StoneType;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            StoneType::Minus => Some(StoneType::Plus),
            StoneType::Plus => Some(StoneType::ReverseL),
            StoneType::ReverseL => Some(StoneType::Tower),
            StoneType::Tower => Some(StoneType::Cube),
            StoneType::Cube => Some(StoneType::Minus)
        }
    }
}

struct Stone {
    blocked_fields: Vec<(u64, u64)>
}

impl Stone {
    fn new(stone_type: &StoneType, height: u64) -> Self {
        let lowest_y = height + 4;
        let lowest_x = 3;
        let mut blocked_fields = Vec::new();
        match stone_type {
            StoneType::Minus => {
                blocked_fields.push((lowest_x, lowest_y));
                blocked_fields.push((lowest_x + 1, lowest_y));
                blocked_fields.push((lowest_x + 2, lowest_y));
                blocked_fields.push((lowest_x + 3, lowest_y));
            }
            StoneType::Plus => {
                blocked_fields.push((lowest_x + 1, lowest_y));
                blocked_fields.push((lowest_x, lowest_y + 1));
                blocked_fields.push((lowest_x + 1, lowest_y + 1));
                blocked_fields.push((lowest_x + 2, lowest_y + 1));
                blocked_fields.push((lowest_x + 1, lowest_y + 2));
            }
            StoneType::ReverseL => {
                blocked_fields.push((lowest_x, lowest_y));
                blocked_fields.push((lowest_x + 1, lowest_y));
                blocked_fields.push((lowest_x + 2, lowest_y));
                blocked_fields.push((lowest_x + 2, lowest_y + 1));
                blocked_fields.push((lowest_x + 2, lowest_y + 2));
            }
            StoneType::Tower => {
                blocked_fields.push((lowest_x, lowest_y));
                blocked_fields.push((lowest_x, lowest_y + 1));
                blocked_fields.push((lowest_x, lowest_y + 2));
                blocked_fields.push((lowest_x, lowest_y + 3));
            }
            StoneType::Cube => {
                blocked_fields.push((lowest_x, lowest_y));
                blocked_fields.push((lowest_x + 1, lowest_y));
                blocked_fields.push((lowest_x, lowest_y + 1));
                blocked_fields.push((lowest_x + 1, lowest_y + 1));
            }
        };
        Stone {
            blocked_fields
        }
    }

    fn move_horizontal(&self, direction: &Direction) -> Stone {
        let mut blocked_fields_new = Vec::new();
        match direction {
            Direction::Left => {
                for (x, y) in &self.blocked_fields {
                    blocked_fields_new.push((x-1, *y))
                }
            }
            Direction::Right => {
                for (x, y) in &self.blocked_fields {
                    blocked_fields_new.push((x+1, *y));
                }
            }
        }
        Stone {
            blocked_fields: blocked_fields_new
        }
    }

    fn move_down(&self) -> Stone {
        let mut blocked_fields_new = Vec::new();
        for (x, y) in &self.blocked_fields {
            blocked_fields_new.push((*x, y-1));
        }
        Stone {
            blocked_fields: blocked_fields_new
        }
    }
}

pub struct Tetris {
    direction_iter: Cycle<IntoIter<Direction>>,
    room_map: HashMap<(u64, u64), bool>,
    stone_type: StoneType,
    highest_points: HashMap<u64, u64>,
    min_x: u64,
    max_x: u64
}

impl Tetris {
    pub fn new(direction_vec: Vec<Direction>, chamber_width: u64) -> Self {
        let mut room_map = HashMap::new();
        let mut highest_points = HashMap::new();
        for x in 1..=chamber_width {
            room_map.insert((x, 0), true);
            highest_points.insert(x, 0);
        }
        Tetris {
            direction_iter: direction_vec.into_iter().cycle(),
            room_map,
            stone_type: StoneType::Cube,
            highest_points,
            min_x: 0,
            max_x: chamber_width + 1
        }
    }

    pub fn play_tetris(&mut self, number_of_rounds: u64) {
        for x in 1..=number_of_rounds {
            let mut stone = Some(self.generate_stone());
            while stone.is_some() {
                let unwrapped_stone = stone.unwrap();
                stone = self.move_stone(unwrapped_stone);
            }
            if 1000 < x {
                self.clean_up_room(&(x-1000));
            }
        }
    }

    fn generate_stone(&mut self) -> Stone {
        self.stone_type = self.stone_type.next().unwrap();
        Stone::new(&self.stone_type, self.get_height())
    }

    fn move_stone(&mut self, mut stone: Stone) -> Option<Stone> {
        let new_stone = stone.move_horizontal(&self.direction_iter.next().unwrap());
        if self.is_stone_valid(&new_stone) {
            stone = new_stone;
        }
        let new_stone = stone.move_down();
        if self.is_stone_valid(&new_stone) {
            Some(new_stone)
        } else {
            for (x, y) in stone.blocked_fields {
                self.room_map.insert((x, y), true);
                let old_height = *self.highest_points.get(&x).unwrap();
                self.highest_points.insert(x, max(old_height, y));
            }
            // to do: self.clean_up_room();
            None
        }
    }

    fn is_stone_valid(&self, stone: &Stone) -> bool {
        for point in &stone.blocked_fields {
            if self.room_map.contains_key(point) || point.0 == self.min_x || point.0 == self.max_x {
                return false
            }
        }
        true
    }

    pub fn get_height(&self) -> u64 {
        *self.highest_points.values().max().unwrap()
    }

    fn clean_up_room(&mut self, lowest_y: &u64) {
        let mut keys_to_delete = Vec::new();
        for (x, y) in self.room_map.keys() {
            if y < lowest_y {
                keys_to_delete.push((*x, *y));
            }
        }
        for key in keys_to_delete {
            self.room_map.remove(&key);
        }
    }
}

pub fn read_in_file_system(file_path: &str) -> Result<Vec<Direction>, io::Error> {
    let lines = read_lines(file_path)?;
    // Valve GS has flow rate=0; tunnels lead to valves KB, GW
    let mut direction_vec = Vec::new();
    for line in lines {
        if let Ok(l) = line{
            for c in l.chars() {
                if c == '<' {
                    direction_vec.push(Direction::Left);
                } else if c == '>' {
                    direction_vec.push(Direction::Right);
                }
            }
        }
    }
    Ok(direction_vec)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::{read_in_file_system, Tetris};

    #[test]
    fn test_read_in() {
        let direction_vec = read_in_file_system("input_test");
        assert!(direction_vec.is_ok())
    }

    #[test]
    fn test_puzzle_01() {
        let direction_vec = read_in_file_system("input_test").unwrap();
        let mut tetris = Tetris::new(direction_vec, 7);
        tetris.play_tetris(2022);
        assert_eq!(tetris.get_height(), 3068)
    }

    #[test]
    fn test_puzzle_01_split() {
        let direction_vec = read_in_file_system("input_test").unwrap();
        let mut tetris = Tetris::new(direction_vec, 7);
        tetris.play_tetris(1011);
        tetris.play_tetris(1011);
        assert_eq!(tetris.get_height(), 3068)
    }
}