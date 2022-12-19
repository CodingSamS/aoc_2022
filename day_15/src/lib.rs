use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

pub mod puzzle2 {
    use std::cmp::{max, min};
    use std::collections::HashMap;
    use crate::manhattan_distance;

    struct RangesVec {
        ranges: Vec<(i32, i32)>
    }

    impl RangesVec {
        fn new() -> Self {
            RangesVec {
                ranges: Vec::new()
            }
        }

        fn insert_range(&mut self, mut range: (i32, i32)) {
            if self.ranges.len() == 0 {
                self.ranges.push(range);
            } else {
                let mut ranges_new = Vec::new();
                for r in &self.ranges {
                    if (r.0 <= range.0 && range.0 <= r.1) || (r.0 <= range.1 && range.1 <= r.1) {
                        // ranges overlap
                        range.0 = min(r.0, range.0);
                        range.1 = max(r.1, range.1);
                    } else {
                        ranges_new.push((r.0, r.1));
                    }
                }
                ranges_new.push(range);
                self.ranges.clear();
                self.ranges.append(&mut ranges_new);
            }
        }

        fn get_other_end(&self, x_input: &i32) -> Option<i32> {
            for (x, y) in &self.ranges {
                if x_input == x {
                    return Some(*y)
                }
            }
            None
        }
    }

    pub struct BeaconMapPerformance {
        map: HashMap<i32, RangesVec>,
        max_index: i32
    }

    impl BeaconMapPerformance {
        pub fn new(max_index: i32) -> Self {
            let mut map = HashMap::new();
            for i in 0..=max_index {
                map.insert(i, RangesVec::new());
            }
            BeaconMapPerformance {
                map,
                max_index
            }
        }

        pub fn calc_spots(&mut self, sensor_coordinates: (i32, i32), beacon_coordinates: (i32, i32)) {
            let distance = manhattan_distance(&sensor_coordinates, &beacon_coordinates);
            for row in 0..=self.max_index {
                let distance_left = distance - (sensor_coordinates.1 - row).abs();
                if 0 <= distance_left {
                    let x1 = min(max(sensor_coordinates.0 - distance_left, 0), self.max_index);
                    let x2 = min(max(sensor_coordinates.0 + distance_left, 0), self.max_index);
                    self.map.get_mut(&row).unwrap().insert_range((x1, x2));
                }
            }
        }

        pub fn find_distress_signal(&self) -> Option<(i32, i32)> {
            for row in 0..=self.max_index {
                let ranges_vec = self.map.get(&row).unwrap();
                match ranges_vec.get_other_end(&0) {
                    Some(x) => {
                        if x < self.max_index {
                            return Some((x+1, row))
                        }
                    }
                    None => return Some((0, row))
                }
            }
            None
        }
    }
}

pub struct BeaconMap {
    map: HashMap<(i32, i32), bool>,
    distance_function: fn (&(i32, i32), &(i32, i32)) -> i32
}

impl BeaconMap {
    pub fn new(distance_function: fn (&(i32, i32), &(i32, i32)) -> i32) -> Self{
        BeaconMap {
            map: HashMap::new(),
            distance_function
        }
    }

    pub fn calc_spots(&mut self, sensor_coordinates: (i32, i32), beacon_coordinates: (i32, i32)) {
        let max_distance = (self.distance_function)(&sensor_coordinates, &beacon_coordinates);
        self.map.insert(sensor_coordinates, true);
        let mut queue = VecDeque::new();
        queue.push_front(sensor_coordinates);
        while !queue.is_empty() {
            let point = queue.pop_back().unwrap();
            for neighbour in self.get_neighbours(&point) {
                if (self.distance_function)(&sensor_coordinates, &neighbour) <= max_distance {
                    self.map.insert(neighbour, true);
                    queue.push_front(neighbour);
                }
            }
        }
    }

    pub fn calc_spots_manhattan_performance(&mut self, sensor_coordinates: (i32, i32), beacon_coordinates: (i32, i32)) {
        let max_distance = (self.distance_function)(&sensor_coordinates, &beacon_coordinates);
        self.map.entry(sensor_coordinates).or_insert(true);
        for d1 in 0..=max_distance {
            for d2 in 0..=max_distance-d1 {
                let x_pos = sensor_coordinates.0 + d1;
                let x_neg = sensor_coordinates.0 - d1;
                let y_pos = sensor_coordinates.1 + d2;
                let y_neg = sensor_coordinates.1 - d2;
                self.map.entry((x_pos, y_pos)).or_insert(true);
                self.map.entry((x_pos, y_neg)).or_insert(true);
                self.map.entry((x_neg, y_pos)).or_insert(true);
                self.map.entry((x_neg, y_neg)).or_insert(true);
            }
        }
    }

    pub fn find_distress_signal(&self, max_index: i32) -> Option<(i32, i32)> {
        for x in 0..max_index {
            for y in 0..max_index {
                if !self.map.contains_key(&(x, y)) {
                    return Some((x, y))
                }
            }
        }
        None
    }

