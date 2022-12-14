use crate::read_lines;

pub fn solve(file_path: &str) -> Result<i32, &str> {
    if let Ok(lines) = read_lines(file_path) {
        let mut result_value = 0;
        let mut current_value = 0;
        for line in lines {
            if let Ok(l) = line {
                if let Ok(value) = l.parse::<i32>() {
                    current_value += value;
                } else {
                    if result_value < current_value {
                        result_value = current_value;
                    }
                    current_value = 0;
                }
            }
        }
        Ok(result_value)
    } else {
        Err("no valid file")
    }
}


#[cfg(test)]
mod tests {
    use crate::puzzle_01::solve;

    #[test]
    fn test1 () {
        assert_eq!(solve("puzzle01_input_test").unwrap(), 24000);
    }
}
