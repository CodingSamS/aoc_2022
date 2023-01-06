use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

type Point = (u32, u32, u32);

fn manhattan_distance_3d(point1: &Point, point2: &Point) -> u32 {
    point1.0.abs_diff(point2.0) + point1.1.abs_diff(point2.1) + point1.2.abs_diff(point2.2)
}

pub struct CubeGrid {
    cube_grid: HashMap<Point, u32>
}

impl CubeGrid {
    pub fn new(point_vec: Vec<Point>) -> Self {
        let mut cube_grid: HashMap<Point, u32> = HashMap::new();
        for point in point_vec {
            cube_grid.insert(point, 6);
        }
        let mut sub_vec = Vec::new();
        for key in cube_grid.keys() {
            for other_key in cube_grid.keys() {
                if manhattan_distance_3d(key, other_key) == 1 {
                    sub_vec.push(*key);
                }
            }
        }
        for point in sub_vec {
            *cube_grid.get_mut(&point).unwrap() -= 1;
        }
        CubeGrid {
            cube_grid
        }
    }

    pub fn get_surface_area(&self) -> u32 {
        let mut sum = 0;
        for val in self.cube_grid.values() {
            sum += val;
        }
        sum
    }
}

pub fn calc_surface(point_vec: Vec<Point>) -> u32 {
    // calc bounds into hashmap

    // insert point_vec into hashmap for performance

    // set starting point thats out of bounds

    // search from starting bounds with bfs/dfs

    // neighbour search: bounds and visited check w/ vec filters

    // at each point when the search terminates add the number of adjacent surfaces
}

pub fn read_in_file_system(file_path: &str) -> Result<Vec<Point>, io::Error> {
    let lines = read_lines(file_path)?;
    // Valve GS has flow rate=0; tunnels lead to valves KB, GW
    let line_regex = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    let mut point_vec = Vec::new();
    for line in lines {
        if let Ok(l) = line{
            if let Some(cap) = line_regex.captures_iter(&l).next() {
                let x: u32 = cap[1].parse().unwrap();
                let y: u32 = cap[2].parse().unwrap();
                let z: u32 = cap[3].parse().unwrap();
                point_vec.push((x, y, z));
            }
        }
    }
    Ok(point_vec)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::{calc_surface, CubeGrid, fill_points, read_in_file_system};

    #[test]
    fn test_puzzle_01() {
        let point_vec = read_in_file_system("input_test").unwrap();
        let cube_grid = CubeGrid::new(point_vec);
        assert_eq!(cube_grid.get_surface_area(), 64)
    }

    #[test]
    fn test_puzzle_02() {
        let mut point_vec = read_in_file_system("input_test").unwrap();
        assert_eq!(calc_surface(point_vec), 58)
    }
}