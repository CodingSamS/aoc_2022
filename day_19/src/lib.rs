use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

enum Material {
    Ore,
    Clay,
    Obsidian
}

enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode
}

struct Blueprint {
    id: u32,
    robot_cost: HashMap<Robot, Vec<(Material, u32)>>
}


pub fn read_in_file_system(file_path: &str) -> Result<Vec<Blueprint>, io::Error> {
    let lines = read_lines(file_path)?;
    let line_regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs 3 ore and 7 obsidian.").unwrap();
    let mut blueprint_vec = Vec::new();
    for line in lines {
        if let Ok(l) = line{
            if let Some(cap) = line_regex.captures_iter(&l).next() {
                let blueprint_id: u32 = cap[1].parse().unwrap();

                let ore_robot_ore_cost: u32 = cap[2].parse().unwrap();

                let clay_robot_ore_cost: u32 = cap[3].parse().unwrap();

                let obsidian_robot_ore_cost: u32 = cap[4].parse().unwrap();
                let obsidian_robot_clay_cost: u32 = cap[5].parse().unwrap();

                let geode_robot_ore_cost: u32 = cap[6].parse().unwrap();
                let geode_robot_obsidian_cost: u32 = cap[7].parse().unwrap();


            }
        }
    }
    Ok(blueprint_vec)
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