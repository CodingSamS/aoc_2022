fn main() {
    let direction_vec = day_17::read_in_file_system("input").unwrap();
    let mut tetris = day_17::Tetris::new(direction_vec, 7);
    tetris.play_tetris(2022);
    println!("Solution 1: {}", tetris.get_height());

    let direction_vec = day_17::read_in_file_system("input").unwrap();
    let mut tetris = day_17::Tetris::new(direction_vec, 7);
    let mut height_vec = Vec::new();
    println!("Playing Tetris");
    for x in 0..1000000000 {
        println!("Round {} of 1000000000", x);
        tetris.play_tetris(1);
        height_vec.push(tetris.get_height());
    }

    match detect_cycle(height_vec) {
        Some(interval) => println!("Cycle found. Interval = {}", interval),
        None => println!("No Cycle found.")
    }
}

fn detect_cycle(value_vec: Vec<u64>) -> Option<usize> {
    let max_interval = value_vec.len() / 5;
    for interval in 1..=max_interval {
        let mut is_cycle = true;
        let mut index = interval - 1;
        let value_first_interval = value_vec[index];
        while is_cycle && index + interval < value_vec.len() {
            let new_index = index + interval;
            if value_vec[new_index] - value_vec[index] != value_first_interval {
                is_cycle = false
            }
            index = new_index;
        }
        if is_cycle {
            return Some(interval)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::detect_cycle;

    #[test]
    fn test_cycle_detection_1() {
        assert_eq!(detect_cycle(vec![1, 5, 500, 503, 507, 1000, 1077, 1092, 1500, 1598, 1610, 2000, 2087, 2390, 2500]), Some(3));
    }

    #[test]
    fn test_cycle_detection_2() {
        assert_eq!(detect_cycle(vec![1, 70, 77, 140, 144, 210, 255, 280, 333, 350]), Some(2));
    }
}
