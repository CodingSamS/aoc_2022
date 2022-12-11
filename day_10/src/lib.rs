use std::fs;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn read_in_file_system(file_path: &str) -> Result<HashMap<i32, i32>, &str> {
    if let Ok(lines) = read_lines(file_path) {
        let mut cycle_number = 1;
        let mut register_value = 1;
        let mut register_states = HashMap::new();
        for line in lines {
            if let Ok(l) = line{
                let line_vec: Vec<&str> = l.split(' ').collect();
                if line_vec.len() == 1 {
                    // noop
                    register_states.insert(cycle_number, register_value);
                    cycle_number += 1;
                } else if line_vec.len() == 2 {
                    // addx
                    let val: i32 = line_vec[1].parse().unwrap();
                    register_states.insert(cycle_number, register_value);
                    cycle_number += 1;
                    register_states.insert(cycle_number, register_value);
                    cycle_number += 1;
                    register_value += val;
                }
            }
        }
        Ok(register_states)
    } else {
        Err("no valid file")
    }
}

pub fn signal_strength_at_cycle(cycle_number: i32, register_map: &HashMap<i32, i32>) -> Option<i32> {
    match register_map.get(&cycle_number) {
        Some(val) => Some(cycle_number * val),
        None => None
    }
}

pub fn get_signal_strength_sum(cycle_numbers: Vec<i32>, register_map: &HashMap<i32, i32>) -> i32 {
    let mut sum = 0;
    for i in cycle_numbers {
        sum += signal_strength_at_cycle(i, &register_map).unwrap();
    }
    sum
}

pub fn is_cycle_in_sprite(cycle_number: &i32, cycle_pos: &i32, register_map: &HashMap<i32, i32>) -> Option<bool> {
    if let Some(s) = register_map.get(cycle_number) {
        let sprite = vec![s-1, *s, s+1];
        //println!("Sprite: {:?}, Pos: {}", sprite, cycle_pos);
        if sprite.contains(cycle_pos) {
            Some(true)
        } else {
            Some(false)
        }
    } else {
        None
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::{get_signal_strength_sum, read_in_file_system, signal_strength_at_cycle};

    #[test]
    fn test_single_states(){
        let register_states = read_in_file_system("input_test").unwrap();

        assert_eq!(signal_strength_at_cycle(20, &register_states).unwrap(), 420);
        assert_eq!(signal_strength_at_cycle(60, &register_states).unwrap(), 1140);
        assert_eq!(signal_strength_at_cycle(100, &register_states).unwrap(), 1800);
        assert_eq!(signal_strength_at_cycle(140, &register_states).unwrap(), 2940);
        assert_eq!(signal_strength_at_cycle(180, &register_states).unwrap(), 2880);
        assert_eq!(signal_strength_at_cycle(220, &register_states).unwrap(), 3960);

    }

    #[test]
    fn test_sum_1() {
        let register_states = read_in_file_system("input_test").unwrap();
        assert_eq!(get_signal_strength_sum(vec![20, 60, 100, 140, 180, 220], &register_states), 13140);
    }
}