    pub fn calc_spots_at_row(&mut self, sensor_coordinates: (i32, i32), beacon_coordinates: (i32, i32), row: i32) {
        let max_distance = (self.distance_function)(&sensor_coordinates, &beacon_coordinates);
        let start_point = (sensor_coordinates.0, row);
        if (self.distance_function)(&sensor_coordinates, &start_point) <= max_distance {
            self.map.entry(start_point).or_insert(true);
            let mut neighbour = (sensor_coordinates.0 + 1, row);
            while (self.distance_function)(&sensor_coordinates, &neighbour) <= max_distance {
                self.map.entry(neighbour).or_insert(true);
                neighbour.0 += 1;
            }
            let mut neighbour = (sensor_coordinates.0 - 1, row);
            while (self.distance_function)(&sensor_coordinates, &neighbour) <= max_distance {
                self.map.entry(neighbour).or_insert(true);
                neighbour.0 -= 1;
            }
            self.map.remove(&beacon_coordinates);
        }
    }

    fn get_neighbours(&self, point: &(i32, i32)) -> Vec<(i32, i32)> {
        let mut neighbour_vec = Vec::new();
        let temp_point = (point.0 - 1, point.1);
        if !self.map.contains_key(&temp_point) {
            neighbour_vec.push(temp_point)
        }
        let temp_point = (point.0 + 1, point.1);
        if !self.map.contains_key(&temp_point) {
            neighbour_vec.push(temp_point)
        }
        let temp_point = (point.0, point.1 - 1);
        if !self.map.contains_key(&temp_point) {
            neighbour_vec.push(temp_point)
        }
        let temp_point = (point.0, point.1 + 1);
        if !self.map.contains_key(&temp_point) {
            neighbour_vec.push(temp_point)
        }
        neighbour_vec
    }

    pub fn get_spots_in_row(&self, row: &i32) -> i32 {
        let mut sum = 0;
        for (_, y) in self.map.keys() {
            if row == y {
                sum += 1;
            }
        }
        sum
    }
}

pub fn manhattan_distance(point_1: &(i32, i32), point_2: &(i32, i32)) -> i32 {
    (point_1.0 - point_2.0).abs() + (point_1.1 - point_2.1).abs()
}

pub fn search_distress_signal(max_index: i32, sensor_beacon_vec: &Vec<((i32, i32), (i32, i32))>) -> Option<(i32, i32)> {
    for x in 0..max_index {
        println!("x: {}", x);
        for y in 0..max_index {
            let mut bool = true;
            for (sensor_coordinates, beacon_coordinates) in sensor_beacon_vec {
                if manhattan_distance(&sensor_coordinates, &(x, y)) <= manhattan_distance(&sensor_coordinates, &beacon_coordinates) {
                    bool = false;
                }
            }
            if bool {
                return Some((x, y))
            }
        }
    }
    None
}

pub fn read_in_file_system(file_path: &str) -> Result<Vec<((i32, i32), (i32, i32))>, io::Error> {
    let lines = read_lines(file_path)?;
    let mut sensor_beacon_vec = Vec::new();
    let regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    for line in lines {
        if let Ok(l) = line{
            if let Some(cap) = regex.captures_iter(&l).next() {
                sensor_beacon_vec.push((
                    (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                    (cap[3].parse().unwrap(), cap[4].parse().unwrap())
                ));
            }
        }
    }
    Ok(sensor_beacon_vec)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::{BeaconMap, manhattan_distance, read_in_file_system, search_distress_signal};

    #[test]
    fn test_read() {
        let sensor_beacon_vec = read_in_file_system("input_test");
        assert!(sensor_beacon_vec.is_ok())
    }

    #[test]
    fn test_puzzle_1() {
        let sensor_beacon_vec = read_in_file_system("input_test").unwrap();
        let mut beacon_map = BeaconMap::new(manhattan_distance);
        for (sensor_coordinates, beacon_coordinates) in sensor_beacon_vec {
            beacon_map.calc_spots(sensor_coordinates, beacon_coordinates);
        }
        assert_eq!(beacon_map.get_spots_in_row(&10), 26)
    }

    #[test]
    fn test_puzzle_1_second() {
        let sensor_beacon_vec = read_in_file_system("input_test").unwrap();
        let mut beacon_map = BeaconMap::new(manhattan_distance);
        for (sensor_coordinates, beacon_coordinates) in sensor_beacon_vec {
            beacon_map.calc_spots_at_row(sensor_coordinates, beacon_coordinates, 10);
        }
        assert_eq!(beacon_map.get_spots_in_row(&10), 26)
    }

    #[test]
    fn test_puzzle_2() {
        let sensor_beacon_vec = read_in_file_system("input_test").unwrap();
        assert_eq!(search_distress_signal(20, &sensor_beacon_vec).unwrap(), (14, 11))
    }

    #[test]
    fn test_puzzle_2_performance() {
        let sensor_beacon_vec = read_in_file_system("input_test").unwrap();
        let mut beacon_map = BeaconMap::new(manhattan_distance);
        for (sensor_coordinates, beacon_coordinates) in sensor_beacon_vec {
            beacon_map.calc_spots_manhattan_performance(sensor_coordinates, beacon_coordinates);
        }
        assert_eq!(beacon_map.find_distress_signal(20).unwrap(), (14, 11))
    }

    #[test]
    fn test_puzzle_2_performance_enhanced() {
        let sensor_beacon_vec = read_in_file_system("input_test").unwrap();
        let mut beacon_map_performance = crate::puzzle2::BeaconMapPerformance::new(20);
        for (sensor_coordinates, beacon_coordinates) in sensor_beacon_vec {
            beacon_map_performance.calc_spots(sensor_coordinates, beacon_coordinates);
        }
        assert_eq!(beacon_map_performance.find_distress_signal().unwrap(), (14, 11))
    }
}